//! Server operations commands implementation

use crate::cli::args::ServerAction;
use crate::client::ApiClient;
use crate::output::OutputFormatter;
use crate::OutputFormat;
use anyhow::Result;

pub async fn handle_server_command(
    command: crate::cli::args::ServerCommand,
    api_client: ApiClient,
    format: OutputFormat,
) -> Result<()> {
    match command.action {
        ServerAction::Status => {
            get_server_status(api_client, format).await
        }
        ServerAction::Health => {
            check_server_health(api_client, format).await
        }
        ServerAction::Metrics => {
            get_server_metrics(api_client, format).await
        }
        ServerAction::Stats => {
            get_server_stats(api_client, format).await
        }
    }
}

async fn get_server_status(api_client: ApiClient, format: OutputFormat) -> Result<()> {
    let stats = api_client.get_server_stats().await?;
    
    let formatter = OutputFormatter::new(format, true);
    let output = formatter.format_server_stats(&stats);
    println!("{}", output);
    
    Ok(())
}

async fn check_server_health(api_client: ApiClient, format: OutputFormat) -> Result<()> {
    let health = api_client.get_server_health().await?;
    
    match format {
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&health)?);
        }
        OutputFormat::Yaml => {
            println!("{}", serde_yaml::to_string(&health)?);
        }
        OutputFormat::Table => {
            println!("Server Health Check:");
            println!("Status: {}", health.get("status").unwrap_or(&serde_json::Value::String("Unknown".to_string())));
            println!("Version: {}", health.get("version").unwrap_or(&serde_json::Value::String("Unknown".to_string())));
            println!("Timestamp: {}", health.get("timestamp").unwrap_or(&serde_json::Value::String("Unknown".to_string())));
        }
    }
    
    Ok(())
}

async fn get_server_metrics(api_client: ApiClient, format: OutputFormat) -> Result<()> {
    let metrics = api_client.get_server_metrics().await?;
    
    match format {
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&metrics)?);
        }
        OutputFormat::Yaml => {
            println!("{}", serde_yaml::to_string(&metrics)?);
        }
        OutputFormat::Table => {
            println!("Server Metrics:");
            println!("{}", serde_json::to_string_pretty(&metrics)?);
        }
    }
    
    Ok(())
}

async fn get_server_stats(api_client: ApiClient, format: OutputFormat) -> Result<()> {
    let stats = api_client.get_server_stats().await?;
    
    let formatter = OutputFormatter::new(format, true);
    let output = formatter.format_server_stats(&stats);
    println!("{}", output);
    
    Ok(())
}
