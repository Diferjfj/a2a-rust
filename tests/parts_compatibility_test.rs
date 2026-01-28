//! Compatibility tests for Part types with Python a2a-sdk
//! 
//! This test verifies that Rust Part types serialize/deserialize
//! in the same way as Python a2a-sdk types.

use a2a_rust::a2a::core_types::*;
use serde_json;
use std::collections::HashMap;

#[test]
fn test_text_part_serialization_compatibility() {
    let text_part = TextPart {
        text: "Hello, World!".to_string(),
        kind: "text".to_string(),
        metadata: Some(HashMap::from([
            ("source".to_string(), serde_json::Value::String("test".to_string())),
            ("priority".to_string(), serde_json::Value::Number(serde_json::Number::from(1))),
        ])),
    };

    let serialized = serde_json::to_string(&text_part).unwrap();
    let deserialized: TextPart = serde_json::from_str(&serialized).unwrap();
    
    assert_eq!(text_part.text, deserialized.text);
    assert_eq!(text_part.kind, deserialized.kind);
    assert_eq!(text_part.metadata, deserialized.metadata);
    
    // Verify the JSON structure matches Python's expected format
    let json_value: serde_json::Value = serde_json::from_str(&serialized).unwrap();
    assert_eq!(json_value["text"], "Hello, World!");
    assert_eq!(json_value["kind"], "text");
    assert_eq!(json_value["metadata"]["source"], "test");
    assert_eq!(json_value["metadata"]["priority"], 1);
}

#[test]
fn test_data_part_serialization_compatibility() {
    let data_part = DataPart {
        data: serde_json::json!({
            "test": true,
            "client": "rust",
            "numbers": [1, 2, 3]
        }),
        kind: "data".to_string(),
        metadata: Some(HashMap::from([
            ("format".to_string(), serde_json::Value::String("json".to_string())),
        ])),
    };

    let serialized = serde_json::to_string(&data_part).unwrap();
    let deserialized: DataPart = serde_json::from_str(&serialized).unwrap();
    
    assert_eq!(data_part.data, deserialized.data);
    assert_eq!(data_part.kind, deserialized.kind);
    assert_eq!(data_part.metadata, deserialized.metadata);
    
    // Verify the JSON structure
    let json_value: serde_json::Value = serde_json::from_str(&serialized).unwrap();
    assert_eq!(json_value["data"]["test"], true);
    assert_eq!(json_value["data"]["client"], "rust");
    assert_eq!(json_value["data"]["numbers"][0], 1);
    assert_eq!(json_value["kind"], "data");
    assert_eq!(json_value["metadata"]["format"], "json");
}

#[test]
fn test_file_part_with_bytes_compatibility() {
    let file_part = FilePart {
        file: FileContent::Bytes(FileWithBytes {
            bytes: "SGVsbG8gV29ybGQ=".to_string(), // "Hello World" in base64
            mime_type: Some("text/plain".to_string()),
            name: Some("hello.txt".to_string()),
        }),
        kind: "file".to_string(),
        metadata: None,
    };

    let serialized = serde_json::to_string(&file_part).unwrap();
    let deserialized: FilePart = serde_json::from_str(&serialized).unwrap();
    
    match (file_part.file, deserialized.file) {
        (FileContent::Bytes(original), FileContent::Bytes(deser)) => {
            assert_eq!(original.bytes, deser.bytes);
            assert_eq!(original.mime_type, deser.mime_type);
            assert_eq!(original.name, deser.name);
        }
        _ => panic!("Expected FileContent::Bytes variants"),
    }
    
    // Verify the JSON structure
    let json_value: serde_json::Value = serde_json::from_str(&serialized).unwrap();
    assert_eq!(json_value["file"]["bytes"], "SGVsbG8gV29ybGQ=");
    assert_eq!(json_value["file"]["mime_type"], "text/plain");
    assert_eq!(json_value["file"]["name"], "hello.txt");
    assert_eq!(json_value["kind"], "file");
}

#[test]
fn test_file_part_with_uri_compatibility() {
    let file_part = FilePart {
        file: FileContent::Uri(FileWithUri {
            uri: "https://example.com/file.pdf".to_string(), // String type to match Python
            mime_type: Some("application/pdf".to_string()),
            name: Some("document.pdf".to_string()),
        }),
        kind: "file".to_string(),
        metadata: Some(HashMap::from([
            ("source".to_string(), serde_json::Value::String("external".to_string())),
        ])),
    };

    let serialized = serde_json::to_string(&file_part).unwrap();
    let deserialized: FilePart = serde_json::from_str(&serialized).unwrap();
    
    match (file_part.file, deserialized.file) {
        (FileContent::Uri(original), FileContent::Uri(deser)) => {
            assert_eq!(original.uri, deser.uri); // Both are strings
            assert_eq!(original.mime_type, deser.mime_type);
            assert_eq!(original.name, deser.name);
        }
        _ => panic!("Expected FileContent::Uri variants"),
    }
    
    // Verify the JSON structure matches Python's format
    let json_value: serde_json::Value = serde_json::from_str(&serialized).unwrap();
    assert_eq!(json_value["file"]["uri"], "https://example.com/file.pdf");
    assert_eq!(json_value["file"]["mime_type"], "application/pdf");
    assert_eq!(json_value["file"]["name"], "document.pdf");
    assert_eq!(json_value["kind"], "file");
    assert_eq!(json_value["metadata"]["source"], "external");
}

