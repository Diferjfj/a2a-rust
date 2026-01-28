//! Authenticated user information
//! 
//! This module provides user authentication abstractions
//! matching a2a-python/src/a2a/auth/user.py

use std::fmt;

/// A representation of an authenticated user
pub trait User {
    /// Returns whether the current user is authenticated
    fn is_authenticated(&self) -> bool;
    
    /// Returns the user name of the current user
    fn user_name(&self) -> &str;
}

/// A representation that no user has been authenticated in the request
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnauthenticatedUser;

impl UnauthenticatedUser {
    /// Creates a new UnauthenticatedUser
    pub fn new() -> Self {
        Self
    }
}

impl Default for UnauthenticatedUser {
    fn default() -> Self {
        Self::new()
    }
}

impl User for UnauthenticatedUser {
    fn is_authenticated(&self) -> bool {
        false
    }

    fn user_name(&self) -> &str {
        ""
    }
}

impl fmt::Display for UnauthenticatedUser {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "UnauthenticatedUser")
    }
}

/// A simple authenticated user implementation
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct AuthenticatedUser {
    username: String,
}

impl AuthenticatedUser {
    /// Creates a new authenticated user with the given username
    pub fn new(username: String) -> Self {
        Self { username }
    }
    
    /// Returns the username
    pub fn username(&self) -> &str {
        &self.username
    }
}

impl Default for AuthenticatedUser {
    fn default() -> Self {
        Self::new("".to_string())
    }
}

impl User for AuthenticatedUser {
    fn is_authenticated(&self) -> bool {
        true
    }

    fn user_name(&self) -> &str {
        &self.username
    }
}

impl fmt::Display for AuthenticatedUser {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AuthenticatedUser({})", self.username)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unauthenticated_user() {
        let user = UnauthenticatedUser::new();
        assert!(!user.is_authenticated());
        assert_eq!(user.user_name(), "");
        assert_eq!(user.to_string(), "UnauthenticatedUser");
    }

    #[test]
    fn test_authenticated_user() {
        let user = AuthenticatedUser::new("testuser".to_string());
        assert!(user.is_authenticated());
        assert_eq!(user.user_name(), "testuser");
        assert_eq!(user.username(), "testuser");
        assert_eq!(user.to_string(), "AuthenticatedUser(testuser)");
    }

    #[test]
    fn test_user_trait() {
        let unauth: Box<dyn User> = Box::new(UnauthenticatedUser::new());
        let auth: Box<dyn User> = Box::new(AuthenticatedUser::new("alice".to_string()));
        
        assert!(!unauth.is_authenticated());
        assert_eq!(unauth.user_name(), "");
        
        assert!(auth.is_authenticated());
        assert_eq!(auth.user_name(), "alice");
    }
}
