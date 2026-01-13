use aws_sdk_dynamodb::types::AttributeValue;
use lambda_http::{Body, Error, Response};
use serde_json::json;

pub async fn handle_list_servers(
    client: &aws_sdk_dynamodb::Client,
    table_name: &str,
) -> Result<Response<Body>, Error> {
    match client.scan().table_name(table_name).send().await {
        Ok(result) => {
            let servers = match result.items {
                Some(items) => {
                    let server_list: Vec<serde_json::Value> = items
                        .into_iter()
                        .map(|item| {
                            let mut server = serde_json::Map::new();
                            if let Some(AttributeValue::S(id)) = item.get("server_id") {
                                server.insert("server_id".to_string(), json!(id));
                            }
                            if let Some(AttributeValue::S(name)) = item.get("server_name") {
                                server.insert("server_name".to_string(), json!(name));
                            }
                            if let Some(AttributeValue::S(config_path)) = item.get("config_file_path") {
                                server.insert("config_file_path".to_string(), json!(config_path));
                            }
                            if let Some(AttributeValue::S(desc)) = item.get("description") {
                                server.insert("description".to_string(), json!(desc));
                            }
                            if let Some(AttributeValue::S(created)) = item.get("created_at") {
                                server.insert("created_at".to_string(), json!(created));
                            }
                            if let Some(AttributeValue::S(updated)) = item.get("updated_at") {
                                server.insert("updated_at".to_string(), json!(updated));
                            }
                            json!(server)
                        })
                        .collect();
                    server_list
                }
                None => Vec::new(),
            };

            Ok(Response::builder()
                .status(200)
                .header("Content-Type", "application/json")
                .body(Body::from(json!({
                    "servers": servers,
                    "count": servers.len()
                }).to_string()))
                .map_err(Box::new)?)
        }
        Err(e) => {
            tracing::error!("Failed to list servers from DynamoDB: {}", e);
            Ok(Response::builder()
                .status(500)
                .header("Content-Type", "application/json")
                .body(Body::from(json!({"error": "Failed to list servers"}).to_string()))
                .map_err(Box::new)?)
        }
    }
}