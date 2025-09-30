# Structured Logging System - Technical Documentation

## Overview
The Structured Logging System provides comprehensive logging capabilities with structured data, different log levels, and integration with the tracing ecosystem for the Task Queue system.

## Architecture

### Core Components

#### 1. LogLevel Enum
```rust
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}
```
- Supports standard logging levels
- Implements Display trait for string representation
- Converts to tracing::Level for integration

#### 2. LogEntry Structure
```rust
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
```
- Structured log entry with metadata
- Supports custom fields for context
- Includes timing and tracing information

#### 3. LoggingConfig
```rust
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
```

### Output Formats
- **Json**: Machine-readable JSON format
- **Pretty**: Human-readable formatted output
- **Compact**: Minimal format for production

### Output Destinations
- **Stdout**: Standard output
- **Stderr**: Standard error
- **File**: Log to file
- **Both**: File + stdout combination

## Implementation Details

### StructuredLogger
The main logger implementation that:
- Initializes tracing subscriber with configured settings
- Supports multiple output formats and destinations
- Provides structured logging methods
- Handles file rotation and management

### Performance Logger
Utility for measuring operation performance:
```rust
pub struct PerformanceLogger {
    start_time: std::time::Instant,
    operation: String,
}
```

### Macros
Convenience macros for structured logging:
- `log_info!()` - Info level logging
- `log_warn!()` - Warning level logging
- `log_error!()` - Error level logging
- `log_debug!()` - Debug level logging
- `log_trace!()` - Trace level logging

## Integration Points

### Main Application
- Initialized in `main.rs` with configuration
- Supports both file and console output
- Configurable log levels via environment variables

### Tracing Integration
- Uses `tracing` crate for underlying logging
- Supports structured fields and spans
- Compatible with tracing ecosystem tools

## Configuration Examples

### Basic Configuration
```rust
let logging_config = LoggingConfig {
    level: LogLevel::Info,
    format: LogFormat::Pretty,
    output: LogOutput::Stdout,
    enable_span_events: true,
    enable_file_logging: false,
    log_file_path: Some("logs/task-queue.log".to_string()),
    max_file_size: Some(10 * 1024 * 1024), // 10MB
    max_files: Some(5),
};
```

### Production Configuration
```rust
let logging_config = LoggingConfig {
    level: LogLevel::Warn,
    format: LogFormat::Compact,
    output: LogOutput::Both("logs/task-queue.log".to_string()),
    enable_span_events: true,
    enable_file_logging: true,
    log_file_path: Some("logs/task-queue.log".to_string()),
    max_file_size: Some(50 * 1024 * 1024), // 50MB
    max_files: Some(10),
};
```

## Error Handling

### File Operations
- Graceful handling of file creation failures
- Automatic directory creation for log files
- Error propagation for initialization failures

### Serialization
- Safe JSON serialization with fallback
- Error handling for malformed log entries
- Graceful degradation on serialization failures

## Performance Considerations

### Async Operations
- Non-blocking log operations
- Efficient string formatting
- Minimal allocation overhead

### Memory Management
- Bounded log entry sizes
- Efficient HashMap usage for fields
- Automatic cleanup of expired entries

## Testing Strategy

### Unit Tests
- Log level conversion tests
- Configuration validation tests
- Performance logger functionality tests

### Integration Tests
- End-to-end logging pipeline tests
- File output verification tests
- Format validation tests

## Security Considerations

### Log Sanitization
- Automatic sanitization of sensitive data
- Configurable field filtering
- Secure file permissions

### Access Control
- Restricted log file access
- Audit trail for log access
- Secure log transmission

## Monitoring and Metrics

### Log Metrics
- Log volume tracking
- Error rate monitoring
- Performance impact measurement

### Health Checks
- Log file accessibility checks
- Disk space monitoring
- Log rotation status

## Future Enhancements

### Planned Features
- Log aggregation support (ELK stack)
- Real-time log streaming
- Advanced filtering capabilities
- Log correlation and tracing

### Scalability Improvements
- Distributed logging support
- Log compression
- Advanced retention policies
