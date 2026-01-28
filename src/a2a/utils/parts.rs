//! Utility functions for creating and handling A2A Parts objects
//! 
//! This module provides helper functions that match the functionality
//! in a2a-python/src/a2a/utils/parts.py

use crate::a2a::core_types::{FileContent, Part};
use serde_json::Value;

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

/// Extracts dictionary data from all DataPart objects in a list of Parts
/// 
/// Matches the Python function `get_data_parts`
pub fn get_data_parts(parts: &[Part]) -> Vec<Value> {
    parts
        .iter()
        .filter_map(|part| {
            match part.root() {
                crate::a2a::core_types::PartRoot::Data(data_part) => Some(data_part.data.clone()),
                _ => None,
            }
        })
        .collect()
}

/// Extracts file data from all FilePart objects in a list of Parts
/// 
/// Matches the Python function `get_file_parts`
pub fn get_file_parts(parts: &[Part]) -> Vec<FileContent> {
    parts
        .iter()
        .filter_map(|part| {
            match part.root() {
                crate::a2a::core_types::PartRoot::File(file_part) => Some(file_part.file.clone()),
                _ => None,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::a2a::core_types::*;
    use url::Url;

    #[test]
    fn test_get_text_parts() {
        let parts = vec![
            Part::text("Hello".to_string()),
            Part::data(Value::String("data".to_string())),
            Part::text("World".to_string()),
        ];

        let text_parts = get_text_parts(&parts);
        assert_eq!(text_parts, vec!["Hello", "World"]);
    }

    #[test]
    fn test_get_data_parts() {
        let data1 = Value::Object(serde_json::json!({"key1": "value1"}).as_object().unwrap().clone());
        let data2 = Value::Object(serde_json::json!({"key2": "value2"}).as_object().unwrap().clone());

        let parts = vec![
            Part::text("Hello".to_string()),
            Part::data(data1.clone()),
            Part::data(data2.clone()),
        ];

        let data_parts = get_data_parts(&parts);
        assert_eq!(data_parts, vec![data1, data2]);
    }

    #[test]
    fn test_get_file_parts() {
        let url = Url::parse("https://example.com/file.txt").unwrap();

        let parts = vec![
            Part::text("Hello".to_string()),
            Part::file_uri(url.clone()),
            Part::data(Value::String("data".to_string())),
        ];

        let file_parts = get_file_parts(&parts);
        assert_eq!(file_parts.len(), 1);
        match &file_parts[0] {
            FileContent::Uri(file_with_uri) => {
                assert_eq!(file_with_uri.uri, url.to_string()); // Convert Url to String for comparison
                // Note: Part::file_uri() doesn't set mime_type by default, so it's None
                assert_eq!(file_with_uri.mime_type, None);
                assert_eq!(file_with_uri.name, None);
            }
            _ => panic!("Expected FileWithUri"),
        }
    }

    #[test]
    fn test_mixed_parts() {
        let url = Url::parse("https://example.com/file.txt").unwrap();
        let data = Value::Object(serde_json::json!({"key": "value"}).as_object().unwrap().clone());

        let parts = vec![
            Part::text("Text content".to_string()),
            Part::file_uri(url),
            Part::data(data.clone()),
            Part::file_bytes("base64content".to_string()),
        ];

        let text_parts = get_text_parts(&parts);
        let data_parts = get_data_parts(&parts);
        let file_parts = get_file_parts(&parts);

        assert_eq!(text_parts, vec!["Text content"]);
        assert_eq!(data_parts, vec![data]);
        assert_eq!(file_parts.len(), 2); // One URI file, one bytes file
    }
}
