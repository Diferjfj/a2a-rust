//! Utility functions for creating A2A Task objects
//! 
//! This module provides helper functions that match the functionality
//! in a2a-python/src/a2a/utils/task.py

use crate::a2a::core_types::{Message, TaskState, TaskStatus};
use crate::a2a::models::{Artifact, Task};
use crate::a2a::error::A2AError;
use uuid::Uuid;

/// Creates a new Task object from an initial user message
/// 
/// Generates task and context IDs if not provided in the message.
/// Matches the Python function `new_task`
pub fn new_task(request: Message) -> Result<Task, A2AError> {
    // Validate message role
    if request.role == crate::a2a::core_types::Role::Agent {
        return Err(A2AError::invalid_params("Message role cannot be Agent for new task"));
    }

    // Validate message parts
    if request.parts.is_empty() {
        return Err(A2AError::invalid_params("Message parts cannot be empty"));
    }

    // Validate text part content
    for part in &request.parts {
        match part.root() {
            crate::a2a::core_types::PartRoot::Text(text_part) => {
                if text_part.text.is_empty() {
                    return Err(A2AError::invalid_params("TextPart content cannot be empty"));
                }
            }
            _ => {} // Other part types are fine
        }
    }

    let task_id = request.task_id.clone()
        .unwrap_or_else(|| Uuid::new_v4().to_string());
    let context_id = request.context_id.clone()
        .unwrap_or_else(|| Uuid::new_v4().to_string());

    Ok(Task::new(
        context_id,
        TaskStatus::new(TaskState::Submitted),
    )
        .with_task_id(task_id)
        .with_history(vec![request]))
}

/// Creates a Task object in the 'completed' state
/// 
/// Useful for constructing a final Task representation when the agent
/// finishes and produces artifacts.
/// Matches the Python function `completed_task`
pub fn completed_task(
    task_id: String,
    context_id: String,
    artifacts: Vec<Artifact>,
    history: Option<Vec<Message>>,
) -> Result<Task, A2AError> {
    // Validate artifacts
    if artifacts.is_empty() {
        return Err(A2AError::invalid_params(
            "artifacts must be a non-empty list of Artifact objects"
        ));
    }

    Ok(Task::new(
        context_id,
        TaskStatus::new(TaskState::Completed),
    )
        .with_task_id(task_id)
        .with_artifacts(artifacts)
        .with_history(history.unwrap_or_default()))
}

/// Applies history_length parameter on task and returns a new task object
/// 
/// Matches the Python function `apply_history_length`
pub fn apply_history_length(task: Task, history_length: Option<i32>) -> Task {
    // Apply history_length parameter if specified
    if let Some(length) = history_length {
        if length > 0 {
            if let Some(ref history) = task.history {
                if !history.is_empty() {
                    // Limit history to the most recent N messages
                    let limited_history = history.iter()
                        .rev()
                        .take(length as usize)
                        .cloned()
                        .collect::<Vec<_>>()
                        .into_iter()
                        .rev()
                        .collect();

                    // Create a new task instance with limited history
                    return Task {
                        history: Some(limited_history),
                        ..task
                    };
                }
            }
        }
    }

    task
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::a2a::core_types::{Role, Part};

    #[test]
    fn test_new_task_success() {
        let message = Message::new(
            Role::User,
            vec![Part::text("Hello".to_string())],
        );

        let task = new_task(message.clone()).unwrap();

        assert_eq!(task.status.state, TaskState::Submitted);
        assert!(task.history.is_some());
        assert_eq!(task.history.as_ref().unwrap().len(), 1);
        assert_eq!(task.history.as_ref().unwrap()[0], message);
    }

    #[test]
    fn test_new_task_with_ids() {
        let message = Message::new(
            Role::User,
            vec![Part::text("Hello".to_string())],
        )
            .with_task_id("task-123".to_string())
            .with_context_id("ctx-456".to_string());

        let task = new_task(message.clone()).unwrap();

        assert_eq!(task.id, "task-123");
        assert_eq!(task.context_id, "ctx-456");
    }

    #[test]
    fn test_new_task_agent_role_fails() {
        let message = Message::new(
            Role::Agent,
            vec![Part::text("Hello".to_string())],
        );

        let result = new_task(message);
        assert!(result.is_err());
    }

    #[test]
    fn test_new_task_empty_parts_fails() {
        let message = Message::new(Role::User, vec![]);

        let result = new_task(message);
        assert!(result.is_err());
    }

    #[test]
    fn test_new_task_empty_text_fails() {
        let message = Message::new(
            Role::User,
            vec![Part::text("".to_string())],
        );

        let result = new_task(message);
        assert!(result.is_err());
    }

    #[test]
    fn test_completed_task_success() {
        let artifact = Artifact::new(vec![Part::text("Result".to_string())]);

        let task = completed_task(
            "task-123".to_string(),
            "ctx-456".to_string(),
            vec![artifact],
            None,
        ).unwrap();

        assert_eq!(task.id, "task-123");
        assert_eq!(task.context_id, "ctx-456");
        assert_eq!(task.status.state, TaskState::Completed);
        assert!(task.artifacts.is_some());
        assert_eq!(task.artifacts.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn test_completed_task_empty_artifacts_fails() {
        let result = completed_task(
            "task-123".to_string(),
            "ctx-456".to_string(),
            vec![],
            None,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_apply_history_length() {
        let messages = vec![
            Message::new(Role::User, vec![Part::text("1".to_string())]),
            Message::new(Role::Agent, vec![Part::text("2".to_string())]),
            Message::new(Role::User, vec![Part::text("3".to_string())]),
        ];

        let task = Task::new(
            "ctx-123".to_string(),
            TaskStatus::new(TaskState::Working),
        )
            .with_history(messages.clone());

        // Apply history length of 2
        let limited_task = apply_history_length(task, Some(2));

        assert!(limited_task.history.is_some());
        assert_eq!(limited_task.history.as_ref().unwrap().len(), 2);
        // Should keep the last 2 messages
        assert_eq!(limited_task.history.as_ref().unwrap()[0], messages[1]);
        assert_eq!(limited_task.history.as_ref().unwrap()[1], messages[2]);
    }

    #[test]
    fn test_apply_history_length_zero() {
        let messages = vec![
            Message::new(Role::User, vec![Part::text("1".to_string())]),
            Message::new(Role::Agent, vec![Part::text("2".to_string())]),
        ];

        let task = Task::new(
            "ctx-123".to_string(),
            TaskStatus::new(TaskState::Working),
        )
            .with_history(messages);

        // Apply history length of 0 should return original task
        let limited_task = apply_history_length(task, Some(0));

        assert!(limited_task.history.is_some());
        assert_eq!(limited_task.history.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn test_apply_history_length_none() {
        let messages = vec![
            Message::new(Role::User, vec![Part::text("1".to_string())]),
            Message::new(Role::Agent, vec![Part::text("2".to_string())]),
        ];

        let task = Task::new(
            "ctx-123".to_string(),
            TaskStatus::new(TaskState::Working),
        )
            .with_history(messages.clone());

        // Apply None history length should return original task
        let limited_task = apply_history_length(task, None);

        assert!(limited_task.history.is_some());
        assert_eq!(limited_task.history.as_ref().unwrap().len(), 2);
    }
}
