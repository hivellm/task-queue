//! Task management commands implementation

use crate::cli::args::{TasksAction, TaskPriority};
use crate::client::ApiClient;
use crate::output::OutputFormatter;
use crate::OutputFormat;
use crate::utils::ProgressManager;
use anyhow::Result;
use uuid::Uuid;

pub async fn handle_tasks_command(
    command: crate::cli::args::TasksCommand,
    api_client: ApiClient,
    format: OutputFormat,
) -> Result<()> {
    match command.action {
        TasksAction::List { status, project, priority } => {
            list_tasks(api_client, format, status, project, priority).await
        }
        TasksAction::Create {
            name,
            command: cmd,
            project,
            description,
            priority,
            working_directory,
        } => {
            create_task(api_client, name, cmd, project, description, priority, working_directory).await
        }
        TasksAction::Get { task_id } => {
            get_task(api_client, format, task_id).await
        }
        TasksAction::Update {
            task_id,
            name,
            command,
            priority,
        } => {
            update_task(api_client, task_id, name, command, priority).await
        }
        TasksAction::Cancel { task_id, reason } => {
            cancel_task(api_client, task_id, reason).await
        }
        TasksAction::Delete { task_id, force } => {
            delete_task(api_client, task_id, force).await
        }
        TasksAction::Wait { task_id, timeout } => {
            wait_for_task(api_client, task_id, timeout).await
        }
    }
}

async fn list_tasks(
    api_client: ApiClient,
    format: OutputFormat,
    status: Option<String>,
    project: Option<String>,
    priority: Option<String>,
) -> Result<()> {
    let tasks = api_client.list_tasks(status, project, priority).await?;
    
    let formatter = OutputFormatter::new(format, true);
    let output = formatter.format_tasks(&tasks);
    println!("{}", output);
    
    Ok(())
}

async fn create_task(
    api_client: ApiClient,
    name: String,
    command: String,
    project: String,
    description: Option<String>,
    priority: Option<TaskPriority>,
    working_directory: Option<String>,
) -> Result<()> {
    let project_id = Uuid::parse_str(&project)?;
    
    let task_data = serde_json::json!({
        "name": name,
        "command": command,
        "project_id": project_id,
        "description": description.unwrap_or_default(),
        "priority": priority.map(|p| format!("{:?}", p)).unwrap_or_else(|| "Normal".to_string()),
        "working_directory": working_directory,
        "task_type": "Simple"
    });
    
    let task = api_client.create_task(task_data).await?;
    
    println!("✅ Task created successfully!");
    println!("ID: {}", task.id);
    println!("Name: {}", task.name);
    println!("Status: {:?}", task.status);
    
    Ok(())
}

async fn get_task(api_client: ApiClient, format: OutputFormat, task_id: String) -> Result<()> {
    let task = api_client.get_task(&task_id).await?;
    
    let formatter = OutputFormatter::new(format, true);
    let output = formatter.format_task_details(&task);
    println!("{}", output);
    
    Ok(())
}

async fn update_task(
    api_client: ApiClient,
    task_id: String,
    name: Option<String>,
    command: Option<String>,
    priority: Option<TaskPriority>,
) -> Result<()> {
    let mut update_data = serde_json::Map::new();
    
    if let Some(name) = name {
        update_data.insert("name".to_string(), serde_json::Value::String(name));
    }
    
    if let Some(command) = command {
        update_data.insert("command".to_string(), serde_json::Value::String(command));
    }
    
    if let Some(priority) = priority {
        update_data.insert("priority".to_string(), serde_json::Value::String(format!("{:?}", priority)));
    }
    
    api_client.update_task(&task_id, serde_json::Value::Object(update_data)).await?;
    
    println!("✅ Task updated successfully!");
    
    Ok(())
}

async fn cancel_task(api_client: ApiClient, task_id: String, reason: Option<String>) -> Result<()> {
    let reason = reason.unwrap_or_else(|| "Cancelled by user".to_string());
    
    api_client.cancel_task(&task_id, &reason).await?;
    
    println!("✅ Task cancelled successfully!");
    
    Ok(())
}

async fn delete_task(api_client: ApiClient, task_id: String, force: bool) -> Result<()> {
    if !force {
        print!("Are you sure you want to delete this task? (y/N): ");
        use std::io::{self, Write};
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        if !input.trim().to_lowercase().starts_with('y') {
            println!("Operation cancelled.");
            return Ok(());
        }
    }
    
    api_client.delete_task(&task_id).await?;
    
    println!("✅ Task deleted successfully!");
    
    Ok(())
}

async fn wait_for_task(api_client: ApiClient, task_id: String, timeout: u64) -> Result<()> {
    let progress_manager = ProgressManager::new();
    let pb = progress_manager.create_task_progress(&format!("Waiting for task {}", &task_id[..8]));
    
    let start_time = std::time::Instant::now();
    
    loop {
        let task = api_client.get_task(&task_id).await?;
        
        match task.status {
            crate::client::TaskStatus::Completed => {
                pb.finish_with_message("✅ Task completed successfully!");
                break;
            }
            crate::client::TaskStatus::Failed => {
                pb.finish_with_message("❌ Task failed!");
                return Err(anyhow::anyhow!("Task failed"));
            }
            crate::client::TaskStatus::Cancelled => {
                pb.finish_with_message("⚠️ Task was cancelled");
                return Err(anyhow::anyhow!("Task was cancelled"));
            }
            _ => {
                if start_time.elapsed().as_secs() > timeout {
                    pb.finish_with_message("⏰ Timeout reached");
                    return Err(anyhow::anyhow!("Timeout reached"));
                }
                
                pb.tick();
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            }
        }
    }
    
    Ok(())
}
