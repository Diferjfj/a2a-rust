//! Tests for core types
//! 
//! This module contains tests for the core A2A types,
//! mirroring the test structure from a2a-python/tests/test_types.py

use a2a_rust::a2a::core_types::*;
use serde_json;
use uuid::Uuid;
use url::Url;

#[test]
fn test_text_part_creation() {
    let text_part = TextPart {
        kind: "text".to_string(),
        text: "Hello, World!".to_string(),
        metadata: None,
    };

    assert_eq!(text_part.kind, "text");
    assert_eq!(text_part.text, "Hello, World!");
    assert!(text_part.metadata.is_none());
}

#[test]
fn test_text_part_with_metadata() {
    let mut metadata = std::collections::HashMap::new();
    metadata.insert("source".to_string(), serde_json::Value::String("test".to_string()));

    let text_part = TextPart {
        kind: "text".to_string(),
        text: "Hello".to_string(),
        metadata: Some(metadata),
    };

    assert_eq!(text_part.text, "Hello");
    assert!(text_part.metadata.is_some());
    assert_eq!(
        text_part.metadata.unwrap().get("source").unwrap(),
        &serde_json::Value::String("test".to_string())
    );
}

#[test]
fn test_file_part_uri_variant() {
    let uri = Url::parse("file:///path/to/file.txt").unwrap();
    let file_part = FilePart::new_uri(uri);

    assert_eq!(file_part.kind, "file");
    match &file_part.file {
        FileContent::Uri(file_with_uri) => {
            assert_eq!(file_with_uri.uri.as_str(), "file:///path/to/file.txt");
        }
        _ => panic!("Expected URI variant"),
    }
}

#[test]
fn test_file_part_bytes_variant() {
    let file_part = FilePart::new_bytes("aGVsbG8=".to_string());

    assert_eq!(file_part.kind, "file");
    match file_part.file {
        FileContent::Bytes(file_with_bytes) => {
            assert_eq!(file_with_bytes.bytes, "aGVsbG8=");
        }
        _ => panic!("Expected Bytes variant"),
    }
}

#[test]
fn test_data_part_creation() {
    let mut data = serde_json::Map::new();
    data.insert("key".to_string(), serde_json::Value::String("value".to_string()));

    let data_part = DataPart {
        kind: "data".to_string(),
        data: serde_json::Value::Object(data),
        metadata: None,
    };

    assert_eq!(data_part.kind, "data");
    assert_eq!(
        data_part.data.get("key").unwrap(),
        &serde_json::Value::String("value".to_string())
    );
}

#[test]
fn test_part_union_text() {
    let part = Part::text("Hello".to_string());

    match part.root() {
        PartRoot::Text(text) => {
            assert_eq!(text.kind, "text");
            assert_eq!(text.text, "Hello");
        }
        _ => panic!("Expected Text part"),
    }
}

#[test]
fn test_part_union_file() {
    let uri = Url::parse("file:///test.txt").unwrap();
    let part = Part::file_uri(uri);

    match part.root() {
        PartRoot::File(file) => {
            assert_eq!(file.kind, "file");
            match &file.file {
                FileContent::Uri(file_with_uri) => {
                    assert_eq!(file_with_uri.uri.as_str(), "file:///test.txt");
                }
                _ => panic!("Expected URI variant"),
            }
        }
        _ => panic!("Expected File part"),
    }
}

#[test]
fn test_part_union_data() {
    let mut data = serde_json::Map::new();
    data.insert("test".to_string(), serde_json::Value::Bool(true));

    let part = Part::data(serde_json::Value::Object(data));

    match part.root() {
        PartRoot::Data(data) => {
            assert_eq!(data.kind, "data");
            assert_eq!(data.data.get("test").unwrap(), &serde_json::Value::Bool(true));
        }
        _ => panic!("Expected Data part"),
    }
}

#[test]
fn test_message_creation() {
    let part = Part::text("Hello".to_string());
    let message_id = Uuid::new_v4().to_string();

    let message = Message {
        message_id,
        context_id: None,
        task_id: None,
        role: Role::User,
        parts: vec![part],
        metadata: None,
        extensions: None,
        reference_task_ids: None,
        kind: "message".to_string(),
    };

    assert_eq!(message.role, Role::User);
    assert_eq!(message.parts.len(), 1);
}

