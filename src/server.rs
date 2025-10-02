//! Task queue server implementation

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_mut)]

use crate::core::*;
use crate::error::{TaskQueueError, Result};
use crate::storage::StorageEngine;
use crate::vectorizer::VectorizerIntegration;
use crate::metrics::MetricsCollector;
use crate::mcp::create_mcp_router;
// MCP will be accessed via crate::
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{Html, Json},
    routing::{delete, get, post, put},
    Router,
};
use tower_http::services::ServeDir;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::cors::CorsLayer;
use tracing::{info, error, warn};

/// Task queue server state
pub struct TaskQueueServer {
    storage: Arc<StorageEngine>,
    vectorizer: Arc<VectorizerIntegration>,
    metrics: Arc<MetricsCollector>,
    tasks: Arc<RwLock<HashMap<uuid::Uuid, Task>>>,
    workflows: Arc<RwLock<HashMap<uuid::Uuid, Workflow>>>,
    projects: Arc<RwLock<HashMap<uuid::Uuid, Project>>>,
}

impl TaskQueueServer {
    /// Create a new task queue server
    pub async fn new() -> Result<Self> {
        let storage = Arc::new(StorageEngine::new().await?);
        let vectorizer = match VectorizerIntegration::new().await {
            Ok(v) => Arc::new(v),
            Err(e) => {
                warn!("Failed to initialize vectorizer (non-critical): {} - Continuing without vectorization", e);
                // Create a dummy vectorizer that does nothing
                Arc::new(VectorizerIntegration::new_dummy())
            }
        };
        let metrics = Arc::new(MetricsCollector::new());

        let server = Self {
            storage,
            vectorizer,
            metrics,
            tasks: Arc::new(RwLock::new(HashMap::new())),
            workflows: Arc::new(RwLock::new(HashMap::new())),
            projects: Arc::new(RwLock::new(HashMap::new())),
        };

        // Load existing data from storage
        server.load_data_from_storage().await?;

        Ok(server)
    }

    /// Load existing data from storage
    async fn load_data_from_storage(&self) -> Result<()> {
        info!("Loading data from storage...");

        // Load tasks
        let stored_tasks = self.storage.list_tasks().await?;
        let mut tasks = self.tasks.write().await;
        for task in stored_tasks {
            tasks.insert(task.id, task);
        }
        info!("Loaded {} tasks from storage", tasks.len());

        // Load workflows
        let stored_workflows = self.storage.list_workflows().await?;
        let mut workflows = self.workflows.write().await;
        for workflow in stored_workflows {
            workflows.insert(workflow.id, workflow);
        }
        info!("Loaded {} workflows from storage", workflows.len());

        // Load projects
        let stored_projects = self.storage.list_projects().await?;
        let mut projects = self.projects.write().await;
        for project in stored_projects {
            projects.insert(project.id, project);
        }
        info!("Loaded {} projects from storage", projects.len());

        Ok(())
    }

    /// Get reference to tasks map (for MCP access)
    pub fn tasks(&self) -> &Arc<RwLock<HashMap<uuid::Uuid, Task>>> {
        &self.tasks
    }

    /// Get reference to projects map
    pub fn projects(&self) -> &Arc<RwLock<HashMap<uuid::Uuid, Project>>> {
        &self.projects
    }

    /// Create a new project
    pub async fn create_project(&self, name: String, description: Option<String>) -> Result<uuid::Uuid> {
        let project = Project {
            id: uuid::Uuid::new_v4(),
            name,
            description,
            status: ProjectStatus::Planning,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            due_date: None,
            tags: Vec::new(),
            metadata: HashMap::new(),
        };

        let project_id = project.id;
        let mut projects = self.projects.write().await;
        projects.insert(project_id, project.clone());
        
        // Store in persistent storage
        self.storage.store_project(&project).await?;

        // Create .tasks file for project tracking
        // This helps AI models avoid creating duplicate projects/tasks
        let tasks_file_path = format!(".tasks");
        if let Ok(mut file) = tokio::fs::File::create(&tasks_file_path).await {
            let tasks_content = format!(
                "# Task IDs for project: {}\n\
                # This file tracks all tasks created for this project\n\
                # AI models should check this file before creating new projects/tasks\n\
                # to avoid duplication. Add new task IDs as they are created.\n\
                # Format: task_id: description\n\n\
                # Project Information:\n\
                # ID: {}\n\
                # Name: {}\n\
                # Description: {}\n\
                # Created: {}\n\n\
                # Task IDs (add new tasks below):\n",
                project.name,
                project_id,
                project.name,
                project.description.as_deref().unwrap_or("No description"),
                project.created_at.to_rfc3339()
            );

            if let Err(e) = tokio::fs::write(&tasks_file_path, tasks_content).await {
                warn!("Failed to create .tasks file for project {}: {}", project_id, e);
            } else {
                info!("Created .tasks tracking file for project: {}", project.name);
            }
        } else {
            warn!("Failed to create .tasks file for project: {}", project.name);
        }

        info!("Created project with ID: {}", project_id);
        Ok(project_id)
    }

    /// Get project by ID
    pub async fn get_project(&self, project_id: &uuid::Uuid) -> Result<Option<Project>> {
        let projects = self.projects.read().await;
        Ok(projects.get(project_id).cloned())
    }

    /// List all projects
    pub async fn list_projects(&self) -> Result<Vec<Project>> {
        let projects = self.projects.read().await;
        Ok(projects.values().cloned().collect())
    }

    /// Update project
    pub async fn update_project(&self, project_id: &uuid::Uuid, updates: ProjectUpdate) -> Result<()> {
        let mut projects = self.projects.write().await;
        if let Some(project) = projects.get_mut(project_id) {
            if let Some(name) = updates.name {
                project.name = name;
            }
            if let Some(description) = updates.description {
                project.description = Some(description);
            }
            if let Some(status) = updates.status {
                project.status = status;
            }
            if let Some(tags) = updates.tags {
                project.tags = tags;
            }
            project.updated_at = chrono::Utc::now();
            
            // Store in persistent storage
            self.storage.store_project(project).await?;
            
            Ok(())
        } else {
            Err(TaskQueueError::ProjectNotFound { project_id: project_id.to_string() })
        }
    }

    /// Delete project
    pub async fn delete_project(&self, project_id: &uuid::Uuid) -> Result<()> {
        let mut projects = self.projects.write().await;
        if projects.remove(project_id).is_some() {
            // Also remove project_id from all tasks
            let mut tasks = self.tasks.write().await;
            for task in tasks.values_mut() {
                if task.project_id == Some(*project_id) {
                    task.project_id = None;
                }
            }
            
            // Delete from persistent storage
            self.storage.delete_project(project_id).await?;
            
            info!("Deleted project: {}", project_id);
            Ok(())
        } else {
            Err(TaskQueueError::ProjectNotFound { project_id: project_id.to_string() })
        }
    }

    /// Get tasks by project
    pub async fn get_tasks_by_project(&self, project_id: &uuid::Uuid) -> Result<Vec<Task>> {
        let tasks = self.tasks.read().await;
        Ok(tasks.values()
            .filter(|task| task.project_id == Some(*project_id))
            .cloned()
            .collect())
    }

    /// Get reference to workflows map (for MCP access)
    pub fn workflows(&self) -> &Arc<RwLock<HashMap<uuid::Uuid, Workflow>>> {
        &self.workflows
    }

    /// Get reference to metrics (for MCP access)
    pub fn metrics(&self) -> &Arc<MetricsCollector> {
        &self.metrics
    }


