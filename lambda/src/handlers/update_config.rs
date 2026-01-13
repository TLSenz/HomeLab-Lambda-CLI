use aws_sdk_dynamodb::types::AttributeValue;
use lambda_http::{Body, Error, Request, Response};
use serde_json::json;
use std::collections::HashMap;
use chrono::Utc;

use crate::models::server_config::UpdateServerRequest;

pub async fn handle_update_config(
    client: &aws_sdk_dynamodb::Client,
    table_name: &str,
    server_id: &str,
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

    let request: UpdateServerRequest = match serde_json::from_str(body) {
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

    // Check if server exists first
    let key = HashMap::from([(
        "server_id".to_string(),
        AttributeValue::S(server_id.to_string()),
    )]);

    match client.get_item().table_name(table_name).set_key(Some(key)).send().await {
        Ok(result) => {
            if result.item.is_none() {
                return Ok(Response::builder()
                    .status(404)
                    .header("Content-Type", "application/json")
                    .body(Body::from(json!({"error": "Server not found"}).to_string()))
                    .map_err(Box::new)?);
            }
        }
        Err(e) => {
            tracing::error!("Failed to check server existence: {}", e);
            return Ok(Response::builder()
                .status(500)
                .header("Content-Type", "application/json")
                .body(Body::from(json!({"error": "Failed to check server"}).to_string()))
                .map_err(Box::new)?);
        }
    }

    // Build update expression
    let mut update_expressions = Vec::new();
    let mut expression_attribute_names = HashMap::new();
    let mut expression_attribute_values = HashMap::new();

    if let Some(config_path) = request.config_file_path {
        update_expressions.push("#config_file_path = :config_file_path");
        expression_attribute_names.insert("#config_file_path".to_string(), "config_file_path".to_string());
        expression_attribute_values.insert(":config_file_path".to_string(), AttributeValue::S(config_path));
    }

    if let Some(description) = request.description {
        update_expressions.push("#description = :description");
        expression_attribute_names.insert("#description".to_string(), "description".to_string());
        expression_attribute_values.insert(":description".to_string(), AttributeValue::S(description));
    }

    update_expressions.push("#updated_at = :updated_at");
    expression_attribute_names.insert("#updated_at".to_string(), "updated_at".to_string());
    expression_attribute_values.insert(":updated_at".to_string(), AttributeValue::S(Utc::now().to_rfc3339()));

    let update_expression = update_expressions.join(", ");

    let update_key = HashMap::from([(
        "server_id".to_string(),
        AttributeValue::S(server_id.to_string()),
    )]);

    match client
        .update_item()
        .table_name(table_name)
        .set_key(Some(update_key))
        .update_expression(update_expression)
        .set_expression_attribute_names(Some(expression_attribute_names))
        .set_expression_attribute_values(Some(expression_attribute_values))
        .send()
        .await
    {
        Ok(_) => {
            tracing::info!("Successfully updated server: {}", server_id);
            Ok(Response::builder()
                .status(200)
                .header("Content-Type", "application/json")
                .body(Body::from(json!({
                    "message": "Server updated successfully",
                    "server_id": server_id
                }).to_string()))
                .map_err(Box::new)?)
        }
        Err(e) => {
            tracing::error!("Failed to update server in DynamoDB: {}", e);
            Ok(Response::builder()
                .status(500)
                .header("Content-Type", "application/json")
                .body(Body::from(json!({"error": "Failed to update server"}).to_string()))
                .map_err(Box::new)?)
        }
    }
}