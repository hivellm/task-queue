//! Configuration management

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_mut)]

use serde::{Deserialize, Serialize};
use std::path::Path;

/// Task queue server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub storage: StorageConfig,
    pub vectorizer: VectorizerConfig,
    pub execution: ExecutionConfig,
    pub monitoring: MonitoringConfig,
}

/// Server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub grpc_port: u16,
    pub mcp_port: u16,
}

/// Storage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub database_path: String,
    pub backup_interval: String,
    pub retention_days: u32,
}

/// Vectorizer configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorizerConfig {
    pub endpoint: String,
    pub collection: String,
    pub auto_index: bool,
}

/// Execution configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionConfig {
    pub max_concurrent_tasks: u32,
    pub default_timeout: String,
    pub retry_attempts: u32,
    pub retry_delay: String,
}

/// Monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    pub metrics_enabled: bool,
    pub metrics_port: u16,
    pub health_check_interval: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                host: "0.0.0.0".to_string(),
                port: 16080,
                grpc_port: 16081,
                mcp_port: 16082,
            },
            storage: StorageConfig {
                database_path: "./data/task-queue.db".to_string(),
                backup_interval: "1h".to_string(),
                retention_days: 30,
            },
            vectorizer: VectorizerConfig {
                endpoint: "http://localhost:15002".to_string(),
                collection: "task-interactions".to_string(),
                auto_index: true,
            },
            execution: ExecutionConfig {
                max_concurrent_tasks: 10,
                default_timeout: "5m".to_string(),
                retry_attempts: 3,
                retry_delay: "1s".to_string(),
            },
            monitoring: MonitoringConfig {
                metrics_enabled: true,
                metrics_port: 9090,
                health_check_interval: "30s".to_string(),
            },
        }
    }
}

impl Config {
    /// Load configuration from file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let config: Config = serde_yaml::from_str(&content)?;
        Ok(config)
    }

    /// Save configuration to file
    pub fn to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let content = serde_yaml::to_string(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    /// Load configuration from environment variables
    pub fn from_env() -> Self {
        let mut config = Self::default();

        if let Ok(host) = std::env::var("TASK_QUEUE_HOST") {
            config.server.host = host;
        }

        if let Ok(port) = std::env::var("TASK_QUEUE_PORT") {
            if let Ok(port) = port.parse() {
                config.server.port = port;
            }
        }

        if let Ok(grpc_port) = std::env::var("TASK_QUEUE_GRPC_PORT") {
            if let Ok(port) = grpc_port.parse() {
                config.server.grpc_port = port;
            }
        }

        if let Ok(mcp_port) = std::env::var("TASK_QUEUE_MCP_PORT") {
            if let Ok(port) = mcp_port.parse() {
                config.server.mcp_port = port;
            }
        }

        if let Ok(db_path) = std::env::var("TASK_QUEUE_DB_PATH") {
            config.storage.database_path = db_path;
        }

        if let Ok(vectorizer_endpoint) = std::env::var("VECTORIZER_ENDPOINT") {
            config.vectorizer.endpoint = vectorizer_endpoint;
        }

        if let Ok(collection) = std::env::var("TASK_QUEUE_COLLECTION") {
            config.vectorizer.collection = collection;
        }

        if let Ok(max_tasks) = std::env::var("TASK_QUEUE_MAX_CONCURRENT") {
            if let Ok(max) = max_tasks.parse() {
                config.execution.max_concurrent_tasks = max;
            }
        }

        if let Ok(timeout) = std::env::var("TASK_QUEUE_DEFAULT_TIMEOUT") {
            config.execution.default_timeout = timeout;
        }

        if let Ok(attempts) = std::env::var("TASK_QUEUE_RETRY_ATTEMPTS") {
            if let Ok(attempts) = attempts.parse() {
                config.execution.retry_attempts = attempts;
            }
        }

        if let Ok(delay) = std::env::var("TASK_QUEUE_RETRY_DELAY") {
            config.execution.retry_delay = delay;
        }

        if let Ok(enabled) = std::env::var("TASK_QUEUE_METRICS_ENABLED") {
            config.monitoring.metrics_enabled = enabled.parse().unwrap_or(true);
        }

        if let Ok(metrics_port) = std::env::var("TASK_QUEUE_METRICS_PORT") {
            if let Ok(port) = metrics_port.parse() {
                config.monitoring.metrics_port = port;
            }
        }

        config
    }
}
