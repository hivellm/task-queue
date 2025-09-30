//! Vectorizer integration for task interaction persistence

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_mut)]

use crate::core::*;
use crate::error::{TaskQueueError, Result};
use reqwest::Client;
use serde_json::json;

/// Vectorizer integration for storing task interactions
pub struct VectorizerIntegration {
    client: Client,
    base_url: String,
    collection: String,
}

impl VectorizerIntegration {
    /// Create a new vectorizer integration
    pub async fn new() -> Result<Self> {
        let client = Client::new();
        let base_url = "http://localhost:15001".to_string();
        let collection = "task-interactions".to_string();
        
        // Test connection (optional - don't fail if vectorizer is not available)
        let response = client
            .get(&format!("{}/api/v1/health", base_url))
            .timeout(std::time::Duration::from_secs(5))
            .send()
            .await;
            
        match response {
            Ok(resp) if resp.status().is_success() => {
                //println!("✅ Vectorizer connection successful");
            }
            _ => {
                println!("⚠️  Vectorizer not available - running without vectorization");
            }
        }
        
        Ok(Self {
            client,
            base_url,
            collection,
        })
    }

    /// Store task context in vectorizer
    pub async fn store_task_context(&self, context: &TaskContext) -> Result<()> {
        // Create a rich text representation of the task context
        let text = self.create_context_text(context);
        
        // Create metadata
        let metadata = json!({
            "task_id": context.task_id,
            "project": context.project,
            "execution_time_ms": context.execution_time.as_millis(),
            "artifacts": context.artifacts,
            "dependencies": context.dependencies,
            "timestamp": chrono::Utc::now(),
            "result_type": match &context.result {
                TaskResult::Success { .. } => "success",
                TaskResult::Failure { .. } => "failure",
                TaskResult::Cancelled { .. } => "cancelled",
            }
        });

        // Insert into vectorizer
        let payload = json!({
            "collection": self.collection,
            "texts": [{
                "id": context.task_id.to_string(),
                "text": text,
                "metadata": metadata
            }]
        });

        let response = self
            .client
            .post(&format!("{}/api/insert_texts", self.base_url))
            .json(&payload)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(TaskQueueError::VectorizerError(
                format!("Failed to store task context: {}", response.status())
            ));
        }

        Ok(())
    }

    /// Search for similar task contexts
    pub async fn search_task_contexts(
        &self,
        query: &str,
        limit: Option<usize>,
    ) -> Result<Vec<TaskContextSearchResult>> {
        let payload = json!({
            "collection": self.collection,
            "query": query,
            "limit": limit.unwrap_or(10)
        });

        let response = self
            .client
            .post(&format!("{}/api/search_vectors", self.base_url))
            .json(&payload)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(TaskQueueError::VectorizerError(
                format!("Failed to search task contexts: {}", response.status())
            ));
        }

        let results: serde_json::Value = response.json().await?;
        let mut search_results = Vec::new();

        if let Some(results_array) = results["results"].as_array() {
            for result in results_array {
                let search_result = TaskContextSearchResult {
                    task_id: result["id"].as_str().unwrap_or("").to_string(),
                    score: result["score"].as_f64().unwrap_or(0.0),
                    text: result["text"].as_str().unwrap_or("").to_string(),
                    metadata: result["metadata"].clone(),
                };
                search_results.push(search_result);
            }
        }

        Ok(search_results)
    }

    /// Get task recommendations based on current context
    pub async fn get_task_recommendations(
        &self,
        current_task: &Task,
        limit: Option<usize>,
    ) -> Result<Vec<TaskRecommendation>> {
        // Create a query based on the current task
        let query = format!(
            "Task: {} Project: {} Command: {}",
            current_task.name,
            current_task.project.as_deref().unwrap_or("unknown"),
            current_task.command
        );

        let search_results = self.search_task_contexts(&query, limit).await?;
        let mut recommendations = Vec::new();

        for result in search_results {
            if let Some(metadata) = result.metadata.as_object() {
                let recommendation = TaskRecommendation {
                    task_id: result.task_id,
                    similarity_score: result.score,
                    suggested_dependencies: metadata
                        .get("dependencies")
                        .and_then(|d| d.as_array())
                        .map(|d| d.iter().filter_map(|v| v.as_str()).map(|s| s.to_string()).collect())
                        .unwrap_or_default(),
                    suggested_artifacts: metadata
                        .get("artifacts")
                        .and_then(|a| a.as_array())
                        .map(|a| a.iter().filter_map(|v| v.as_str()).map(|s| s.to_string()).collect())
                        .unwrap_or_default(),
                    execution_time_estimate: metadata
                        .get("execution_time_ms")
                        .and_then(|t| t.as_u64())
                        .map(|t| std::time::Duration::from_millis(t))
                        .unwrap_or_default(),
                };
                recommendations.push(recommendation);
            }
        }

        Ok(recommendations)
    }

    /// Create a rich text representation of task context
    fn create_context_text(&self, context: &TaskContext) -> String {
        let mut text = format!("Task ID: {}\n", context.task_id);
        
        if let Some(project) = &context.project {
            text.push_str(&format!("Project: {}\n", project));
        }
        
        text.push_str(&format!("Execution Time: {}ms\n", context.execution_time.as_millis()));
        
        text.push_str("Dependencies: ");
        for dep in &context.dependencies {
            text.push_str(&format!("{}, ", dep));
        }
        text.push('\n');
        
        text.push_str("Artifacts: ");
        for artifact in &context.artifacts {
            text.push_str(&format!("{}, ", artifact));
        }
        text.push('\n');
        
        text.push_str("Logs: ");
        for log in &context.logs {
            text.push_str(&format!("{}, ", log));
        }
        text.push('\n');
        
        match &context.result {
            TaskResult::Success { output, artifacts, metrics } => {
                text.push_str(&format!("Result: SUCCESS\nOutput: {}\n", output));
                text.push_str(&format!("Artifacts: {:?}\n", artifacts));
                text.push_str(&format!("Metrics: {:?}\n", metrics));
            }
            TaskResult::Failure { error, exit_code, logs } => {
                text.push_str(&format!("Result: FAILURE\nError: {}\n", error));
                if let Some(code) = exit_code {
                    text.push_str(&format!("Exit Code: {}\n", code));
                }
                text.push_str(&format!("Logs: {:?}\n", logs));
            }
            TaskResult::Cancelled { reason } => {
                text.push_str(&format!("Result: CANCELLED\nReason: {}\n", reason));
            }
        }
        
        text
    }
}

/// Task context search result
#[derive(Debug, Clone)]
pub struct TaskContextSearchResult {
    pub task_id: String,
    pub score: f64,
    pub text: String,
    pub metadata: serde_json::Value,
}

/// Task recommendation
#[derive(Debug, Clone)]
pub struct TaskRecommendation {
    pub task_id: String,
    pub similarity_score: f64,
    pub suggested_dependencies: Vec<String>,
    pub suggested_artifacts: Vec<String>,
    pub execution_time_estimate: std::time::Duration,
}
