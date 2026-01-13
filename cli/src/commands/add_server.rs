use anyhow::Result;
use serde_json::json;
use reqwest::Client;

pub async fn execute(
    api_url: &str,
    server: String,
    config_path: String,
    description: Option<String>,
) -> Result<()> {
    let client = Client::new();
    
    let request_body = json!({
        "server_name": server,
        "config_file_path": config_path,
        "description": description
    });
    
    let url = format!("{}/servers", api_url);
    
    println!("Adding server configuration...");
    
    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await?;
    
    if response.status().is_success() {
        let result: serde_json::Value = response.json().await?;
        println!("✅ Server added successfully!");
        println!("Server ID: {}", result["server_id"]);
    } else {
        let error: serde_json::Value = response.json().await?;
        anyhow::bail!("Failed to add server: {}", error["error"]);
    }
    
    Ok(())
}