//! Tests for user authentication functionality
//! 
//! This module contains tests for user authentication and authorization,
//! mirroring the structure from a2a-python/tests/auth/test_user.py

use a2a_rust::a2a::auth::user::*;

#[test]
fn test_user_creation() {
    let user = User {
        id: "user123".to_string(),
        username: "testuser".to_string(),
        email: Some("test@example.com".to_string()),
        is_authenticated: false,
        roles: vec![],
        metadata: None,
    };

    assert_eq!(user.id, "user123");
    assert_eq!(user.username, "testuser");
    assert_eq!(user.email, Some("test@example.com".to_string()));
    assert!(!user.is_authenticated);
    assert!(user.roles.is_empty());
}

#[test]
fn test_authenticated_user() {
    let user = User {
        id: "user456".to_string(),
        username: "authuser".to_string(),
        email: Some("auth@example.com".to_string()),
        is_authenticated: true,
        roles: vec!["admin".to_string(), "user".to_string()],
        metadata: None,
    };

    assert!(user.is_authenticated);
    assert_eq!(user.roles.len(), 2);
    assert!(user.roles.contains(&"admin".to_string()));
    assert!(user.roles.contains(&"user".to_string()));
}

#[test]
fn test_user_with_metadata() {
    let mut metadata = std::collections::HashMap::new();
    metadata.insert("department".to_string(), serde_json::Value::String("engineering".to_string()));
    metadata.insert("level".to_string(), serde_json::Value::Number(serde_json::Number::from(5)));

    let user = User {
        id: "user789".to_string(),
        username: "metauser".to_string(),
        email: None,
        is_authenticated: true,
        roles: vec!["developer".to_string()],
        metadata: Some(metadata),
    };

    assert!(user.metadata.is_some());
    let meta = user.metadata.unwrap();
    assert_eq!(
        meta.get("department").unwrap(),
        &serde_json::Value::String("engineering".to_string())
    );
    assert_eq!(
        meta.get("level").unwrap(),
        &serde_json::Value::Number(serde_json::Number::from(5))
    );
}

#[test]
fn test_user_trait_implementations() {
    let user1 = User {
        id: "user1".to_string(),
        username: "user1".to_string(),
        email: None,
        is_authenticated: false,
        roles: vec![],
        metadata: None,
    };

    let user2 = User {
        id: "user1".to_string(),
        username: "user1".to_string(),
        email: None,
        is_authenticated: false,
        roles: vec![],
        metadata: None,
    };

    let user3 = User {
        id: "user2".to_string(),
        username: "user2".to_string(),
        email: None,
        is_authenticated: false,
        roles: vec![],
        metadata: None,
    };

    // Test PartialEq
    assert_eq!(user1, user2);
    assert_ne!(user1, user3);

    // Test Clone
    let user1_clone = user1.clone();
    assert_eq!(user1, user1_clone);

    // Test Debug
    let debug_str = format!("{:?}", user1);
    assert!(debug_str.contains("User"));
    assert!(debug_str.contains("user1"));
}

#[test]
fn test_user_serialization() {
    let user = User {
        id: "serialize_user".to_string(),
        username: "serializetest".to_string(),
        email: Some("serialize@example.com".to_string()),
        is_authenticated: true,
        roles: vec!["admin".to_string()],
        metadata: None,
    };

    let json = serde_json::to_string(&user).unwrap();
    let deserialized: User = serde_json::from_str(&json).unwrap();

    assert_eq!(user.id, deserialized.id);
    assert_eq!(user.username, deserialized.username);
    assert_eq!(user.email, deserialized.email);
    assert_eq!(user.is_authenticated, deserialized.is_authenticated);
    assert_eq!(user.roles, deserialized.roles);
}

#[test]
fn test_user_default() {
    let default_user = User::default();

    assert_eq!(default_user.id, "");
    assert_eq!(default_user.username, "");
    assert_eq!(default_user.email, None);
    assert!(!default_user.is_authenticated);
    assert!(default_user.roles.is_empty());
    assert!(default_user.metadata.is_none());
}

#[test]
fn test_user_builder_pattern() {
    let user = User::builder()
        .id("builder_user".to_string())
        .username("builder".to_string())
        .email("builder@example.com".to_string())
        .authenticated(true)
        .add_role("admin".to_string())
        .add_role("user".to_string())
        .build();

    assert_eq!(user.id, "builder_user");
    assert_eq!(user.username, "builder");
    assert_eq!(user.email, Some("builder@example.com".to_string()));
    assert!(user.is_authenticated);
    assert_eq!(user.roles.len(), 2);
}

#[test]
fn test_user_role_management() {
    let mut user = User {
        id: "role_user".to_string(),
        username: "roleuser".to_string(),
        email: None,
        is_authenticated: true,
        roles: vec!["user".to_string()],
        metadata: None,
    };

    // Test has_role
    assert!(user.has_role("user"));
    assert!(!user.has_role("admin"));

    // Test add_role
    user.add_role("admin".to_string());
    assert!(user.has_role("admin"));
    assert_eq!(user.roles.len(), 2);

    // Test remove_role
    user.remove_role("user");
    assert!(!user.has_role("user"));
    assert!(user.has_role("admin"));
    assert_eq!(user.roles.len(), 1);
}

#[test]
fn test_user_validation() {
    // Test valid user
    let valid_user = User {
        id: "valid123".to_string(),
        username: "validuser".to_string(),
        email: Some("valid@example.com".to_string()),
        is_authenticated: true,
        roles: vec!["user".to_string()],
        metadata: None,
    };
    assert!(valid_user.is_valid());

    // Test invalid user (empty ID)
    let invalid_user = User {
        id: "".to_string(),
        username: "invalid".to_string(),
        email: None,
        is_authenticated: false,
        roles: vec![],
        metadata: None,
    };
    assert!(!invalid_user.is_valid());

    // Test invalid user (empty username)
    let invalid_user2 = User {
        id: "id123".to_string(),
        username: "".to_string(),
        email: None,
        is_authenticated: false,
        roles: vec![],
        metadata: None,
    };
    assert!(!invalid_user2.is_valid());
}
