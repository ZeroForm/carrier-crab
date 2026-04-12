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
