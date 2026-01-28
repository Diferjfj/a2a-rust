//! Interoperability tests for A2A Rust client with Python server
//! 
//! This module contains tests that verify the Rust client can successfully
//! communicate with a Python A2A server implementation.

use a2a_rust::a2a::client::config::ClientConfig;
use a2a_rust::a2a::client::factory::ClientFactory;
use a2a_rust::a2a::client::client_trait::ClientEventOrMessage;
use a2a_rust::{Message, Part, Role};
use a2a_rust::a2a::models::*;
use futures::StreamExt;
use serde_json;
use tokio;

/// Test URL for Python A2A server (adjust as needed)
const PYTHON_SERVER_URL: &str = "http://localhost:8000";

/// Test that Rust client can connect to Python server and get agent card
#[tokio::test]
async fn test_python_server_agent_card() {
    // Skip test if server is not running
    if !is_server_running(PYTHON_SERVER_URL).await {
        println!("Skipping test - Python server not running at {}", PYTHON_SERVER_URL);
        return;
    }

    let config = ClientConfig::new()
        .with_timeout(std::time::Duration::from_secs(10));

    match ClientFactory::connect(
        PYTHON_SERVER_URL.to_string(),
        Some(config),
        None,
        None,
        None,
        None,
        None,
        None,
    ).await {
        Ok(client) => {
            // Test getting agent card
            let card = client.get_card(None, None).await.expect("Failed to get agent card");
            
            // Verify card structure
            assert!(!card.name.is_empty(), "Agent name should not be empty");
            assert!(!card.url.is_empty(), "Agent URL should not be empty");
            assert!(!card.version.is_empty(), "Agent version should not be empty");
            
            println!("✓ Successfully retrieved agent card from Python server:");
            println!("  Name: {}", card.name);
            println!("  URL: {}", card.url);
            println!("  Version: {}", card.version);
        }
        Err(e) => {
            panic!("Failed to connect to Python server: {}", e);
        }
    }
}

/// Test message sending from Rust client to Python server
#[tokio::test]
async fn test_message_send_to_python_server() {
    if !is_server_running(PYTHON_SERVER_URL).await {
        println!("Skipping test - Python server not running");
        return;
    }

    let config = ClientConfig::new()
        .with_timeout(std::time::Duration::from_secs(30))
        .with_polling(true); // Use polling for non-streaming

    match ClientFactory::connect(
        PYTHON_SERVER_URL.to_string(),
        Some(config),
        None,
        None,
        None,
        None,
        None,
        None,
    ).await {
        Ok(client) => {
            // Create a test message
            let message = Message {
                kind: "message".to_string(),
                message_id: uuid::Uuid::new_v4().to_string(),
                context_id: Some(uuid::Uuid::new_v4().to_string()),
                task_id: None,
                role: Role::User,
                parts: vec![
                    Part::text("Hello from Rust client! What is 2+2?".to_string()),
                ],
                metadata: None,
                extensions: None,
                reference_task_ids: None,
            };

            println!(" Sending test message to Python server: {:?}", message);

            // Send message
            let mut stream = client.send_message(
                message,
                None,
                None,
                None,
            ).await;

            let mut received_response = false;
            
            // Process response
            while let Some(result) = stream.next().await {
                match result {
                    Ok(event_or_message) => {
                        received_response = true;
                        match event_or_message {
                            ClientEventOrMessage::Message(msg) => {
                                println!("✓ Received message response: {:?}", msg);
                                // Verify response structure
                                assert_eq!(msg.role, Role::Agent);
                                assert!(!msg.parts.is_empty());
                            }
                            ClientEventOrMessage::Event((task, _update)) => {
                                println!("✓ Received task response: {}", task.id);
                                // Verify task structure
                                assert!(!task.id.is_empty());
                                assert_eq!(task.kind, "task");
                            }
                        }
                        break; // We only need the first response for this test
                    }
                    Err(e) => {
                        panic!("Error receiving response from Python server: {}", e);
                    }
                }
            }

            assert!(received_response, "Should have received a response from Python server");
        }
        Err(e) => {
            panic!("Failed to connect to Python server: {}", e);
        }
    }
}

