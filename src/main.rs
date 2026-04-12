mod collection;
mod environment;
mod template;

use clap::Parser;
use collection::CollectionItem;
use environment::Environment;
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about = "OpenCollection CLI Runner")]
struct Args {
    /// The OpenCollection YAML file to execute
    file: PathBuf,

    /// Environment to use
    #[arg(short, long)]
    env: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    // Attempt to load .env file if it exists, without failing if it doesn't
    let _ = dotenvy::dotenv();
    
    let environment = args.env.as_ref().map(|env_name| {
        Environment::load_from_file(&args.file, env_name).unwrap_or_else(|err| {
            eprintln!("Failed to load environment '{}': {}", env_name, err);
            std::process::exit(1);
        })
    });

    let content = fs::read_to_string(&args.file).unwrap_or_else(|err| {
        eprintln!("Failed to read file '{}': {}", args.file.display(), err);
        std::process::exit(1);
    });
    
    let item: CollectionItem = serde_yaml::from_str(&content).unwrap_or_else(|err| {
        eprintln!("Failed to parse YAML spec: {}", err);
        std::process::exit(1);
    });
    
    println!("Loaded Request: {}", item.info.name);
    
    if let Some(http) = item.http {
        let url = template::interpolate(&http.url, environment.as_ref()).unwrap_or_else(|err| {
            eprintln!("URL interpolation error: {}", err);
            std::process::exit(1);
        });
        
        println!("Executing: {} {}", http.method, url);
        let client = reqwest::Client::new();
        
        let method = match http.method.to_uppercase().as_str() {
            "GET" => reqwest::Method::GET,
            "POST" => reqwest::Method::POST,
            "PUT" => reqwest::Method::PUT,
            "DELETE" => reqwest::Method::DELETE,
            "PATCH" => reqwest::Method::PATCH,
            _ => {
                eprintln!("Unsupported HTTP method: {}", http.method);
                std::process::exit(1);
            }
        };

        let mut req = client.request(method, &url);
        
        if let Some(headers) = http.headers {
            for header in headers {
                let key = template::interpolate(&header.name, environment.as_ref()).unwrap_or_else(|err| {
                    eprintln!("Header name interpolation error: {}", err);
                    std::process::exit(1);
                });
                let value = template::interpolate(&header.value, environment.as_ref()).unwrap_or_else(|err| {
                    eprintln!("Header value interpolation error: {}", err);
                    std::process::exit(1);
                });
                req = req.header(key, value);
            }
        }
        
        if let Some(body) = http.body {
            match body.body_type.as_str() {
                "json" | "xml" | "text" | "graphql" => {
                    if let Some(serde_yaml::Value::String(s)) = body.data {
                        let parsed_body = template::interpolate(&s, environment.as_ref()).unwrap_or_else(|err| {
                            eprintln!("Body interpolation error: {}", err);
                            std::process::exit(1);
                        });
                        req = req.body(parsed_body);
                    }
                }
                "form-urlencoded" => {
                    if let Some(serde_yaml::Value::Sequence(seq)) = body.data {
                        let mut params = Vec::new();
                        for item in seq {
                            if let serde_yaml::Value::Mapping(m) = item {
                                if let (Some(serde_yaml::Value::String(k)), Some(serde_yaml::Value::String(v))) = (
                                    m.get(&serde_yaml::Value::String("name".to_string())),
                                    m.get(&serde_yaml::Value::String("value".to_string()))
                                ) {
                                    let key = template::interpolate(k, environment.as_ref()).unwrap_or_else(|err| {
                                        eprintln!("Form key interpolation error: {}", err);
                                        std::process::exit(1);
                                    });
                                    let val = template::interpolate(v, environment.as_ref()).unwrap_or_else(|err| {
                                        eprintln!("Form value interpolation error: {}", err);
                                        std::process::exit(1);
                                    });
                                    params.push((key, val));
                                }
                            }
                        }
                        req = req.form(&params);
                    }
                }
                "multipart-form" => {
                    if let Some(serde_yaml::Value::Sequence(seq)) = body.data {
                        let mut form = reqwest::multipart::Form::new();
                        for item in seq {
                            if let serde_yaml::Value::Mapping(m) = item {
                                if let (Some(serde_yaml::Value::String(k)), Some(serde_yaml::Value::String(v))) = (
                                    m.get(&serde_yaml::Value::String("name".to_string())),
                                    m.get(&serde_yaml::Value::String("value".to_string()))
                                ) {
                                    let key = template::interpolate(k, environment.as_ref()).unwrap_or_else(|err| {
                                        eprintln!("Multipart key interpolation error: {}", err);
                                        std::process::exit(1);
                                    });
                                    let val = template::interpolate(v, environment.as_ref()).unwrap_or_else(|err| {
                                        eprintln!("Multipart value interpolation error: {}", err);
                                        std::process::exit(1);
                                    });
                                    
                                    if val.starts_with("@file(") && val.ends_with(")") {
                                        let file_path_str = &val["@file(".len() .. val.len()-1];
                                        let resolved_path = args.file.parent().unwrap_or(std::path::Path::new("")).join(file_path_str);
                                        
                                        let file = tokio::fs::File::open(&resolved_path).await.unwrap_or_else(|err| {
                                            eprintln!("Failed to open multipart file '{}': {}", resolved_path.display(), err);
                                            std::process::exit(1);
                                        });
                                        
                                        let file_name = resolved_path.file_name().unwrap_or_default().to_string_lossy().to_string();
                                        
                                        let stream = reqwest::Body::from(file);
                                        let part = reqwest::multipart::Part::stream(stream).file_name(file_name);
                                        form = form.part(key, part);
                                    } else {
                                        form = form.text(key, val);
                                    }
                                }
                            }
                        }
                        req = req.multipart(form);
                    }
                }
                _ => {
                    println!("Warning: unsupported body type: {} or missing data", body.body_type);
                }
            }
        }
        
        let response = req.send().await?;
        let status = response.status();
        let text = response.text().await?;
        
        println!("\n=== Response: {} ===", status);
        println!("{}", text);
    } else {
        println!("No HTTP config found in this item.");
    }
    
    Ok(())
}
