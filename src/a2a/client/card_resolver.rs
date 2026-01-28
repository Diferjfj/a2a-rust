//! Agent Card Resolver for A2A clients
//! 
//! This module provides functionality to resolve and fetch agent cards,
//! mirroring the functionality of a2a-python's card resolver.

use crate::a2a::models::*;
use crate::a2a::error::A2AError;
use reqwest;
use serde_json::Value;
use std::collections::HashMap;
use url::Url;

/// A2A Card Resolver for fetching agent cards from servers
/// 
/// This mirrors a2a-python's A2ACardResolver functionality
pub struct A2ACardResolver {
    /// Base URL of the agent
    base_url: String,
}

impl A2ACardResolver {
    /// Create a new card resolver for the given agent URL
    pub fn new(base_url: String) -> Self {
        Self { base_url }
    }
    
    /// Get the agent card from the well-known endpoint
    pub async fn get_agent_card(&self) -> Result<AgentCard, A2AError> {
        let card_url = format!("{}/.well-known/agent-card.json", 
                              self.base_url.trim_end_matches('/'));
        
        let client = reqwest::Client::new();
        let response = client.get(&card_url)
            .send()
            .await
            .map_err(|e| A2AError::transport_error(format!("Failed to fetch agent card: {}", e)))?;
        
        if !response.status().is_success() {
            return Err(A2AError::http_error(
                response.status().as_u16(),
                format!("Failed to fetch agent card: {}", response.status()),
            ));
        }
        
        let card_json: Value = response
            .json()
            .await
            .map_err(|e| A2AError::json_error(format!("Failed to parse agent card JSON: {}", e)))?;
        
        serde_json::from_value(card_json)
            .map_err(|e| A2AError::json_error(format!("Failed to deserialize agent card: {}", e)))
    }
    
    /// Get agent card with optional relative path and additional HTTP kwargs
    pub async fn get_agent_card_with_path(
        &self,
        relative_path: Option<String>,
        http_kwargs: Option<HashMap<String, Value>>,
    ) -> Result<AgentCard, A2AError> {
        let card_url = if let Some(path) = relative_path {
            let base = Url::parse(&self.base_url)
                .map_err(|e| A2AError::invalid_url(&format!("Invalid base URL: {}", e)))?;
            base.join(path.as_str())
                .map_err(|e| A2AError::invalid_url(&format!("Failed to join path: {}", e)))?
                .to_string()
        } else {
            format!("{}/.well-known/agent-card.json", 
                   self.base_url.trim_end_matches('/'))
        };
        
        let client = reqwest::Client::new();
        let mut request = client.get(&card_url);
        
        // Apply HTTP kwargs if provided
        if let Some(kwargs) = http_kwargs {
            // Add headers
            if let Some(headers) = kwargs.get("headers").and_then(|h| h.as_object()) {
                for (key, value) in headers {
                    if let Some(value_str) = value.as_str() {
                        if let Ok(header_name) = reqwest::header::HeaderName::from_bytes(key.as_bytes()) {
                            if let Ok(header_value) = reqwest::header::HeaderValue::from_str(value_str) {
                                request = request.header(header_name, header_value);
                            }
                        }
                    }
                }
            }
            
            // Add query parameters
            if let Some(params) = kwargs.get("params").and_then(|p| p.as_object()) {
                for (key, value) in params {
                    if let Some(value_str) = value.as_str() {
                        request = request.query(&[(key, value_str)]);
                    }
                }
            }
            
            // Add timeout
            if let Some(timeout) = kwargs.get("timeout").and_then(|t| t.as_u64()) {
                request = request.timeout(std::time::Duration::from_secs(timeout));
            }
        }
        
        let response = request
            .send()
            .await
            .map_err(|e| A2AError::transport_error(format!("Failed to fetch agent card: {}", e)))?;
        
        if !response.status().is_success() {
            return Err(A2AError::http_error(
                response.status().as_u16(),
                format!("Failed to fetch agent card: {}", response.status()),
            ));
        }
        
        let card_json: Value = response
            .json()
            .await
            .map_err(|e| A2AError::json_error(format!("Failed to parse agent card JSON: {}", e)))?;
        
        serde_json::from_value(card_json)
            .map_err(|e| A2AError::json_error(format!("Failed to deserialize agent card: {}", e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_resolver_creation() {
        let resolver = A2ACardResolver::new("http://localhost:8080".to_string());
        assert_eq!(resolver.base_url, "http://localhost:8080");
    }

    #[test]
    fn test_card_resolver_with_trailing_slash() {
        let resolver = A2ACardResolver::new("http://localhost:8080/".to_string());
        // The trailing slash should be handled when building URLs
        assert_eq!(resolver.base_url, "http://localhost:8080/");
    }
}
