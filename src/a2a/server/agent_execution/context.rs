//! Request Context implementation
//! 
//! This module defines the RequestContext which holds information about the current
//! request being processed by the server, including the incoming message, task and
//! context identifiers, and related tasks.

use crate::{A2AError, Message, MessageSendConfiguration, MessageSendParams, Task};
use crate::a2a::server::context::ServerCallContext;
use crate::a2a::server::id_generator::{IDGenerator, IDGeneratorContext, UUIDGenerator};
use std::collections::HashMap;
use std::sync::Arc;

/// Request Context
/// 
/// Holds information about the current request being processed by the server,
/// including the incoming message, task and context identifiers, and related tasks.
#[derive(Clone)]
pub struct RequestContext {
    /// The incoming MessageSendParams request payload
    pub request: Option<MessageSendParams>,
    
    /// The ID of the task explicitly provided in the request or path
    pub task_id: Option<String>,
    
    /// The ID of the context explicitly provided in the request or path
    pub context_id: Option<String>,
    
    /// The existing Task object retrieved from the store, if any
    pub current_task: Option<Task>,
    
    /// A list of other tasks related to the current request (e.g., for tool use)
    pub related_tasks: Vec<Task>,
    
    /// The server call context associated with this request
    pub call_context: Option<ServerCallContext>,
    
    /// ID generator for new task IDs
    task_id_generator: Arc<dyn IDGenerator>,
    
    /// ID generator for new context IDs
    context_id_generator: Arc<dyn IDGenerator>,
}

impl RequestContext {
    /// Creates a new RequestContext
    /// 
    /// # Arguments
    /// * `request` - The incoming MessageSendParams request payload
    /// * `task_id` - The ID of the task explicitly provided in the request or path
    /// * `context_id` - The ID of the context explicitly provided in the request or path
    /// * `task` - The existing Task object retrieved from the store, if any
    /// * `related_tasks` - A list of other tasks related to the current request
    /// * `call_context` - The server call context associated with this request
    /// * `task_id_generator` - ID generator for new task IDs
    /// * `context_id_generator` - ID generator for new context IDs
    pub async fn new(
        request: Option<MessageSendParams>,
        task_id: Option<String>,
        context_id: Option<String>,
        task: Option<Task>,
        related_tasks: Option<Vec<Task>>,
        call_context: Option<ServerCallContext>,
        task_id_generator: Option<Arc<dyn IDGenerator>>,
        context_id_generator: Option<Arc<dyn IDGenerator>>,
    ) -> Result<Self, A2AError> {
        let task_id_generator = task_id_generator.unwrap_or_else(|| Arc::new(UUIDGenerator::new()));
        let context_id_generator = context_id_generator.unwrap_or_else(|| Arc::new(UUIDGenerator::new()));
        
        let mut context = Self {
            request,
            task_id: task_id.clone(),
            context_id: context_id.clone(),
            current_task: task,
            related_tasks: related_tasks.unwrap_or_default(),
            call_context,
            task_id_generator,
            context_id_generator,
        };
        
        // Validate and set IDs if request is present
        if context.request.is_some() {
            if let Some(ref task_id) = task_id {
                // Set task_id on the message
                {
                    let params = context.request.as_mut().unwrap();
                    if let Some(ref message) = params.message.task_id {
                        if message.to_string() != *task_id {
                            return Err(A2AError::invalid_params("bad task id"));
                        }
                    } else {
                        params.message.task_id = Some(uuid::Uuid::parse_str(task_id).map_err(|_| A2AError::invalid_params("invalid task id format"))?.to_string());
                    }
                }
                
                // Validate against current task if present
                if let Some(ref current_task) = context.current_task {
                    if current_task.id.to_string() != *task_id {
                        return Err(A2AError::invalid_params("bad task id"));
                    }
                }
            } else {
                // Generate task_id if not present
                context.check_or_generate_task_id().await?;
            }
            
            if let Some(ref context_id) = context_id {
                // Set context_id on the message
                {
                    let params = context.request.as_mut().unwrap();
                    if let Some(ref message) = params.message.context_id {
                        if message != context_id {
                            return Err(A2AError::invalid_params("bad context id"));
                        }
                    } else {
                        params.message.context_id = Some(context_id.clone());
                    }
                }
                
                // Validate against current task if present
                if let Some(ref current_task) = context.current_task {
                    if current_task.context_id.to_string() != *context_id {
                        return Err(A2AError::invalid_params("bad context id"));
                    }
                }
            } else {
                // Generate context_id if not present
                context.check_or_generate_context_id().await?;
            }
        }
        
        Ok(context)
    }
    
