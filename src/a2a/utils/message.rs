//! Utility functions for creating and handling A2A Message objects
//! 
//! This module provides helper functions that match the functionality
//! in a2a-python/src/a2a/utils/message.py

use crate::a2a::core_types::{Message, Part, Role};

/// Creates a new agent message containing a single TextPart
/// 
/// Matches the Python function `new_agent_text_message`
pub fn new_agent_text_message(
    text: String,
    context_id: Option<String>,
    task_id: Option<String>,
) -> Message {
    Message::new(Role::Agent, vec![Part::text(text)])
        .with_context_id(context_id.unwrap_or_default())
        .with_task_id(task_id.unwrap_or_default())
}

/// Creates a new agent message containing a list of Parts
/// 
/// Matches the Python function `new_agent_parts_message`
pub fn new_agent_parts_message(
    parts: Vec<Part>,
    context_id: Option<String>,
    task_id: Option<String>,
) -> Message {
    Message::new(Role::Agent, parts)
        .with_context_id(context_id.unwrap_or_default())
        .with_task_id(task_id.unwrap_or_default())
}

/// Extracts and joins all text content from a Message's parts
/// 
/// Matches the Python function `get_message_text`
pub fn get_message_text(message: &Message, delimiter: &str) -> String {
    get_text_parts(&message.parts).join(delimiter)
}

/// Extracts text content from all TextPart objects in a list of Parts
/// 
/// Matches the Python function `get_text_parts`
pub fn get_text_parts(parts: &[Part]) -> Vec<String> {
    parts
        .iter()
        .filter_map(|part| {
            match part.root() {
                crate::a2a::core_types::PartRoot::Text(text_part) => Some(text_part.text.clone()),
                _ => None,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_agent_text_message() {
        let message = new_agent_text_message(
            "Hello".to_string(),
            Some("ctx-123".to_string()),
            Some("task-456".to_string()),
        );

        assert_eq!(message.role, Role::Agent);
        assert_eq!(message.context_id, Some("ctx-123".to_string()));
        assert_eq!(message.task_id, Some("task-456".to_string()));
        assert_eq!(message.parts.len(), 1);
        
        match &message.parts[0].root() {
            crate::PartRoot::Text(text_part) => assert_eq!(text_part.text, "Hello"),
            _ => panic!("Expected TextPart"),
        }
    }

    #[test]
    fn test_get_message_text() {
        let message = Message::new(
            Role::User,
            vec![
                Part::text("Hello".to_string()),
                Part::text("World".to_string()),
            ],
        );

        let text = get_message_text(&message, " ");
        assert_eq!(text, "Hello World");
    }

    #[test]
    fn test_get_text_parts() {
        let parts = vec![
            Part::text("Hello".to_string()),
            Part::data(serde_json::json!({"key": "value"})),
            Part::text("World".to_string()),
        ];

        let text_parts = get_text_parts(&parts);
        assert_eq!(text_parts, vec!["Hello", "World"]);
    }
}
