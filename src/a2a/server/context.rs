//! Server Call Context implementation
//! 
//! This module defines the ServerCallContext which holds information about the current
//! server call, including authentication, headers, and other request metadata.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Trait for building server call contexts from HTTP requests
#[async_trait]
pub trait ServerCallContextBuilder: Send + Sync {
    /// Build a ServerCallContext from HTTP headers
    async fn build(&self, headers: &axum::http::HeaderMap) -> ServerCallContext;
}

/// Default implementation of ServerCallContextBuilder
pub struct DefaultServerCallContextBuilder;

#[async_trait]
impl ServerCallContextBuilder for DefaultServerCallContextBuilder {
    async fn build(&self, _headers: &axum::http::HeaderMap) -> ServerCallContext {
        ServerCallContext::new()
    }
}

/// Server Call Context
/// 
/// A context passed when calling a server method.
/// This class allows storing arbitrary user data in the state attribute.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerCallContext {
    /// Arbitrary user-provided state data
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub state: HashMap<String, serde_json::Value>,
    
    /// Authenticated user information
    #[serde(default)]
    pub user: crate::a2a::auth::user::AuthenticatedUser,
    
    /// Set of extensions that were requested by the client
    #[serde(default, skip_serializing_if = "std::collections::HashSet::is_empty")]
    pub requested_extensions: std::collections::HashSet<String>,
    
    /// Set of extensions that were activated for this request
    #[serde(default, skip_serializing_if = "std::collections::HashSet::is_empty")]
    pub activated_extensions: std::collections::HashSet<String>,
}

impl Default for ServerCallContext {
    fn default() -> Self {
        Self {
            state: HashMap::new(),
            user: crate::a2a::auth::user::AuthenticatedUser::default(),
            requested_extensions: std::collections::HashSet::new(),
            activated_extensions: std::collections::HashSet::new(),
        }
    }
}

impl ServerCallContext {
    /// Creates a new ServerCallContext with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new ServerCallContext with the given user
    pub fn with_user(user: crate::a2a::auth::user::AuthenticatedUser) -> Self {
        Self {
            user,
            ..Default::default()
        }
    }

    /// Adds a state value to the context
    pub fn set_state(&mut self, key: String, value: serde_json::Value) {
        self.state.insert(key, value);
    }

    /// Gets a state value from the context
    pub fn get_state(&self, key: &str) -> Option<&serde_json::Value> {
        self.state.get(key)
    }

    /// Removes a state value from the context
    pub fn remove_state(&mut self, key: &str) -> Option<serde_json::Value> {
        self.state.remove(key)
    }

    /// Adds a requested extension
    pub fn add_requested_extension(&mut self, uri: String) {
        self.requested_extensions.insert(uri);
    }

    /// Adds an activated extension
    pub fn add_activated_extension(&mut self, uri: String) {
        self.activated_extensions.insert(uri);
    }

    /// Checks if an extension was requested
    pub fn is_extension_requested(&self, uri: &str) -> bool {
        self.requested_extensions.contains(uri)
    }

    /// Checks if an extension is activated
    pub fn is_extension_activated(&self, uri: &str) -> bool {
        self.activated_extensions.contains(uri)
    }

    /// Gets the requested extensions as a vector
    pub fn get_requested_extensions(&self) -> Vec<String> {
        self.requested_extensions.iter().cloned().collect()
    }

    /// Gets the activated extensions as a vector
    pub fn get_activated_extensions(&self) -> Vec<String> {
        self.activated_extensions.iter().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::a2a::auth::user::{AuthenticatedUser};

    #[test]
    fn test_server_call_context_default() {
        let context = ServerCallContext::default();
        assert!(context.state.is_empty());
        assert_eq!(context.user.username(), "");
        assert!(context.requested_extensions.is_empty());
        assert!(context.activated_extensions.is_empty());
    }

    #[test]
    fn test_server_call_context_with_user() {
        let user = AuthenticatedUser::new("user123".to_string());
        let context = ServerCallContext::with_user(user.clone());
        assert_eq!(context.user.username(), user.username());
    }

    #[test]
    fn test_state_management() {
        let mut context = ServerCallContext::new();
        
        // Test setting state
        context.set_state("key1".to_string(), serde_json::json!("value1"));
        context.set_state("key2".to_string(), serde_json::json!(42));
        
        // Test getting state
        assert_eq!(context.get_state("key1"), Some(&serde_json::json!("value1")));
        assert_eq!(context.get_state("key2"), Some(&serde_json::json!(42)));
        assert_eq!(context.get_state("nonexistent"), None);
        
        // Test removing state
        let removed = context.remove_state("key1");
        assert_eq!(removed, Some(serde_json::json!("value1")));
        assert_eq!(context.get_state("key1"), None);
    }

    #[test]
    fn test_extension_management() {
        let mut context = ServerCallContext::new();
        
        // Test adding requested extensions
        context.add_requested_extension("ext1".to_string());
        context.add_requested_extension("ext2".to_string());
        
        assert!(context.is_extension_requested("ext1"));
        assert!(context.is_extension_requested("ext2"));
        assert!(!context.is_extension_requested("ext3"));
        
        // Test adding activated extensions
        context.add_activated_extension("ext1".to_string());
        
        assert!(context.is_extension_activated("ext1"));
        assert!(!context.is_extension_activated("ext2"));
        
        // Test getting extensions as vectors
        let requested = context.get_requested_extensions();
        assert_eq!(requested.len(), 2);
        assert!(requested.contains(&"ext1".to_string()));
        assert!(requested.contains(&"ext2".to_string()));
        
        let activated = context.get_activated_extensions();
        assert_eq!(activated.len(), 1);
        assert!(activated.contains(&"ext1".to_string()));
    }

    #[test]
    fn test_serialization() {
        let mut context = ServerCallContext::new();
        context.set_state("test".to_string(), serde_json::json!("value"));
        context.add_requested_extension("ext1".to_string());
        context.add_activated_extension("ext1".to_string());
        
        let serialized = serde_json::to_string(&context).unwrap();
        let deserialized: ServerCallContext = serde_json::from_str(&serialized).unwrap();
        
        assert_eq!(deserialized.get_state("test"), Some(&serde_json::json!("value")));
        assert!(deserialized.is_extension_requested("ext1"));
        assert!(deserialized.is_extension_activated("ext1"));
    }
}
