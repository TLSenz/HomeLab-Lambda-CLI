use aws_sdk_dynamodb::types::AttributeValue;
use lambda_http::{Body, Error, Response};
use serde_json::json;
use std::collections::HashMap;

pub async fn handle_delete_config(
    client: &aws_sdk_dynamodb::Client,
    table_name: &str,
    server_id: &str,
) -> Result<Response<Body>, Error> {
    let key = HashMap::from([(
        "server_id".to_string(),
        AttributeValue::S(server_id.to_string()),
    )]);

    match client
        .delete_item()
        .table_name(table_name)
        .set_key(Some(key))
        .send()
        .await
    {
        Ok(_) => {
            tracing::info!("Successfully deleted server: {}", server_id);
            Ok(Response::builder()
                .status(200)
                .header("Content-Type", "application/json")
                .body(Body::from(json!({
                    "message": "Server deleted successfully",
                    "server_id": server_id
                }).to_string()))
                .map_err(Box::new)?)
        }
        Err(e) => {
            tracing::error!("Failed to delete server from DynamoDB: {}", e);
            Ok(Response::builder()
                .status(500)
                .header("Content-Type", "application/json")
                .body(Body::from(json!({"error": "Failed to delete server"}).to_string()))
                .map_err(Box::new)?)
        }
    }
}