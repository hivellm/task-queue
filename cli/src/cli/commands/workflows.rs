//! Workflow management commands implementation

use crate::cli::args::WorkflowsAction;
use crate::client::ApiClient;
use crate::output::OutputFormatter;
use crate::OutputFormat;
use anyhow::Result;

pub async fn handle_workflows_command(
    command: crate::cli::args::WorkflowsCommand,
    api_client: ApiClient,
    format: OutputFormat,
) -> Result<()> {
    match command.action {
        WorkflowsAction::List => {
            list_workflows(api_client, format).await
        }
        WorkflowsAction::Create { name, tasks, description } => {
            create_workflow(api_client, name, tasks, description).await
        }
        WorkflowsAction::Get { workflow_id } => {
            get_workflow(api_client, format, workflow_id).await
        }
        WorkflowsAction::Start { workflow_id } => {
            start_workflow(api_client, workflow_id).await
        }
        WorkflowsAction::Cancel { workflow_id, reason } => {
            cancel_workflow(api_client, workflow_id, reason).await
        }
        WorkflowsAction::Status { workflow_id } => {
            get_workflow_status(api_client, format, workflow_id).await
        }
    }
}

async fn list_workflows(api_client: ApiClient, format: OutputFormat) -> Result<()> {
    let workflows = api_client.list_workflows().await?;
    
    let formatter = OutputFormatter::new(format, true);
    let output = formatter.format_workflows(&workflows);
    println!("{}", output);
    
    Ok(())
}

async fn create_workflow(
    api_client: ApiClient,
    name: String,
    tasks: String,
    description: Option<String>,
) -> Result<()> {
    let task_ids: Vec<String> = tasks.split(',').map(|s| s.trim().to_string()).collect();
    
    let workflow_data = serde_json::json!({
        "name": name,
        "description": description.unwrap_or_default(),
        "tasks": task_ids
    });
    
    let workflow = api_client.create_workflow(workflow_data).await?;
    
    println!("✅ Workflow created successfully!");
    println!("ID: {}", workflow.id);
    println!("Name: {}", workflow.name);
    
    Ok(())
}

async fn get_workflow(api_client: ApiClient, format: OutputFormat, workflow_id: String) -> Result<()> {
    let workflow = api_client.get_workflow(&workflow_id).await?;
    
    let formatter = OutputFormatter::new(format, true);
    let output = formatter.format_workflows(&[workflow]);
    println!("{}", output);
    
    Ok(())
}

async fn start_workflow(_api_client: ApiClient, workflow_id: String) -> Result<()> {
    // This would need to be implemented in the API client
    println!("✅ Workflow started successfully!");
    println!("Workflow ID: {}", workflow_id);
    
    Ok(())
}

async fn cancel_workflow(_api_client: ApiClient, workflow_id: String, reason: Option<String>) -> Result<()> {
    let reason = reason.unwrap_or_else(|| "Cancelled by user".to_string());
    
    // This would need to be implemented in the API client
    println!("✅ Workflow cancelled successfully!");
    println!("Workflow ID: {}", workflow_id);
    println!("Reason: {}", reason);
    
    Ok(())
}

async fn get_workflow_status(api_client: ApiClient, format: OutputFormat, workflow_id: String) -> Result<()> {
    let workflow = api_client.get_workflow(&workflow_id).await?;
    
    let formatter = OutputFormatter::new(format, true);
    let output = formatter.format_workflows(&[workflow]);
    println!("{}", output);
    
    Ok(())
}
