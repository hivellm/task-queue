//! Structured Logging Module
//!
//! Provides comprehensive logging capabilities with structured data,
//! different log levels, and integration with tracing ecosystem.
//!
//! ## 🚨 CRITICAL TESTING REQUIREMENTS
//!
//! **BEFORE ANY CHANGES TO THIS MODULE:**
//! 1. Execute tests: `cargo test --lib logging`
//! 2. Verify all tests pass: `cargo test --lib logging -- --nocapture`
//! 3. Check coverage: `cargo test --lib logging -- --nocapture --test-threads=1`
//!
//! **TESTING COMMANDS:**
//! ```bash
//! # Run Logging module tests
//! cargo test --lib logging
//!
//! # Run with verbose output
//! cargo test --lib logging -- --nocapture
//!
//! # Run specific test
//! cargo test --lib logging test_log_level_conversion
//!
//! # Run all Logging tests with coverage
//! cargo test --lib logging -- --nocapture --test-threads=1
//! ```
//!
//! **⚠️ NO COMMITS WITHOUT PASSING TESTS!**

use std::collections::HashMap;
use tracing::{info, warn, error, debug, trace, Level};
use tracing_subscriber::{fmt, EnvFilter};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Log levels supported by the system
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Trace => write!(f, "TRACE"),
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Warn => write!(f, "WARN"),
            LogLevel::Error => write!(f, "ERROR"),
        }
    }
}

impl From<LogLevel> for Level {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Trace => Level::TRACE,
            LogLevel::Debug => Level::DEBUG,
            LogLevel::Info => Level::INFO,
            LogLevel::Warn => Level::WARN,
            LogLevel::Error => Level::ERROR,
        }
    }
}

/// Structured log entry with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: DateTime<Utc>,
    pub level: LogLevel,
    pub message: String,
    pub module: String,
    pub target: String,
    pub fields: HashMap<String, serde_json::Value>,
    pub span_id: Option<String>,
    pub trace_id: Option<String>,
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: LogLevel,
    pub format: LogFormat,
    pub output: LogOutput,
    pub enable_span_events: bool,
    pub enable_file_logging: bool,
    pub log_file_path: Option<String>,
    pub max_file_size: Option<u64>,
    pub max_files: Option<usize>,
}

/// Log output format
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LogFormat {
    Json,
    Pretty,
    Compact,
}

/// Log output destination
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogOutput {
    Stdout,
    Stderr,
    File(String),
    Both(String), // File + stdout
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: LogLevel::Info,
            format: LogFormat::Pretty,
            output: LogOutput::Stdout,
            enable_span_events: true,
            enable_file_logging: false,
            log_file_path: Some("logs/task-queue.log".to_string()),
            max_file_size: Some(10 * 1024 * 1024), // 10MB
            max_files: Some(5),
        }
    }
}

/// Structured logger implementation
pub struct StructuredLogger {
    config: LoggingConfig,
}

impl StructuredLogger {
    /// Create a new structured logger with the given configuration
    pub fn new(config: LoggingConfig) -> Self {
        Self { config }
    }

    /// Initialize the logging system
    pub fn init(&self) -> Result<(), Box<dyn std::error::Error>> {
        let filter = EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new(format!("{}", self.config.level)));

        match self.config.output {
            LogOutput::Stdout => {
                self.init_stdout(&filter)?;
            }
            LogOutput::Stderr => {
                self.init_stderr(&filter)?;
            }
            LogOutput::File(ref path) => {
                self.init_file(path, &filter)?;
            }
            LogOutput::Both(ref path) => {
                self.init_both(path, &filter)?;
            }
        }

