//! Agent Executor trait implementation
//! 
//! This module defines the AgentExecutor interface which contains the core logic
//! of the agent, executing tasks based on requests and publishing updates to an event queue.

use async_trait::async_trait;
use std::sync::Arc;
use crate::a2a::server::agent_execution::RequestContext;
use crate::a2a::server::events::{EventQueue, Event};
use crate::{A2AError, TaskStatusUpdateEvent, TaskState, Message, Part, Role};

/// Agent Executor interface
/// 
/// Implementations of this interface contain the core logic of the agent,
/// executing tasks based on requests and publishing updates to an event queue.
#[async_trait]
pub trait AgentExecutor: Send + Sync {
    /// Execute the agent's logic for a given request context
    /// 
    /// The agent should read necessary information from the `context` and
    /// publish `Task` or `Message` events, or `TaskStatusUpdateEvent` /
    /// `TaskArtifactUpdateEvent` to the `event_queue`. This method should
    /// return once the agent's execution for this request is complete or
    /// yields control (e.g., enters an input-required state).
    /// 
    /// # Arguments
    /// * `context` - The request context containing the message, task ID, etc.
    /// * `event_queue` - The queue to publish events to
    /// 
    /// # Returns
    /// * `Ok(())` - If execution completed successfully
    /// * `Err(A2AError)` - If an error occurred during execution
    async fn execute(
        &self,
        context: RequestContext,
        event_queue: Arc<dyn EventQueue>,
    ) -> Result<(), A2AError>;

    /// Request the agent to cancel an ongoing task
    /// 
    /// The agent should attempt to stop the task identified by the task_id
    /// in the context and publish a `TaskStatusUpdateEvent` with state
    /// `TaskState.canceled` to the `event_queue`.
    /// 
    /// # Arguments
    /// * `context` - The request context containing the task ID to cancel
    /// * `event_queue` - The queue to publish the cancellation status update to
    /// 
    /// # Returns
    /// * `Ok(())` - If cancellation was requested successfully
    /// * `Err(A2AError)` - If an error occurred during cancellation
    async fn cancel(
        &self,
        context: RequestContext,
        event_queue: Arc<dyn EventQueue>,
    ) -> Result<(), A2AError>;
}

/// A simple mock agent executor for testing purposes
#[derive(Debug, Clone)]
pub struct MockAgentExecutor {
    /// Whether to simulate an error during execution
    pub simulate_error: bool,
    /// Whether to simulate a delay during execution
    pub simulate_delay: bool,
    /// Delay duration in milliseconds
    pub delay_ms: u64,
}

impl MockAgentExecutor {
    /// Creates a new MockAgentExecutor
    pub fn new() -> Self {
        Self {
            simulate_error: false,
            simulate_delay: false,
            delay_ms: 100,
        }
    }

    /// Sets whether to simulate an error
    pub fn with_error(mut self, simulate_error: bool) -> Self {
        self.simulate_error = simulate_error;
        self
    }

    /// Sets whether to simulate a delay
    pub fn with_delay(mut self, simulate_delay: bool, delay_ms: u64) -> Self {
        self.simulate_delay = simulate_delay;
        self.delay_ms = delay_ms;
        self
    }
}

impl Default for MockAgentExecutor {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl AgentExecutor for MockAgentExecutor {
    async fn execute(
        &self,
        context: RequestContext,
        event_queue: Arc<dyn EventQueue>,
    ) -> Result<(), A2AError> {
        // Simulate delay if requested
        if self.simulate_delay {
            tokio::time::sleep(tokio::time::Duration::from_millis(self.delay_ms)).await;
        }

        // Simulate error if requested
        if self.simulate_error {
            return Err(A2AError::internal("Mock agent execution error"));
        }

        // Get task and context IDs
        let task_id = context.task_id.clone().unwrap_or_else(|| "unknown".to_string());
        let context_id = context.context_id.clone().unwrap_or_else(|| "unknown".to_string());

        // Get user input if available
        let user_input = context.get_user_input(" ");

        // Create initial task status
        use crate::a2a::server::events::Event;
        use crate::TaskStatusUpdateEvent;
        use crate::{TaskState, TaskStatus};

        let initial_status = TaskStatusUpdateEvent {
            task_id: task_id.clone(),
            context_id: context_id.clone(),
            status: TaskStatus {
                state: TaskState::Working,
                timestamp: Some(chrono::Utc::now().to_string()),
                message: None, // We'll use the status message field differently
            },
            r#final: false,
            kind: "status-update".to_string(),
            metadata: None,
        };

        event_queue.enqueue_event(Event::TaskStatusUpdate(initial_status)).await?;

        // Simulate some work
        if self.simulate_delay {
            tokio::time::sleep(tokio::time::Duration::from_millis(self.delay_ms)).await;
        }

        // Create final task status
        let final_status = TaskStatusUpdateEvent {
            task_id,
            context_id,
            status: TaskStatus {
                state: TaskState::Completed,
                timestamp: Some(chrono::Utc::now().to_string()),
                message: None,
            },
            r#final: true,
            kind: "status-update".to_string(),
            metadata: None,
        };

        event_queue.enqueue_event(Event::TaskStatusUpdate(final_status)).await?;

        Ok(())
    }

