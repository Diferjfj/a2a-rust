//! ID Generator implementation
//! 
//! This module provides interfaces and implementations for generating unique identifiers
//! for tasks and contexts in the A2A server.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

/// Context for providing additional information to ID generators
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IDGeneratorContext {
    /// Optional task ID
    pub task_id: Option<String>,
    /// Optional context ID
    pub context_id: Option<String>,
}

impl Default for IDGeneratorContext {
    fn default() -> Self {
        Self {
            task_id: None,
            context_id: None,
        }
    }
}

impl IDGeneratorContext {
    /// Creates a new IDGeneratorContext
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new IDGeneratorContext with task_id
    pub fn with_task_id(task_id: String) -> Self {
        Self {
            task_id: Some(task_id),
            context_id: None,
        }
    }

    /// Creates a new IDGeneratorContext with context_id
    pub fn with_context_id(context_id: String) -> Self {
        Self {
            task_id: None,
            context_id: Some(context_id),
        }
    }

    /// Creates a new IDGeneratorContext with both task_id and context_id
    pub fn with_ids(task_id: String, context_id: String) -> Self {
        Self {
            task_id: Some(task_id),
            context_id: Some(context_id),
        }
    }
}

/// Interface for generating unique identifiers
#[async_trait]
pub trait IDGenerator: Send + Sync {
    /// Generates a unique identifier
    /// 
    /// # Arguments
    /// * `context` - Additional context information that may influence ID generation
    /// 
    /// # Returns
    /// A unique identifier string
    async fn generate(&self, context: &IDGeneratorContext) -> Result<String, crate::A2AError>;
}

/// UUID implementation of the IDGenerator interface
#[derive(Debug, Clone)]
pub struct UUIDGenerator;

impl UUIDGenerator {
    /// Creates a new UUIDGenerator
    pub fn new() -> Self {
        Self
    }
}

impl Default for UUIDGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl IDGenerator for UUIDGenerator {
    async fn generate(&self, _context: &IDGeneratorContext) -> Result<String, crate::A2AError> {
        Ok(Uuid::new_v4().to_string())
    }
}

/// Sequential ID generator that generates incrementing IDs
/// Useful for testing and scenarios where predictable IDs are desired
#[derive(Debug, Clone)]
pub struct SequentialIDGenerator {
    next_id: Arc<std::sync::atomic::AtomicU64>,
}

impl SequentialIDGenerator {
    /// Creates a new SequentialIDGenerator starting from 1
    pub fn new() -> Self {
        Self {
            next_id: Arc::new(std::sync::atomic::AtomicU64::new(1)),
        }
    }

    /// Creates a new SequentialIDGenerator starting from the given value
    pub fn with_start(start: u64) -> Self {
        Self {
            next_id: Arc::new(std::sync::atomic::AtomicU64::new(start)),
        }
    }

    /// Gets the next ID without generating it (for testing)
    pub fn peek_next_id(&self) -> u64 {
        self.next_id.load(std::sync::atomic::Ordering::SeqCst)
    }
}

impl Default for SequentialIDGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl IDGenerator for SequentialIDGenerator {
    async fn generate(&self, _context: &IDGeneratorContext) -> Result<String, crate::A2AError> {
        let id = self.next_id.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        Ok(id.to_string())
    }
}

/// Prefix-based ID generator that combines a prefix with a UUID
#[derive(Debug, Clone)]
pub struct PrefixedUUIDGenerator {
    prefix: String,
}

impl PrefixedUUIDGenerator {
    /// Creates a new PrefixedUUIDGenerator with the given prefix
    pub fn new(prefix: String) -> Self {
        Self { prefix }
    }
}

