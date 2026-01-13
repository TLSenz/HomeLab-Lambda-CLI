use anyhow::Result;
use reqwest::Client;

pub async fn execute(api_url: &str, id: String) -> Result<()> {
    let client = Client::new();
    
    let url = format!("{}/servers/{}", api_url, id);
    
    println!("Deleting server configuration for ID: {}", id);
    
    let response = client
        .delete(&url)
        .send()
        .await?;
    
    if response.status().is_success() {
        let result: serde_json::Value = response.json().await?;
        println!("✅ Server deleted successfully!");
        println!("Message: {}", result["message"]);
    } else {
        let error: serde_json::Value = response.json().await?;
        anyhow::bail!("Failed to delete server: {}", error["error"]);
    }
    
    Ok(())
}