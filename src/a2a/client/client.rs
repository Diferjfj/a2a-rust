//! Main client implementation for A2A protocol

use crate::a2a::client::base_client::DefaultBaseClient;

/// Main A2A client
pub struct A2AClient {
    #[allow(dead_code)]
    base_client: DefaultBaseClient,
}

impl A2AClient {
    /// Create a new A2A client
    pub fn new() -> Self {
        Self { 
            base_client: DefaultBaseClient::new()
        }
    }
    
    /// Create a new A2A client with custom base client
    pub fn with_base_client(base_client: DefaultBaseClient) -> Self {
        Self { base_client }
    }
}

impl Default for A2AClient {
    fn default() -> Self {
        Self::new()
    }
}

// TODO: Implement more client functionality