    /// Start the server
    pub async fn start(&self) -> Result<()> {
        // Create MCP router (main server)
        let mcp_router = create_mcp_router(Arc::new(self.clone())).await;
        
        // Create REST API router to add to MCP
        let rest_routes = Router::new()
            // API routes
            .route("/health", get(health_check))
            .route("/tasks", post(submit_task))
            .route("/tasks/{id}", get(get_task))
            .route("/tasks/{id}/status", get(get_task_status))
            .route("/tasks/{id}/result", get(get_task_result))
            .route("/tasks/{id}/cancel", post(cancel_task))
            .route("/tasks/{id}/retry", post(retry_task))
            .route("/tasks/{id}", delete(delete_task))
            .route("/tasks/{id}", put(update_task))
            .route("/tasks/upsert", post(upsert_task))
            .route("/tasks/{id}/priority", put(update_task_priority))
            .route("/tasks/{id}/dependencies", post(add_task_dependency))
            .route("/tasks/{id}/dependencies", get(get_task_dependencies))
            .route("/tasks/{id}/advance-phase", post(advance_task_phase))
            .route("/tasks/{id}/status", put(set_task_status))
            .route("/tasks/{id}/correlations", get(get_task_correlations))
            .route("/tasks", get(list_tasks))
            .route("/workflows", get(list_workflows))
            .route("/workflows", post(submit_workflow))
            .route("/workflows/{id}", get(get_workflow))
            .route("/workflows/{id}/status", get(get_workflow_status))
            .route("/projects", post(create_project))
            .route("/projects", get(list_projects))
            .route("/projects/{id}", get(get_project))
            .route("/projects/{id}", put(update_project))
            .route("/projects/{id}", post(delete_project))
            .route("/projects/{id}/tasks", get(get_project_tasks))
            .route("/metrics", get(get_metrics))
            .route("/stats", get(get_stats))
            // Dashboard routes - serve static files
            .nest_service("/dashboard", ServeDir::new("dashboard/public"))
            .route("/", get(serve_dashboard))
            .layer(CorsLayer::permissive())
            .with_state(Arc::new(self.clone()));

        // Merge REST routes into MCP router
        let app = mcp_router.merge(rest_routes);

        let listener = tokio::net::TcpListener::bind("0.0.0.0:16080").await
            .map_err(|e| TaskQueueError::ConfigurationError(format!("Failed to bind listener: {}", e)))?;
        info!("MCP server with REST API listening on 0.0.0.0:16080");
        info!("MCP SSE endpoint: http://localhost:16080/mcp/sse");
        info!("MCP POST endpoint: http://localhost:16080/mcp/message");
        info!("Dashboard available at: http://localhost:16080");
        
        axum::serve(listener, app).await
            .map_err(|e| TaskQueueError::ConfigurationError(format!("Server error: {}", e)))?;
        Ok(())
    }

    /// Submit a new task
    pub async fn submit_task(&self, task: Task) -> Result<uuid::Uuid> {
        // Validate task
        self.validate_task(&task).await?;

        // Store in memory
        let task_id = task.id;
        {
            let mut tasks = self.tasks.write().await;
            tasks.insert(task_id, task.clone());
        }
        
        // Store in persistent storage
        self.storage.store_task(&task).await?;

        // Update .tasks file with new task ID
        // This helps AI models track existing tasks and avoid duplication
        if let Ok(tasks_content) = tokio::fs::read_to_string(".tasks").await {
            let task_entry = format!("# {}: {}\n#   Created: {}\n#   Status: {}\n#   Command: {}\n\n",
                task_id,
                task.name,
                task.created_at.to_rfc3339(),
                format!("{:?}", task.status),
                task.command
            );

            let updated_content = if tasks_content.contains("# Task IDs (add new tasks below):") {
                // Insert after the header
                tasks_content.replace("# Task IDs (add new tasks below):\n", &format!("# Task IDs (add new tasks below):\n{}", task_entry))
            } else {
                // Append to end if header not found
                format!("{}\n{}", tasks_content, task_entry)
            };

            if let Err(e) = tokio::fs::write(".tasks", updated_content).await {
                warn!("Failed to update .tasks file for task {}: {}", task_id, e);
            } else {
                info!("Updated .tasks file with new task: {}", task.name);
            }
        } else {
            warn!("Could not read .tasks file to update with new task: {}", task.name);
        }

        // Store in vectorizer (non-blocking - don't fail task submission if vectorizer fails)
        let context = TaskContext {
            task_id,
            project: task.project.clone(),
            dependencies: task.dependencies.iter().map(|d| d.task_id).collect(),
            parameters: HashMap::new(),
            execution_time: std::time::Duration::from_secs(0),
            result: TaskResult::Success {
                output: "Task submitted".to_string(),
                artifacts: vec![],
                metrics: TaskMetrics {
                    execution_time: std::time::Duration::from_secs(0),
                    memory_usage: 0,
                    cpu_usage: 0.0,
                    disk_usage: 0,
                    network_io: 0,
                },
            },
            artifacts: vec![],
            logs: vec!["Task submitted to queue".to_string()],
        };
        
        // Try to store in vectorizer, but don't fail the task submission if it fails
        match self.vectorizer.store_task_context(&context).await {
            Ok(_) => {
                info!("Task context stored in vectorizer: {}", task_id);
            },
            Err(e) => {
                warn!("Failed to store task context in vectorizer (non-critical): {} - Error: {}", task_id, e);
            }
        }
        
        // Update metrics
        self.metrics.increment_tasks_submitted();
        
        info!("Task submitted: {} ({})", task.name, task_id);
        Ok(task_id)
    }

    /// Get task by ID
    pub async fn get_task(&self, task_id: uuid::Uuid) -> Result<Task> {
        let tasks = self.tasks.read().await;
        tasks.get(&task_id)
            .cloned()
            .ok_or_else(|| TaskQueueError::TaskNotFound { 
                task_id: task_id.to_string() 
            })
    }

    /// Get task status
    pub async fn get_task_status(&self, task_id: uuid::Uuid) -> Result<TaskStatus> {
        let task = self.get_task(task_id).await?;
        Ok(task.status)
    }

    /// Get task result
    pub async fn get_task_result(&self, task_id: uuid::Uuid) -> Result<Option<TaskResult>> {
        let task = self.get_task(task_id).await?;
        Ok(task.result)
    }

    /// List tasks with optional filters
    pub async fn list_tasks(
        &self,
        project: Option<String>,
        status: Option<String>
    ) -> Result<Vec<Task>> {
        let tasks = self.tasks.read().await;
        let mut filtered_tasks: Vec<Task> = tasks.values().cloned().collect();

        if let Some(project) = project {
            filtered_tasks.retain(|task| task.project.as_ref() == Some(&project));
        }

        if let Some(status) = status {
            filtered_tasks.retain(|task| {
                // Get the effective status considering workflow status
                let effective_status = Self::get_effective_task_status(task);
                match status.as_str() {
                    "planning" => matches!(effective_status, TaskStatus::Planning),
                    "pending" => matches!(effective_status, TaskStatus::Pending),
                    "running" => matches!(effective_status, TaskStatus::Running),
                    "completed" => matches!(effective_status, TaskStatus::Completed),
                    "failed" => matches!(effective_status, TaskStatus::Failed),
                    "cancelled" => matches!(effective_status, TaskStatus::Cancelled),
                    "implementation" => matches!(effective_status, TaskStatus::InImplementation),
                    "testcreation" => matches!(effective_status, TaskStatus::TestCreation),
                    "testing" => matches!(effective_status, TaskStatus::Testing),
                    "aireview" => matches!(effective_status, TaskStatus::AIReview),
                    _ => false,
                }
            });
        }

        // Update tasks to show effective status for display
        let mut display_tasks = Vec::new();
        for task in filtered_tasks {
            let mut display_task = task.clone();
            display_task.status = Self::get_effective_task_status(&task);
            display_tasks.push(display_task);
        }

        Ok(display_tasks)
    }

    /// Get the effective task status considering workflow status and current phase
    pub fn get_effective_task_status(task: &Task) -> TaskStatus {
        // If task has an active development workflow, use workflow status
        if let Some(ref workflow) = task.development_workflow {
            info!("Task {} has workflow status: {:?}, current phase: {:?}", task.name, workflow.workflow_status, task.current_phase);
            
            // If workflow is NotStarted but task has advanced phases, use current_phase
            if workflow.workflow_status == crate::core::DevelopmentWorkflowStatus::NotStarted {
                match task.current_phase {
                    crate::core::TaskStatus::Planning => TaskStatus::Planning,
                    crate::core::TaskStatus::Implementation => TaskStatus::Implementation,
                    crate::core::TaskStatus::TestCreation => TaskStatus::TestCreation,
                    crate::core::TaskStatus::Testing => TaskStatus::Testing,
                    crate::core::TaskStatus::AIReview => TaskStatus::AIReview,
                    crate::core::TaskStatus::Finalized => TaskStatus::Finalized,
                    crate::core::TaskStatus::Completed => TaskStatus::Completed,
                    crate::core::TaskStatus::Failed => TaskStatus::Failed,
                    crate::core::TaskStatus::Cancelled => TaskStatus::Cancelled,
                    _ => TaskStatus::Planning, // Default fallback
                }
            } else {
                // Use workflow status for active workflows
                match workflow.workflow_status {
                    crate::core::DevelopmentWorkflowStatus::NotStarted => TaskStatus::Planning,
                    crate::core::DevelopmentWorkflowStatus::Planning => TaskStatus::Planning,
                    crate::core::DevelopmentWorkflowStatus::InImplementation => TaskStatus::Implementation,
                    crate::core::DevelopmentWorkflowStatus::TestCreation => TaskStatus::TestCreation,
                    crate::core::DevelopmentWorkflowStatus::Testing => TaskStatus::Testing,
                    crate::core::DevelopmentWorkflowStatus::AIReview => TaskStatus::AIReview,
                    crate::core::DevelopmentWorkflowStatus::Completed => {
                        info!("Task {} workflow is completed, returning Completed status", task.name);
                        TaskStatus::Completed
                    },
                    crate::core::DevelopmentWorkflowStatus::Failed => TaskStatus::Failed,
                }
            }
        } else {
            info!("Task {} has no workflow, using current phase: {:?}", task.name, task.current_phase);
            // Fall back to the task's current phase
            task.current_phase.clone()
        }
    }