    /// Extracts text content from the user's message parts
    /// 
    /// # Arguments
    /// * `delimiter` - The string to use when joining multiple text parts
    /// 
    /// # Returns
    /// A single string containing all text content from the user message,
    /// joined by the specified delimiter. Returns an empty string if no
    /// user message is present or if it contains no text parts.
    pub fn get_user_input(&self, delimiter: &str) -> String {
        if let Some(ref params) = self.request {
            self.extract_text_from_message(&params.message, delimiter)
        } else {
            String::new()
        }
    }
    
    /// Attaches a related task to the context
    /// 
    /// This is useful for scenarios like tool execution where a new task
    /// might be spawned.
    /// 
    /// # Arguments
    /// * `task` - The Task object to attach
    pub fn attach_related_task(&mut self, task: Task) {
        self.related_tasks.push(task);
    }
    
    /// Adds an extension to the set of activated extensions for this request
    /// 
    /// This causes the extension to be indicated back to the client in the response.
    /// 
    /// # Arguments
    /// * `uri` - The extension URI to activate
    pub fn add_activated_extension(&mut self, uri: String) {
        if let Some(ref mut call_context) = self.call_context {
            call_context.add_activated_extension(uri);
        }
    }
    
    /// Gets the incoming Message object from the request, if available
    pub fn message(&self) -> Option<&Message> {
        self.request.as_ref().map(|params| &params.message)
    }
    
    /// Gets the configuration from the request, if available
    pub fn configuration(&self) -> Option<&MessageSendConfiguration> {
        self.request.as_ref().and_then(|params| params.configuration.as_ref())
    }
    
    /// Gets the metadata associated with the request, if available
    pub fn metadata(&self) -> HashMap<String, serde_json::Value> {
        self.request
            .as_ref()
            .and_then(|params| params.metadata.clone())
            .unwrap_or_default()
    }
    
    /// Gets the requested extensions
    pub fn requested_extensions(&self) -> Vec<String> {
        self.call_context
            .as_ref()
            .map(|ctx| ctx.get_requested_extensions())
            .unwrap_or_default()
    }
    
    /// Checks if an extension is activated
    pub fn is_extension_activated(&self, uri: &str) -> bool {
        self.call_context
            .as_ref()
            .map(|ctx| ctx.is_extension_activated(uri))
            .unwrap_or(false)
    }
    
    /// Ensures a task ID is present, generating one if necessary
    async fn check_or_generate_task_id(&mut self) -> Result<(), A2AError> {
        if self.request.is_none() {
            return Ok(());
        }
        
        let request = self.request.as_mut().unwrap();
        
        if self.task_id.is_none() && request.message.task_id.is_none() {
            let id_context = IDGeneratorContext::with_context_id(
                self.context_id.clone().unwrap_or_default()
            );
            let generated_id = self.task_id_generator.generate(&id_context).await?;
            request.message.task_id = Some(generated_id.clone());
            self.task_id = Some(generated_id);
        } else if let Some(ref task_id) = request.message.task_id {
            self.task_id = Some(task_id.clone());
        }
        
        Ok(())
    }
    
