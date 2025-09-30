//! API client for Task Queue

use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use anyhow::Result;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ApiClient {
    client: Client,
    base_url: String,
    api_key: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub name: String,
    pub command: String,
    pub description: String,
    pub project_id: Option<Uuid>,
    pub priority: String,
    pub status: TaskStatus,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TaskStatus {
    Planning,
    Implementation,
    TestCreation,
    Testing,
    AIReview,
    Completed,
    Failed,
    Cancelled,
    Pending,
    Running,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Workflow {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerStats {
    pub total_tasks: u32,
    pub active_tasks: u32,
    pub pending_tasks: u32,
    pub completed_tasks: u32,
    pub failed_tasks: u32,
    pub total_workflows: u32,
}

impl ApiClient {
    pub fn new(base_url: String, api_key: Option<String>, timeout: u64, _retry_attempts: u32) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(timeout))
            .build()
            .expect("Failed to create HTTP client");
        
        Self {
            client,
            base_url,
            api_key,
        }
    }
    
    async fn make_request<T>(&self, method: reqwest::Method, path: &str, body: Option<serde_json::Value>) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let mut request = self.client
            .request(method, &format!("{}{}", self.base_url, path));
        
        if let Some(api_key) = &self.api_key {
            request = request.header("Authorization", format!("Bearer {}", api_key));
        }
        
        if let Some(body) = body {
            request = request.json(&body);
        }
        
        let response = request.send().await?;
        
        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow::anyhow!("API error: {}", error_text));
        }
        
        let result: T = response.json().await?;
        Ok(result)
    }
    
    // Task operations
    pub async fn list_tasks(&self, status: Option<String>, project: Option<String>, priority: Option<String>) -> Result<Vec<Task>> {
        let mut params = Vec::new();
        
        if let Some(status) = status {
            params.push(format!("status={}", status));
        }
        if let Some(project) = project {
            params.push(format!("project={}", project));
        }
        if let Some(priority) = priority {
            params.push(format!("priority={}", priority));
        }
        
        let path = if params.is_empty() {
            "/tasks".to_string()
        } else {
            format!("/tasks?{}", params.join("&"))
        };
        
        self.make_request(reqwest::Method::GET, &path, None).await
    }
    
    pub async fn create_task(&self, task_data: serde_json::Value) -> Result<Task> {
        self.make_request(reqwest::Method::POST, "/tasks", Some(task_data)).await
    }
    
    pub async fn get_task(&self, task_id: &str) -> Result<Task> {
        self.make_request(reqwest::Method::GET, &format!("/tasks/{}", task_id), None).await
    }
    
    pub async fn update_task(&self, task_id: &str, update_data: serde_json::Value) -> Result<Task> {
        self.make_request(reqwest::Method::PUT, &format!("/tasks/{}", task_id), Some(update_data)).await
    }
    
    pub async fn cancel_task(&self, task_id: &str, reason: &str) -> Result<()> {
        let body = serde_json::json!({ "reason": reason });
        self.make_request::<serde_json::Value>(reqwest::Method::POST, &format!("/tasks/{}/cancel", task_id), Some(body)).await?;
        Ok(())
    }
    
    pub async fn delete_task(&self, task_id: &str) -> Result<()> {
        self.make_request::<serde_json::Value>(reqwest::Method::DELETE, &format!("/tasks/{}", task_id), None).await?;
        Ok(())
    }
    
    // Project operations
    pub async fn list_projects(&self) -> Result<Vec<Project>> {
        self.make_request(reqwest::Method::GET, "/projects", None).await
    }
    
    pub async fn create_project(&self, project_data: serde_json::Value) -> Result<Project> {
        self.make_request(reqwest::Method::POST, "/projects", Some(project_data)).await
    }
    
    pub async fn get_project(&self, project_id: &str) -> Result<Project> {
        self.make_request(reqwest::Method::GET, &format!("/projects/{}", project_id), None).await
    }
    
    pub async fn update_project(&self, project_id: &str, update_data: serde_json::Value) -> Result<Project> {
        self.make_request(reqwest::Method::PUT, &format!("/projects/{}", project_id), Some(update_data)).await
    }
    
    pub async fn delete_project(&self, project_id: &str) -> Result<()> {
        self.make_request::<serde_json::Value>(reqwest::Method::DELETE, &format!("/projects/{}", project_id), None).await?;
        Ok(())
    }
    
    // Workflow operations
    pub async fn list_workflows(&self) -> Result<Vec<Workflow>> {
        self.make_request(reqwest::Method::GET, "/workflows", None).await
    }
    
    pub async fn create_workflow(&self, workflow_data: serde_json::Value) -> Result<Workflow> {
        self.make_request(reqwest::Method::POST, "/workflows", Some(workflow_data)).await
    }
    
    pub async fn get_workflow(&self, workflow_id: &str) -> Result<Workflow> {
        self.make_request(reqwest::Method::GET, &format!("/workflows/{}", workflow_id), None).await
    }
    
    // Server operations
    pub async fn get_server_stats(&self) -> Result<ServerStats> {
        self.make_request(reqwest::Method::GET, "/stats", None).await
    }
    
    pub async fn get_server_health(&self) -> Result<serde_json::Value> {
        self.make_request(reqwest::Method::GET, "/health", None).await
    }
    
    pub async fn get_server_metrics(&self) -> Result<serde_json::Value> {
        self.make_request(reqwest::Method::GET, "/metrics", None).await
    }
}