        info!("Structured logging initialized with level: {}", self.config.level);
        Ok(())
    }

    fn init_stdout(&self, filter: &EnvFilter) -> Result<(), Box<dyn std::error::Error>> {
        match self.config.format {
            LogFormat::Json => {
                fmt::Subscriber::builder()
                    .with_env_filter(filter.clone())
                    .init();
            }
            LogFormat::Pretty => {
                fmt::Subscriber::builder()
                    .with_env_filter(filter.clone())
                    .pretty()
                    .init();
            }
            LogFormat::Compact => {
                fmt::Subscriber::builder()
                    .with_env_filter(filter.clone())
                    .compact()
                    .init();
            }
        }
        Ok(())
    }

    fn init_stderr(&self, filter: &EnvFilter) -> Result<(), Box<dyn std::error::Error>> {
        match self.config.format {
            LogFormat::Json => {
                fmt::Subscriber::builder()
                    .with_env_filter(filter.clone())
                    .with_writer(std::io::stderr)
                    .init();
            }
            LogFormat::Pretty => {
                fmt::Subscriber::builder()
                    .with_env_filter(filter.clone())
                    .pretty()
                    .with_writer(std::io::stderr)
                    .init();
            }
            LogFormat::Compact => {
                fmt::Subscriber::builder()
                    .with_env_filter(filter.clone())
                    .compact()
                    .with_writer(std::io::stderr)
                    .init();
            }
        }
        Ok(())
    }

    fn init_file(&self, path: &str, filter: &EnvFilter) -> Result<(), Box<dyn std::error::Error>> {
        let file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?;
        
        match self.config.format {
            LogFormat::Json => {
                fmt::Subscriber::builder()
                    .with_env_filter(filter.clone())
                    .with_writer(file)
                    .init();
            }
            LogFormat::Pretty => {
                fmt::Subscriber::builder()
                    .with_env_filter(filter.clone())
                    .pretty()
                    .with_writer(file)
                    .init();
            }
            LogFormat::Compact => {
                fmt::Subscriber::builder()
                    .with_env_filter(filter.clone())
                    .compact()
                    .with_writer(file)
                    .init();
            }
        }
        Ok(())
    }

    fn init_both(&self, path: &str, filter: &EnvFilter) -> Result<(), Box<dyn std::error::Error>> {
        let file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?;
        
        match self.config.format {
            LogFormat::Json => {
                fmt::Subscriber::builder()
                    .with_env_filter(filter.clone())
                    .with_writer(file)
                    .init();
            }
            LogFormat::Pretty => {
                fmt::Subscriber::builder()
                    .with_env_filter(filter.clone())
                    .pretty()
                    .with_writer(file)
                    .init();
            }
            LogFormat::Compact => {
                fmt::Subscriber::builder()
                    .with_env_filter(filter.clone())
                    .compact()
                    .with_writer(file)
                    .init();
            }
        }
        Ok(())
    }

    /// Log a structured message
    pub fn log(&self, level: LogLevel, message: &str, fields: HashMap<String, serde_json::Value>) {
        let log_entry = LogEntry {
            timestamp: Utc::now(),
            level,
            message: message.to_string(),
            module: module_path!().to_string(),
            target: "task-queue".to_string(),
            fields,
            span_id: None, // Would be populated by tracing in real implementation
            trace_id: None,
        };

        match level {
            LogLevel::Trace => trace!("{}", serde_json::to_string(&log_entry).unwrap_or_default()),
            LogLevel::Debug => debug!("{}", serde_json::to_string(&log_entry).unwrap_or_default()),
            LogLevel::Info => info!("{}", serde_json::to_string(&log_entry).unwrap_or_default()),
            LogLevel::Warn => warn!("{}", serde_json::to_string(&log_entry).unwrap_or_default()),
            LogLevel::Error => error!("{}", serde_json::to_string(&log_entry).unwrap_or_default()),
        }
    }

    /// Log with automatic field extraction
    pub fn log_with_fields<F>(&self, level: LogLevel, message: &str, field_extractor: F)
    where
        F: FnOnce() -> HashMap<String, serde_json::Value>,
    {
        let fields = field_extractor();
        self.log(level, message, fields);
    }
}

/// Convenience macros for structured logging
#[macro_export]
macro_rules! log_info {
    ($msg:expr) => {
        tracing::info!($msg);
    };
    ($msg:expr, $($field:ident = $value:expr),*) => {
        tracing::info!($msg, $($field = $value),*);
    };
}

#[macro_export]
macro_rules! log_warn {
    ($msg:expr) => {
        tracing::warn!($msg);
    };
    ($msg:expr, $($field:ident = $value:expr),*) => {
        tracing::warn!($msg, $($field = $value),*);
    };
}

#[macro_export]
macro_rules! log_error {
    ($msg:expr) => {
        tracing::error!($msg);
    };
    ($msg:expr, $($field:ident = $value:expr),*) => {
        tracing::error!($msg, $($field = $value),*);
    };
}

#[macro_export]
macro_rules! log_debug {
    ($msg:expr) => {
        tracing::debug!($msg);
    };
    ($msg:expr, $($field:ident = $value:expr),*) => {
        tracing::debug!($msg, $($field = $value),*);
    };
}

#[macro_export]
macro_rules! log_trace {
    ($msg:expr) => {
        tracing::trace!($msg);
    };
    ($msg:expr, $($field:ident = $value:expr),*) => {
        tracing::trace!($msg, $($field = $value),*);
    };
}

/// Performance logging utilities
pub struct PerformanceLogger {
    start_time: std::time::Instant,
    operation: String,
}

impl PerformanceLogger {
    pub fn start(operation: &str) -> Self {
        let start_time = std::time::Instant::now();
        info!("Starting operation: {}", operation);
        Self {
            start_time,
            operation: operation.to_string(),
        }
    }

    pub fn finish(self) {
        let duration = self.start_time.elapsed();
        info!(
            "Completed operation: {} in {:?}",
            self.operation, duration
        );
    }

    pub fn finish_with_result<T>(self, result: &Result<T, Box<dyn std::error::Error>>) {
        let duration = self.start_time.elapsed();
        match result {
            Ok(_) => {
                info!(
                    "Completed operation: {} successfully in {:?}",
                    self.operation, duration
                );
            }
            Err(e) => {
                error!(
                    "Failed operation: {} after {:?} with error: {}",
                    self.operation, duration, e
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_level_conversion() {
        assert_eq!(Level::from(LogLevel::Trace), Level::TRACE);
        assert_eq!(Level::from(LogLevel::Debug), Level::DEBUG);
        assert_eq!(Level::from(LogLevel::Info), Level::INFO);
        assert_eq!(Level::from(LogLevel::Warn), Level::WARN);
        assert_eq!(Level::from(LogLevel::Error), Level::ERROR);
    }

    #[test]
    fn test_log_level_display() {
        assert_eq!(LogLevel::Trace.to_string(), "TRACE");
        assert_eq!(LogLevel::Debug.to_string(), "DEBUG");
        assert_eq!(LogLevel::Info.to_string(), "INFO");
        assert_eq!(LogLevel::Warn.to_string(), "WARN");
        assert_eq!(LogLevel::Error.to_string(), "ERROR");
    }

    #[test]
    fn test_logging_config_default() {
        let config = LoggingConfig::default();
        assert_eq!(config.level, LogLevel::Info);
        assert_eq!(config.format, LogFormat::Pretty);
        assert!(matches!(config.output, LogOutput::Stdout));
    }

    #[test]
    fn test_performance_logger() {
        let logger = PerformanceLogger::start("test_operation");
        std::thread::sleep(std::time::Duration::from_millis(10));
        logger.finish();
    }
}

