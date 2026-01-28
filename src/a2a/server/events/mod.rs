//! Event system for A2A server
//! 
//! This module provides the event queue system that handles asynchronous
//! communication between the agent executor and request handlers.

pub mod event_queue;
pub mod event_consumer;
pub mod queue_manager;
pub mod in_memory_queue_manager;
pub mod in_memory_queue;

pub use event_queue::{Event, EventQueue, QueueConfig, QueueError};
pub use event_consumer::EventConsumer;
pub use queue_manager::{QueueManager, QueueManagerConfig, QueueManagerError, validate_queue_id};
pub use in_memory_queue_manager::InMemoryQueueManager;
pub use in_memory_queue::{InMemoryEventQueue, InMemoryEventQueueChild};
