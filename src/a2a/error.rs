//! Error types for the A2A protocol
//! 
//! This module contains all error types used throughout the A2A protocol,
//! including JSON-RPC errors and A2A-specific errors.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents a JSON-RPC 2.0 Error object, included in an error response
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JSONRPCError {
    /// A number that indicates the error type that occurred
    pub code: i32,
    /// A string providing a short description of the error
    pub message: String,
    /// A primitive or structured value containing additional information about the error
    pub data: Option<serde_json::Value>,
}

impl JSONRPCError {
    pub fn new(code: i32, message: String) -> Self {
        Self {
            code,
            message,
            data: None,
        }
    }

    pub fn with_data(mut self, data: serde_json::Value) -> Self {
        self.data = Some(data);
        self
    }
}

/// An error indicating that the server received invalid JSON
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JSONParseError {
    /// The error code for a JSON parse error
    pub code: i32,
    /// The error message
    pub message: String,
    /// A primitive or structured value containing additional information about the error
    pub data: Option<serde_json::Value>,
}

impl Default for JSONParseError {
    fn default() -> Self {
        Self {
            code: -32700,
            message: "Invalid JSON payload".to_string(),
            data: None,
        }
    }
}

/// An error indicating that the JSON sent is not a valid Request object
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InvalidRequestError {
    /// The error code for an invalid request
    pub code: i32,
    /// The error message
    pub message: String,
    /// A primitive or structured value containing additional information about the error
    pub data: Option<serde_json::Value>,
}

impl Default for InvalidRequestError {
    fn default() -> Self {
        Self {
            code: -32600,
            message: "Request payload validation error".to_string(),
            data: None,
        }
    }
}

/// An error indicating that the requested method does not exist or is not available
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MethodNotFoundError {
    /// The error code for a method not found error
    pub code: i32,
    /// The error message
    pub message: String,
    /// A primitive or structured value containing additional information about the error
    pub data: Option<serde_json::Value>,
}

impl Default for MethodNotFoundError {
    fn default() -> Self {
        Self {
            code: -32601,
            message: "Method not found".to_string(),
            data: None,
        }
    }
}

/// An error indicating that the method parameters are invalid
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InvalidParamsError {
    /// The error code for an invalid parameters error
    pub code: i32,
    /// The error message
    pub message: String,
    /// A primitive or structured value containing additional information about the error
    pub data: Option<serde_json::Value>,
}

impl Default for InvalidParamsError {
    fn default() -> Self {
        Self {
            code: -32602,
            message: "Invalid parameters".to_string(),
            data: None,
        }
    }
}

/// An error indicating an internal error on the server
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InternalError {
    /// The error code for an internal server error
    pub code: i32,
    /// The error message
    pub message: String,
    /// A primitive or structured value containing additional information about the error
    pub data: Option<serde_json::Value>,
}

impl Default for InternalError {
    fn default() -> Self {
        Self {
            code: -32603,
            message: "Internal error".to_string(),
            data: None,
        }
    }
}

/// An A2A-specific error indicating that the requested task ID was not found
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaskNotFoundError {
    /// The error code for a task not found error
    pub code: i32,
    /// The error message
    pub message: String,
    /// A primitive or structured value containing additional information about the error
    pub data: Option<serde_json::Value>,
}

impl Default for TaskNotFoundError {
    fn default() -> Self {
        Self {
            code: -32001,
            message: "Task not found".to_string(),
            data: None,
        }
    }
}

/// An A2A-specific error indicating that the task is in a state where it cannot be canceled
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaskNotCancelableError {
    /// The error code for a task that cannot be canceled
    pub code: i32,
    /// The error message
    pub message: String,
    /// A primitive or structured value containing additional information about the error
    pub data: Option<serde_json::Value>,
}

impl Default for TaskNotCancelableError {
    fn default() -> Self {
        Self {
            code: -32002,
            message: "Task cannot be canceled".to_string(),
            data: None,
        }
    }
}

/// An A2A-specific error indicating that the agent does not support push notifications
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PushNotificationNotSupportedError {
    /// The error code for when push notifications are not supported
    pub code: i32,
    /// The error message
    pub message: String,
    /// A primitive or structured value containing additional information about the error
    pub data: Option<serde_json::Value>,
}

impl Default for PushNotificationNotSupportedError {
    fn default() -> Self {
        Self {
            code: -32003,
            message: "Push Notification is not supported".to_string(),
            data: None,
        }
    }
}

/// An A2A-specific error indicating that the requested operation is not supported by the agent
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnsupportedOperationError {
    /// The error code for an unsupported operation
    pub code: i32,
    /// The error message
    pub message: String,
    /// A primitive or structured value containing additional information about the error
    pub data: Option<serde_json::Value>,
}

impl Default for UnsupportedOperationError {
    fn default() -> Self {
        Self {
            code: -32004,
            message: "This operation is not supported".to_string(),
            data: None,
        }
    }
}