    async fn cancel(
        &self,
        context: RequestContext,
        event_queue: Arc<dyn EventQueue>,
    ) -> Result<(), A2AError> {
        let task_id = context.task_id.clone().unwrap_or_else(|| "unknown".to_string());
        let context_id = context.context_id.clone().unwrap_or_else(|| "unknown".to_string());

        use crate::a2a::server::events::Event;
        use crate::TaskStatusUpdateEvent;
        use crate::{TaskState, TaskStatus};

        let cancel_status = TaskStatusUpdateEvent {
            task_id,
            context_id,
            status: TaskStatus {
                state: TaskState::Canceled,
                timestamp: Some(chrono::Utc::now().to_string()),
                message: None,
            },
            r#final: true,
            kind: "status-update".to_string(),
            metadata: None,
        };

        event_queue.enqueue_event(Event::TaskStatusUpdate(cancel_status)).await?;

        Ok(())
    }
}

/// A simple echo agent executor that echoes back the user input
#[derive(Debug, Clone)]
pub struct EchoAgentExecutor {
    /// Prefix to add to the echoed response
    pub prefix: String,
}

impl EchoAgentExecutor {
    /// Creates a new EchoAgentExecutor
    pub fn new() -> Self {
        Self {
            prefix: "Echo: ".to_string(),
        }
    }

    /// Creates a new EchoAgentExecutor with a custom prefix
    pub fn with_prefix(prefix: String) -> Self {
        Self { prefix }
    }
}

impl Default for EchoAgentExecutor {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl AgentExecutor for EchoAgentExecutor {
    async fn execute(
        &self,
        context: RequestContext,
        event_queue: Arc<dyn EventQueue>,
    ) -> Result<(), A2AError> {
        let task_id = context.task_id.clone().unwrap_or_else(|| "unknown".to_string());
        let context_id = context.context_id.clone().unwrap_or_else(|| "unknown".to_string());

        let user_input = context.get_user_input(" ");
        let echoed_response = format!("{}{}", self.prefix, user_input);

        use crate::a2a::server::events::Event;
        use crate::a2a::models::TaskStatusUpdateEvent;
        use crate::{TaskState, TaskStatus};

        // Initial working status
        let initial_status = TaskStatusUpdateEvent {
            task_id: task_id.clone(),
            context_id: context_id.clone(),
            status: TaskStatus {
                state: TaskState::Working,
                timestamp: Some(chrono::Utc::now().to_string()),
                message: None,
            },
            r#final: false,
            kind: "status-update".to_string(),
            metadata: None,
        };

        event_queue.enqueue_event(Event::TaskStatusUpdate(initial_status)).await?;

        // Create a message event with the echoed response
        use crate::{Message, Part, Role};
        let echo_message = Message::new(
            Role::Agent,
            vec![Part::text(echoed_response)],
        );

        event_queue.enqueue_event(Event::Message(echo_message)).await?;

        // Final completed status
        let final_status = TaskStatusUpdateEvent {
            task_id,
            context_id,
            status: TaskStatus {
                state: TaskState::Completed,
                timestamp: Some(chrono::Utc::now().to_string()),
                message: None,
            },
            r#final: true,
            kind: "status-update".to_string(),
            metadata: None,
        };

        event_queue.enqueue_event(Event::TaskStatusUpdate(final_status)).await?;

        Ok(())
    }

