mod collection;

use clap::Parser;
use collection::CollectionItem;
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about = "OpenCollection CLI Runner")]
struct Args {
    /// The OpenCollection YAML file to execute
    file: PathBuf,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
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
        println!("Executing: {} {}", http.method, http.url);
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

        let mut req = client.request(method, &http.url);
        
        if let Some(headers) = http.headers {
            for header in headers {
                req = req.header(header.name, header.value);
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
