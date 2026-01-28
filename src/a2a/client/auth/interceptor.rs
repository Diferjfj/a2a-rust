//! Authentication interceptor for client requests
//! 
//! This module provides an interceptor that automatically adds authentication
//! details to requests based on the agent's security schemes,
//! matching a2a-python's AuthInterceptor.

use crate::a2a::client::auth::credentials::CredentialService;
use crate::a2a::client::client_trait::ClientCallContext;
use crate::a2a::client::client_trait::ClientCallInterceptor;
use crate::a2a::models::*;
use crate::a2a::core_types::*;
use crate::a2a::error::A2AError;
use async_trait::async_trait;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

/// An interceptor that automatically adds authentication details to requests
/// 
/// Based on the agent's security schemes, this interceptor will automatically
/// apply appropriate authentication headers to outgoing requests.
pub struct AuthInterceptor {
    /// Credential service for retrieving authentication credentials
    credential_service: Arc<dyn CredentialService>,
}

impl AuthInterceptor {
    /// Create a new authentication interceptor
    pub fn new(credential_service: Arc<dyn CredentialService>) -> Self {
        Self {
            credential_service,
        }
    }
    
    /// Create an authentication interceptor with an in-memory credential store
    pub fn with_memory_store() -> (Self, crate::a2a::client::auth::credentials::InMemoryContextCredentialStore) {
        let store = crate::a2a::client::auth::credentials::InMemoryContextCredentialStore::new();
        let interceptor = Self::new(Arc::new(store.clone()));
        (interceptor, store)
    }
    
    /// Create an authentication interceptor with an environment-based credential service
    pub fn with_env_credentials() -> Self {
        let service = crate::a2a::client::auth::credentials::EnvironmentCredentialService::default();
        Self::new(Arc::new(service))
    }
}

#[async_trait]
impl ClientCallInterceptor for AuthInterceptor {
    async fn intercept(
        &self,
        method_name: &str,
        request_payload: Value,
        mut http_kwargs: HashMap<String, Value>,
        agent_card: &AgentCard,
        context: Option<&ClientCallContext>,
    ) -> Result<(Value, HashMap<String, Value>), A2AError> {
        // Skip authentication if no security schemes
        if agent_card.security.is_none() || agent_card.security_schemes.is_none() {
            return Ok((request_payload, http_kwargs));
        }
        
        let security = match &agent_card.security {
            Some(security) => security,
            None => return Ok((request_payload, http_kwargs)),
        };
        
        let security_schemes = match &agent_card.security_schemes {
            Some(schemes) => schemes,
            None => return Ok((request_payload, http_kwargs)),
        };
        
        // Try each security requirement until we find one with available credentials
        for requirement in security {
            for (scheme_name, _scopes) in requirement {
                // Get credentials for this scheme
                let credential = match self.credential_service.get_credentials(scheme_name, context).await {
                    Ok(Some(cred)) => cred,
                    Ok(None) => continue, // No credentials available for this scheme
                    Err(e) => {
                        // Log error but continue trying other schemes
                        eprintln!("Error getting credentials for scheme '{}': {}", scheme_name, e);
                        continue;
                    }
                };
                
                // Get the security scheme definition
                let scheme_def = match security_schemes.get(scheme_name) {
                    Some(scheme) => scheme,
                    None => continue,
                };
                
                // Apply authentication based on scheme type
                if self.apply_authentication(&mut http_kwargs, scheme_name, &credential, scheme_def).await? {
                    // Successfully applied authentication, return early
                    tracing::debug!(
                        "Applied authentication for scheme '{}' (method: {})",
                        scheme_name,
                        method_name
                    );
                    return Ok((request_payload, http_kwargs));
                }
            }
        }
        
        // No authentication was applied
        tracing::debug!("No authentication applied for method: {}", method_name);
        Ok((request_payload, http_kwargs))
    }
}