/// An A2A-specific error indicating an incompatibility between the requested content types and the agent's capabilities
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContentTypeNotSupportedError {
    /// The error code for an unsupported content type
    pub code: i32,
    /// The error message
    pub message: String,
    /// A primitive or structured value containing additional information about the error
    pub data: Option<serde_json::Value>,
}

impl Default for ContentTypeNotSupportedError {
    fn default() -> Self {
        Self {
            code: -32005,
            message: "Incompatible content types".to_string(),
            data: None,
        }
    }
}

/// An A2A-specific error indicating that the agent returned a response that does not conform to the specification
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InvalidAgentResponseError {
    /// The error code for an invalid agent response
    pub code: i32,
    /// The error message
    pub message: String,
    /// A primitive or structured value containing additional information about the error
    pub data: Option<serde_json::Value>,
}

impl Default for InvalidAgentResponseError {
    fn default() -> Self {
        Self {
            code: -32006,
            message: "Invalid agent response".to_string(),
            data: None,
        }
    }
}

/// An A2A-specific error indicating that the agent does not have an Authenticated Extended Card configured
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthenticatedExtendedCardNotConfiguredError {
    /// The error code for when an authenticated extended card is not configured
    pub code: i32,
    /// The error message
    pub message: String,
    /// A primitive or structured value containing additional information about the error
    pub data: Option<serde_json::Value>,
}

impl Default for AuthenticatedExtendedCardNotConfiguredError {
    fn default() -> Self {
        Self {
            code: -32007,
            message: "Authenticated Extended Card is not configured".to_string(),
            data: None,
        }
    }
}

/// A discriminated union of all standard JSON-RPC and A2A-specific error types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum A2AError {
    JSONParse(JSONParseError),
    InvalidRequest(InvalidRequestError),
    MethodNotFound(MethodNotFoundError),
    InvalidParams(InvalidParamsError),
    Internal(InternalError),
    TaskNotFound(TaskNotFoundError),
    TaskNotCancelable(TaskNotCancelableError),
    PushNotificationNotSupported(PushNotificationNotSupportedError),
    UnsupportedOperation(UnsupportedOperationError),
    ContentTypeNotSupported(ContentTypeNotSupportedError),
    InvalidAgentResponse(InvalidAgentResponseError),
    AuthenticatedExtendedCardNotConfigured(AuthenticatedExtendedCardNotConfiguredError),
    Generic(JSONRPCError),
}

impl A2AError {
    pub fn code(&self) -> i32 {
        match self {
            A2AError::JSONParse(e) => e.code,
            A2AError::InvalidRequest(e) => e.code,
            A2AError::MethodNotFound(e) => e.code,
            A2AError::InvalidParams(e) => e.code,
            A2AError::Internal(e) => e.code,
            A2AError::TaskNotFound(e) => e.code,
            A2AError::TaskNotCancelable(e) => e.code,
            A2AError::PushNotificationNotSupported(e) => e.code,
            A2AError::UnsupportedOperation(e) => e.code,
            A2AError::ContentTypeNotSupported(e) => e.code,
            A2AError::InvalidAgentResponse(e) => e.code,
            A2AError::AuthenticatedExtendedCardNotConfigured(e) => e.code,
            A2AError::Generic(e) => e.code,
        }
    }

    pub fn message(&self) -> &str {
        match self {
            A2AError::JSONParse(e) => &e.message,
            A2AError::InvalidRequest(e) => &e.message,
            A2AError::MethodNotFound(e) => &e.message,
            A2AError::InvalidParams(e) => &e.message,
            A2AError::Internal(e) => &e.message,
            A2AError::TaskNotFound(e) => &e.message,
            A2AError::TaskNotCancelable(e) => &e.message,
            A2AError::PushNotificationNotSupported(e) => &e.message,
            A2AError::UnsupportedOperation(e) => &e.message,
            A2AError::ContentTypeNotSupported(e) => &e.message,
            A2AError::InvalidAgentResponse(e) => &e.message,
            A2AError::AuthenticatedExtendedCardNotConfigured(e) => &e.message,
            A2AError::Generic(e) => &e.message,
        }
    }

    pub fn data(&self) -> Option<&serde_json::Value> {
        match self {
            A2AError::JSONParse(e) => e.data.as_ref(),
            A2AError::InvalidRequest(e) => e.data.as_ref(),
            A2AError::MethodNotFound(e) => e.data.as_ref(),
            A2AError::InvalidParams(e) => e.data.as_ref(),
            A2AError::Internal(e) => e.data.as_ref(),
            A2AError::TaskNotFound(e) => e.data.as_ref(),
            A2AError::TaskNotCancelable(e) => e.data.as_ref(),
            A2AError::PushNotificationNotSupported(e) => e.data.as_ref(),
            A2AError::UnsupportedOperation(e) => e.data.as_ref(),
            A2AError::ContentTypeNotSupported(e) => e.data.as_ref(),
            A2AError::InvalidAgentResponse(e) => e.data.as_ref(),
            A2AError::AuthenticatedExtendedCardNotConfigured(e) => e.data.as_ref(),
            A2AError::Generic(e) => e.data.as_ref(),
        }
    }
}

