use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use aws_sdk_dynamodb::types::AttributeValue;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub server_id: String,
    pub server_name: String,
    pub config_file_path: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateServerRequest {
    pub server_name: String,
    pub config_file_path: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateServerRequest {
    pub config_file_path: Option<String>,
    pub description: Option<String>,
}

impl From<ServerConfig> for AttributeValue {
    fn from(config: ServerConfig) -> Self {
use aws_sdk_dynamodb::types::AttributeValue;
        
        let mut item = std::collections::HashMap::new();
        item.insert("server_id".to_string(), AttributeValue::S(config.server_id));
        item.insert("server_name".to_string(), AttributeValue::S(config.server_name));
        item.insert("config_file_path".to_string(), AttributeValue::S(config.config_file_path));
        item.insert("created_at".to_string(), AttributeValue::S(config.created_at.to_rfc3339()));
        item.insert("updated_at".to_string(), AttributeValue::S(config.updated_at.to_rfc3339()));
        
        if let Some(desc) = config.description {
            item.insert("description".to_string(), AttributeValue::S(desc));
        }
        
        AttributeValue::M(item)
    }
}