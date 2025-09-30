//! Task queue client implementation

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_mut)]

use crate::core::*;
use crate::error::{TaskQueueError, Result};
use reqwest::Client;
use serde_json::json;
use std::collections::HashMap;

/// Task queue client for interacting with the server
pub struct TaskQueueClient {
    client: Client,
    base_url: String,
}

impl TaskQueueClient {
    /// Create a new task queue client
    pub async fn new(base_url: &str) -> Result<Self> {
        let client = Client::new();
        
        // Test connection
        let response = client
            .get(&format!("{}/health", base_url))
            .send()
            .await?;
            
        if !response.status().is_success() {
            return Err(TaskQueueError::NetworkError(
                reqwest::Error::from(response.error_for_status().unwrap_err())
            ));
        }
        
        Ok(Self {
            client,
            base_url: base_url.to_string(),
        })
    }

    /// Submit a new task
    pub async fn submit_task(&self, task: Task) -> Result<uuid::Uuid> {
        let response = self
            .client
            .post(&format!("{}/tasks", self.base_url))
            .json(&task)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(TaskQueueError::NetworkError(
                reqwest::Error::from(response.error_for_status().unwrap_err())
            ));
        }

        let result: serde_json::Value = response.json().await?;
        let task_id_str = result["task_id"]
            .as_str()
            .ok_or_else(|| TaskQueueError::InternalError("Invalid response format".to_string()))?;
        
