//! Utility functions for creating A2A Artifact objects
//! 
//! This module provides helper functions that match the functionality
//! in a2a-python/src/a2a/utils/artifact.py

use crate::a2a::core_types::Part;
use crate::a2a::models::Artifact;
use crate::a2a::utils::parts::get_text_parts;

/// Creates a new Artifact object
/// 
/// Matches the Python function `new_artifact`
pub fn new_artifact(
    parts: Vec<Part>,
    name: String,
    description: Option<String>,
) -> Artifact {
    let mut artifact = Artifact::new(parts).with_name(name);
    if let Some(desc) = description {
        artifact = artifact.with_description(desc);
    }
    artifact
}

/// Creates a new Artifact object containing only a single TextPart
/// 
/// Matches the Python function `new_text_artifact`
pub fn new_text_artifact(
    name: String,
    text: String,
    description: Option<String>,
) -> Artifact {
    let part = Part::text(text);
    new_artifact(vec![part], name, description)
}

/// Creates a new Artifact object containing only a single DataPart
/// 
/// Matches the Python function `new_data_artifact`
pub fn new_data_artifact(
    name: String,
    data: serde_json::Value,
    description: Option<String>,
) -> Artifact {
    let part = Part::data(data);
    new_artifact(vec![part], name, description)
}

/// Extracts and joins all text content from an Artifact's parts
/// 
/// Matches the Python function `get_artifact_text`
pub fn get_artifact_text(artifact: &Artifact, delimiter: &str) -> String {
    get_text_parts(&artifact.parts).join(delimiter)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::PartRoot;
    use serde_json::json;

    #[test]
    fn test_new_artifact() {
        let parts = vec![
            Part::text("Hello".to_string()),
            Part::data(json!({"key": "value"})),
        ];

        let artifact = new_artifact(
            parts.clone(),
            "Test Artifact".to_string(),
            Some("A test artifact".to_string()),
        );

        assert_eq!(artifact.name, Some("Test Artifact".to_string()));
        assert_eq!(artifact.description, Some("A test artifact".to_string()));
        assert_eq!(artifact.parts, parts);
        assert!(!artifact.artifact_id.is_empty());
    }

    #[test]
    fn test_new_text_artifact() {
        let artifact = new_text_artifact(
            "Text Artifact".to_string(),
            "Hello, World!".to_string(),
            Some("A text artifact".to_string()),
        );

        assert_eq!(artifact.name, Some("Text Artifact".to_string()));
        assert_eq!(artifact.description, Some("A text artifact".to_string()));
        assert_eq!(artifact.parts.len(), 1);
        
        match &artifact.parts[0].root() {
            PartRoot::Text(text_part) => assert_eq!(text_part.text, "Hello, World!"),
            _ => panic!("Expected TextPart"),
        }
    }

    #[test]
    fn test_new_data_artifact() {
        let data = json!({"message": "Hello", "count": 42});
        let artifact = new_data_artifact(
            "Data Artifact".to_string(),
            data.clone(),
            Some("A data artifact".to_string()),
        );

        assert_eq!(artifact.name, Some("Data Artifact".to_string()));
        assert_eq!(artifact.description, Some("A data artifact".to_string()));
        assert_eq!(artifact.parts.len(), 1);
        
        match &artifact.parts[0].root() {
            PartRoot::Data(data_part) => assert_eq!(data_part.data, data),
            _ => panic!("Expected DataPart"),
        }
    }

    #[test]
    fn test_get_artifact_text() {
        let artifact = new_artifact(
            vec![
                Part::text("Hello".to_string()),
                Part::data(json!({"key": "value"})),
                Part::text("World".to_string()),
            ],
            "Test".to_string(),
            None,
        );

        let text = get_artifact_text(&artifact, " ");
        assert_eq!(text, "Hello World");

        let text_newline = get_artifact_text(&artifact, "\n");
        assert_eq!(text_newline, "Hello\nWorld");
    }

    #[test]
    fn test_get_artifact_text_empty() {
        let artifact = new_artifact(
            vec![Part::data(json!({"key": "value"}))],
            "Test".to_string(),
            None,
        );

        let text = get_artifact_text(&artifact, " ");
        assert_eq!(text, "");
    }
}