/// Test task management operations with Python server
#[tokio::test]
async fn test_task_management_with_python_server() {
    if !is_server_running(PYTHON_SERVER_URL).await {
        println!("Skipping test - Python server not running");
        return;
    }

    let config = ClientConfig::new()
        .with_timeout(std::time::Duration::from_secs(30));

    match ClientFactory::connect(
        PYTHON_SERVER_URL.to_string(),
        Some(config),
        None,
        None,
        None,
        None,
        None,
        None,
    ).await {
        Ok(client) => {
            // First, create a task by sending a message
            let message = Message {
                kind: "message".to_string(),
                message_id: uuid::Uuid::new_v4().to_string(),
                context_id: Some(uuid::Uuid::new_v4().to_string()),
                task_id: None,
                role: Role::User,
                parts: vec![
                    Part::text("Create a task for testing".to_string()),
                ],
                metadata: None,
                extensions: None,
                reference_task_ids: None,
            };

            println!("Creating task for testing...");

            // Send message to create task
            let mut stream = client.send_message(
                message,
                None,
                None,
                None,
            ).await;

            let mut task_id = None;
            
            // Get task from response
            while let Some(result) = stream.next().await {
                match result {
                    Ok(ClientEventOrMessage::Event((task, _))) => {
                        task_id = Some(task.id.clone());
                        println!("✓ Created task: {}", task.id);
                        break;
                    }
                    Ok(_) => {
                        println!("⚠ Received non-task response");
                        // For this test, we'll skip if we don't get a task
                        return;
                    }
                    Err(e) => {
                        panic!("Error creating task: {}", e);
                    }
                }
            }

            if let Some(id) = task_id {
                // Test getting task details
                let task_params = TaskQueryParams::new(id.clone());
                match client.get_task(task_params, None, None).await {
                    Ok(task) => {
                        println!("✓ Retrieved task details: {}", task.id);
                        assert_eq!(task.id, id);
                    }
                    Err(e) => {
                        panic!("Failed to get task details: {}", e);
                    }
                }

                // Test task cancellation (cleanup)
                let cancel_params = TaskIdParams::new(id.clone());
                match client.cancel_task(cancel_params, None, None).await {
                    Ok(task) => {
                        println!("✓ Cancelled task: {} (status: {:?})", task.id, task.status.state);
                    }
                    Err(e) => {
                        println!("⚠ Failed to cancel task (may not be supported): {}", e);
                    }
                }
            } else {
                println!("⚠ No task created, skipping task management tests");
            }
        }
        Err(e) => {
            panic!("Failed to connect to Python server: {}", e);
        }
    }
}

/// Test streaming communication with Python server
#[tokio::test]
async fn test_streaming_with_python_server() {
    if !is_server_running(PYTHON_SERVER_URL).await {
        println!("Skipping test - Python server not running");
        return;
    }

    let config = ClientConfig::new()
        .with_streaming(true)
        .with_polling(false)
        .with_timeout(std::time::Duration::from_secs(60));

    match ClientFactory::connect(
        PYTHON_SERVER_URL.to_string(),
        Some(config),
        None,
        None,
        None,
        None,
        None,
        None,
    ).await {
        Ok(client) => {
            // Get agent card to check streaming capability
            let card = client.get_card(None, None).await.expect("Failed to get agent card");
            
            if !card.capabilities.streaming.unwrap_or(false) {
                println!("⚠ Server does not support streaming, skipping test");
                return;
            }

            // Create a message that might trigger streaming
            let message = Message {
                kind: "message".to_string(),
                message_id: uuid::Uuid::new_v4().to_string(),
                context_id: Some(uuid::Uuid::new_v4().to_string()),
                task_id: None,
                role: Role::User,
                parts: vec![
                    Part::text("Generate a long response about Tokyo travel".to_string()),
                ],
                metadata: None,
                extensions: None,
                reference_task_ids: None,
            };

            println!("Testing streaming with message: {:?}", message);

            // Send streaming message
            let mut stream = client.send_message(
                message,
                None,
                None,
                None,
            ).await;

            let mut event_count = 0;
            let mut received_updates = false;
            
            // Process streaming response
            while let Some(result) = stream.next().await {
                match result {
                    Ok(event_or_message) => {
                        event_count += 1;
                        match event_or_message {
                            ClientEventOrMessage::Message(msg) => {
                                println!("✓ Streamed message: {:?}", msg);
                            }
                            ClientEventOrMessage::Event((task, update)) => {
                                println!("✓ Streamed event {}: task={}", event_count, task.id);
                                
                                if let Some(_) = update {
                                    received_updates = true;
                                }
                            }
                        }
                        
                        // Limit events to avoid very long test
                        if event_count >= 10 {
                            println!("Reached event limit, stopping stream");
                            break;
                        }
                    }
                    Err(e) => {
                        panic!("Error in streaming: {}", e);
                    }
                }
            }

            println!("✓ Streaming test completed with {} events", event_count);
            assert!(received_updates, "not received_updates");

            if event_count > 1 {
                println!("✓ Streaming worked - received multiple events");
            } else {
                println!("⚠ Only received one event - streaming may not be fully supported");
            }
        }
        Err(e) => {
            panic!("Failed to connect for streaming test: {}", e);
        }
    }
}

