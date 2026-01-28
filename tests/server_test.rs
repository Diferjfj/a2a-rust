//! Server integration tests
//! 
//! This module contains integration tests for the A2A server implementation.

use a2a_rust::a2a::{
    models::*,
    server::{
        apps::jsonrpc::{A2AServerBuilder, ServerConfig},
        context::DefaultServerCallContextBuilder,
        request_handlers::request_handler::MockRequestHandler,
    },
    utils::constants::*,
};
use axum::{
    body::Body,
    http::{Request, StatusCode, Method},
    response::Response,
    Router,
};
use serde_json::json;
use tower::util::ServiceExt;

#[tokio::test]
async fn test_server_builder() {
    let agent_card = create_test_agent_card();
    let request_handler = std::sync::Arc::new(MockRequestHandler::new());
    let context_builder = std::sync::Arc::new(DefaultServerCallContextBuilder);

    let server = A2AServerBuilder::new()
        .with_agent_card(agent_card.clone())
        .with_request_handler(request_handler)
        .with_context_builder(context_builder)
        .build();

    assert!(server.is_ok());
}

#[tokio::test]
async fn test_server_agent_card_endpoint() {
    let agent_card = create_test_agent_card();
    let request_handler = std::sync::Arc::new(MockRequestHandler::new());
    let context_builder = std::sync::Arc::new(DefaultServerCallContextBuilder);

    let config = ServerConfig {
        bind_addr: "127.0.0.1:0".parse().unwrap(), // Use port 0 for random port
        ..Default::default()
    };

    let server = A2AServerBuilder::new()
        .with_agent_card(agent_card.clone())
        .with_request_handler(request_handler)
        .with_context_builder(context_builder)
        .with_config(config)
        .build()
        .unwrap();

    // Build the router for testing
    let router: Router = server.build_router().await;

    // Test agent card endpoint
    let request = Request::builder()
        .method(Method::GET)
        .uri(AGENT_CARD_WELL_KNOWN_PATH)
        .body(Body::empty())
        .unwrap();

    let response: Response = router.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // Extract the body and verify it contains the agent card
    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let response_json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    
    assert_eq!(response_json["name"], agent_card.name);
    assert_eq!(response_json["description"], agent_card.description);
}

#[tokio::test]
async fn test_server_jsonrpc_endpoint() {
    let agent_card = create_test_agent_card();
    let request_handler = std::sync::Arc::new(MockRequestHandler::new());
    let context_builder = std::sync::Arc::new(DefaultServerCallContextBuilder);

    let config = ServerConfig {
        bind_addr: "127.0.0.1:0".parse().unwrap(),
        ..Default::default()
    };

    let server = A2AServerBuilder::new()
        .with_agent_card(agent_card)
        .with_request_handler(request_handler)
        .with_context_builder(context_builder)
        .with_config(config)
        .build()
        .unwrap();

    // Build the router for testing
    let router: Router = server.build_router().await;

    // Test JSON-RPC endpoint with a valid request
    let jsonrpc_request = json!({
        "jsonrpc": "2.0",
        "method": "message/send",
        "params": {
            "message": {
                "kind": "message",
                "messageId": "test-msg-123",
                "role": "user",
                "parts": [
                    {
                        "kind": "text",
                        "text": "Hello, world!"
                    }
                ]
            }
        },
        "id": 1
    });

    let request = Request::builder()
        .method(Method::POST)
        .uri(DEFAULT_RPC_URL)
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&jsonrpc_request).unwrap()))
        .unwrap();

    let response: Response = router.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // Extract the body and verify it's a valid JSON-RPC response
    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let response_json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    
    assert_eq!(response_json["jsonrpc"], "2.0");
    assert_eq!(response_json["id"], 1);
    assert!(response_json["result"].is_object() || response_json["result"].is_string());
}