    /// Submit a workflow
    pub async fn submit_workflow(&self, workflow: Workflow) -> Result<uuid::Uuid> {
        // Validate workflow
        self.validate_workflow(&workflow)?;
        
        // Store in memory
        let workflow_id = workflow.id;
        {
            let mut workflows = self.workflows.write().await;
            workflows.insert(workflow_id, workflow.clone());
        }
        
        // Store in persistent storage
        self.storage.store_workflow(&workflow).await?;
        
        // Update metrics
        self.metrics.increment_workflows_submitted();
        
        info!("Workflow submitted: {} ({})", workflow.name, workflow_id);
        Ok(workflow_id)
    }

    /// Get workflow by ID
    pub async fn get_workflow(&self, workflow_id: uuid::Uuid) -> Result<Workflow> {
        let workflows = self.workflows.read().await;
        workflows.get(&workflow_id)
            .cloned()
            .ok_or_else(|| TaskQueueError::WorkflowNotFound { 
                workflow_id: workflow_id.to_string() 
            })
    }

    /// Get workflow status
    pub async fn get_workflow_status(&self, workflow_id: uuid::Uuid) -> Result<WorkflowStatus> {
        let workflow = self.get_workflow(workflow_id).await?;
        Ok(workflow.status)
    }

    /// Validate task definition
    async fn validate_task(&self, task: &Task) -> Result<()> {
        if task.name.is_empty() {
            return Err(TaskQueueError::InvalidTaskDefinition {
                reason: "Task name cannot be empty".to_string(),
            });
        }

        if task.command.is_empty() {
            return Err(TaskQueueError::InvalidTaskDefinition {
                reason: "Task command cannot be empty".to_string(),
            });
        }

        if task.project_id.is_none() {
            return Err(TaskQueueError::InvalidTaskDefinition {
                reason: "Task must be associated with a project".to_string(),
            });
        }

        // Validate that the project exists
        if let Some(project_id) = &task.project_id {
            if self.get_project(project_id).await?.is_none() {
                return Err(TaskQueueError::InvalidTaskDefinition {
                    reason: format!("Project with ID {} does not exist", project_id),
                });
            }
        }

        Ok(())
    }

    /// Validate workflow definition
    fn validate_workflow(&self, workflow: &Workflow) -> Result<()> {
        if workflow.name.is_empty() {
            return Err(TaskQueueError::WorkflowValidationFailed {
                reason: "Workflow name cannot be empty".to_string(),
            });
        }
        
        if workflow.tasks.is_empty() {
            return Err(TaskQueueError::WorkflowValidationFailed {
                reason: "Workflow must contain at least one task".to_string(),
            });
        }
        
        // Check for circular dependencies
        self.check_circular_dependencies(workflow)?;
        
        Ok(())
    }

    /// Check for circular dependencies in workflow
    fn check_circular_dependencies(&self, workflow: &Workflow) -> Result<()> {
        // Simple cycle detection using DFS
        let mut visited = std::collections::HashSet::new();
        let mut rec_stack = std::collections::HashSet::new();
        
        for task in &workflow.tasks {
            if !visited.contains(&task.id) {
                if self.has_cycle(task.id, &workflow.tasks, &mut visited, &mut rec_stack) {
                    return Err(TaskQueueError::CircularDependency {
                        cycle: "Circular dependency detected in workflow".to_string(),
                    });
                }
            }
        }
        
        Ok(())
    }

    /// Helper function for cycle detection
    fn has_cycle(
        &self,
        task_id: uuid::Uuid,
        tasks: &[Task],
        visited: &mut std::collections::HashSet<uuid::Uuid>,
        rec_stack: &mut std::collections::HashSet<uuid::Uuid>,
    ) -> bool {
        visited.insert(task_id);
        rec_stack.insert(task_id);
        
        if let Some(task) = tasks.iter().find(|t| t.id == task_id) {
            for dependency in &task.dependencies {
                if !visited.contains(&dependency.task_id) {
                    if self.has_cycle(dependency.task_id, tasks, visited, rec_stack) {
                        return true;
                    }
                } else if rec_stack.contains(&dependency.task_id) {
                    return true;
                }
            }
        }
        
        rec_stack.remove(&task_id);
        false
    }

    /// Add dependency to a task
    pub async fn add_task_dependency(&self, task_id: uuid::Uuid, dependency_task_id: uuid::Uuid, task_name: Option<String>, condition: crate::core::DependencyCondition, required: bool, correlation_id: Option<String>) -> Result<()> {
        let mut tasks = self.tasks.write().await;
        if let Some(task) = tasks.get_mut(&task_id) {
            if let Some(correlation_id) = correlation_id {
                task.add_correlated_dependency(dependency_task_id, task_name, condition, required, correlation_id);
            } else {
                task.add_dependency(dependency_task_id, task_name, condition, required);
            }
            
            // Update in storage
            self.storage.store_task(task).await?;
            
            info!("Dependency added to task: {} -> {} ({})", dependency_task_id, task.name, task_id);
            Ok(())
        } else {
            Err(TaskQueueError::TaskNotFound { 
                task_id: task_id.to_string() 
            })
        }
    }

    /// Get task dependencies
    pub async fn get_task_dependencies(&self, task_id: uuid::Uuid) -> Result<Vec<crate::core::Dependency>> {
        let tasks = self.tasks.read().await;
        if let Some(task) = tasks.get(&task_id) {
            Ok(task.dependencies.clone())
        } else {
            Err(TaskQueueError::TaskNotFound { 
                task_id: task_id.to_string() 
            })
        }
    }

    /// Advance task development phase
    pub async fn advance_task_phase(&self, task_id: uuid::Uuid) -> Result<bool> {
        let mut tasks = self.tasks.write().await;
        if let Some(task) = tasks.get_mut(&task_id) {
            match task.advance_phase() {
                Ok(()) => {
                    // Update in storage
                    self.storage.store_task(task).await?;
                    
                    info!("Task phase advanced: {} ({})", task.name, task_id);
                    Ok(true)
                },
                Err(e) => {
                    error!("Failed to advance task phase: {}", e);
                    Ok(false)
                }
            }
        } else {
            Err(TaskQueueError::TaskNotFound { 
                task_id: task_id.to_string() 
            })
        }
    }

    /// Set task status with validation
    pub async fn set_task_status(&self, task_id: uuid::Uuid, new_status: TaskStatus) -> Result<()> {
        let mut tasks = self.tasks.write().await;
        if let Some(task) = tasks.get_mut(&task_id) {
            task.set_status(new_status)?;
            
            // Update in storage
            self.storage.store_task(task).await?;
            
            info!("Task status updated: {} ({})", task.name, task_id);
            Ok(())
        } else {
            Err(TaskQueueError::TaskNotFound { 
                task_id: task_id.to_string() 
            })
        }
    }

    /// Get task correlations
    pub async fn get_task_correlations(&self, task_id: uuid::Uuid) -> Result<Vec<String>> {
        let tasks = self.tasks.read().await;
        if let Some(task) = tasks.get(&task_id) {
            let correlations: Vec<String> = task.dependencies.iter()
                .filter_map(|dep| dep.correlation_id.clone())
                .collect();
            Ok(correlations)
        } else {
            Err(TaskQueueError::TaskNotFound { 
                task_id: task_id.to_string() 
            })
        }
    }

    /// Cancel a task
    pub async fn cancel_task(&self, task_id: uuid::Uuid, reason: String) -> Result<()> {
        let mut tasks = self.tasks.write().await;
        if let Some(task) = tasks.get_mut(&task_id) {
            task.status = crate::core::TaskStatus::Cancelled;
            task.result = Some(crate::core::TaskResult::Cancelled { reason: reason.clone() });
            task.updated_at = std::time::SystemTime::now();
            
            // Update in storage
            self.storage.store_task(task).await?;
            
            // Update metrics
            self.metrics.increment_tasks_cancelled();
            
            info!("Task cancelled: {} ({})", task.name, task_id);
            Ok(())
        } else {
            Err(TaskQueueError::TaskNotFound { 
                task_id: task_id.to_string() 
            })
        }
    }