/// Test error handling and compatibility
#[tokio::test]
async fn test_error_handling_compatibility() {
    if !is_server_running(PYTHON_SERVER_URL).await {
        println!("Skipping test - Python server not running");
        return;
    }

    let config = ClientConfig::new()
        .with_timeout(std::time::Duration::from_secs(10));

    match ClientFactory::connect(
        PYTHON_SERVER_URL.to_string(),
        Some(config),
        None,
        None,
        None,
        None,
        None,
        None,
    ).await {
        Ok(client) => {
            // Test getting non-existent task
            let fake_task_id = "non-existent-task-id";
            let task_params = TaskQueryParams::new(fake_task_id.to_string());
            
            match client.get_task(task_params, None, None).await {
                Ok(_) => {
                    panic!("Should have failed for non-existent task");
                }
                Err(e) => {
                    println!("✓ Correctly handled error for non-existent task: {}", e);
                    // Verify it's a proper error type
                    let error_str = e.to_string();
                    assert!(!error_str.is_empty(), "Error message should not be empty");
                }
            }

            // Test cancelling non-existent task
            let cancel_params = TaskIdParams::new(fake_task_id.to_string());
            
            match client.cancel_task(cancel_params, None, None).await {
                Ok(_) => {
                    println!("⚠ Server didn't fail for non-existent task cancellation");
                }
                Err(e) => {
                    println!("✓ Correctly handled error for non-existent task cancellation: {}", e);
                }
            }
        }
        Err(e) => {
            panic!("Failed to connect for error handling test: {}", e);
        }
    }
}

/// Helper function to check if server is running
async fn is_server_running(url: &str) -> bool {
    let client = reqwest::Client::new();
    let well_known_url = format!("{}/.well-known/agent-card.json", url);
    
    match client.get(&well_known_url).send().await {
        Ok(response) => response.status().is_success(),
        Err(_) => false,
    }
}

/// Test JSON-RPC protocol compatibility
#[tokio::test]
async fn test_jsonrpc_protocol_compatibility() {
    if !is_server_running(PYTHON_SERVER_URL).await {
        println!("Skipping test - Python server not running");
        return;
    }

    // Test direct JSON-RPC call to verify protocol compatibility
    let jsonrpc_request = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "tasks/get",
        "params": {
            "id": "test-task-id"
        },
        "id": "test-request-id"
    });

    let client = reqwest::Client::new();
    
    match client.post(PYTHON_SERVER_URL)
        .json(&jsonrpc_request)
        .send()
        .await {
        Ok(response) => {
            println!("✓ JSON-RPC request sent successfully");
            println!("  Status: {}", response.status());
            
            if response.status().is_success() {
                let response_json: serde_json::Value = response.json().await.expect("Failed to parse JSON response");
                println!("  Response: {}", response_json);
                
                // Verify it's a valid JSON-RPC response
                assert!(response_json.get("jsonrpc").is_some(), "Response should have jsonrpc field");
                assert!(response_json.get("id").is_some(), "Response should have id field");
                
                if response_json.get("result").is_some() {
                    println!("✓ Received valid JSON-RPC result");
                } else if response_json.get("error").is_some() {
                    println!("✓ Received valid JSON-RPC error (expected for test)");
                } else {
                    panic!("Response should have either result or error field");
                }
            } else {
                println!("⚠ Server returned non-success status: {}", response.status());
            }
        }
        Err(e) => {
            panic!("Failed to send JSON-RPC request: {}", e);
        }
    }
}

/// Test agent card well-known endpoint
#[tokio::test]
async fn test_agent_card_well_known_endpoint() {
    if !is_server_running(PYTHON_SERVER_URL).await {
        println!("Skipping test - Python server not running");
        return;
    }

    let well_known_url = format!("{}/.well-known/agent-card.json", PYTHON_SERVER_URL);
    let client = reqwest::Client::new();
    
    match client.get(&well_known_url).send().await {
        Ok(response) => {
            assert!(response.status().is_success(), "Should return 200 for agent card");
            
            let card_json: serde_json::Value = response.json().await.expect("Failed to parse agent card JSON");
            println!("✓ Retrieved agent card from well-known endpoint");
            
            // Verify required fields
            assert!(card_json.get("name").is_some(), "Agent card should have name");
            assert!(card_json.get("description").is_some(), "Agent card should have description");
            assert!(card_json.get("url").is_some(), "Agent card should have url");
            assert!(card_json.get("version").is_some(), "Agent card should have version");
            assert!(card_json.get("capabilities").is_some(), "Agent card should have capabilities");
            
            println!("✓ Agent card has all required fields");
        }
        Err(e) => {
            panic!("Failed to get agent card from well-known endpoint: {}", e);
        }
    }
}
