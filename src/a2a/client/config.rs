//! Client configuration for A2A Rust client
//! 
//! This module provides configuration options for the A2A client,
//! mirroring the functionality of a2a-python's ClientConfig.

use crate::a2a::models::*;
use crate::a2a::core_types::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// Configuration for the A2A client
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientConfig {
    /// Whether client supports streaming
    pub streaming: bool,
    
    /// Whether client prefers to poll for updates from message:send
    pub polling: bool,
    
    /// Request timeout
    pub timeout: Option<Duration>,
    
    /// Ordered list of transports for connecting to agent (in order of preference)
    /// Empty implies JSON-RPC only
    pub supported_transports: Vec<TransportProtocol>,
    
    /// Whether to use client transport preferences over server preferences
    /// Recommended to use server preferences in most situations
    pub use_client_preference: bool,
    
    /// The set of accepted output modes for the client
    pub accepted_output_modes: Vec<String>,
    
    /// Push notification callbacks to use for every request
    pub push_notification_configs: Vec<PushNotificationConfig>,
    
    /// A list of extension URIs the client supports
    pub extensions: Vec<String>,
    
    /// HTTP headers to include in all requests
    pub headers: HashMap<String, String>,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            streaming: true,
            polling: false,
            timeout: Some(Duration::from_secs(30)),
            supported_transports: vec![TransportProtocol::Jsonrpc],
            use_client_preference: false,
            accepted_output_modes: vec![],
            push_notification_configs: vec![],
            extensions: vec![],
            headers: HashMap::new(),
        }
    }
}

impl ClientConfig {
    /// Create a new client config with default settings
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Set streaming support
    pub fn with_streaming(mut self, streaming: bool) -> Self {
        self.streaming = streaming;
        self
    }
    
    /// Set polling preference
    pub fn with_polling(mut self, polling: bool) -> Self {
        self.polling = polling;
        self
    }
    
    /// Set request timeout
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }
    
    /// Set supported transports
    pub fn with_supported_transports(mut self, transports: Vec<TransportProtocol>) -> Self {
        self.supported_transports = transports;
        self
    }
    
    /// Set whether to use client preference for transport selection
    pub fn with_client_preference(mut self, use_client_preference: bool) -> Self {
        self.use_client_preference = use_client_preference;
        self
    }
    
    /// Set accepted output modes
    pub fn with_accepted_output_modes(mut self, modes: Vec<String>) -> Self {
        self.accepted_output_modes = modes;
        self
    }
    
    /// Set push notification configurations
    pub fn with_push_notification_configs(mut self, configs: Vec<PushNotificationConfig>) -> Self {
        self.push_notification_configs = configs;
        self
    }
    
    /// Set extensions
    pub fn with_extensions(mut self, extensions: Vec<String>) -> Self {
        self.extensions = extensions;
        self
    }
    
    /// Set HTTP headers
    pub fn with_headers(mut self, headers: HashMap<String, String>) -> Self {
        self.headers = headers;
        self
    }
    
    /// Add a single HTTP header
    pub fn with_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(key.into(), value.into());
        self
    }
}

/// Configuration for sending a message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageSendConfiguration {
    /// Accepted output modes for this specific message
    pub accepted_output_modes: Option<Vec<String>>,
    
    /// Whether to wait for completion (blocking) or return immediately
    pub blocking: Option<bool>,
    
    /// Push notification configuration for this message
    pub push_notification_config: Option<PushNotificationConfig>,
}

impl Default for MessageSendConfiguration {
    fn default() -> Self {
        Self {
            accepted_output_modes: None,
            blocking: Some(true),
            push_notification_config: None,
        }
    }
}

impl MessageSendConfiguration {
    /// Create a new message send configuration
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Set accepted output modes
    pub fn with_accepted_output_modes(mut self, modes: Vec<String>) -> Self {
        self.accepted_output_modes = Some(modes);
        self
    }
    
    /// Set blocking behavior
    pub fn with_blocking(mut self, blocking: bool) -> Self {
        self.blocking = Some(blocking);
        self
    }
    
    /// Set push notification configuration
    pub fn with_push_notification_config(mut self, config: PushNotificationConfig) -> Self {
        self.push_notification_config = Some(config);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_config_default() {
        let config = ClientConfig::default();
        assert!(config.streaming);
        assert!(!config.polling);
        assert_eq!(config.supported_transports, vec![TransportProtocol::Jsonrpc]);
        assert!(!config.use_client_preference);
    }

    #[test]
    fn test_client_config_builder() {
        let config = ClientConfig::new()
            .with_streaming(false)
            .with_polling(true)
            .with_timeout(Duration::from_secs(60))
             .with_header("Authorization", "Bearer token")
            .with_client_preference(true);
        
        assert!(!config.streaming);
        assert!(config.polling);
        assert_eq!(config.timeout, Some(Duration::from_secs(60)));
        assert!(config.use_client_preference);
        assert_eq!(config.headers.get("Authorization"), Some(&"Bearer token".to_string()));
    }

    #[test]
    fn test_message_send_configuration() {
        let config = MessageSendConfiguration::new()
            .with_blocking(false)
            .with_accepted_output_modes(vec!["text/plain".to_string()]);
        
        assert_eq!(config.blocking, Some(false));
        assert_eq!(
            config.accepted_output_modes,
            Some(vec!["text/plain".to_string()])
        );
    }
}
