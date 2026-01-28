//! Server-side components for implementing an A2A agent
//! 
//! This module provides the core server components for implementing an A2A agent,
//! including HTTP server, WebSocket support, and request handling.

pub mod apps;
pub mod context;
pub mod request_handlers;

// Re-export commonly used types
pub use context::{ServerCallContext, ServerCallContextBuilder};
pub use request_handlers::{RequestHandler, JSONRPCHandler};
