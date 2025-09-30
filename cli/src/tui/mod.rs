//! Interactive TUI mode implementation

use crate::client::ApiClient;
use crate::config::CliConfig;
use anyhow::Result;

pub async fn run_interactive_mode(
    api_client: ApiClient,
    _config: CliConfig,
) -> Result<()> {
    println!("🚀 Starting Task Queue Interactive Mode...");
    println!("Press Ctrl+C to exit");
    
    // This is a placeholder implementation
    // In a real implementation, this would launch the ratatui interface
    
    loop {
        println!("Interactive mode is not yet implemented.");
        println!("Available commands:");
        println!("  - tasks list");
        println!("  - projects list");
        println!("  - server status");
        println!("  - quit");
        
        use std::io::{self, Write};
        print!("task-queue> ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        let command = input.trim();
        
        match command {
            "quit" | "exit" => {
                println!("Goodbye!");
                break;
            }
            "tasks list" => {
                let tasks = api_client.list_tasks(None, None, None).await?;
                println!("Found {} tasks", tasks.len());
                for task in tasks {
                    println!("  - {} ({})", task.name, task.id);
                }
            }
            "projects list" => {
                let projects = api_client.list_projects().await?;
                println!("Found {} projects", projects.len());
                for project in projects {
                    println!("  - {} ({})", project.name, project.id);
                }
            }
            "server status" => {
                let stats = api_client.get_server_stats().await?;
                println!("Server Status:");
                println!("  Total Tasks: {}", stats.total_tasks);
                println!("  Active Tasks: {}", stats.active_tasks);
                println!("  Completed Tasks: {}", stats.completed_tasks);
            }
            _ => {
                println!("Unknown command: {}", command);
            }
        }
    }
    
    Ok(())
}
