//! Simple A2A Server Example
//! 
//! This example demonstrates how to create a basic A2A server using the a2a-rust library.

use a2a_rust::a2a::{
    models::*,
    server::{
        apps::jsonrpc::{A2AServerBuilder, ServerConfig},
        context::DefaultServerCallContextBuilder,
        request_handlers::{RequestHandler, MessageSendResult, TaskPushNotificationConfigQueryParams},
    },
    core_types::{Message, Part, Role},
};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio;

/// Simple request handler that echoes back messages
struct EchoHandler;

impl EchoHandler {
    fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl RequestHandler for EchoHandler {
    async fn on_get_task(
        &self,
        _params: TaskQueryParams,
        _context: Option<&a2a_rust::a2a::server::context::ServerCallContext>,
    ) -> Result<Option<Task>, a2a_rust::a2a::error::A2AError> {
        // For this simple example, we don't support task retrieval
        Ok(None)
    }

    async fn on_cancel_task(
        &self,
        _params: TaskIdParams,
        _context: Option<&a2a_rust::a2a::server::context::ServerCallContext>,
    ) -> Result<Option<Task>, a2a_rust::a2a::error::A2AError> {
        // For this simple example, we don't support task cancellation
        Ok(None)
    }

    async fn on_message_send(
        &self,
        params: MessageSendParams,
        _context: Option<&a2a_rust::a2a::server::context::ServerCallContext>,
    ) -> Result<MessageSendResult, a2a_rust::a2a::error::A2AError> {
        // Echo back the message
        let mut response_parts = Vec::new();
        
        // Add a text part indicating this is an echo
        response_parts.push(Part::text("Echo from Rust server: ".to_string()));
        
        // Add all parts from the incoming message
        for part in &params.message.parts {
            response_parts.push(part.clone());
        }
        
        let response_message = Message::new(Role::Agent, response_parts)
            .with_context_id(params.message.context_id.clone().unwrap_or_default())
            .with_task_id(params.message.task_id.clone().unwrap_or_default());

        Ok(MessageSendResult::Message(response_message))
    }

    async fn on_set_task_push_notification_config(
        &self,
        _params: TaskPushNotificationConfig,
        _context: Option<&a2a_rust::a2a::server::context::ServerCallContext>,
    ) -> Result<TaskPushNotificationConfig, a2a_rust::a2a::error::A2AError> {
        Err(a2a_rust::a2a::error::A2AError::unsupported_operation("Push notifications not supported"))
    }

    async fn on_get_task_push_notification_config(
        &self,
        _params: TaskPushNotificationConfigQueryParams,
        _context: Option<&a2a_rust::a2a::server::context::ServerCallContext>,
    ) -> Result<TaskPushNotificationConfig, a2a_rust::a2a::error::A2AError> {
        Err(a2a_rust::a2a::error::A2AError::unsupported_operation("Push notifications not supported"))
    }

    async fn on_list_task_push_notification_config(
        &self,
        _params: TaskIdParams,
        _context: Option<&a2a_rust::a2a::server::context::ServerCallContext>,
    ) -> Result<Vec<TaskPushNotificationConfig>, a2a_rust::a2a::error::A2AError> {
        Ok(vec![])
    }

    async fn on_delete_task_push_notification_config(
        &self,
        _params: DeleteTaskPushNotificationConfigParams,
        _context: Option<&a2a_rust::a2a::server::context::ServerCallContext>,
    ) -> Result<(), a2a_rust::a2a::error::A2AError> {
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Create agent card
    let agent_card = AgentCard::new(
        "Echo Server".to_string(),
        "A simple echo server implemented in Rust".to_string(),
        "http://localhost:8080".to_string(),
        "1.0.0".to_string(),
        vec!["text/plain".to_string()],
        vec!["text/plain".to_string()],
        AgentCapabilities::new(),
        vec![],
    );

    // Create request handler
    let request_handler = Arc::new(EchoHandler::new());

    // Create context builder
    let context_builder = Arc::new(DefaultServerCallContextBuilder);

    // Configure server
    let config = ServerConfig {
        bind_addr: "127.0.0.1:8080".parse::<SocketAddr>()?,
        ..Default::default()
    };

    // Build and start server
    let server = A2AServerBuilder::new()
        .with_agent_card(agent_card)
        .with_request_handler(request_handler)
        .with_context_builder(context_builder)
        .with_config(config)
        .build()?;

    println!("🚀 Starting A2A Echo Server on http://127.0.0.1:8080");
    println!("📋 Agent Card available at: http://127.0.0.1:8080/.well-known/agent.json");
    println!("🔌 JSON-RPC endpoint at: http://127.0.0.1:8080/rpc");
    println!("✨ Server is ready to accept connections!");

    // Start the server
    server.serve().await?;

    Ok(())
}
