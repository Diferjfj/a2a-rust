//! Interoperability tests between Python and Rust A2A implementations
//! 
//! This module ensures that the Rust implementation produces JSON output
//! that is compatible with the Python implementation and vice versa.

use serde_json;
use a2a_rust::{
    Message, Part, Role, Task, TaskStatus, TaskState,
    TaskStatusUpdateEvent, Artifact,
    PushNotificationConfig, TaskPushNotificationConfig,
    DeleteTaskPushNotificationConfigParams, GetTaskPushNotificationConfigParams,
    ListTaskPushNotificationConfigParams,
};
use url::Url;
use std::collections::HashMap;

#[test]
fn test_message_serialization_compatibility() {
    // Create a message that matches Python's Message structure
    let message = Message {
        kind: "message".to_string(),
        message_id: "msg-123".to_string(),
        role: Role::User,
        parts: vec![
            Part::text("Hello, world!".to_string()),
            Part::data(serde_json::json!({"key": "value"})),
        ],
        context_id: Some("ctx-456".to_string()),
        task_id: Some("task-789".to_string()),
        metadata: None,
        extensions: None,
        reference_task_ids: None,
    };

    // Serialize to JSON
    let json = serde_json::to_string_pretty(&message).expect("Failed to serialize message");
    
    // Verify the JSON structure matches Python expectations
    let parsed: serde_json::Value = serde_json::from_str(&json).expect("Failed to parse JSON");
    
    assert_eq!(parsed["kind"], "message");
    assert_eq!(parsed["messageId"], "msg-123");
    assert_eq!(parsed["role"], "user");
    assert_eq!(parsed["contextId"], "ctx-456");
    assert_eq!(parsed["taskId"], "task-789");
    assert!(parsed["parts"].is_array());
    assert_eq!(parsed["parts"].as_array().unwrap().len(), 2);
    
    // Verify part structure
    let first_part = &parsed["parts"][0];
    assert_eq!(first_part["kind"], "text");
    assert_eq!(first_part["text"], "Hello, world!");
    
    let second_part = &parsed["parts"][1];
    assert_eq!(second_part["kind"], "data");
    assert_eq!(second_part["data"]["key"], "value");
}

#[test]
fn test_task_serialization_compatibility() {
    // Create a task that matches Python's Task structure
    let task = Task {
        kind: "task".to_string(),
        id: "task-123".to_string(),
        context_id: "ctx-456".to_string(),
        status: TaskStatus {
            state: TaskState::Working,
            timestamp: Some("2023-10-27T10:00:00Z".to_string()),
            message: None,
        },
        artifacts: Some(vec![
            Artifact {
                artifact_id: "artifact-789".to_string(),
                name: Some("Test Artifact".to_string()),
                description: Some("A test artifact".to_string()),
                parts: vec![Part::text("Artifact content".to_string())],
                metadata: None,
                extensions: None,
            }
        ]),
        history: Some(vec![
            Message {
                kind: "message".to_string(),
                message_id: "msg-456".to_string(),
                role: Role::User,
                parts: vec![Part::text("Initial message".to_string())],
                context_id: Some("ctx-456".to_string()),
                task_id: Some("task-123".to_string()),
                metadata: None,
                extensions: None,
                reference_task_ids: None,
            }
        ]),
        metadata: None,
    };

    // Serialize to JSON
    let json = serde_json::to_string_pretty(&task).expect("Failed to serialize task");
    
    // Verify the JSON structure matches Python expectations
    let parsed: serde_json::Value = serde_json::from_str(&json).expect("Failed to parse JSON");
    
    assert_eq!(parsed["kind"], "task");
    assert_eq!(parsed["id"], "task-123");
    assert_eq!(parsed["context_id"], "ctx-456");
    assert_eq!(parsed["status"]["state"], "working");
    assert_eq!(parsed["status"]["timestamp"], "2023-10-27T10:00:00Z");
    assert!(parsed["artifacts"].is_array());
    assert_eq!(parsed["artifacts"].as_array().unwrap().len(), 1);
    assert!(parsed["history"].is_array());
    assert_eq!(parsed["history"].as_array().unwrap().len(), 1);
}

#[test]
fn test_task_status_update_event_compatibility() {
    let event = TaskStatusUpdateEvent {
        kind: "status-update".to_string(),
        task_id: "task-123".to_string(),
        context_id: "ctx-456".to_string(),
        status: TaskStatus {
            state: TaskState::Completed,
            timestamp: Some("2023-10-27T11:00:00Z".to_string()),
            message: None,
        },
        r#final: true,
        metadata: None,
    };

    // Serialize to JSON
    let json = serde_json::to_string_pretty(&event).expect("Failed to serialize event");
    
    // Verify the JSON structure matches Python expectations
    let parsed: serde_json::Value = serde_json::from_str(&json).expect("Failed to parse JSON");
    
    assert_eq!(parsed["kind"], "status-update");
    assert_eq!(parsed["task_id"], "task-123");
    assert_eq!(parsed["context_id"], "ctx-456");
    assert_eq!(parsed["status"]["state"], "completed");
    assert_eq!(parsed["final"], true);
}

#[test]
fn test_push_notification_config_compatibility() {
    let url = Url::parse("https://example.com/webhook").expect("Invalid URL");
    let push_notification_config = PushNotificationConfig {
        id: Some("config-123".to_string()),
        url,
        token: Some("token-456".to_string()),
        authentication: None,
    };

    // Serialize to JSON
    let json = serde_json::to_string_pretty(&push_notification_config).expect("Failed to serialize config");
    
    // Verify the JSON structure matches Python expectations
    let parsed: serde_json::Value = serde_json::from_str(&json).expect("Failed to parse JSON");
    
    assert_eq!(parsed["id"], "config-123");
    assert_eq!(parsed["url"], "https://example.com/webhook");
    assert_eq!(parsed["token"], "token-456");
}

