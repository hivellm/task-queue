//! Task Queue HTTP Server
//!
//! This binary starts the Task Queue HTTP server with all necessary components.

use tracing::{info, error};
use crate::server::TaskQueueServer;
use crate::logging::{StructuredLogger, LoggingConfig, LogLevel, LogFormat, LogOutput};
use std::sync::Arc;

mod cache;
mod client;
mod config;
mod core;
mod error;
mod logging;
mod metrics;
mod rate_limiting;
mod server;
mod storage;
mod vectorizer;
mod websocket;
mod mcp;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize structured logging
    let logging_config = LoggingConfig {
        level: LogLevel::Info,
        format: LogFormat::Pretty,
        output: LogOutput::Both("logs/task-queue.log".to_string()),
        enable_span_events: true,
        enable_file_logging: true,
        log_file_path: Some("logs/task-queue.log".to_string()),
        max_file_size: Some(10 * 1024 * 1024), // 10MB
        max_files: Some(5),
    };

    let logger = StructuredLogger::new(logging_config);
    logger.init()?;

    info!("🚀 Starting Task Queue Server with MCP integration");

    // Create the task queue server
    info!("🔧 Creating TaskQueueServer...");
    let server = Arc::new(TaskQueueServer::new().await?);
    info!("✅ TaskQueueServer created successfully");

    // Start the MCP server with REST API routes
    info!("🚀 Starting MCP server with REST API integration...");
    if let Err(e) = server.start().await {
        error!("❌ Failed to start server: {}", e);
        return Err(e.into());
    }

    Ok(())
}