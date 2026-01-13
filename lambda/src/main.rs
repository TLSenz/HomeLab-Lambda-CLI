use lambda_http::{run, service_fn, Body, Error, Request, Response};
use serde_json::json;
use std::env;

mod handlers;
mod models;

use handlers::{add_server, delete_config, list_servers, update_config};


#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .json()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    let config = aws_config::load_from_env().await;
    let dynamodb_client = aws_sdk_dynamodb::Client::new(&config);

    run(service_fn(|event: Request| {
        function_handler(&dynamodb_client, event)
    }))
    .await
}

pub async fn function_handler(
    dynamodb_client: &aws_sdk_dynamodb::Client,
    event: Request,
) -> Result<Response<Body>, Error> {
    let table_name = env::var("TABLE_NAME").unwrap_or_else(|_| "homelab-servers".to_string());
    
    tracing::info!("Received request: {} {}", event.method(), event.uri().path());

    let method = event.method().clone();
    let path = event.uri().path().to_string();
    let path_parts: Vec<&str> = path.trim_start_matches('/').split('/').collect();
    
    let response = match (method, path_parts.as_slice()) {
        (http::Method::POST, ["servers"]) => {
            add_server::handle_add_server(dynamodb_client, &table_name, event).await
        }
        (http::Method::GET, ["servers"]) => {
            list_servers::handle_list_servers(dynamodb_client, &table_name).await
        }
        (http::Method::PUT, ["servers", server_id]) => {
            update_config::handle_update_config(dynamodb_client, &table_name, server_id, event).await
        }
        (http::Method::DELETE, ["servers", server_id]) => {
            delete_config::handle_delete_config(dynamodb_client, &table_name, server_id).await
        }
        _ => {
            Ok(Response::builder()
                .status(404)
                .header("Content-Type", "application/json")
                .body(Body::from(json!({"error": "Endpoint not found"}).to_string()))
                .map_err(Box::new)?)
        }
    };

    // Add CORS headers
    match response {
        Ok(mut resp) => {
            let headers = resp.headers_mut();
            headers.insert("Access-Control-Allow-Origin", hyper::header::HeaderValue::from_static("*"));
            headers.insert("Access-Control-Allow-Methods", hyper::header::HeaderValue::from_static("GET, POST, PUT, DELETE, OPTIONS"));
            headers.insert("Access-Control-Allow-Headers", hyper::header::HeaderValue::from_static("Content-Type, Authorization"));
            Ok(resp)
        }
        Err(e) => Err(e),
    }
}