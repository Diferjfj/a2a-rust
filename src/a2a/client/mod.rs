//! Client module for A2A protocol
//! 
//! This module contains client-side functionality
//! matching a2a-python/src/a2a/client/

pub mod base_client;
pub mod card_resolver;
pub mod client_factory;
pub mod client_task_manager;
pub mod client_trait;
pub mod client;
pub mod config;
pub mod errors;
pub mod factory;
pub mod helpers;
pub mod legacy_grpc;
pub mod legacy;
pub mod middleware;
pub mod optionals;

// Auth submodule
pub mod auth;

// Transports submodule
pub mod transports;

// Re-export main client types
pub use base_client::BaseClient;
pub use client_trait::{
    Client, ClientTransport, ClientCallContext, ClientCallInterceptor, 
    ClientEvent, ClientEventOrMessage, Consumer, TaskUpdateEvent
};
pub use client::*;
pub use config::*;
pub use errors::*;
pub use factory::*;

// Re-export auth types
pub use auth::{
    CredentialService, InMemoryContextCredentialStore, EnvironmentCredentialService,
    CompositeCredentialService, AuthInterceptor
};