impl From<JSONParseError> for A2AError {
    fn from(error: JSONParseError) -> Self {
        A2AError::JSONParse(error)
    }
}

impl From<InvalidRequestError> for A2AError {
    fn from(error: InvalidRequestError) -> Self {
        A2AError::InvalidRequest(error)
    }
}

impl From<MethodNotFoundError> for A2AError {
    fn from(error: MethodNotFoundError) -> Self {
        A2AError::MethodNotFound(error)
    }
}

impl From<InvalidParamsError> for A2AError {
    fn from(error: InvalidParamsError) -> Self {
        A2AError::InvalidParams(error)
    }
}

impl From<InternalError> for A2AError {
    fn from(error: InternalError) -> Self {
        A2AError::Internal(error)
    }
}

impl From<TaskNotFoundError> for A2AError {
    fn from(error: TaskNotFoundError) -> Self {
        A2AError::TaskNotFound(error)
    }
}

impl From<TaskNotCancelableError> for A2AError {
    fn from(error: TaskNotCancelableError) -> Self {
        A2AError::TaskNotCancelable(error)
    }
}

impl From<PushNotificationNotSupportedError> for A2AError {
    fn from(error: PushNotificationNotSupportedError) -> Self {
        A2AError::PushNotificationNotSupported(error)
    }
}

impl From<UnsupportedOperationError> for A2AError {
    fn from(error: UnsupportedOperationError) -> Self {
        A2AError::UnsupportedOperation(error)
    }
}

impl From<ContentTypeNotSupportedError> for A2AError {
    fn from(error: ContentTypeNotSupportedError) -> Self {
        A2AError::ContentTypeNotSupported(error)
    }
}

impl From<InvalidAgentResponseError> for A2AError {
    fn from(error: InvalidAgentResponseError) -> Self {
        A2AError::InvalidAgentResponse(error)
    }
}

impl From<AuthenticatedExtendedCardNotConfiguredError> for A2AError {
    fn from(error: AuthenticatedExtendedCardNotConfiguredError) -> Self {
        A2AError::AuthenticatedExtendedCardNotConfigured(error)
    }
}

impl From<JSONRPCError> for A2AError {
    fn from(error: JSONRPCError) -> Self {
        A2AError::Generic(error)
    }
}

impl fmt::Display for A2AError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (code: {})", self.message(), self.code())
    }
}

impl std::error::Error for A2AError {}

// Convenience constructors
impl A2AError {
    pub fn task_not_found(task_id: &str) -> Self {
        TaskNotFoundError {
            code: -32001,
            message: format!("Task not found: {}", task_id),
            data: Some(serde_json::json!({ "task_id": task_id })),
        }.into()
    }

    pub fn task_not_cancelable(reason: &str) -> Self {
        TaskNotCancelableError {
            code: -32002,
            message: format!("Task cannot be canceled: {}", reason),
            data: Some(serde_json::json!({ "reason": reason })),
        }.into()
    }

    pub fn invalid_params(message: &str) -> Self {
        InvalidParamsError {
            code: -32602,
            message: message.to_string(),
            data: None,
        }.into()
    }

    pub fn internal(message: &str) -> Self {
        InternalError {
            code: -32603,
            message: message.to_string(),
            data: None,
        }.into()
    }

    pub fn unsupported_operation(message: &str) -> Self {
        UnsupportedOperationError {
            code: -32004,
            message: message.to_string(),
            data: None,
        }.into()
    }

    pub fn transport_error(message: String) -> Self {
        A2AError::internal(&format!("Transport error: {}", message))
    }

    pub fn http_error(status: u16, message: String) -> Self {
        A2AError::internal(&format!("HTTP error {}: {}", status, message))
    }

    pub fn json_error(message: String) -> Self {
        JSONParseError {
            code: -32700,
            message,
            data: None,
        }.into()
    }

    pub fn jsonrpc_error(code: i32, message: String) -> Self {
        JSONRPCError {
            code,
            message,
            data: None,
        }.into()
    }

    pub fn invalid_url(message: &str) -> Self {
        A2AError::invalid_params(&format!("Invalid URL: {}", message))
    }

    pub fn invalid_request(message: &str) -> Self {
        InvalidRequestError {
            code: -32600,
            message: message.to_string(),
            data: None,
        }.into()
    }

    pub fn invalid_response(message: &str) -> Self {
        InvalidAgentResponseError {
            code: -32006,
            message: message.to_string(),
            data: None,
        }.into()
    }
}

// Add conversions from common error types
impl From<serde_json::Error> for A2AError {
    fn from(err: serde_json::Error) -> Self {
        A2AError::internal(&format!("Serialization error: {}", err))
    }
}

impl From<std::io::Error> for A2AError {
    fn from(err: std::io::Error) -> Self {
        A2AError::internal(&format!("IO error: {}", err))
    }
}

impl From<tokio::task::JoinError> for A2AError {
    fn from(err: tokio::task::JoinError) -> Self {
        A2AError::internal(&format!("Task join error: {}", err))
    }
}
