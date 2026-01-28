//! A2A (Agent-to-Agent) Protocol Implementation in Rust
//! 
//! This crate provides a Rust implementation of the A2A protocol,
//! which enables communication between AI agents and clients.

// Core modules
pub mod core_types;
pub mod models;
pub mod types;
pub mod error;
pub mod serde;
pub mod jsonrpc;

// Sub-modules matching a2a-python structure
pub mod auth;
pub mod client;
pub mod server;
pub mod utils;
pub mod extensions;
pub mod grpc;

// Re-export main types for convenience
pub use types::*;
pub use utils::*;