    async fn cancel(
        &self,
        context: RequestContext,
        event_queue: Arc<dyn EventQueue>,
    ) -> Result<(), A2AError> {
        let task_id = context.task_id.clone().unwrap_or_else(|| "unknown".to_string());
        let context_id = context.context_id.clone().unwrap_or_else(|| "unknown".to_string());

        use crate::a2a::server::events::Event;
        use crate::a2a::models::TaskStatusUpdateEvent;
        use crate::{TaskState, TaskStatus};

        let cancel_status = TaskStatusUpdateEvent {
            task_id,
            context_id,
            status: TaskStatus {
                state: TaskState::Canceled,
                timestamp: Some(chrono::Utc::now().to_string()),
                message: None,
            },
            r#final: true,
            kind: "status-update".to_string(),
            metadata: None,
        };

        event_queue.enqueue_event(Event::TaskStatusUpdate(cancel_status)).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::a2a::server::events::InMemoryEventQueue;
    use crate::{Part, Role};

    #[tokio::test]
    async fn test_mock_agent_executor_execute() {
        let executor = MockAgentExecutor::new();
        let queue = Arc::new(InMemoryEventQueue::new().unwrap());
        
        let message = Message::new(
            Role::User,
            vec![Part::text("Hello".to_string())],
        );
        let context = RequestContext::new(
            Some(crate::MessageSendParams {
                message,
                configuration: None,
                metadata: None,
            }),
            Some("task123".to_string()),
            Some("ctx456".to_string()),
            None,
            None,
            None,
            None,
            None,
        ).await.unwrap();

        let result = executor.execute(context, queue.clone()).await;
        assert!(result.is_ok());

        // Check that events were enqueued
        let event1: crate::a2a::server::events::Event = queue.dequeue_event(false).await.unwrap();
        let event2: crate::a2a::server::events::Event = queue.dequeue_event(false).await.unwrap();
        
        match event1 {
            Event::TaskStatusUpdate(status) => {
                assert_eq!(status.task_id, "task123");
                assert_eq!(status.context_id, "ctx456");
                assert_eq!(status.status.state, TaskState::Working);
            }
            _ => panic!("Expected TaskStatusUpdate event"),
        }

        match event2 {
            Event::TaskStatusUpdate(status) => {
                assert_eq!(status.task_id, "task123");
                assert_eq!(status.status.state, TaskState::Completed);
            }
            _ => panic!("Expected TaskStatusUpdate event"),
        }
    }

    #[tokio::test]
    async fn test_mock_agent_executor_cancel() {
        let executor = MockAgentExecutor::new();
        let queue = Arc::new(InMemoryEventQueue::new().unwrap());
        
        let context = RequestContext::new(
            None,
            Some("task123".to_string()),
            Some("ctx456".to_string()),
            None,
            None,
            None,
            None,
            None,
        ).await.unwrap();

        let result = executor.cancel(context, queue.clone()).await;
        assert!(result.is_ok());

        let event: crate::a2a::server::events::Event = queue.dequeue_event(false).await.unwrap();
        match event {
            Event::TaskStatusUpdate(status) => {
                assert_eq!(status.task_id, "task123");
                assert_eq!(status.status.state, TaskState::Canceled);
            }
            _ => panic!("Expected TaskStatusUpdate event"),
        }
    }

    #[tokio::test]
    async fn test_mock_agent_executor_error() {
        let executor = MockAgentExecutor::new().with_error(true);
        let queue = Arc::new(InMemoryEventQueue::new().unwrap());
        
        let context = RequestContext::new(
            None,
            Some("task123".to_string()),
            Some("ctx456".to_string()),
            None,
            None,
            None,
            None,
            None,
        ).await.unwrap();

        let result = executor.execute(context, queue).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_echo_agent_executor() {
        let executor = EchoAgentExecutor::new();
        let queue = Arc::new(InMemoryEventQueue::new().await.unwrap());
        
        let message = Message::new(
            Role::User,
            vec![Part::text("Hello World".to_string())],
        );
        let context = RequestContext::new(
            Some(crate::MessageSendParams {
                message,
                configuration: None,
                metadata: None,
            }),
            Some("task123".to_string()),
            Some("ctx456".to_string()),
            None,
            None,
            None,
            None,
            None,
        ).await.unwrap();

        let result = executor.execute(context, queue.clone()).await;
        assert!(result.is_ok());

        // Should have 3 events: Working status, Message, Completed status
        let event1: crate::a2a::server::events::Event = queue.dequeue_event().await.unwrap();
        let event2: crate::a2a::server::events::Event = queue.dequeue_event().await.unwrap();
        let event3: crate::a2a::server::events::Event = queue.dequeue_event().await.unwrap();

        match &event1 {
            Event::TaskStatusUpdate(status) => {
                assert_eq!(status.status.state, TaskState::Working);
            }
            _ => panic!("Expected TaskStatusUpdate event"),
        }

        match &event2 {
            Event::Message(message) => {
                assert_eq!(message.role, Role::Agent);
                assert_eq!(message.parts.len(), 1);
                if let crate::PartRoot::Text(text_part) = &message.parts[0].root() {
                    assert_eq!(text_part.text, "Echo: Hello World");
                } else {
                    panic!("Expected Text part");
                }
            }
            _ => panic!("Expected Message event"),
        }

        match &event3 {
            Event::TaskStatusUpdate(status) => {
                assert_eq!(status.status.state, TaskState::Completed);
            }
            _ => panic!("Expected TaskStatusUpdate event"),
        }
    }

    #[tokio::test]
    async fn test_echo_agent_executor_with_custom_prefix() {
        let executor = EchoAgentExecutor::with_prefix("Reply: ".to_string());
        let queue = Arc::new(InMemoryEventQueue::new().await.unwrap());
        
        let message = Message::new(
            Role::User,
            vec![Part::text("Test".to_string())],
        );
        let context = RequestContext::new(
            Some(crate::MessageSendParams {
                message,
                configuration: None,
                metadata: None,
            }),
            Some("task123".to_string()),
            Some("ctx456".to_string()),
            None,
            None,
            None,
            None,
            None,
        ).await.unwrap();

        executor.execute(context, queue.clone()).await.unwrap();

        // Skip the first event (working status)
        queue.dequeue_event().await.unwrap();
        
        let event2: crate::a2a::server::events::Event = queue.dequeue_event().await.unwrap();
        match &event2 {
            Event::Message(message) => {
                if let crate::PartRoot::Text(text_part) = &message.parts[0].root() {
                    assert_eq!(text_part.text, "Reply: Test");
                } else {
                    panic!("Expected Text part");
                }
            }
            _ => panic!("Expected Message event"),
        }
    }
}