#[test]
fn test_task_push_notification_config_compatibility() {
    let url = Url::parse("https://example.com/webhook").expect("Invalid URL");
    let push_config = PushNotificationConfig {
        id: Some("config-123".to_string()),
        url,
        token: Some("token-456".to_string()),
        authentication: None,
    };

    let task_config = TaskPushNotificationConfig {
        task_id: "task-789".to_string(),
        push_notification_config: push_config,
    };

    // Serialize to JSON
    let json = serde_json::to_string_pretty(&task_config).expect("Failed to serialize task config");
    
    // Verify the JSON structure matches Python expectations
    let parsed: serde_json::Value = serde_json::from_str(&json).expect("Failed to parse JSON");
    
    assert_eq!(parsed["task_id"], "task-789");
    assert_eq!(parsed["push_notification_config"]["id"], "config-123");
    assert_eq!(parsed["push_notification_config"]["token"], "token-456");
}

#[test]
fn test_delete_task_push_notification_config_params_compatibility() {
    let params = DeleteTaskPushNotificationConfigParams {
        id: "task-123".to_string(),
        push_notification_config_id: "config-456".to_string(),
        metadata: None,
    };

    // Serialize to JSON
    let json = serde_json::to_string_pretty(&params).expect("Failed to serialize params");
    
    // Verify the JSON structure matches Python expectations
    let parsed: serde_json::Value = serde_json::from_str(&json).expect("Failed to parse JSON");
    
    assert_eq!(parsed["id"], "task-123");
    assert_eq!(parsed["push_notification_config_id"], "config-456");
}

#[test]
fn test_get_task_push_notification_config_params_compatibility() {
    let params = GetTaskPushNotificationConfigParams {
        id: "task-123".to_string(),
        push_notification_config_id: Some("config-456".to_string()),
        metadata: None,
    };

    // Serialize to JSON
    let json = serde_json::to_string_pretty(&params).expect("Failed to serialize params");
    
    // Verify the JSON structure matches Python expectations
    let parsed: serde_json::Value = serde_json::from_str(&json).expect("Failed to parse JSON");
    
    assert_eq!(parsed["id"], "task-123");
    assert_eq!(parsed["push_notification_config_id"], "config-456");
}

#[test]
fn test_list_task_push_notification_config_params_compatibility() {
    let params = ListTaskPushNotificationConfigParams {
        id: "task-123".to_string(),
        metadata: None,
    };

    // Serialize to JSON
    let json = serde_json::to_string_pretty(&params).expect("Failed to serialize params");
    
    // Verify the JSON structure matches Python expectations
    let parsed: serde_json::Value = serde_json::from_str(&json).expect("Failed to parse JSON");
    
    assert_eq!(parsed["id"], "task-123");
}

#[test]
fn test_roundtrip_compatibility() {
    // Test that we can serialize and deserialize back to the same structure
    let original_message = Message {
        kind: "message".to_string(),
        message_id: "msg-123".to_string(),
        role: Role::Agent,
        parts: vec![
            Part::text("Response message".to_string()),
            Part::data(serde_json::json!({"result": "success"})),
        ],
        context_id: Some("ctx-456".to_string()),
        task_id: Some("task-789".to_string()),
        metadata: Some({
            let mut meta = HashMap::new();
            meta.insert("version".to_string(), serde_json::Value::String("1.0".to_string()));
            meta
        }),
        extensions: Some(vec!["ext-1".to_string(), "ext-2".to_string()]),
        reference_task_ids: Some(vec!["ref-1".to_string()]),
    };

    // Serialize to JSON
    let json = serde_json::to_string(&original_message).expect("Failed to serialize");
    
    // Deserialize back
    let deserialized: Message = serde_json::from_str(&json).expect("Failed to deserialize");
    
    // Verify they are equivalent
    assert_eq!(original_message.kind, deserialized.kind);
    assert_eq!(original_message.message_id, deserialized.message_id);
    assert_eq!(original_message.role, deserialized.role);
    assert_eq!(original_message.context_id, deserialized.context_id);
    assert_eq!(original_message.task_id, deserialized.task_id);
    assert_eq!(original_message.parts.len(), deserialized.parts.len());
    assert_eq!(original_message.extensions, deserialized.extensions);
    assert_eq!(original_message.reference_task_ids, deserialized.reference_task_ids);
}

#[test]
fn test_python_json_compatibility() {
    // Test JSON that would be produced by Python implementation
    let python_json = r#"
    {
        "kind": "message",
        "messageId": "python-msg-123",
        "role": "user",
        "parts": [
            {
                "kind": "text",
                "text": "Hello from Python"
            },
            {
                "kind": "data",
                "data": {"source": "python", "version": "0.3.0"}
            }
        ],
        "contextId": "python-ctx-456",
        "taskId": "python-task-789"
    }
    "#;

    // Should be able to deserialize Python JSON
    let message: Message = serde_json::from_str(python_json).expect("Failed to deserialize Python JSON");
    
    assert_eq!(message.kind, "message");
    assert_eq!(message.message_id, "python-msg-123");
    assert_eq!(message.role, Role::User);
    assert_eq!(message.context_id, Some("python-ctx-456".to_string()));
    assert_eq!(message.task_id, Some("python-task-789".to_string()));
    assert_eq!(message.parts.len(), 2);
}