impl AuthInterceptor {
    /// Apply authentication based on the security scheme
    async fn apply_authentication(
        &self,
        http_kwargs: &mut HashMap<String, Value>,
        scheme_name: &str,
        credential: &str,
        scheme_def: &SecurityScheme,
    ) -> Result<bool, A2AError> {
        // Get or create headers map
        let headers = http_kwargs
            .entry("headers".to_string())
            .or_insert_with(|| Value::Object(serde_json::Map::new()))
            .as_object_mut()
            .ok_or_else(|| A2AError::invalid_request("headers must be an object"))?;
        
        match scheme_def {
            SecurityScheme::HTTPAuth(http_scheme) => {
                // Handle HTTP authentication schemes
                if http_scheme.scheme.to_lowercase() == "bearer" {
                    // Bearer token
                    headers.insert(
                        "Authorization".to_string(),
                        Value::String(format!("Bearer {}", credential)),
                    );
                    tracing::debug!("Added Bearer token for scheme '{}'", scheme_name);
                    Ok(true)
                } else {
                    // Other HTTP schemes (Basic, Digest, etc.)
                    headers.insert(
                        "Authorization".to_string(),
                        Value::String(format!("{} {}", http_scheme.scheme, credential)),
                    );
                    tracing::debug!("Added {} header for scheme '{}'", http_scheme.scheme, scheme_name);
                    Ok(true)
                }
            }
            
            SecurityScheme::OAuth2(_) => {
                // OAuth2 is implicitly Bearer token
                headers.insert(
                    "Authorization".to_string(),
                    Value::String(format!("Bearer {}", credential)),
                );
                tracing::debug!("Added OAuth2 Bearer token for scheme '{}'", scheme_name);
                Ok(true)
            }
            
            SecurityScheme::OpenIdConnect(_) => {
                // OIDC is also implicitly Bearer token
                headers.insert(
                    "Authorization".to_string(),
                    Value::String(format!("Bearer {}", credential)),
                );
                tracing::debug!("Added OIDC Bearer token for scheme '{}'", scheme_name);
                Ok(true)
            }
            
            SecurityScheme::APIKey(api_key_scheme) => {
                // Handle API key based on location
                match api_key_scheme.in_ {
                    In::Header => {
                        headers.insert(
                            api_key_scheme.name.clone(),
                            Value::String(credential.to_string()),
                        );
                        tracing::debug!("Added API Key header '{}' for scheme '{}'", api_key_scheme.name, scheme_name);
                        Ok(true)
                    }
                    In::Query => {
                        // For query parameters, we need to modify the URL
                        // This is more complex and depends on the transport
                        // For now, we'll add it to a special query_params field
                        let query_params = http_kwargs
                            .entry("query_params".to_string())
                            .or_insert_with(|| Value::Object(serde_json::Map::new()))
                            .as_object_mut()
                            .ok_or_else(|| A2AError::invalid_request("query_params must be an object"))?;
                        
                        query_params.insert(
                            api_key_scheme.name.clone(),
                            Value::String(credential.to_string()),
                        );
                        tracing::debug!("Added API Key query parameter '{}' for scheme '{}'", api_key_scheme.name, scheme_name);
                        Ok(true)
                    }
                    In::Cookie => {
                        // For cookies, we can add to Cookie header
                        let cookie_header = headers.get("Cookie").and_then(|v| v.as_str()).unwrap_or("");
                        let new_cookie = if cookie_header.is_empty() {
                            format!("{}={}", api_key_scheme.name, credential)
                        } else {
                            format!("{}; {}={}", cookie_header, api_key_scheme.name, credential)
                        };
                        headers.insert("Cookie".to_string(), Value::String(new_cookie));
                        tracing::debug!("Added API Key cookie '{}' for scheme '{}'", api_key_scheme.name, scheme_name);
                        Ok(true)
                    }
                }
            }
            
            SecurityScheme::MutualTLS(_) => {
                // Mutual TLS is handled at the transport level
                // We can't easily apply it here, so we'll just log it
                tracing::debug!("Mutual TLS authentication required for scheme '{}', but cannot be applied at interceptor level", scheme_name);
                Ok(false)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::a2a::client::auth::credentials::InMemoryContextCredentialStore;
    
    fn create_test_agent_card() -> AgentCard {
        let mut security_schemes = std::collections::HashMap::new();
        
        // Add Bearer token scheme
        security_schemes.insert(
            "bearerAuth".to_string(),
            SecurityScheme::HTTPAuth(HTTPAuthSecurityScheme {
                scheme: "bearer".to_string(),
                bearer_format: Some("JWT".to_string()),
                description: Some("Bearer token authentication".to_string()),
            }),
        );
        
        // Add API Key scheme
        security_schemes.insert(
            "apiKey".to_string(),
            SecurityScheme::APIKey(APIKeySecurityScheme {
                name: "X-API-Key".to_string(),
                in_: In::Header,
                description: Some("API key authentication".to_string()),
            }),
        );
        
        AgentCard::new(
            "Test Agent".to_string(),
            "Test agent for authentication".to_string(),
            "http://localhost:8080".to_string(),
            "1.0.0".to_string(),
            vec![],
            vec![],
            AgentCapabilities::new(),
            vec![],
        )
        .with_security_schemes(security_schemes)
        .with_security(vec![std::collections::HashMap::from([
            ("bearerAuth".to_string(), vec![]),
        ])])
    }
    
    
    #[tokio::test]
    async fn test_bearer_token_authentication() {
        let mut store = InMemoryContextCredentialStore::new();
        store.add_credential("bearerAuth", "test-jwt-token");
        
        let interceptor = AuthInterceptor::new(Arc::new(store));
        let agent_card = create_test_agent_card();
        
        let payload = serde_json::json!({"test": "data"});
        let http_kwargs = HashMap::new();
        
        let (new_payload, new_http_kwargs) = interceptor
            .intercept("test_method", payload, http_kwargs, &agent_card, None)
            .await
            .unwrap();
        
        // Check that Authorization header was added
        let headers = new_http_kwargs.get("headers").unwrap();
        let auth_header = headers.get("Authorization").unwrap();
        assert_eq!(auth_header, "Bearer test-jwt-token");
        
        // Payload should remain unchanged
        assert_eq!(new_payload, serde_json::json!({"test": "data"}));
    }
    
    #[tokio::test]
    async fn test_api_key_authentication() {
        let mut card = create_test_agent_card();
        
        // Modify card to require API key
        card.security = Some(vec![std::collections::HashMap::from([
            ("apiKey".to_string(), vec![]),
        ])]);
        
        let mut store = InMemoryContextCredentialStore::new();
        store.add_credential("apiKey", "test-api-key");
        
        let interceptor = AuthInterceptor::new(Arc::new(store));
        
        let payload = serde_json::json!({"test": "data"});
        let http_kwargs = HashMap::new();
        
        let (_new_payload, new_http_kwargs) = interceptor
            .intercept("test_method", payload, http_kwargs, &card, None)
            .await
            .unwrap();
        
        // Check that API key header was added
        let headers = new_http_kwargs.get("headers").unwrap();
        let api_key_header = headers.get("X-API-Key").unwrap();
        assert_eq!(api_key_header, "test-api-key");
    }
    
    #[tokio::test]
    async fn test_no_authentication_when_no_credentials() {
        let store = InMemoryContextCredentialStore::new(); // Empty store
        let interceptor = AuthInterceptor::new(Arc::new(store));
        let agent_card = create_test_agent_card();
        
        let payload = serde_json::json!({"test": "data"});
        let http_kwargs = HashMap::new();
        
        let (new_payload, new_http_kwargs) = interceptor
            .intercept("test_method", payload, http_kwargs, &agent_card, None)
            .await
            .unwrap();
        
        // No headers should have been added since no credentials are available
        assert!(!new_http_kwargs.contains_key("headers"));
        
        // Payload should remain unchanged
        assert_eq!(new_payload, serde_json::json!({"test": "data"}));
    }
    
    #[tokio::test]
    async fn test_no_authentication_when_no_schemes() {
        let mut store = InMemoryContextCredentialStore::new();
        store.add_credential("bearerAuth", "test-token");
        
        let interceptor = AuthInterceptor::new(Arc::new(store));
        
        // Create agent card without security schemes
        let agent_card = AgentCard::new(
            "Test Agent".to_string(),
            "Test agent".to_string(),
            "http://localhost:8080".to_string(),
            "1.0.0".to_string(),
            vec![],
            vec![],
            AgentCapabilities::new(),
            vec![],
        );
        
        let payload = serde_json::json!({"test": "data"});
        let http_kwargs = HashMap::new();
        
        let (new_payload, new_http_kwargs) = interceptor
            .intercept("test_method", payload, http_kwargs, &agent_card, None)
            .await
            .unwrap();
        
        // No headers should have been added since no security schemes are configured
        assert!(!new_http_kwargs.contains_key("headers"));
        
        // Payload should remain unchanged
        assert_eq!(new_payload, serde_json::json!({"test": "data"}));
    }
}