#[tokio::test]
async fn test_server_jsonrpc_error_handling() {
    let agent_card = create_test_agent_card();
    let request_handler = std::sync::Arc::new(MockRequestHandler::new());
    let context_builder = std::sync::Arc::new(DefaultServerCallContextBuilder);

    let config = ServerConfig {
        bind_addr: "127.0.0.1:0".parse().unwrap(),
        ..Default::default()
    };

    let server = A2AServerBuilder::new()
        .with_agent_card(agent_card)
        .with_request_handler(request_handler)
        .with_context_builder(context_builder)
        .with_config(config)
        .build()
        .unwrap();

    // Build the router for testing
    let router: Router = server.build_router().await;

    // Test JSON-RPC endpoint with invalid JSON
    let request = Request::builder()
        .method(Method::POST)
        .uri(DEFAULT_RPC_URL)
        .header("content-type", "application/json")
        .body(Body::from("{invalid json}"))
        .unwrap();

    let response: Response = router.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // Extract the body and verify it's a JSON-RPC error response
    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let response_json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    
    assert_eq!(response_json["jsonrpc"], "2.0");
    assert!(response_json["error"].is_object());
    assert_eq!(response_json["error"]["code"], -32700); // Parse error
}

#[tokio::test]
async fn test_server_jsonrpc_method_not_found() {
    let agent_card = create_test_agent_card();
    let request_handler = std::sync::Arc::new(MockRequestHandler::new());
    let context_builder = std::sync::Arc::new(DefaultServerCallContextBuilder);

    let config = ServerConfig {
        bind_addr: "127.0.0.1:0".parse().unwrap(),
        ..Default::default()
    };

    let server = A2AServerBuilder::new()
        .with_agent_card(agent_card)
        .with_request_handler(request_handler)
        .with_context_builder(context_builder)
        .with_config(config)
        .build()
        .unwrap();

    // Build the router for testing
    let router: Router = server.build_router().await;

    // Test JSON-RPC endpoint with unknown method
    let jsonrpc_request = json!({
        "jsonrpc": "2.0",
        "method": "unknown/method",
        "params": {},
        "id": 1
    });

    let request = Request::builder()
        .method(Method::POST)
        .uri(DEFAULT_RPC_URL)
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&jsonrpc_request).unwrap()))
        .unwrap();

    let response: Response = router.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // Extract the body and verify it's a JSON-RPC error response
    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let response_json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    
    assert_eq!(response_json["jsonrpc"], "2.0");
    assert!(response_json["error"].is_object());
    assert_eq!(response_json["error"]["code"], -32601); // Method not found
}

#[tokio::test]
async fn test_server_extended_agent_card_endpoint() {
    let mut agent_card = create_test_agent_card();
    agent_card.supports_authenticated_extended_card = Some(true);

    let extended_card = AgentCard::new(
        "Extended Test Agent".to_string(),
        "An extended test agent".to_string(),
        "http://localhost:8080".to_string(),
        "1.0.0".to_string(),
        vec!["text/plain".to_string()],
        vec!["text/plain".to_string()],
        AgentCapabilities::new(),
        vec![],
    );

    let request_handler = std::sync::Arc::new(MockRequestHandler::new());
    let context_builder = std::sync::Arc::new(DefaultServerCallContextBuilder);

    let config = ServerConfig {
        bind_addr: "127.0.0.1:0".parse().unwrap(),
        ..Default::default()
    };

    let server = A2AServerBuilder::new()
        .with_agent_card(agent_card)
        .with_request_handler(request_handler)
        .with_context_builder(context_builder)
        .with_extended_agent_card(extended_card.clone())
        .with_config(config)
        .build()
        .unwrap();

    // Build the router for testing
    let router: Router = server.build_router().await;

    // Test extended agent card endpoint
    let request = Request::builder()
        .method(Method::GET)
        .uri(EXTENDED_AGENT_CARD_PATH)
        .body(Body::empty())
        .unwrap();

    let response: Response = router.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // Extract the body and verify it contains the extended agent card
    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let response_json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    
    assert_eq!(response_json["name"], extended_card.name);
    assert_eq!(response_json["description"], extended_card.description);
}

fn create_test_agent_card() -> AgentCard {
    AgentCard::new(
        "Test Agent".to_string(),
        "A test agent for testing".to_string(),
        "http://localhost:8080".to_string(),
        "1.0.0".to_string(),
        vec!["text/plain".to_string()],
        vec!["text/plain".to_string()],
        AgentCapabilities::new(),
        vec![],
    )
}