    /// Delete a task
    pub async fn delete_task(&self, task_id: uuid::Uuid) -> Result<()> {
        let mut tasks = self.tasks.write().await;
        if let Some(task) = tasks.get(&task_id) {
            let task_name = task.name.clone();
            
            // Remove from memory
            tasks.remove(&task_id);
            
            // Remove from storage
            self.storage.delete_task(&task_id).await?;
            
            info!("Task deleted: {} ({})", task_name, task_id);
            Ok(())
        } else {
            Err(TaskQueueError::TaskNotFound { 
                task_id: task_id.to_string() 
            })
        }
    }

    /// Update a task
    pub async fn update_task(
        &self,
        task_id: uuid::Uuid,
        name: Option<String>,
        command: Option<String>,
        description: Option<String>,
        priority: Option<crate::core::TaskPriority>,
        status: Option<crate::core::TaskStatus>,
        project_id: Option<Option<uuid::Uuid>>,
    ) -> Result<crate::core::Task> {
        let mut tasks = self.tasks.write().await;
        if let Some(task) = tasks.get_mut(&task_id) {
            if let Some(name) = name {
                task.name = name;
            }
            if let Some(command) = command {
                task.command = command;
            }
            if let Some(description) = description {
                task.description = description;
            }
            if let Some(priority) = priority {
                task.priority = priority;
            }
            if let Some(status) = status {
                task.set_status(status)?;
            }
            if let Some(project_id) = project_id {
                task.project_id = project_id;
            }
            
            task.updated_at = std::time::SystemTime::now();
            
            // Update in storage
            self.storage.store_task(task).await?;
            
            info!("Task updated: {} ({})", task.name, task_id);
            Ok(task.clone())
        } else {
            Err(TaskQueueError::TaskNotFound { 
                task_id: task_id.to_string() 
            })
        }
    }

    /// Upsert a task (create or update by name)
    pub async fn upsert_task(
        &self,
        name: String,
        command: String,
        description: String,
        project_id: uuid::Uuid,
        priority: crate::core::TaskPriority,
        technical_specs: Option<String>,
        acceptance_criteria: Option<Vec<String>>,
    ) -> Result<crate::core::Task> {
        let mut tasks = self.tasks.write().await;
        
        // Check if task with same name exists
        let existing_id = tasks.iter().find(|(_, task)| task.name == name).map(|(id, _)| *id);
        
        if let Some(existing_id) = existing_id {
            // Update existing task
            let task = tasks.get_mut(&existing_id).unwrap();
            task.command = command;
            task.description = description;
            task.priority = priority;
            task.project_id = Some(project_id);
            if let Some(specs) = technical_specs {
                task.technical_specs = Some(specs);
            }
            if let Some(criteria) = acceptance_criteria {
                task.acceptance_criteria = criteria;
            }
            task.updated_at = std::time::SystemTime::now();

            // Validate task
            self.validate_task(task).await?;

            // Update in storage
            self.storage.store_task(task).await?;
            
            info!("Task upserted (updated): {} ({})", task.name, existing_id);
            Ok(task.clone())
        } else {
            // Create new task
            let new_task = crate::core::Task {
                id: uuid::Uuid::new_v4(),
                name: name.clone(),
                command,
                description,
                technical_specs,
                acceptance_criteria: acceptance_criteria.unwrap_or_default(),
                project: None,
                task_type: crate::core::TaskType::Simple,
                priority,
                project_id: Some(project_id),
                dependencies: Vec::new(),
                timeout: None,
                retry_attempts: 3,
                retry_delay: std::time::Duration::from_secs(30),
                environment: std::collections::HashMap::new(),
                working_directory: None,
                created_at: std::time::SystemTime::now(),
                updated_at: std::time::SystemTime::now(),
                status: crate::core::TaskStatus::Planning,
                result: None,
                phases: vec![crate::core::TaskPhase {
                    phase: crate::core::TaskStatus::Planning,
                    started_at: Some(chrono::Utc::now()),
                    completed_at: None,
                    documentation: None,
                    artifacts: Vec::new(),
                    ai_reviews: Vec::new(),
                }],
                current_phase: crate::core::TaskStatus::Planning,
                ai_reviews_required: 3,
                ai_reviews_completed: 0,
                development_workflow: Some(crate::core::DevelopmentWorkflow {
                    technical_documentation_path: None,
                    test_coverage_percentage: None,
                    ai_review_reports: vec![],
                    workflow_status: crate::core::DevelopmentWorkflowStatus::NotStarted,
                    started_at: Some(chrono::Utc::now()),
                    completed_at: None,
                }),
                metadata: std::collections::HashMap::new(),
            };

            // Validate task
            self.validate_task(&new_task).await?;

            let task_id = new_task.id;
            tasks.insert(task_id, new_task.clone());

            // Store in storage
            self.storage.store_task(&new_task).await?;
            
            // Update metrics
            self.metrics.increment_tasks_submitted();
            
            info!("Task upserted (created): {} ({})", new_task.name, task_id);
            Ok(new_task)
        }
    }

    /// Retry a task
    pub async fn retry_task(&self, task_id: uuid::Uuid, reset_retry_count: bool) -> Result<()> {
        let mut tasks = self.tasks.write().await;
        if let Some(task) = tasks.get_mut(&task_id) {
            if reset_retry_count {
                task.retry_attempts = 0;
            }
            task.status = crate::core::TaskStatus::Pending;
            task.result = None;
            task.updated_at = std::time::SystemTime::now();
            
            // Update in storage
            self.storage.store_task(task).await?;
            
            info!("Task retry initiated: {} ({})", task.name, task_id);
            Ok(())
        } else {
            Err(TaskQueueError::TaskNotFound { 
                task_id: task_id.to_string() 
            })
        }
    }

    /// Update task priority
    pub async fn update_task_priority(&self, task_id: uuid::Uuid, priority: crate::core::TaskPriority) -> Result<()> {
        let mut tasks = self.tasks.write().await;
        if let Some(task) = tasks.get_mut(&task_id) {
            task.priority = priority;
            task.updated_at = std::time::SystemTime::now();
            
            // Update in storage
            self.storage.store_task(task).await?;
            
            info!("Task priority updated: {} ({})", task.name, task_id);
            Ok(())
        } else {
            Err(TaskQueueError::TaskNotFound { 
                task_id: task_id.to_string() 
            })
        }
    }

    /// Cancel workflow
    pub async fn cancel_workflow(&self, workflow_id: uuid::Uuid, _reason: String) -> Result<()> {
        let mut workflows = self.workflows.write().await;
        if let Some(workflow) = workflows.get_mut(&workflow_id) {
            workflow.status = crate::core::WorkflowStatus::Cancelled;
            workflow.updated_at = std::time::SystemTime::now();
            
            // Update in storage
            self.storage.store_workflow(workflow).await?;
            
            info!("Workflow cancelled: {} ({})", workflow.name, workflow_id);
            Ok(())
        } else {
            Err(TaskQueueError::WorkflowNotFound { 
                workflow_id: workflow_id.to_string() 
            })
        }
    }

    /// Approve workflow
    pub async fn approve_workflow(&self, workflow_id: uuid::Uuid, _message: String) -> Result<()> {
        let mut workflows = self.workflows.write().await;
        if let Some(workflow) = workflows.get_mut(&workflow_id) {
            workflow.status = crate::core::WorkflowStatus::Running;
            workflow.updated_at = std::time::SystemTime::now();

            // Update in storage
            self.storage.store_workflow(workflow).await?;

            info!("Workflow approved: {} ({})", workflow.name, workflow_id);
            Ok(())
        } else {
            Err(TaskQueueError::WorkflowNotFound {
                workflow_id: workflow_id.to_string()
            })
        }
    }

    // ===== DEVELOPMENT WORKFLOW METHODS =====

