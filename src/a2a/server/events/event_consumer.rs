//! Event consumer for processing events from event queues
//! 
//! This module provides the EventConsumer that processes events from EventQueues
//! and forwards them to appropriate handlers.

use crate::a2a::error::A2AError;
use crate::a2a::server::events::{Event, EventQueue};
use async_trait::async_trait;
use std::pin::Pin;
use std::sync::Arc;
use futures::Stream;

/// Consumer for events from an event queue
pub struct EventConsumer {
    queue: Arc<dyn EventQueue>,
}

impl EventConsumer {
    /// Create a new event consumer for the given queue
    pub fn new(queue: Arc<dyn EventQueue>) -> Self {
        Self { queue }
    }

    /// Get the underlying queue
    pub fn queue(&self) -> &Arc<dyn EventQueue> {
        &self.queue
    }

    /// Callback for when the agent task is done
    pub fn agent_task_callback(&self) {
        // This can be used to clean up resources when the agent task completes
        tracing::debug!("Agent task completed");
    }

    /// Consume a single event from the queue
    pub async fn consume_one(&self) -> Result<Event, A2AError> {
        self.queue.dequeue_event(false).await
    }

    /// Try to consume a single event without waiting
    pub async fn try_consume_one(&self) -> Result<Event, A2AError> {
        self.queue.dequeue_event(true).await
    }
}

/// Trait for event processing strategies
#[async_trait]
pub trait EventProcessor: Send + Sync {
    /// Process a single event
    async fn process_event(&self, event: Event) -> Result<(), A2AError>;
}

/// Stream of events from an event queue
pub struct EventStream {
    consumer: EventConsumer,
}

impl EventStream {
    /// Create a new event stream
    pub fn new(consumer: EventConsumer) -> Self {
        Self { consumer }
    }
}

impl Stream for EventStream {
    type Item = Result<Event, A2AError>;

    fn poll_next(self: Pin<&mut Self>, _cx: &mut std::task::Context<'_>) -> std::task::Poll<Option<Self::Item>> {
        // This is a simplified implementation
        // In a real implementation, this would use proper async notification
        if self.consumer.queue().is_closed() && self.consumer.queue().size() == 0 {
            std::task::Poll::Ready(None)
        } else {
            std::task::Poll::Pending
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::a2a::server::events::InMemoryEventQueue;
    use crate::a2a::core_types::*;

    #[tokio::test]
    async fn test_event_consumer() {
        let queue = InMemoryEventQueue::new().unwrap();
        let consumer = EventConsumer::new(Arc::new(queue));

        // Test that we can create a consumer
        assert_eq!(consumer.queue().size(), 0);
    }

    struct TestProcessor {
        events_processed: Arc<std::sync::atomic::AtomicUsize>,
    }

    #[async_trait]
    impl EventProcessor for TestProcessor {
        async fn process_event(&self, _event: Event) -> Result<(), A2AError> {
            self.events_processed.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_event_processor() {
        let events_processed = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let processor = TestProcessor {
            events_processed: events_processed.clone(),
        };

        let event = Event::Message(Message::new(
            Role::User,
            vec![Part::text("Hello".to_string())],
        ));

        processor.process_event(event).await.unwrap();
        assert_eq!(events_processed.load(std::sync::atomic::Ordering::Relaxed), 1);
    }
}
