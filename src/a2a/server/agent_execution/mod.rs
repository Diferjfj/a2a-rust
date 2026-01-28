//! Agent Execution module
//! 
//! This module provides the core components for agent execution in the A2A server,
//! including the RequestContext, AgentExecutor trait, and related utilities.

pub mod context;
pub mod agent_executor;

pub use context::RequestContext;
pub use agent_executor::AgentExecutor;