    /// Advance development workflow to next phase
    pub async fn advance_development_workflow(&self, task_id: uuid::Uuid) -> Result<crate::core::DevelopmentWorkflowStatus> {
        let mut tasks = self.tasks.write().await;
        if let Some(task) = tasks.get_mut(&task_id) {
            if let Some(ref mut workflow) = task.development_workflow {
                let next_status = match &workflow.workflow_status {
                    crate::core::DevelopmentWorkflowStatus::NotStarted => {
                        workflow.started_at = Some(chrono::Utc::now());
                        crate::core::DevelopmentWorkflowStatus::Planning
                    },
                    crate::core::DevelopmentWorkflowStatus::Planning => {
                        crate::core::DevelopmentWorkflowStatus::InImplementation
                    },
                    crate::core::DevelopmentWorkflowStatus::InImplementation => {
                        crate::core::DevelopmentWorkflowStatus::TestCreation
                    },
                    crate::core::DevelopmentWorkflowStatus::TestCreation => {
                        crate::core::DevelopmentWorkflowStatus::Testing
                    },
                    crate::core::DevelopmentWorkflowStatus::Testing => {
                        crate::core::DevelopmentWorkflowStatus::AIReview
                    },
                    crate::core::DevelopmentWorkflowStatus::AIReview => {
                        workflow.completed_at = Some(chrono::Utc::now());
                        crate::core::DevelopmentWorkflowStatus::Completed
                    },
                    current_status => (*current_status).clone(), // Stay in current status if already completed/failed
                };

                workflow.workflow_status = next_status.clone();
                task.updated_at = std::time::SystemTime::now();

                // Update in storage
                self.storage.store_task(task).await?;

                info!("Task {} advanced to workflow status: {:?}", task.name, next_status);
                Ok(next_status)
            } else {
                // Initialize workflow if not exists
                task.development_workflow = Some(crate::core::DevelopmentWorkflow {
                    technical_documentation_path: None,
                    test_coverage_percentage: None,
                    ai_review_reports: vec![],
                    workflow_status: crate::core::DevelopmentWorkflowStatus::Planning,
                    started_at: Some(chrono::Utc::now()),
                    completed_at: None,
                });
                task.updated_at = std::time::SystemTime::now();
                self.storage.store_task(task).await?;
                Ok(crate::core::DevelopmentWorkflowStatus::Planning)
            }
        } else {
            Err(TaskQueueError::TaskNotFound {
                task_id: task_id.to_string()
            })
        }
    }

    /// Set technical documentation path for planning phase
    pub async fn set_technical_documentation(&self, task_id: uuid::Uuid, doc_path: String) -> Result<()> {
        let mut tasks = self.tasks.write().await;
        if let Some(task) = tasks.get_mut(&task_id) {
            if let Some(ref mut workflow) = task.development_workflow {
                workflow.technical_documentation_path = Some(doc_path.clone());
                task.updated_at = std::time::SystemTime::now();
                self.storage.store_task(task).await?;
                info!("Technical documentation set for task {}: {}", task.name, doc_path);
                Ok(())
            } else {
                Err(TaskQueueError::ValidationError {
                    reason: "Task has no development workflow initialized".to_string()
                })
            }
        } else {
            Err(TaskQueueError::TaskNotFound {
                task_id: task_id.to_string()
            })
        }
    }

    /// Set test coverage percentage
    pub async fn set_test_coverage(&self, task_id: uuid::Uuid, coverage: f64) -> Result<()> {
        let mut tasks = self.tasks.write().await;
        if let Some(task) = tasks.get_mut(&task_id) {
            if let Some(ref mut workflow) = task.development_workflow {
                workflow.test_coverage_percentage = Some(coverage);
                task.updated_at = std::time::SystemTime::now();
                self.storage.store_task(task).await?;
                info!("Test coverage set for task {}: {}%", task.name, coverage);
                Ok(())
            } else {
                Err(TaskQueueError::ValidationError {
                    reason: "Task has no development workflow initialized".to_string()
                })
            }
        } else {
            Err(TaskQueueError::TaskNotFound {
                task_id: task_id.to_string()
            })
        }
    }

    /// Add AI review report
    pub async fn add_ai_review_report(&self, task_id: uuid::Uuid, review: crate::core::AIDevelopmentReview) -> Result<()> {
        let mut tasks = self.tasks.write().await;
        if let Some(task) = tasks.get_mut(&task_id) {
            if let Some(ref mut workflow) = task.development_workflow {
                workflow.ai_review_reports.push(review);
                task.ai_reviews_completed = workflow.ai_review_reports.len() as u32;
                task.updated_at = std::time::SystemTime::now();
                self.storage.store_task(task).await?;
                info!("AI review report added for task {}", task.name);
                Ok(())
            } else {
                Err(TaskQueueError::ValidationError {
                    reason: "Task has no development workflow initialized".to_string()
                })
            }
        } else {
            Err(TaskQueueError::TaskNotFound {
                task_id: task_id.to_string()
            })
        }
    }

    /// Update workflow status
    pub async fn update_workflow_status(&self, workflow_id: uuid::Uuid, status: crate::core::WorkflowStatus, _message: String) -> Result<()> {
        let mut workflows = self.workflows.write().await;
        if let Some(workflow) = workflows.get_mut(&workflow_id) {
            workflow.status = status;
            workflow.updated_at = std::time::SystemTime::now();
            
            // Update in storage
            self.storage.store_workflow(workflow).await?;
            
            info!("Workflow status updated: {} ({})", workflow.name, workflow_id);
            Ok(())
        } else {
            Err(TaskQueueError::WorkflowNotFound { 
                workflow_id: workflow_id.to_string() 
            })
        }
    }

    /// List workflows
    pub async fn list_workflows(&self, _project: Option<String>, _status: Option<String>) -> Result<Vec<Workflow>> {
        self.storage.list_workflows().await
    }
}

impl Clone for TaskQueueServer {
    fn clone(&self) -> Self {
        Self {
            storage: self.storage.clone(),
            vectorizer: self.vectorizer.clone(),
            metrics: self.metrics.clone(),
            tasks: self.tasks.clone(),
            workflows: self.workflows.clone(),
            projects: self.projects.clone(),
        }
    }
}

// HTTP handlers

pub async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now(),
        "version": env!("CARGO_PKG_VERSION")
    }))
}

pub async fn submit_task(
    State(server): State<Arc<TaskQueueServer>>,
    Json(task_request): Json<crate::core::CreateTaskRequest>,
) -> std::result::Result<Json<Value>, StatusCode> {
    let task = task_request.to_task();
    match server.submit_task(task).await {
        Ok(task_id) => Ok(Json(json!({
            "task_id": task_id,
            "status": "submitted"
        }))),
        Err(e) => {
            error!("Failed to submit task: {}", e);
            Err(StatusCode::BAD_REQUEST)
        }
    }
}