#[async_trait]
impl IDGenerator for PrefixedUUIDGenerator {
    async fn generate(&self, _context: &IDGeneratorContext) -> Result<String, crate::A2AError> {
        let uuid = Uuid::new_v4().to_string();
        Ok(format!("{}_{}", self.prefix, uuid))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_uuid_generator() {
        let generator = UUIDGenerator::new();
        let context = IDGeneratorContext::new();
        
        let id1 = generator.generate(&context).await.unwrap();
        let id2 = generator.generate(&context).await.unwrap();
        
        assert_ne!(id1, id2);
        assert_eq!(id1.len(), 36); // Standard UUID length
        assert_eq!(id2.len(), 36);
        
        // Verify it's a valid UUID format
        Uuid::parse_str(&id1).unwrap();
        Uuid::parse_str(&id2).unwrap();
    }

    #[tokio::test]
    async fn test_sequential_generator() {
        let generator = SequentialIDGenerator::new();
        let context = IDGeneratorContext::new();
        
        let id1 = generator.generate(&context).await.unwrap();
        let id2 = generator.generate(&context).await.unwrap();
        let id3 = generator.generate(&context).await.unwrap();
        
        assert_eq!(id1, "1");
        assert_eq!(id2, "2");
        assert_eq!(id3, "3");
        
        // Test start value
        let generator2 = SequentialIDGenerator::with_start(100);
        let id4 = generator2.generate(&context).await.unwrap();
        assert_eq!(id4, "100");
    }

    #[tokio::test]
    async fn test_prefixed_uuid_generator() {
        let generator = PrefixedUUIDGenerator::new("task".to_string());
        let context = IDGeneratorContext::new();
        
        let id1 = generator.generate(&context).await.unwrap();
        let id2 = generator.generate(&context).await.unwrap();
        
        assert_ne!(id1, id2);
        assert!(id1.starts_with("task_"));
        assert!(id2.starts_with("task_"));
        
        // Extract UUID part and verify it's valid
        let uuid_part1 = &id1[5..];
        let uuid_part2 = &id2[5..];
        
        assert_eq!(uuid_part1.len(), 36);
        assert_eq!(uuid_part2.len(), 36);
        
        Uuid::parse_str(uuid_part1).unwrap();
        Uuid::parse_str(uuid_part2).unwrap();
    }

    #[tokio::test]
    async fn test_id_generator_context() {
        // Test default context
        let context = IDGeneratorContext::default();
        assert!(context.task_id.is_none());
        assert!(context.context_id.is_none());

        // Test with task_id
        let context = IDGeneratorContext::with_task_id("task123".to_string());
        assert_eq!(context.task_id, Some("task123".to_string()));
        assert!(context.context_id.is_none());

        // Test with context_id
        let context = IDGeneratorContext::with_context_id("ctx456".to_string());
        assert!(context.task_id.is_none());
        assert_eq!(context.context_id, Some("ctx456".to_string()));

        // Test with both
        let context = IDGeneratorContext::with_ids("task123".to_string(), "ctx456".to_string());
        assert_eq!(context.task_id, Some("task123".to_string()));
        assert_eq!(context.context_id, Some("ctx456".to_string()));
    }

    #[tokio::test]
    async fn test_generator_with_context() {
        let generator = UUIDGenerator::new();
        
        // Test with different contexts - should still generate unique IDs
        let context1 = IDGeneratorContext::with_task_id("task1".to_string());
        let context2 = IDGeneratorContext::with_context_id("ctx1".to_string());
        let context3 = IDGeneratorContext::with_ids("task2".to_string(), "ctx2".to_string());
        
        let id1 = generator.generate(&context1).await.unwrap();
        let id2 = generator.generate(&context2).await.unwrap();
        let id3 = generator.generate(&context3).await.unwrap();
        
        assert_ne!(id1, id2);
        assert_ne!(id2, id3);
        assert_ne!(id1, id3);
    }

    #[test]
    fn test_sequential_generator_peek() {
        let generator = SequentialIDGenerator::with_start(42);
        assert_eq!(generator.peek_next_id(), 42);
    }

    #[tokio::test]
    async fn test_concurrent_generation() {
        let generator = Arc::new(UUIDGenerator::new());
        let context = IDGeneratorContext::new();
        
        // Generate IDs concurrently
        let mut handles = Vec::new();
        for _ in 0..10 {
            let gen = generator.clone();
            let ctx = context.clone();
            handles.push(tokio::spawn(async move {
                gen.generate(&ctx).await.unwrap()
            }));
        }
        
        let mut ids = Vec::new();
        for handle in handles {
            ids.push(handle.await.unwrap());
        }
        
        // All IDs should be unique
        let mut unique_ids = std::collections::HashSet::new();
        for id in ids {
            assert!(unique_ids.insert(id.clone()), "Duplicate ID found: {}", id);
        }
    }
}
