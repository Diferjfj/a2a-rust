//! Credential service for client authentication
//! 
//! This module provides credential management functionality
//! matching a2a-python's credential service.

use crate::a2a::client::client_trait::ClientCallContext;
use crate::a2a::error::A2AError;
use async_trait::async_trait;
use std::collections::HashMap;

/// Trait for providing credentials for authentication
#[async_trait]
pub trait CredentialService: Send + Sync {
    /// Get credentials for a specific scheme and context
    async fn get_credentials(
        &self,
        scheme_name: &str,
        context: Option<&ClientCallContext>,
    ) -> Result<Option<String>, A2AError>;
}

/// In-memory credential store for contexts
#[derive(Debug, Clone)]
pub struct InMemoryContextCredentialStore {
    /// Default credentials to use when context-specific ones aren't found
    default_credentials: HashMap<String, String>,
}

impl InMemoryContextCredentialStore {
    /// Create a new in-memory credential store
    pub fn new() -> Self {
        Self {
            default_credentials: HashMap::new(),
        }
    }
    
    /// Add a credential for a specific scheme
    pub fn add_credential(&mut self, scheme: impl Into<String>, credential: impl Into<String>) {
        self.default_credentials.insert(scheme.into(), credential.into());
    }
    
    /// Add multiple credentials for different schemes
    pub fn add_credentials<I, K, V>(&mut self, credentials: I)
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<String>,
    {
        for (scheme, credential) in credentials {
            self.add_credential(scheme, credential);
        }
    }
}

impl Default for InMemoryContextCredentialStore {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl CredentialService for InMemoryContextCredentialStore {
    async fn get_credentials(
        &self,
        scheme_name: &str,
        _context: Option<&ClientCallContext>,
    ) -> Result<Option<String>, A2AError> {
        // For now, just return default credentials
        // In a more sophisticated implementation, we could use context
        // to look up context-specific credentials
        Ok(self.default_credentials.get(scheme_name).cloned())
    }
}

/// Environment-based credential service
#[derive(Debug, Clone)]
pub struct EnvironmentCredentialService {
    /// Prefix for environment variables
    prefix: String,
}

impl EnvironmentCredentialService {
    /// Create a new environment credential service with a custom prefix
    pub fn new(prefix: impl Into<String>) -> Self {
        Self {
            prefix: prefix.into(),
        }
    }
    
    /// Create a new environment credential service with default prefix "A2A_"
    pub fn default() -> Self {
        Self::new("A2A_")
    }
    
    /// Get environment variable name for a scheme
    fn env_var_name(&self, scheme_name: &str) -> String {
        format!("{}{}", self.prefix, scheme_name.to_uppercase().replace("-", "_"))
    }
}

#[async_trait]
impl CredentialService for EnvironmentCredentialService {
    async fn get_credentials(
        &self,
        scheme_name: &str,
        _context: Option<&ClientCallContext>,
    ) -> Result<Option<String>, A2AError> {
        let env_var = self.env_var_name(scheme_name);
        Ok(std::env::var(&env_var).ok())
    }
}

/// Composite credential service that tries multiple services in order
pub struct CompositeCredentialService {
    /// List of credential services to try in order
    services: Vec<Box<dyn CredentialService>>,
}

impl CompositeCredentialService {
    /// Create a new composite credential service
    pub fn new() -> Self {
        Self {
            services: Vec::new(),
        }
    }
    
    /// Add a credential service to the composite
    pub fn add_service(mut self, service: Box<dyn CredentialService>) -> Self {
        self.services.push(service);
        self
    }
    
    /// Create from a vector of services
    pub fn from_services(services: Vec<Box<dyn CredentialService>>) -> Self {
        Self { services }
    }
}

impl Default for CompositeCredentialService {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl CredentialService for CompositeCredentialService {
    async fn get_credentials(
        &self,
        scheme_name: &str,
        context: Option<&ClientCallContext>,
    ) -> Result<Option<String>, A2AError> {
        for service in &self.services {
            if let Ok(Some(credential)) = service.get_credentials(scheme_name, context).await {
                return Ok(Some(credential));
            }
        }
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_in_memory_credential_store() {
        let mut store = InMemoryContextCredentialStore::new();
        store.add_credential("Bearer", "test-token");
        store.add_credential("api-key", "test-key");

        let credential = store.get_credentials("Bearer", None).await.unwrap();
        assert_eq!(credential, Some("test-token".to_string()));

        let credential = store.get_credentials("api-key", None).await.unwrap();
        assert_eq!(credential, Some("test-key".to_string()));

        let credential = store.get_credentials("unknown", None).await.unwrap();
        assert_eq!(credential, None);
    }

    #[tokio::test]
    async fn test_environment_credential_service() {
        // Set environment variables for testing
        std::env::set_var("A2A_BEARER", "env-token");
        std::env::set_var("A2A_API_KEY", "env-key");

        let service = EnvironmentCredentialService::default();

        let credential = service.get_credentials("Bearer", None).await.unwrap();
        assert_eq!(credential, Some("env-token".to_string()));

        let credential = service.get_credentials("api-key", None).await.unwrap();
        assert_eq!(credential, Some("env-key".to_string()));

        // Clean up environment variables
        std::env::remove_var("A2A_BEARER");
        std::env::remove_var("A2A_API_KEY");
    }

    #[tokio::test]
    async fn test_composite_credential_service() {
        let mut memory_store = InMemoryContextCredentialStore::new();
        memory_store.add_credential("api-key", "memory-key");

        let services: Vec<Box<dyn CredentialService>> = vec![
            Box::new(memory_store),
            Box::new(EnvironmentCredentialService::default()),
        ];

        let composite = CompositeCredentialService::from_services(services);

        // Should find the first available credential
        let credential = composite.get_credentials("api-key", None).await.unwrap();
        assert_eq!(credential, Some("memory-key".to_string()));
    }
}
