//! Storage engine for persistent task and workflow storage

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_mut)]

use crate::core::*;
use crate::error::{TaskQueueError, Result as TaskQueueResult};
use sled::{Db, Tree};
use std::sync::Arc;

/// Storage engine using Sled embedded database
pub struct StorageEngine {
    db: Arc<Db>,
    tasks_tree: Tree,
    workflows_tree: Tree,
    projects_tree: Tree,
}

impl StorageEngine {
    /// Create a new storage engine
    pub async fn new() -> TaskQueueResult<Self> {
        // Try to create data directory, fallback to temp if it fails
        let data_dir = std::env::current_dir().unwrap_or_else(|_| std::env::temp_dir()).join("task-queue-data");
        let _ = std::fs::create_dir_all(&data_dir);
        
        let db_path = data_dir.join("task-queue.db");
        println!("Opening database at: {:?}", db_path);
        
        // Try to open database, fallback to in-memory if it fails
        let db = match sled::open(&db_path) {
            Ok(db) => Arc::new(db),
            Err(e) => {
                println!("Failed to open persistent database: {}. Using in-memory database.", e);
                Arc::new(sled::Config::new().temporary(true).open()?)
            }
        };
        
        let tasks_tree = db.open_tree("tasks")?;
        let workflows_tree = db.open_tree("workflows")?;
        let projects_tree = db.open_tree("projects")?;
        
        Ok(Self {
            db,
            tasks_tree,
            workflows_tree,
            projects_tree,
        })
    }

    /// Store a task
    pub async fn store_task(&self, task: &Task) -> TaskQueueResult<()> {
        let key = task.id.to_string();
        let value = serde_json::to_vec(task)?;
        
        self.tasks_tree.insert(key, value)?;
        self.tasks_tree.flush_async().await?;
        
        Ok(())
    }

    /// Load a task by ID
    pub async fn load_task(&self, task_id: &uuid::Uuid) -> TaskQueueResult<Option<Task>> {
        let key = task_id.to_string();
        
        if let Some(value) = self.tasks_tree.get(key)? {
            let task: Task = serde_json::from_slice(&value)?;
            Ok(Some(task))
        } else {
            Ok(None)
        }
    }

    /// Store a workflow
    pub async fn store_workflow(&self, workflow: &Workflow) -> TaskQueueResult<()> {
        let key = workflow.id.to_string();
        let value = serde_json::to_vec(workflow)?;
        
        self.workflows_tree.insert(key, value)?;
        self.workflows_tree.flush_async().await?;
        
        Ok(())
    }

    /// Load a workflow by ID
    pub async fn load_workflow(&self, workflow_id: &uuid::Uuid) -> TaskQueueResult<Option<Workflow>> {
        let key = workflow_id.to_string();
        
        if let Some(value) = self.workflows_tree.get(key)? {
            let workflow: Workflow = serde_json::from_slice(&value)?;
            Ok(Some(workflow))
        } else {
            Ok(None)
        }
    }

    /// List all tasks
    pub async fn list_tasks(&self) -> TaskQueueResult<Vec<Task>> {
        let mut tasks = Vec::new();
        
        for result in self.tasks_tree.iter() {
            let (_, value) = result?;
            let task: Task = serde_json::from_slice(&value.to_vec())?;
            tasks.push(task);
        }
        
        Ok(tasks)
    }

    /// List all workflows
    pub async fn list_workflows(&self) -> TaskQueueResult<Vec<Workflow>> {
        let mut workflows = Vec::new();
        
        for result in self.workflows_tree.iter() {
            let (_, value) = result?;
            let workflow: Workflow = serde_json::from_slice(&value.to_vec())?;
            workflows.push(workflow);
        }
        
        Ok(workflows)
    }

    /// Delete a task
    pub async fn delete_task(&self, task_id: &uuid::Uuid) -> TaskQueueResult<()> {
        let key = task_id.to_string();
        self.tasks_tree.remove(key)?;
        self.tasks_tree.flush_async().await?;
        Ok(())
    }

    /// Delete a workflow
    pub async fn delete_workflow(&self, workflow_id: &uuid::Uuid) -> TaskQueueResult<()> {
        let key = workflow_id.to_string();
        self.workflows_tree.remove(key)?;
        self.workflows_tree.flush_async().await?;
        Ok(())
    }

    /// Get storage statistics
    pub async fn get_stats(&self) -> TaskQueueResult<StorageStats> {
        let task_count = self.tasks_tree.len();
        let workflow_count = self.workflows_tree.len();
        let project_count = self.projects_tree.len();
        let db_size = self.db.size_on_disk()?;
        
        Ok(StorageStats {
            task_count,
            workflow_count,
            project_count,
            db_size_bytes: db_size,
        })
    }

    /// Store a project
    pub async fn store_project(&self, project: &Project) -> TaskQueueResult<()> {
        let key = project.id.to_string();
        let value = serde_json::to_vec(project)?;
        
        self.projects_tree.insert(key, value)?;
        self.projects_tree.flush_async().await?;
        
        Ok(())
    }

    /// Load a project by ID
    pub async fn load_project(&self, project_id: &uuid::Uuid) -> TaskQueueResult<Option<Project>> {
        let key = project_id.to_string();
        
        if let Some(value) = self.projects_tree.get(key)? {
            let project: Project = serde_json::from_slice(&value)?;
            Ok(Some(project))
        } else {
            Ok(None)
        }
    }

    /// List all projects
    pub async fn list_projects(&self) -> TaskQueueResult<Vec<Project>> {
        let mut projects = Vec::new();
        
        for result in self.projects_tree.iter() {
            let (_, value) = result?;
            let project: Project = serde_json::from_slice(&value)?;
            projects.push(project);
        }
        
        Ok(projects)
    }

    /// Delete a project
    pub async fn delete_project(&self, project_id: &uuid::Uuid) -> TaskQueueResult<()> {
        let key = project_id.to_string();
        self.projects_tree.remove(key)?;
        self.projects_tree.flush_async().await?;
        Ok(())
    }
}

/// Storage statistics
#[derive(Debug, Clone)]
pub struct StorageStats {
    pub task_count: usize,
    pub workflow_count: usize,
    pub project_count: usize,
    pub db_size_bytes: u64,
}
