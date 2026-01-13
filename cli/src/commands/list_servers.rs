use anyhow::Result;
use serde_json::Value;
use reqwest::Client;
use tabled::{Table, Tabled, settings::Style};

#[derive(Tabled)]
struct ServerRow {
    id: String,
    name: String,
    config_path: String,
    description: String,
    created_at: String,
}

pub async fn execute(api_url: &str) -> Result<()> {
    let client = Client::new();
    
    let url = format!("{}/servers", api_url);
    
    println!("Listing all server configurations...");
    
    let response = client
        .get(&url)
        .send()
        .await?;
    
    if response.status().is_success() {
        let result: Value = response.json().await?;
        
        if let Some(servers) = result["servers"].as_array() {
            if servers.is_empty() {
                println!("📋 No servers configured.");
                return Ok(());
            }
            
            let mut rows = Vec::new();
            
            for server in servers {
                rows.push(ServerRow {
                    id: server["server_id"].as_str().unwrap_or("N/A").to_string(),
                    name: server["server_name"].as_str().unwrap_or("N/A").to_string(),
                    config_path: server["config_file_path"].as_str().unwrap_or("N/A").to_string(),
                    description: server["description"].as_str().unwrap_or("").to_string(),
                    created_at: server["created_at"].as_str().unwrap_or("N/A").to_string(),
                });
            }
            
            println!("📋 Server Configurations:");
            println!("{}", Table::new(&rows).with(Style::modern()));
            println!("Total servers: {}", servers.len());
        } else {
            println!("📋 No servers configured.");
        }
    } else {
        let error: Value = response.json().await?;
        anyhow::bail!("Failed to list servers: {}", error["error"]);
    }
    
    Ok(())
}