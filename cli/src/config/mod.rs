use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub api_url: Option<String>,
    pub default_timeout_seconds: Option<u64>,
    pub default_region: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub name: String,
    pub config_file_path: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HomelabConfigFile {
    pub servers: Vec<ServerConfig>,
    pub api_url: Option<String>,
    pub timeout_seconds: Option<u64>,
    pub region: Option<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            api_url: None,
            default_timeout_seconds: Some(30),
            default_region: Some("us-east-1".to_string()),
        }
    }
}

pub async fn load_config(config_path: Option<&std::path::Path>) -> Result<AppConfig> {
    if let Some(path) = config_path {
        load_config_from_file(path).await
    } else {
        // Try to load from default locations
        if let Some(home_dir) = dirs::home_dir() {
            let config_paths = [
                home_dir.join(".config/homelab/config.yaml"),
                home_dir.join(".homelab.yaml"),
                PathBuf::from("./homelab.yaml"),
            ];
            
            for path in &config_paths {
                if path.exists() {
                    return load_config_from_file(path).await;
                }
            }
        }
        
        Ok(AppConfig::default())
    }
}

async fn load_config_from_file(path: &std::path::Path) -> Result<AppConfig> {
    let content = tokio::fs::read_to_string(path).await?;
    let config_file: HomelabConfigFile = serde_yaml::from_str(&content)?;
    
    Ok(AppConfig {
        api_url: config_file.api_url,
        default_timeout_seconds: config_file.timeout_seconds,
        default_region: config_file.region,
    })
}