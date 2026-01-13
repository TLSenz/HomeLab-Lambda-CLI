use anyhow::Result;
use serde_json::json;
use reqwest::Client;

pub async fn execute(
    api_url: &str,
    id: String,
    config_path: Option<String>,
    description: Option<String>,
) -> Result<()> {
    let client = Client::new();
    
    // Build update request with only provided fields
    let mut request_body = json!({});
    
    if let Some(path) = config_path {
        request_body["config_file_path"] = json!(path);
    }
    
    if let Some(desc) = description {
        request_body["description"] = json!(desc);
    }
    
    // Check if any fields were provided
    if request_body.as_object().unwrap().is_empty() {
        anyhow::bail!("No updates provided. Use --config-path or --description to update.");
    }
    
    let url = format!("{}/servers/{}", api_url, id);
    
    println!("Updating server configuration for ID: {}", id);
    
    let response = client
        .put(&url)
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await?;
    
    if response.status().is_success() {
        let result: serde_json::Value = response.json().await?;
        println!("✅ Server updated successfully!");
        println!("Message: {}", result["message"]);
    } else {
        let error: serde_json::Value = response.json().await?;
        anyhow::bail!("Failed to update server: {}", error["error"]);
    }
    
    Ok(())
}