pub async fn get_task(
    State(server): State<Arc<TaskQueueServer>>,
    Path(task_id): Path<String>,
) -> std::result::Result<Json<Task>, StatusCode> {
    let task_id = match uuid::Uuid::parse_str(&task_id) {
        Ok(id) => id,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };
    
    match server.get_task(task_id).await {
        Ok(task) => {
            let mut display_task = task.clone();
            display_task.status = TaskQueueServer::get_effective_task_status(&task);
            Ok(Json(display_task))
        },
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn get_task_status(
    State(server): State<Arc<TaskQueueServer>>,
    Path(task_id): Path<String>,
) -> std::result::Result<Json<Value>, StatusCode> {
    let task_id = match uuid::Uuid::parse_str(&task_id) {
        Ok(id) => id,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };
    
    match server.get_task_status(task_id).await {
        Ok(status) => Ok(Json(json!({ "status": status }))),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn get_task_result(
    State(server): State<Arc<TaskQueueServer>>,
    Path(task_id): Path<String>,
) -> std::result::Result<Json<Value>, StatusCode> {
    let task_id = match uuid::Uuid::parse_str(&task_id) {
        Ok(id) => id,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };
    
    match server.get_task_result(task_id).await {
        Ok(result) => Ok(Json(json!({ "result": result }))),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn list_tasks(
    State(server): State<Arc<TaskQueueServer>>,
    Query(params): Query<HashMap<String, String>>,
) -> std::result::Result<Json<Vec<Task>>, StatusCode> {
    let project = params.get("project").cloned();
    let status = params.get("status").cloned();
    
    match server.list_tasks(project, status).await {
        Ok(tasks) => Ok(Json(tasks)),
        Err(e) => {
            error!("Failed to list tasks: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn submit_workflow(
    State(server): State<Arc<TaskQueueServer>>,
    Json(workflow): Json<Workflow>,
) -> std::result::Result<Json<Value>, StatusCode> {
    match server.submit_workflow(workflow).await {
        Ok(workflow_id) => Ok(Json(json!({
            "workflow_id": workflow_id,
            "status": "submitted"
        }))),
        Err(e) => {
            error!("Failed to submit workflow: {}", e);
            Err(StatusCode::BAD_REQUEST)
        }
    }
}

pub async fn get_workflow(
    State(server): State<Arc<TaskQueueServer>>,
    Path(workflow_id): Path<String>,
) -> std::result::Result<Json<Workflow>, StatusCode> {
    let workflow_id = match uuid::Uuid::parse_str(&workflow_id) {
        Ok(id) => id,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };
    
    match server.get_workflow(workflow_id).await {
        Ok(workflow) => Ok(Json(workflow)),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn get_workflow_status(
    State(server): State<Arc<TaskQueueServer>>,
    Path(workflow_id): Path<String>,
) -> std::result::Result<Json<Value>, StatusCode> {
    let workflow_id = match uuid::Uuid::parse_str(&workflow_id) {
        Ok(id) => id,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };
    
    match server.get_workflow_status(workflow_id).await {
        Ok(status) => Ok(Json(json!({ "status": status }))),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn get_metrics(
    State(server): State<Arc<TaskQueueServer>>,
) -> Json<Value> {
    Json(server.metrics.get_metrics())
}

/// Cancel a task
pub async fn cancel_task(
    State(server): State<Arc<TaskQueueServer>>,
    Path(task_id): Path<String>,
    Json(payload): Json<serde_json::Value>,
) -> std::result::Result<Json<Value>, StatusCode> {
    let task_id = match uuid::Uuid::parse_str(&task_id) {
        Ok(id) => id,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };
    
    let reason = payload.get("reason")
        .and_then(|r| r.as_str())
        .unwrap_or("User requested cancellation");
    
    match server.cancel_task(task_id, reason.to_string()).await {
        Ok(_) => Ok(Json(json!({
            "message": "Task cancelled successfully",
            "task_id": task_id
        }))),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

/// Retry a task
pub async fn retry_task(
    State(server): State<Arc<TaskQueueServer>>,
    Path(task_id): Path<String>,
    Json(payload): Json<serde_json::Value>,
) -> std::result::Result<Json<Value>, StatusCode> {
    let task_id = match uuid::Uuid::parse_str(&task_id) {
        Ok(id) => id,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };
    
    let reset_retry_count = payload.get("reset_retry_count")
        .and_then(|r| r.as_bool())
        .unwrap_or(false);
    
    match server.retry_task(task_id, reset_retry_count).await {
        Ok(_) => Ok(Json(json!({
            "message": "Task retry initiated successfully",
            "task_id": task_id
        }))),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

/// Delete a task
pub async fn delete_task(
    State(server): State<Arc<TaskQueueServer>>,
    Path(task_id): Path<String>,
) -> std::result::Result<Json<Value>, StatusCode> {
    let task_id = match uuid::Uuid::parse_str(&task_id) {
        Ok(id) => id,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };
    
    match server.delete_task(task_id).await {
        Ok(_) => Ok(Json(json!({
            "message": "Task deleted successfully",
            "task_id": task_id
        }))),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

/// Update a task
pub async fn update_task(
    State(server): State<Arc<TaskQueueServer>>,
    Path(task_id): Path<String>,
    Json(payload): Json<serde_json::Value>,
) -> std::result::Result<Json<Value>, StatusCode> {
    let task_id = match uuid::Uuid::parse_str(&task_id) {
        Ok(id) => id,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };
    
    let name = payload.get("name").and_then(|n| n.as_str()).map(|s| s.to_string());
    let command = payload.get("command").and_then(|c| c.as_str()).map(|s| s.to_string());
    let description = payload.get("description").and_then(|d| d.as_str()).map(|s| s.to_string());
    let priority = payload.get("priority").and_then(|p| p.as_str()).and_then(|p| match p {
        "Low" => Some(crate::core::TaskPriority::Low),
        "Normal" => Some(crate::core::TaskPriority::Normal),
        "High" => Some(crate::core::TaskPriority::High),
        "Critical" => Some(crate::core::TaskPriority::Critical),
        _ => None,
    });
    let status = payload.get("status").and_then(|s| s.as_str()).and_then(|s| match s {
        "Planning" => Some(crate::core::TaskStatus::Planning),
        "Implementation" => Some(crate::core::TaskStatus::Implementation),
        "TestCreation" => Some(crate::core::TaskStatus::TestCreation),
        "Testing" => Some(crate::core::TaskStatus::Testing),
        "AIReview" => Some(crate::core::TaskStatus::AIReview),
        "Finalized" => Some(crate::core::TaskStatus::Finalized),
        "Pending" => Some(crate::core::TaskStatus::Pending),
        "Running" => Some(crate::core::TaskStatus::Running),
        "Completed" => Some(crate::core::TaskStatus::Completed),
        "Failed" => Some(crate::core::TaskStatus::Failed),
        "Cancelled" => Some(crate::core::TaskStatus::Cancelled),
        _ => None,
    });
    let project_id = payload.get("project_id").and_then(|p| {
        if p.is_null() {
            Some(None)
        } else {
            p.as_str().and_then(|s| uuid::Uuid::parse_str(s).ok()).map(Some)
        }
    });
    
    match server.update_task(task_id, name, command, description, priority, status, project_id).await {
        Ok(task) => Ok(Json(json!({
            "message": "Task updated successfully",
            "task": {
                "id": task.id,
                "name": task.name,
                "command": task.command,
                "description": task.description,
                "status": format!("{:?}", task.status),
                "priority": format!("{:?}", task.priority),
                "updated_at": task.updated_at,
            }
        }))),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

/// Upsert a task
pub async fn upsert_task(
    State(server): State<Arc<TaskQueueServer>>,
    Json(payload): Json<serde_json::Value>,
) -> std::result::Result<Json<Value>, StatusCode> {
    let name = payload.get("name")
        .and_then(|n| n.as_str())
        .ok_or(StatusCode::BAD_REQUEST)?;
    
    let command = payload.get("command")
        .and_then(|c| c.as_str())
        .ok_or(StatusCode::BAD_REQUEST)?;
    
    let description = payload.get("description")
        .and_then(|d| d.as_str())
        .ok_or(StatusCode::BAD_REQUEST)?;

    let project_id_str = payload.get("project_id")
        .and_then(|p| p.as_str())
        .ok_or(StatusCode::BAD_REQUEST)?;

    let project_id = uuid::Uuid::parse_str(project_id_str)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let priority = payload.get("priority").and_then(|p| p.as_str()).map(|p| match p {
        "Low" => crate::core::TaskPriority::Low,
        "Normal" => crate::core::TaskPriority::Normal,
        "High" => crate::core::TaskPriority::High,
        "Critical" => crate::core::TaskPriority::Critical,
        _ => crate::core::TaskPriority::Normal,
    }).unwrap_or(crate::core::TaskPriority::Normal);
    
    let technical_specs = payload.get("technical_specs").and_then(|t| t.as_str()).map(|s| s.to_string());
    let acceptance_criteria = payload.get("acceptance_criteria").and_then(|a| a.as_array()).map(|arr| {
        arr.iter().filter_map(|v| v.as_str()).map(|s| s.to_string()).collect()
    });
    
    match server.upsert_task(
        name.to_string(),
        command.to_string(),
        description.to_string(),
        project_id,
        priority,
        technical_specs,
        acceptance_criteria,
    ).await {
        Ok(task) => Ok(Json(json!({
            "message": "Task upserted successfully",
            "task": {
                "id": task.id,
                "name": task.name,
                "command": task.command,
                "description": task.description,
                "status": format!("{:?}", task.status),
                "priority": format!("{:?}", task.priority),
                "created_at": task.created_at,
                "updated_at": task.updated_at,
            }
        }))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// Update task priority
pub async fn update_task_priority(
    State(server): State<Arc<TaskQueueServer>>,
    Path(task_id): Path<String>,
    Json(payload): Json<serde_json::Value>,
) -> std::result::Result<Json<Value>, StatusCode> {
    let task_id = match uuid::Uuid::parse_str(&task_id) {
        Ok(id) => id,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };
    
    let priority_str = payload.get("priority")
        .and_then(|p| p.as_str())
        .unwrap_or("Normal");
    
    let priority = match priority_str {
        "Low" => crate::core::TaskPriority::Low,
        "Normal" => crate::core::TaskPriority::Normal,
        "High" => crate::core::TaskPriority::High,
        "Critical" => crate::core::TaskPriority::Critical,
        _ => return Err(StatusCode::BAD_REQUEST),
    };
    
    match server.update_task_priority(task_id, priority).await {
        Ok(_) => Ok(Json(json!({
            "message": "Task priority updated successfully",
            "task_id": task_id
        }))),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

/// Get system stats
pub async fn get_stats(
    State(server): State<Arc<TaskQueueServer>>,
) -> Json<Value> {
    let tasks = server.tasks.read().await;
    let workflows = server.workflows.read().await;
    
    let total_tasks = tasks.len();
    let active_tasks = tasks.values().filter(|t| {
        let effective_status = TaskQueueServer::get_effective_task_status(t);
        effective_status == crate::core::TaskStatus::Running
    }).count();
    let pending_tasks = tasks.values().filter(|t| {
        let effective_status = TaskQueueServer::get_effective_task_status(t);
        effective_status == crate::core::TaskStatus::Pending
    }).count();
    let completed_tasks = tasks.values().filter(|t| {
        info!("Checking task: {}", t.name);
        let effective_status = TaskQueueServer::get_effective_task_status(t);
        info!("Task {} effective status: {:?}", t.name, effective_status);
        let is_completed = effective_status == crate::core::TaskStatus::Completed;
        info!("Is completed: {}", is_completed);
        if is_completed {
            info!("Task {} is completed!", t.name);
        }
        is_completed
    }).count();
    let failed_tasks = tasks.values().filter(|t| {
        let effective_status = TaskQueueServer::get_effective_task_status(t);
        effective_status == crate::core::TaskStatus::Failed
    }).count();
    let total_workflows = workflows.len();
    
    Json(json!({
        "total_tasks": total_tasks,
        "active_tasks": active_tasks,
        "pending_tasks": pending_tasks,
        "completed_tasks": completed_tasks,
        "failed_tasks": failed_tasks,
        "total_workflows": total_workflows,
        "cpu_usage_percent": 0.0,
        "memory_usage_mb": 0.0,
        "uptime_seconds": 0,
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

/// Add dependency to a task
pub async fn add_task_dependency(
    State(server): State<Arc<TaskQueueServer>>,
    Path(task_id): Path<String>,
    Json(request): Json<serde_json::Value>,
) -> std::result::Result<Json<Value>, StatusCode> {
    let task_id = match uuid::Uuid::parse_str(&task_id) {
        Ok(id) => id,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };
    
    let dependency_task_id = match request.get("dependency_task_id").and_then(|v| v.as_str()) {
        Some(id) => match uuid::Uuid::parse_str(id) {
            Ok(id) => id,
            Err(_) => return Err(StatusCode::BAD_REQUEST),
        },
        None => return Err(StatusCode::BAD_REQUEST),
    };
    
    let task_name = request.get("task_name").and_then(|v| v.as_str()).map(|s| s.to_string());
    let condition = match request.get("condition").and_then(|v| v.as_str()) {
        Some("Success") => crate::core::DependencyCondition::Success,
        Some("Failure") => crate::core::DependencyCondition::Failure,
        Some("Completion") => crate::core::DependencyCondition::Completion,
        _ => crate::core::DependencyCondition::Success,
    };
    let required = request.get("required").and_then(|v| v.as_bool()).unwrap_or(true);
    let correlation_id = request.get("correlation_id").and_then(|v| v.as_str()).map(|s| s.to_string());
    
    match server.add_task_dependency(task_id, dependency_task_id, task_name, condition, required, correlation_id).await {
        Ok(_) => Ok(Json(json!({
            "message": "Dependency added successfully",
            "task_id": task_id
        }))),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

/// Get task dependencies
pub async fn get_task_dependencies(
    State(server): State<Arc<TaskQueueServer>>,
    Path(task_id): Path<String>,
) -> std::result::Result<Json<Value>, StatusCode> {
    let task_id = match uuid::Uuid::parse_str(&task_id) {
        Ok(id) => id,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };
    
    match server.get_task_dependencies(task_id).await {
        Ok(dependencies) => Ok(Json(json!(dependencies))),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

/// Advance task development phase
pub async fn advance_task_phase(
    State(server): State<Arc<TaskQueueServer>>,
    Path(task_id): Path<String>,
) -> std::result::Result<Json<Value>, StatusCode> {
    let task_id = match uuid::Uuid::parse_str(&task_id) {
        Ok(id) => id,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };
    
    match server.advance_task_phase(task_id).await {
        Ok(advanced) => Ok(Json(json!({
            "advanced": advanced,
            "task_id": task_id
        }))),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

/// Set task status
pub async fn set_task_status(
    State(server): State<Arc<TaskQueueServer>>,
    Path(task_id): Path<String>,
    Json(payload): Json<serde_json::Value>,
) -> std::result::Result<Json<Value>, StatusCode> {
    let task_id = match uuid::Uuid::parse_str(&task_id) {
        Ok(id) => id,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };

    let status_str = payload.get("status")
        .and_then(|s| s.as_str())
        .ok_or(StatusCode::BAD_REQUEST)?;

    let new_status = match status_str {
        "Planning" => TaskStatus::Planning,
        "Implementation" => TaskStatus::Implementation,
        "TestCreation" => TaskStatus::TestCreation,
        "Testing" => TaskStatus::Testing,
        "AIReview" => TaskStatus::AIReview,
        "Finalized" => TaskStatus::Finalized,
        "Cancelled" => TaskStatus::Cancelled,
        "Failed" => TaskStatus::Failed,
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    match server.set_task_status(task_id, new_status).await {
        Ok(()) => Ok(Json(json!({
            "status": "updated",
            "message": "Task status updated successfully"
        }))),
        Err(e) => {
            error!("Failed to update task status: {}", e);
            Err(StatusCode::BAD_REQUEST)
        }
    }
}

/// Get task correlations
pub async fn get_task_correlations(
    State(server): State<Arc<TaskQueueServer>>,
    Path(task_id): Path<String>,
) -> std::result::Result<Json<Value>, StatusCode> {
    let task_id = match uuid::Uuid::parse_str(&task_id) {
        Ok(id) => id,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };
    
    match server.get_task_correlations(task_id).await {
        Ok(correlations) => Ok(Json(json!(correlations))),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

/// Get workflow result
pub async fn get_workflow_result(
    State(server): State<Arc<TaskQueueServer>>,
    Path(workflow_id): Path<String>,
) -> std::result::Result<Json<Value>, StatusCode> {
    let workflow_id = match uuid::Uuid::parse_str(&workflow_id) {
        Ok(id) => id,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };
    
    match server.get_workflow(workflow_id).await {
        Ok(workflow) => Ok(Json(json!({
            "workflow_id": workflow.id,
            "status": workflow.status,
            "result": "No result field in workflow struct"
        }))),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

/// Cancel workflow
pub async fn cancel_workflow(
    State(server): State<Arc<TaskQueueServer>>,
    Path(workflow_id): Path<String>,
    Json(payload): Json<serde_json::Value>,
) -> std::result::Result<Json<Value>, StatusCode> {
    let workflow_id = match uuid::Uuid::parse_str(&workflow_id) {
        Ok(id) => id,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };
    
    let reason = payload.get("reason")
        .and_then(|r| r.as_str())
        .unwrap_or("User requested cancellation");
    
    match server.cancel_workflow(workflow_id, reason.to_string()).await {
        Ok(_) => Ok(Json(json!({
            "message": "Workflow cancelled successfully",
            "workflow_id": workflow_id
        }))),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

/// Approve workflow
pub async fn approve_workflow(
    State(server): State<Arc<TaskQueueServer>>,
    Path(workflow_id): Path<String>,
    Json(payload): Json<serde_json::Value>,
) -> std::result::Result<Json<Value>, StatusCode> {
    let workflow_id = match uuid::Uuid::parse_str(&workflow_id) {
        Ok(id) => id,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };
    
    let message = payload.get("message")
        .and_then(|m| m.as_str())
        .unwrap_or("Workflow approved");
    
    match server.approve_workflow(workflow_id, message.to_string()).await {
        Ok(_) => Ok(Json(json!({
            "message": "Workflow approved successfully",
            "workflow_id": workflow_id
        }))),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

/// Update workflow status
pub async fn update_workflow_status(
    State(server): State<Arc<TaskQueueServer>>,
    Path(workflow_id): Path<String>,
    Json(payload): Json<serde_json::Value>,
) -> std::result::Result<Json<Value>, StatusCode> {
    let workflow_id = match uuid::Uuid::parse_str(&workflow_id) {
        Ok(id) => id,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };
    
    let status_str = payload.get("status")
        .and_then(|s| s.as_str())
        .unwrap_or("Pending");
    
    let status = match status_str {
        "Pending" => crate::core::WorkflowStatus::Pending,
        "Running" => crate::core::WorkflowStatus::Running,
        "Completed" => crate::core::WorkflowStatus::Completed,
        "Failed" => crate::core::WorkflowStatus::Failed,
        "Cancelled" => crate::core::WorkflowStatus::Cancelled,
        _ => return Err(StatusCode::BAD_REQUEST),
    };
    
    let message = payload.get("message")
        .and_then(|m| m.as_str())
        .unwrap_or("Status updated");
    
    match server.update_workflow_status(workflow_id, status, message.to_string()).await {
        Ok(_) => Ok(Json(json!({
            "message": "Workflow status updated successfully",
            "workflow_id": workflow_id
        }))),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

/// List workflows
pub async fn list_workflows(
    State(server): State<Arc<TaskQueueServer>>,
    Query(params): Query<HashMap<String, String>>,
) -> std::result::Result<Json<Vec<Workflow>>, StatusCode> {
    let project = params.get("project").cloned();
    let status = params.get("status").cloned();
    
    match server.list_workflows(project, status).await {
        Ok(workflows) => Ok(Json(workflows)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// Serve dashboard HTML
pub async fn serve_dashboard() -> Html<&'static str> {
    Html(r#"
<!DOCTYPE html>
<html lang="pt-BR">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Task Queue Dashboard</title>
    <style>
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            margin: 0;
            padding: 20px;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
        }
        .container {
            max-width: 1200px;
            margin: 0 auto;
            background: white;
            border-radius: 12px;
            box-shadow: 0 20px 40px rgba(0,0,0,0.1);
            overflow: hidden;
        }
        .header {
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            padding: 30px;
            text-align: center;
        }
        .header h1 {
            margin: 0;
            font-size: 2.5em;
            font-weight: 300;
        }
        .header p {
            margin: 10px 0 0 0;
            opacity: 0.9;
        }
        .content {
            padding: 40px;
        }
        .grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
            gap: 30px;
            margin-bottom: 40px;
        }
        .card {
            background: #f8f9fa;
            border-radius: 8px;
            padding: 25px;
            border-left: 4px solid #667eea;
        }
        .card h3 {
            margin: 0 0 15px 0;
            color: #333;
        }
        .card p {
            margin: 0 0 20px 0;
            color: #666;
            line-height: 1.6;
        }
        .btn {
            display: inline-block;
            background: #667eea;
            color: white;
            padding: 12px 24px;
            text-decoration: none;
            border-radius: 6px;
            font-weight: 500;
            transition: all 0.3s ease;
        }
        .btn:hover {
            background: #5a6fd8;
            transform: translateY(-2px);
        }
        .api-info {
            background: #e3f2fd;
            border-radius: 8px;
            padding: 20px;
            margin-top: 30px;
        }
        .api-info h3 {
            margin: 0 0 15px 0;
            color: #1976d2;
        }
        .endpoint {
            background: white;
            padding: 10px 15px;
            margin: 8px 0;
            border-radius: 4px;
            font-family: 'Courier New', monospace;
            font-size: 14px;
            border-left: 3px solid #1976d2;
        }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>🚀 Task Queue Dashboard</h1>
            <p>Sistema de Gerenciamento de Tarefas e Workflows</p>
        </div>
        
        <div class="content">
            <div class="grid">
                <div class="card">
                    <h3>📊 Overview</h3>
                    <p>Visualize estatísticas gerais do sistema, tarefas recentes e métricas de performance.</p>
                    <a href="/dashboard/" class="btn">Acessar Dashboard</a>
                </div>
                
                <div class="card">
                    <h3>📋 Tasks</h3>
                    <p>Gerencie tarefas individuais, monitore status e execute operações avançadas.</p>
                    <a href="/dashboard/" class="btn">Gerenciar Tarefas</a>
                </div>
                
                <div class="card">
                    <h3>🔄 Workflows</h3>
                    <p>Visualize e controle workflows complexos com dependências e correlações.</p>
                    <a href="/dashboard/" class="btn">Ver Workflows</a>
                </div>
                
                <div class="card">
                    <h3>📈 Metrics</h3>
                    <p>Acompanhe métricas em tempo real, performance e utilização de recursos.</p>
                    <a href="/dashboard/" class="btn">Ver Métricas</a>
                </div>
            </div>
            
            <div class="api-info">
                <h3>🔧 API Endpoints</h3>
                <div class="endpoint">GET /health - Status do servidor</div>
                <div class="endpoint">GET /stats - Estatísticas do sistema</div>
                <div class="endpoint">GET /tasks - Listar tarefas</div>
                <div class="endpoint">POST /tasks - Criar tarefa</div>
                <div class="endpoint">GET /workflows - Listar workflows</div>
                <div class="endpoint">GET /metrics - Métricas detalhadas</div>
            </div>
        </div>
    </div>
</body>
</html>
    "#)
}

// Project handlers

/// Create a new project
async fn create_project(
    State(server): State<Arc<TaskQueueServer>>,
    Json(payload): Json<serde_json::Value>,
) -> std::result::Result<Json<serde_json::Value>, StatusCode> {
    let name = payload.get("name")
        .and_then(|n| n.as_str())
        .ok_or(StatusCode::BAD_REQUEST)?;
    
    let description = payload.get("description")
        .and_then(|d| d.as_str())
        .map(|s| s.to_string());

    match server.create_project(name.to_string(), description).await {
        Ok(project_id) => Ok(Json(json!({
            "id": project_id,
            "status": "created"
        }))),
        Err(e) => {
            error!("Failed to create project: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// List all projects
async fn list_projects(
    State(server): State<Arc<TaskQueueServer>>,
) -> std::result::Result<Json<Vec<Project>>, StatusCode> {
    match server.list_projects().await {
        Ok(projects) => Ok(Json(projects)),
        Err(e) => {
            error!("Failed to list projects: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Get project by ID
async fn get_project(
    State(server): State<Arc<TaskQueueServer>>,
    Path(project_id): Path<String>,
) -> std::result::Result<Json<Project>, StatusCode> {
    let project_id = match uuid::Uuid::parse_str(&project_id) {
        Ok(id) => id,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };

    match server.get_project(&project_id).await {
        Ok(Some(project)) => Ok(Json(project)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            error!("Failed to get project: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Update project
async fn update_project(
    State(server): State<Arc<TaskQueueServer>>,
    Path(project_id): Path<String>,
    Json(payload): Json<serde_json::Value>,
) -> std::result::Result<Json<serde_json::Value>, StatusCode> {
    let project_id = match uuid::Uuid::parse_str(&project_id) {
        Ok(id) => id,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };

    let updates = ProjectUpdate {
        name: payload.get("name").and_then(|n| n.as_str()).map(|s| s.to_string()),
        description: payload.get("description").and_then(|d| d.as_str()).map(|s| s.to_string()),
        status: payload.get("status").and_then(|s| s.as_str()).and_then(|s| match s {
            "Planning" => Some(ProjectStatus::Planning),
            "Active" => Some(ProjectStatus::Active),
            "OnHold" => Some(ProjectStatus::OnHold),
            "Completed" => Some(ProjectStatus::Completed),
            "Cancelled" => Some(ProjectStatus::Cancelled),
            _ => None,
        }),
        tags: payload.get("tags").and_then(|t| t.as_array()).map(|arr| {
            arr.iter().filter_map(|v| v.as_str()).map(|s| s.to_string()).collect()
        }),
    };

    match server.update_project(&project_id, updates).await {
        Ok(()) => Ok(Json(json!({"status": "updated"}))),
        Err(e) => {
            error!("Failed to update project: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Delete project
async fn delete_project(
    State(server): State<Arc<TaskQueueServer>>,
    Path(project_id): Path<String>,
) -> std::result::Result<Json<serde_json::Value>, StatusCode> {
    let project_id = match uuid::Uuid::parse_str(&project_id) {
        Ok(id) => id,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };

    match server.delete_project(&project_id).await {
        Ok(()) => Ok(Json(json!({"status": "deleted"}))),
        Err(e) => {
            error!("Failed to delete project: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Get tasks by project
async fn get_project_tasks(
    State(server): State<Arc<TaskQueueServer>>,
    Path(project_id): Path<String>,
) -> std::result::Result<Json<Vec<Task>>, StatusCode> {
    let project_id = match uuid::Uuid::parse_str(&project_id) {
        Ok(id) => id,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };

    match server.get_tasks_by_project(&project_id).await {
        Ok(tasks) => Ok(Json(tasks)),
        Err(e) => {
            error!("Failed to get project tasks: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