    /// Ensures a context ID is present, generating one if necessary
    async fn check_or_generate_context_id(&mut self) -> Result<(), A2AError> {
        if self.request.is_none() {
            return Ok(());
        }
        
        let request = self.request.as_mut().unwrap();
        
        if self.context_id.is_none() && request.message.context_id.is_none() {
            let id_context = IDGeneratorContext::with_task_id(
                self.task_id.clone().unwrap_or_default()
            );
            let generated_id = self.context_id_generator.generate(&id_context).await?;
            request.message.context_id = Some(generated_id.clone());
            self.context_id = Some(generated_id);
        } else if let Some(ref context_id) = request.message.context_id {
            self.context_id = Some(context_id.clone());
        }
        
        Ok(())
    }
    
    /// Extracts text content from a message
    fn extract_text_from_message(&self, message: &Message, delimiter: &str) -> String {
        let mut text_parts = Vec::new();
        
        for part in &message.parts {
            if let crate::PartRoot::Text(text_part) = part.root() {
                text_parts.push(&text_part.text);
            }
        }
        
        text_parts.iter().map(|s| s.to_string()).collect::<Vec<_>>().join(delimiter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Part, Role, TaskState};
    use crate::a2a::auth::user::{AuthenticatedUser};
    use crate::a2a::server::id_generator::SequentialIDGenerator;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_request_context_creation() {
        let message = Message::new(
            Role::User,
            vec![Part::text("Hello".to_string())],
        );
        let params = MessageSendParams {
            message,
            configuration: None,
            metadata: None,
        };
        
        let context = RequestContext::new(
            Some(params),
            Some("task123".to_string()),
            Some("ctx456".to_string()),
            None,
            None,
            None,
            None,
            None,
        ).await.unwrap();
        
        assert_eq!(context.task_id, Some("task123".to_string()));
        assert_eq!(context.context_id, Some("ctx456".to_string()));
        assert!(context.current_task.is_none());
        assert!(context.related_tasks.is_empty());
    }

    #[tokio::test]
    async fn test_request_context_id_generation() {
        let message = Message::new(
            Role::User,
            vec![Part::text("Hello".to_string())],
        );
        let params = MessageSendParams {
            message,
            configuration: None,
            metadata: None,
        };
        
        let task_gen = Arc::new(SequentialIDGenerator::new());
        let ctx_gen = Arc::new(SequentialIDGenerator::new());
        
        let context = RequestContext::new(
            Some(params),
            None,
            None,
            None,
            None,
            None,
            Some(task_gen),
            Some(ctx_gen),
        ).await.unwrap();
        
        assert_eq!(context.task_id, Some("1".to_string()));
        assert_eq!(context.context_id, Some("1".to_string()));
    }

    #[tokio::test]
    async fn test_request_context_validation() {
        let task_id = Uuid::new_v4().to_string();
        let context_id = Uuid::new_v4().to_string();
        
        let message = Message::new(
            Role::User,
            vec![Part::text("Hello".to_string())],
        );
        let params = MessageSendParams {
            message,
            configuration: None,
            metadata: None,
        };
        
        let task = Task {
            id: Uuid::parse_str(&task_id).unwrap(),
            context_id: Uuid::parse_str(&context_id).unwrap(),
            status: crate::TaskStatus {
                state: TaskState::Working,
                timestamp: Some(chrono::Utc::now()),
                message: None,
            },
            artifacts: None,
            history: None,
            metadata: None,
            kind: "task".to_string(),
        };
        
        // Test matching task_id - should succeed
        let result = RequestContext::new(
            Some(params.clone()),
            Some(task_id.clone()),
            Some(context_id.clone()),
            Some(task.clone()),
            None,
            None,
            None,
            None,
        ).await;
        assert!(result.is_ok());
        
        // Test non-matching task_id - should fail
        let wrong_task_id = Uuid::new_v4().to_string();
        let result = RequestContext::new(
            Some(params.clone()),
            Some(wrong_task_id),
            Some(context_id.clone()),
            Some(task.clone()),
            None,
            None,
            None,
            None,
        ).await;
        assert!(result.is_err());
        
        // Test non-matching context_id - should fail
        let wrong_context_id = Uuid::new_v4().to_string();
        let result = RequestContext::new(
            Some(params),
            Some(task_id.clone()),
            Some(wrong_context_id),
            Some(task),
            None,
            None,
            None,
            None,
        ).await;
        assert!(result.is_err());
    }

    #[test]
    fn test_get_user_input() {
        let message = Message::new(
            Role::User,
            vec![
                Part::text("Hello".to_string()),
                Part::text("World".to_string()),
            ],
        );
        let params = MessageSendParams {
            message,
            configuration: None,
            metadata: None,
        };
        
        let context = RequestContext {
            request: Some(params),
            task_id: None,
            context_id: None,
            current_task: None,
            related_tasks: Vec::new(),
            call_context: None,
            task_id_generator: Arc::new(UUIDGenerator::new()),
            context_id_generator: Arc::new(UUIDGenerator::new()),
        };
        
        assert_eq!(context.get_user_input(" "), "Hello World");
        assert_eq!(context.get_user_input("\n"), "Hello\nWorld");
    }

    #[test]
    fn test_attach_related_task() {
        let mut context = RequestContext {
            request: None,
            task_id: None,
            context_id: None,
            current_task: None,
            related_tasks: Vec::new(),
            call_context: None,
            task_id_generator: Arc::new(UUIDGenerator::new()),
            context_id_generator: Arc::new(UUIDGenerator::new()),
        };
        
        assert!(context.related_tasks.is_empty());
        
        let task = Task {
            id: Uuid::new_v4(),
            context_id: Uuid::new_v4(),
            status: crate::TaskStatus {
                state: TaskState::Working,
                timestamp: Some(chrono::Utc::now()),
                message: None,
            },
            artifacts: None,
            history: None,
            metadata: None,
            kind: "task".to_string(),
        };
        
        context.attach_related_task(task);
        assert_eq!(context.related_tasks.len(), 1);
    }

    #[test]
    fn test_add_activated_extension() {
        let user = AuthenticatedUser::new("user123".to_string());
        let mut call_context = ServerCallContext::with_user(user);
        
        let mut context = RequestContext {
            request: None,
            task_id: None,
            context_id: None,
            current_task: None,
            related_tasks: Vec::new(),
            call_context: Some(call_context),
            task_id_generator: Arc::new(UUIDGenerator::new()),
            context_id_generator: Arc::new(UUIDGenerator::new()),
        };
        
        assert!(!context.is_extension_activated("ext1"));
        
        context.add_activated_extension("ext1".to_string());
        assert!(context.is_extension_activated("ext1"));
    }

    #[test]
    fn test_requested_extensions() {
        let mut call_context = ServerCallContext::new();
        call_context.add_requested_extension("ext1".to_string());
        call_context.add_requested_extension("ext2".to_string());
        
        let context = RequestContext {
            request: None,
            task_id: None,
            context_id: None,
            current_task: None,
            related_tasks: Vec::new(),
            call_context: Some(call_context),
            task_id_generator: Arc::new(UUIDGenerator::new()),
            context_id_generator: Arc::new(UUIDGenerator::new()),
        };
        
        let requested = context.requested_extensions();
        assert_eq!(requested.len(), 2);
        assert!(requested.contains(&"ext1".to_string()));
        assert!(requested.contains(&"ext2".to_string()));
    }

    #[test]
    fn test_metadata() {
        let mut metadata = HashMap::new();
        metadata.insert("key1".to_string(), serde_json::json!("value1"));
        metadata.insert("key2".to_string(), serde_json::json!(42));
        
        let message = Message::new(
            Role::User,
            vec![Part::text("Hello".to_string())],
        );
        let params = MessageSendParams {
            message,
            configuration: None,
            metadata: Some(metadata.clone()),
        };
        
        let context = RequestContext {
            request: Some(params),
            task_id: None,
            context_id: None,
            current_task: None,
            related_tasks: Vec::new(),
            call_context: None,
            task_id_generator: Arc::new(UUIDGenerator::new()),
            context_id_generator: Arc::new(UUIDGenerator::new()),
        };
        
        let retrieved_metadata = context.metadata();
        assert_eq!(retrieved_metadata, metadata);
    }
}