        uuid::Uuid::parse_str(task_id_str)
            .map_err(|_| TaskQueueError::InternalError("Invalid task ID format".to_string()))
    }

    /// Get task by ID
    pub async fn get_task(&self, task_id: &uuid::Uuid) -> Result<Task> {
        let response = self
            .client
            .get(&format!("{}/tasks/{}", self.base_url, task_id))
            .send()
            .await?;

        if response.status() == 404 {
            return Err(TaskQueueError::TaskNotFound {
                task_id: task_id.to_string(),
            });
        }

        if !response.status().is_success() {
            return Err(TaskQueueError::NetworkError(
                reqwest::Error::from(response.error_for_status().unwrap_err())
            ));
        }

        let task: Task = response.json().await?;
        Ok(task)
    }

    /// Get task status
    pub async fn get_task_status(&self, task_id: &uuid::Uuid) -> Result<TaskStatus> {
        let response = self
            .client
            .get(&format!("{}/tasks/{}/status", self.base_url, task_id))
            .send()
            .await?;

        if response.status() == 404 {
            return Err(TaskQueueError::TaskNotFound {
                task_id: task_id.to_string(),
            });
        }

        if !response.status().is_success() {
            return Err(TaskQueueError::NetworkError(
                reqwest::Error::from(response.error_for_status().unwrap_err())
            ));
        }

        let result: serde_json::Value = response.json().await?;
        let status_str = result["status"]
            .as_str()
            .ok_or_else(|| TaskQueueError::InternalError("Invalid response format".to_string()))?;
        
        match status_str {
            "Pending" => Ok(TaskStatus::Pending),
            "Running" => Ok(TaskStatus::Running),
            "Completed" => Ok(TaskStatus::Completed),
            "Failed" => Ok(TaskStatus::Failed),
            "Cancelled" => Ok(TaskStatus::Cancelled),
            "WaitingForDependencies" => Ok(TaskStatus::WaitingForDependencies),
            _ => Err(TaskQueueError::InternalError("Unknown task status".to_string())),
        }
    }

    /// Get task result
    pub async fn get_task_result(&self, task_id: &uuid::Uuid) -> Result<Option<TaskResult>> {
        let response = self
            .client
            .get(&format!("{}/tasks/{}/result", self.base_url, task_id))
            .send()
            .await?;

        if response.status() == 404 {
            return Err(TaskQueueError::TaskNotFound {
                task_id: task_id.to_string(),
            });
        }

        if !response.status().is_success() {
            return Err(TaskQueueError::NetworkError(
                reqwest::Error::from(response.error_for_status().unwrap_err())
            ));
        }

        let result: serde_json::Value = response.json().await?;
        let result_value = result["result"].clone();
        
        if result_value.is_null() {
            Ok(None)
        } else {
            let task_result: TaskResult = serde_json::from_value(result_value)?;
            Ok(Some(task_result))
        }
    }

    /// List tasks with optional filters
    pub async fn list_tasks(
        &self,
        project: Option<String>,
        status: Option<String>,
    ) -> Result<Vec<Task>> {
        let mut url = format!("{}/tasks", self.base_url);
        let mut query_params = Vec::new();
        
        if let Some(project) = project {
            query_params.push(format!("project={}", project));
        }
        
        if let Some(status) = status {
            query_params.push(format!("status={}", status));
        }
        
        if !query_params.is_empty() {
            url.push('?');
            url.push_str(&query_params.join("&"));
        }

        let response = self
            .client
            .get(&url)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(TaskQueueError::NetworkError(
                reqwest::Error::from(response.error_for_status().unwrap_err())
            ));
        }

        let tasks: Vec<Task> = response.json().await?;
        Ok(tasks)
    }

    /// Submit a workflow
    pub async fn submit_workflow(&self, workflow: Workflow) -> Result<uuid::Uuid> {
        let response = self
            .client
            .post(&format!("{}/workflows", self.base_url))
            .json(&workflow)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(TaskQueueError::NetworkError(
                reqwest::Error::from(response.error_for_status().unwrap_err())
            ));
        }

        let result: serde_json::Value = response.json().await?;
        let workflow_id_str = result["workflow_id"]
            .as_str()
            .ok_or_else(|| TaskQueueError::InternalError("Invalid response format".to_string()))?;
        
        uuid::Uuid::parse_str(workflow_id_str)
            .map_err(|_| TaskQueueError::InternalError("Invalid workflow ID format".to_string()))
    }

    /// Get workflow by ID
    pub async fn get_workflow(&self, workflow_id: &uuid::Uuid) -> Result<Workflow> {
        let response = self
            .client
            .get(&format!("{}/workflows/{}", self.base_url, workflow_id))
            .send()
            .await?;

        if response.status() == 404 {
            return Err(TaskQueueError::WorkflowNotFound {
                workflow_id: workflow_id.to_string(),
            });
        }

        if !response.status().is_success() {
            return Err(TaskQueueError::NetworkError(
                reqwest::Error::from(response.error_for_status().unwrap_err())
            ));
        }

        let workflow: Workflow = response.json().await?;
        Ok(workflow)
    }

    /// Get workflow status
    pub async fn get_workflow_status(&self, workflow_id: &uuid::Uuid) -> Result<WorkflowStatus> {
        let response = self
            .client
            .get(&format!("{}/workflows/{}/status", self.base_url, workflow_id))
            .send()
            .await?;

        if response.status() == 404 {
            return Err(TaskQueueError::WorkflowNotFound {
                workflow_id: workflow_id.to_string(),
            });
        }

        if !response.status().is_success() {
            return Err(TaskQueueError::NetworkError(
                reqwest::Error::from(response.error_for_status().unwrap_err())
            ));
        }

        let result: serde_json::Value = response.json().await?;
        let status_str = result["status"]
            .as_str()
            .ok_or_else(|| TaskQueueError::InternalError("Invalid response format".to_string()))?;
        
        match status_str {
            "Pending" => Ok(WorkflowStatus::Pending),
            "Running" => Ok(WorkflowStatus::Running),
            "Completed" => Ok(WorkflowStatus::Completed),
            "Failed" => Ok(WorkflowStatus::Failed),
            "Cancelled" => Ok(WorkflowStatus::Cancelled),
            _ => Err(TaskQueueError::InternalError("Unknown workflow status".to_string())),
        }
    }

    /// Get system metrics
    pub async fn get_metrics(&self) -> Result<serde_json::Value> {
        let response = self
            .client
            .get(&format!("{}/metrics", self.base_url))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(TaskQueueError::NetworkError(
                reqwest::Error::from(response.error_for_status().unwrap_err())
            ));
        }

        let metrics: serde_json::Value = response.json().await?;
        Ok(metrics)
    }

    /// Wait for task completion
    pub async fn wait_for_task_completion(
        &self,
        task_id: &uuid::Uuid,
        timeout: Option<std::time::Duration>,
    ) -> Result<TaskResult> {
        let start_time = std::time::Instant::now();
        let timeout_duration = timeout.unwrap_or(std::time::Duration::from_secs(300)); // 5 minutes default
        
        loop {
            let status = self.get_task_status(task_id).await?;
            
            match status {
                TaskStatus::Completed => {
                    let result = self.get_task_result(task_id).await?;
                    return result.ok_or_else(|| TaskQueueError::InternalError(
                        "Task completed but no result available".to_string()
                    ));
                }
                TaskStatus::Failed => {
                    let result = self.get_task_result(task_id).await?;
                    return result.ok_or_else(|| TaskQueueError::InternalError(
                        "Task failed but no result available".to_string()
                    ));
                }
                TaskStatus::Cancelled => {
                    let result = self.get_task_result(task_id).await?;
                    return result.ok_or_else(|| TaskQueueError::InternalError(
                        "Task cancelled but no result available".to_string()
                    ));
                }
                _ => {
                    if start_time.elapsed() > timeout_duration {
                        return Err(TaskQueueError::TimeoutError {
                            operation: "wait_for_task_completion".to_string(),
                        });
                    }
                    
                    // Wait before checking again
                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                }
            }
        }
    }

    /// Wait for workflow completion
    pub async fn wait_for_workflow_completion(
        &self,
        workflow_id: &uuid::Uuid,
        timeout: Option<std::time::Duration>,
    ) -> Result<WorkflowStatus> {
        let start_time = std::time::Instant::now();
        let timeout_duration = timeout.unwrap_or(std::time::Duration::from_secs(1800)); // 30 minutes default
        
        loop {
            let status = self.get_workflow_status(workflow_id).await?;
            
            match status {
                WorkflowStatus::Completed | WorkflowStatus::Failed | WorkflowStatus::Cancelled => {
                    return Ok(status);
                }
                _ => {
                    if start_time.elapsed() > timeout_duration {
                        return Err(TaskQueueError::TimeoutError {
                            operation: "wait_for_workflow_completion".to_string(),
                        });
                    }
                    
                    // Wait before checking again
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                }
            }
        }
    }
}