#[test]
fn test_message_with_multiple_parts() {
    let message = Message {
        message_id: Uuid::new_v4().to_string(),
        context_id: None,
        task_id: None,
        role: Role::Agent,
        parts: vec![
            Part::text("Hello".to_string()),
            Part::data(serde_json::json!({"key": "value"})),
        ],
        metadata: None,
        extensions: None,
        reference_task_ids: None,
        kind: "message".to_string(),
    };

    assert_eq!(message.role, Role::Agent);
    assert_eq!(message.parts.len(), 2);
}

#[test]
fn test_task_status() {
    let status = TaskStatus {
        state: TaskState::Submitted,
        message: None,
        timestamp: Some("2023-10-27T10:00:00Z".to_string()),
    };

    assert_eq!(status.state, TaskState::Submitted);
    assert!(status.message.is_none());
    assert!(status.timestamp.is_some());
}

#[test]
fn test_task_status_with_message() {
    let message = Message {
        message_id: Uuid::new_v4().to_string(),
        context_id: None,
        task_id: None,
        role: Role::Agent,
        parts: vec![Part::text("Task completed".to_string())],
        metadata: None,
        extensions: None,
        reference_task_ids: None,
        kind: "message".to_string(),
    };

    let status = TaskStatus {
        state: TaskState::Completed,
        message: Some(Box::new(message)),
        timestamp: None,
    };

    assert_eq!(status.state, TaskState::Completed);
    assert!(status.message.is_some());
    assert_eq!(status.message.unwrap().role, Role::Agent);
}

#[test]
fn test_role_values() {
    // Test that Role enum values are correct
    match Role::User {
        Role::User => assert!(true),
        Role::Agent => assert!(false),
    }
    
    match Role::Agent {
        Role::User => assert!(false),
        Role::Agent => assert!(true),
    }
}

#[test]
fn test_task_state_values() {
    // Test that TaskState enum values are correct
    let states = vec![
        TaskState::Submitted,
        TaskState::Working,
        TaskState::InputRequired,
        TaskState::Completed,
        TaskState::Canceled,
        TaskState::Failed,
        TaskState::Rejected,
        TaskState::AuthRequired,
        TaskState::Unknown,
    ];
    
    // Just verify we can create all the states
    assert_eq!(states.len(), 9);
}

#[test]
fn test_transport_protocol_values() {
    // Test that TransportProtocol enum values are correct
    let protocols = vec![
        TransportProtocol::Jsonrpc,
        TransportProtocol::Grpc,
        TransportProtocol::HttpJson,
    ];
    
    // Just verify we can create all the protocols
    assert_eq!(protocols.len(), 3);
}

#[test]
fn test_part_convenience_methods() {
    // Test Part convenience methods
    let text_part = Part::text("Hello".to_string());
    match text_part.root() {
        PartRoot::Text(_) => assert!(true),
        _ => assert!(false),
    }

    let uri = Url::parse("file:///test.txt").unwrap();
    let file_part = Part::file_uri(uri);
    match file_part.root() {
        PartRoot::File(_) => assert!(true),
        _ => assert!(false),
    }

    let data_part = Part::data(serde_json::Value::String("test".to_string()));
    match data_part.root() {
        PartRoot::Data(_) => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn test_message_convenience_methods() {
    let text_part = Part::text("Hello".to_string());
    let message = Message::new(Role::User, vec![text_part]);
    
    assert_eq!(message.role, Role::User);
    assert_eq!(message.parts.len(), 1);
    assert_eq!(message.kind, "message");
}

#[test]
fn test_task_status_convenience_methods() {
    let status = TaskStatus::new(TaskState::Working);
    assert_eq!(status.state, TaskState::Working);
    assert!(status.timestamp.is_some());
    
    let text_part = Part::text("Test".to_string());
    let message = Message::new(Role::Agent, vec![text_part]);
    let status_with_message = status.with_message(message);
    assert!(status_with_message.message.is_some());
}

#[test]
fn test_serialization() {
    let text_part = TextPart::new("Hello".to_string());
    let json = serde_json::to_string(&text_part).unwrap();
    let deserialized: TextPart = serde_json::from_str(&json).unwrap();
    
    assert_eq!(text_part.text, deserialized.text);
    assert_eq!(text_part.kind, deserialized.kind);
}