#[test]
fn test_part_with_root_format_compatibility() {
    // Test the {"root": {...}} format that Python uses
    let json_with_root = serde_json::json!({
        "root": {
            "text": "Test message",
            "kind": "text",
            "metadata": null
        }
    });

    let part: Part = serde_json::from_value(json_with_root).unwrap();
    
    match part {
        Part::WithRoot { root } => {
            match root {
                PartRoot::Text(text_part) => {
                    assert_eq!(text_part.text, "Test message");
                    assert_eq!(text_part.kind, "text");
                    assert_eq!(text_part.metadata, None);
                }
                _ => panic!("Expected PartRoot::Text"),
            }
        }
        _ => panic!("Expected Part::WithRoot"),
    }
}

#[test]
fn test_part_direct_format_compatibility() {
    // Test the direct format that can also be used
    let json_direct = serde_json::json!({
        "text": "Direct message",
        "kind": "text"
    });

    let part: Part = serde_json::from_value(json_direct).unwrap();
    
    match part {
        Part::Direct(root) => {
            match root {
                PartRoot::Text(text_part) => {
                    assert_eq!(text_part.text, "Direct message");
                    assert_eq!(text_part.kind, "text");
                    assert_eq!(text_part.metadata, None);
                }
                _ => panic!("Expected PartRoot::Text"),
            }
        }
        _ => panic!("Expected Part::Direct"),
    }
}

#[test]
fn test_message_with_parts_compatibility() {
    let message = Message {
        message_id: "test-123".to_string(),
        context_id: Some("ctx-456".to_string()),
        task_id: Some("task-789".to_string()),
        role: Role::User,
        parts: vec![
            Part::Direct(PartRoot::Text(TextPart {
                text: "Hello".to_string(),
                kind: "text".to_string(),
                metadata: None,
            })),
            Part::Direct(PartRoot::Data(DataPart {
                data: serde_json::json!({"key": "value"}),
                kind: "data".to_string(),
                metadata: None,
            })),
        ],
        metadata: None,
        extensions: None,
        reference_task_ids: None,
        kind: "message".to_string(),
    };

    let serialized = serde_json::to_string(&message).unwrap();
    let deserialized: Message = serde_json::from_str(&serialized).unwrap();
    
    assert_eq!(message.message_id, deserialized.message_id);
    assert_eq!(message.role, deserialized.role);
    assert_eq!(message.parts.len(), deserialized.parts.len());
    
    // Verify the JSON structure matches Python's expected format
    let json_value: serde_json::Value = serde_json::from_str(&serialized).unwrap();
    assert_eq!(json_value["messageId"], "test-123");
    assert_eq!(json_value["contextId"], "ctx-456");
    assert_eq!(json_value["taskId"], "task-789");
    assert_eq!(json_value["role"], "user");
    assert_eq!(json_value["kind"], "message");
    
    // Check parts structure
    assert_eq!(json_value["parts"][0]["text"], "Hello");
    assert_eq!(json_value["parts"][0]["kind"], "text");
    assert_eq!(json_value["parts"][1]["data"]["key"], "value");
    assert_eq!(json_value["parts"][1]["kind"], "data");
}

#[test]
fn test_part_convenience_methods() {
    // Test the convenience methods
    let text_part = Part::text("Test text".to_string());
    let data_part = Part::data(serde_json::json!({"test": true}));
    
    match text_part {
        Part::Direct(PartRoot::Text(tp)) => {
            assert_eq!(tp.text, "Test text");
            assert_eq!(tp.kind, "text");
        }
        _ => panic!("Expected text part"),
    }
    
    match data_part {
        Part::Direct(PartRoot::Data(dp)) => {
            assert_eq!(dp.data["test"], true);
            assert_eq!(dp.kind, "data");
        }
        _ => panic!("Expected data part"),
    }
}

#[test]
fn test_file_part_convenience_methods() {
    use url::Url;
    
    let uri_part = Part::file_uri(Url::parse("https://example.com/file.txt").unwrap());
    let bytes_part = Part::file_bytes("SGVsbG8=".to_string());
    
    match uri_part {
        Part::Direct(PartRoot::File(fp)) => {
            match fp.file {
                FileContent::Uri(fwu) => {
                    assert_eq!(fwu.uri, "https://example.com/file.txt");
                }
                _ => panic!("Expected URI file content"),
            }
        }
        _ => panic!("Expected file part"),
    }
    
    match bytes_part {
        Part::Direct(PartRoot::File(fp)) => {
            match fp.file {
                FileContent::Bytes(fwb) => {
                    assert_eq!(fwb.bytes, "SGVsbG8=");
                }
                _ => panic!("Expected bytes file content"),
            }
        }
        _ => panic!("Expected file part"),
    }
}
