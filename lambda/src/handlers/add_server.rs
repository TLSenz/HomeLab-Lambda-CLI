use aws_sdk_dynamodb::types::AttributeValue;
use lambda_http::{Body, Error, Request, Response};
use serde_json::json;
use std::collections::HashMap;
use uuid::Uuid;
use chrono::Utc;

use crate::models::server_config::CreateServerRequest;

pub async fn handle_add_server(
    client: &aws_sdk_dynamodb::Client,
    table_name: &str,
    event: Request,
) -> Result<Response<Body>, Error> {
    let body = match event.body() {
        Body::Empty => {
            return Ok(Response::builder()
                .status(400)
                .header("Content-Type", "application/json")
                .body(Body::from(json!({"error": "Request body is required"}).to_string()))
                .map_err(Box::new)?);
        }
        Body::Text(text) => text,
        _ => {
            return Ok(Response::builder()
                .status(400)
                .header("Content-Type", "application/json")
                .body(Body::from(json!({"error": "Invalid request body"}).to_string()))
                .map_err(Box::new)?);
        }
    };

    let request: CreateServerRequest = match serde_json::from_str(body) {
        Ok(req) => req,
        Err(e) => {
            tracing::error!("Failed to parse request body: {}", e);
            return Ok(Response::builder()
                .status(400)
                .header("Content-Type", "application/json")
                .body(Body::from(json!({"error": "Invalid JSON format"}).to_string()))
                .map_err(Box::new)?);
        }
    };

    let server_id = Uuid::new_v4().to_string();
    let now = Utc::now();

    let mut item = HashMap::new();
    item.insert("server_id".to_string(), AttributeValue::S(server_id.clone()));
    item.insert("server_name".to_string(), AttributeValue::S(request.server_name));
    item.insert("config_file_path".to_string(), AttributeValue::S(request.config_file_path));
    item.insert("created_at".to_string(), AttributeValue::S(now.to_rfc3339()));
    item.insert("updated_at".to_string(), AttributeValue::S(now.to_rfc3339()));

    if let Some(desc) = request.description {
        item.insert("description".to_string(), AttributeValue::S(desc));
    }

    match client
        .put_item()
        .table_name(table_name)
        .set_item(Some(item))
        .send()
        .await
    {
        Ok(_) => {
            tracing::info!("Successfully added server: {}", server_id);
            Ok(Response::builder()
                .status(201)
                .header("Content-Type", "application/json")
                .body(Body::from(json!({
                    "message": "Server added successfully",
                    "server_id": server_id
                }).to_string()))
                .map_err(Box::new)?)
        }
        Err(e) => {
            tracing::error!("Failed to add server to DynamoDB: {}", e);
            Ok(Response::builder()
                .status(500)
                .header("Content-Type", "application/json")
                .body(Body::from(json!({"error": "Failed to add server"}).to_string()))
                .map_err(Box::new)?)
        }
    }
}