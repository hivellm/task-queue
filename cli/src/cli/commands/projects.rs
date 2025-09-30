//! Project management commands implementation

use crate::cli::args::ProjectsAction;
use crate::client::ApiClient;
use crate::output::OutputFormatter;
use crate::OutputFormat;
use anyhow::Result;

pub async fn handle_projects_command(
    command: crate::cli::args::ProjectsCommand,
    api_client: ApiClient,
    format: OutputFormat,
) -> Result<()> {
    match command.action {
        ProjectsAction::List => {
            list_projects(api_client, format).await
        }
        ProjectsAction::Create { name, description } => {
            create_project(api_client, name, description).await
        }
        ProjectsAction::Get { project_id } => {
            get_project(api_client, format, project_id).await
        }
        ProjectsAction::Update { project_id, name, description } => {
            update_project(api_client, project_id, name, description).await
        }
        ProjectsAction::Delete { project_id, force } => {
            delete_project(api_client, project_id, force).await
        }
        ProjectsAction::Tasks { project_id } => {
            list_project_tasks(api_client, format, project_id).await
        }
    }
}

async fn list_projects(api_client: ApiClient, format: OutputFormat) -> Result<()> {
    let projects = api_client.list_projects().await?;
    
    let formatter = OutputFormatter::new(format, true);
    let output = formatter.format_projects(&projects);
    println!("{}", output);
    
    Ok(())
}

async fn create_project(api_client: ApiClient, name: String, description: Option<String>) -> Result<()> {
    let project_data = serde_json::json!({
        "name": name,
        "description": description.unwrap_or_default()
    });
    
    let project = api_client.create_project(project_data).await?;
    
    println!("✅ Project created successfully!");
    println!("ID: {}", project.id);
    println!("Name: {}", project.name);
    
    Ok(())
}

async fn get_project(api_client: ApiClient, format: OutputFormat, project_id: String) -> Result<()> {
    let project = api_client.get_project(&project_id).await?;
    
    let formatter = OutputFormatter::new(format, true);
    let output = formatter.format_projects(&[project]);
    println!("{}", output);
    
    Ok(())
}

async fn update_project(
    api_client: ApiClient,
    project_id: String,
    name: Option<String>,
    description: Option<String>,
) -> Result<()> {
    let mut update_data = serde_json::Map::new();
    
    if let Some(name) = name {
        update_data.insert("name".to_string(), serde_json::Value::String(name));
    }
    
    if let Some(description) = description {
        update_data.insert("description".to_string(), serde_json::Value::String(description));
    }
    
    api_client.update_project(&project_id, serde_json::Value::Object(update_data)).await?;
    
    println!("✅ Project updated successfully!");
    
    Ok(())
}

async fn delete_project(api_client: ApiClient, project_id: String, force: bool) -> Result<()> {
    if !force {
        print!("Are you sure you want to delete this project? (y/N): ");
        use std::io::{self, Write};
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        if !input.trim().to_lowercase().starts_with('y') {
            println!("Operation cancelled.");
            return Ok(());
        }
    }
    
    api_client.delete_project(&project_id).await?;
    
    println!("✅ Project deleted successfully!");
    
    Ok(())
}

async fn list_project_tasks(api_client: ApiClient, format: OutputFormat, project_id: String) -> Result<()> {
    let tasks = api_client.list_tasks(None, Some(project_id), None).await?;
    
    let formatter = OutputFormatter::new(format, true);
    let output = formatter.format_tasks(&tasks);
    println!("{}", output);
    
    Ok(())
}
