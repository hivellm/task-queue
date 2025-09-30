//! Error types for the task queue system

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_mut)]

use thiserror::Error;

/// Main error type for the task queue system
#[derive(Error, Debug)]
pub enum TaskQueueError {
    #[error("Task not found: {task_id}")]
    TaskNotFound { task_id: String },

    #[error("Workflow not found: {workflow_id}")]
    WorkflowNotFound { workflow_id: String },

    #[error("Project not found: {project_id}")]
    ProjectNotFound { project_id: String },

    #[error("Circular dependency detected: {cycle}")]
    CircularDependency { cycle: String },

    #[error("Dependency not satisfied: {dependency}")]
    DependencyNotSatisfied { dependency: String },

    #[error("Task execution failed: {reason}")]
    TaskExecutionFailed { reason: String },

    #[error("Storage error: {0}")]
    StorageError(#[from] sled::Error),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Vectorizer error: {0}")]
    VectorizerError(String),

    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    #[error("Invalid status transition: {0}")]
    InvalidStatusTransition(String),

    #[error("Timeout error: {operation}")]
    TimeoutError { operation: String },

    #[error("Resource limit exceeded: {resource}")]
    ResourceLimitExceeded { resource: String },

    #[error("Permission denied: {operation}")]
    PermissionDenied { operation: String },

    #[error("Invalid task definition: {reason}")]
    InvalidTaskDefinition { reason: String },

    #[error("Workflow validation failed: {reason}")]
    WorkflowValidationFailed { reason: String },

    #[error("Validation error: {reason}")]
    ValidationError { reason: String },

    #[error("Internal error: {0}")]
    InternalError(String),
}

/// Result type alias
pub type Result<T> = std::result::Result<T, TaskQueueError>;

impl From<String> for TaskQueueError {
    fn from(err: String) -> Self {
        TaskQueueError::InvalidStatusTransition(err)
    }
}

