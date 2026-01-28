//! JSON-RPC 2.0 protocol types for A2A
//! 
//! This module contains all the JSON-RPC 2.0 request/response types
//! that match the Python implementation in a2a-python/src/a2a/types.py

use serde::{Deserialize, Serialize};
use crate::a2a::models::*;
use crate::Message;

/// JSON-RPC 2.0 base message structure
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JSONRPCMessage {
    /// A unique identifier established by the client
    pub id: Option<JSONRPCId>,
    /// The version of the JSON-RPC protocol. MUST be exactly "2.0"
    pub jsonrpc: String,
}

impl JSONRPCMessage {
    pub fn new(id: Option<JSONRPCId>) -> Self {
        Self {
            id,
            jsonrpc: "2.0".to_string(),
        }
    }
}

/// JSON-RPC 2.0 identifier (can be string, number, or null)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JSONRPCId {
    String(String),
    Number(i64),
    Null,
}

/// JSON-RPC 2.0 Request object
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JSONRPCRequest {
    /// A unique identifier established by the client
    pub id: Option<JSONRPCId>,
    /// The version of the JSON-RPC protocol. MUST be exactly "2.0"
    pub jsonrpc: String,
    /// A string containing the name of the method to be invoked
    pub method: String,
    /// A structured value holding the parameter values
    pub params: Option<serde_json::Value>,
}

impl JSONRPCRequest {
    pub fn new(method: String, params: Option<serde_json::Value>, id: Option<JSONRPCId>) -> Self {
        Self {
            id,
            jsonrpc: "2.0".to_string(),
            method,
            params,
        }
    }
}

/// JSON-RPC 2.0 Success Response object
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JSONRPCSuccessResponse {
    /// The identifier established by the client
    pub id: Option<JSONRPCId>,
    /// The version of the JSON-RPC protocol. MUST be exactly "2.0"
    pub jsonrpc: String,
    /// The value of this member is determined by the method invoked on the Server
    pub result: serde_json::Value,
}

impl JSONRPCSuccessResponse {
    pub fn new(id: Option<JSONRPCId>, result: serde_json::Value) -> Self {
        Self {
            id,
            jsonrpc: "2.0".to_string(),
            result,
        }
    }
}

/// JSON-RPC 2.0 Error object
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

/// JSON-RPC 2.0 Error Response object
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JSONRPCErrorResponse {
    /// The identifier established by the client
    pub id: Option<JSONRPCId>,
    /// The version of the JSON-RPC protocol. MUST be exactly "2.0"
    pub jsonrpc: String,
    /// An object describing the error that occurred
    pub error: JSONRPCError,
}

impl JSONRPCErrorResponse {
    pub fn new(id: Option<JSONRPCId>, error: JSONRPCError) -> Self {
        Self {
            id,
            jsonrpc: "2.0".to_string(),
            error,
        }
    }
}

/// JSON-RPC 2.0 Response (can be success or error)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JSONRPCResponse {
    Success(JSONRPCSuccessResponse),
    Error(JSONRPCErrorResponse),
}

impl JSONRPCResponse {
    pub fn success(id: Option<JSONRPCId>, result: serde_json::Value) -> Self {
        Self::Success(JSONRPCSuccessResponse::new(id, result))
    }

    pub fn error(id: Option<JSONRPCId>, error: JSONRPCError) -> Self {
        Self::Error(JSONRPCErrorResponse::new(id, error))
    }

    pub fn get_id(&self) -> Option<&JSONRPCId> {
        match self {
            JSONRPCResponse::Success(resp) => resp.id.as_ref(),
            JSONRPCResponse::Error(resp) => resp.id.as_ref(),
        }
    }
}

/// A2A-specific error codes
pub mod error_codes {
    pub const TASK_NOT_FOUND: i32 = -32001;
    pub const TASK_NOT_CANCELABLE: i32 = -32002;
    pub const PUSH_NOTIFICATION_NOT_SUPPORTED: i32 = -32003;
    pub const UNSUPPORTED_OPERATION: i32 = -32004;
    pub const CONTENT_TYPE_NOT_SUPPORTED: i32 = -32005;
    pub const INVALID_AGENT_RESPONSE: i32 = -32006;
    pub const AUTHENTICATED_EXTENDED_CARD_NOT_CONFIGURED: i32 = -32007;
}

