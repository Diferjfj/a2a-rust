//! Client authentication module
//! 
//! This module contains client-side authentication functionality
//! matching a2a-python/src/a2a/client/auth/

pub mod credentials;
pub mod interceptor;

// Re-export auth types
pub use credentials::{
    CredentialService,
    InMemoryContextCredentialStore,
    EnvironmentCredentialService,
    CompositeCredentialService,
};

pub use interceptor::AuthInterceptor;