/// Standard JSON-RPC error codes
pub mod standard_error_codes {
    pub const PARSE_ERROR: i32 = -32700;
    pub const INVALID_REQUEST: i32 = -32600;
    pub const METHOD_NOT_FOUND: i32 = -32601;
    pub const INVALID_PARAMS: i32 = -32602;
    pub const INTERNAL_ERROR: i32 = -32603;
}

/// A2A Request types (discriminated union)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "method")]
pub enum A2ARequest {
    #[serde(rename = "message/send")]
    SendMessage { params: MessageSendParams },
    #[serde(rename = "message/stream")]
    SendStreamingMessage { params: MessageSendParams },
    #[serde(rename = "tasks/get")]
    GetTask { params: TaskQueryParams },
    #[serde(rename = "tasks/cancel")]
    CancelTask { params: TaskIdParams },
    #[serde(rename = "tasks/pushNotificationConfig/set")]
    SetTaskPushNotificationConfig { params: TaskPushNotificationConfig },
    #[serde(rename = "tasks/pushNotificationConfig/get")]
    GetTaskPushNotificationConfig { params: TaskQueryParams },
    #[serde(rename = "tasks/pushNotificationConfig/list")]
    ListTaskPushNotificationConfig { params: TaskIdParams },
    #[serde(rename = "tasks/pushNotificationConfig/delete")]
    DeleteTaskPushNotificationConfig { params: TaskIdParams },
    #[serde(rename = "tasks/resubscribe")]
    TaskResubscription { params: TaskIdParams },
    #[serde(rename = "agent/getAuthenticatedExtendedCard")]
    GetAuthenticatedExtendedCard,
}

/// A2A Response types (discriminated union)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum A2AResponse {
    SendMessage(Task),
    SendStreamingMessage(SendStreamingMessageResult),
    GetTask(Task),
    CancelTask(Task),
    SetTaskPushNotificationConfig(TaskPushNotificationConfig),
    GetTaskPushNotificationConfig(TaskPushNotificationConfig),
    ListTaskPushNotificationConfig(Vec<TaskPushNotificationConfig>),
    DeleteTaskPushNotificationConfig(()),
    TaskResubscription(Task),
    GetAuthenticatedExtendedCard(AgentCard),
}

/// Result for streaming message response
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SendStreamingMessageResult {
    Task(Task),
    Message(Message),
    TaskStatusUpdate(TaskStatusUpdateEvent),
    TaskArtifactUpdate(TaskArtifactUpdateEvent),
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_jsonrpc_request_serialization() {
        let request = JSONRPCRequest::new(
            "message/send".to_string(),
            Some(serde_json::json!({
                "message": {
                    "message_id": "msg-123",
                    "role": "user",
                    "parts": [{"text": "Hello", "kind": "text"}],
                    "kind": "message"
                }
            })),
            Some(JSONRPCId::String("req-1".to_string())),
        );

        let json = serde_json::to_string(&request).unwrap();
        let parsed: JSONRPCRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.method, "message/send");
        assert_eq!(parsed.jsonrpc, "2.0");
    }

    #[test]
    fn test_jsonrpc_response_serialization() {
        let response = JSONRPCResponse::success(
            Some(JSONRPCId::String("req-1".to_string())),
            serde_json::json!({"status": "ok"}),
        );

        let json = serde_json::to_string(&response).unwrap();
        let parsed: JSONRPCResponse = serde_json::from_str(&json).unwrap();

        match parsed {
            JSONRPCResponse::Success(resp) => {
                assert_eq!(resp.result, serde_json::json!({"status": "ok"}));
            }
            _ => panic!("Expected success response"),
        }
    }

    #[test]
    fn test_error_response() {
        let error = JSONRPCError::new(
            error_codes::TASK_NOT_FOUND,
            "Task not found".to_string(),
        );

        let response = JSONRPCResponse::error(
            Some(JSONRPCId::String("req-1".to_string())),
            error,
        );

        let json = serde_json::to_string(&response).unwrap();
        let parsed: JSONRPCResponse = serde_json::from_str(&json).unwrap();

        match parsed {
            JSONRPCResponse::Error(resp) => {
                assert_eq!(resp.error.code, error_codes::TASK_NOT_FOUND);
                assert_eq!(resp.error.message, "Task not found");
            }
            _ => panic!("Expected error response"),
        }
    }
}
