# WebSocket Support - Technical Documentation

## Overview
The WebSocket Support module provides comprehensive real-time communication capabilities for the Task Queue system. It enables bidirectional communication between clients and the server, supporting task updates, notifications, and live system monitoring.

## Architecture

### Core Components

#### 1. WebSocket Message Types
```rust
pub enum WebSocketMessage {
    TaskUpdate { task_id: String, status: String, progress: Option<f32>, message: Option<String> },
    QueueStatus { total_tasks: u32, pending_tasks: u32, running_tasks: u32, completed_tasks: u32, failed_tasks: u32 },
    Notification { level: String, title: String, message: String, timestamp: String },
    Ping,
    Pong,
    Error { code: String, message: String },
    Custom { event_type: String, data: serde_json::Value },
}
```

#### 2. Connection State Management
```rust
pub enum ConnectionState {
    Connecting,
    Connected,
    Disconnecting,
    Disconnected,
    Error,
}
```

#### 3. WebSocket Client Information
```rust
pub struct WebSocketClient {
    pub id: String,
    pub user_id: Option<String>,
    pub session_id: Option<String>,
    pub connected_at: Instant,
    pub last_ping: Instant,
    pub state: ConnectionState,
    pub subscriptions: Vec<String>,
}
```

### WebSocket Configuration

#### WebSocketConfig
```rust
pub struct WebSocketConfig {
    pub max_connections: u32,
    pub ping_interval: Duration,
    pub pong_timeout: Duration,
    pub max_message_size: usize,
    pub enable_compression: bool,
    pub enable_heartbeat: bool,
    pub cleanup_interval: Duration,
}
```

#### Configuration Options
- **max_connections**: Maximum number of concurrent connections
- **ping_interval**: Interval for sending ping messages
- **pong_timeout**: Timeout for pong responses
- **max_message_size**: Maximum size of WebSocket messages
- **enable_compression**: Enable WebSocket compression
- **enable_heartbeat**: Enable heartbeat mechanism
- **cleanup_interval**: Interval for cleaning up disconnected clients

## WebSocket Manager

### WebSocketManager
The main WebSocket connection manager that:
- Manages client connections and state
- Handles message broadcasting
- Provides subscription management
- Monitors connection health
- Tracks metrics and performance

#### Key Features
- **Connection Management**: Add, remove, and monitor connections
- **Message Broadcasting**: Send messages to all or specific clients
- **Subscription System**: Clients can subscribe to specific event types
- **Heartbeat Monitoring**: Ping/pong mechanism for connection health
- **Metrics Collection**: Comprehensive connection and message metrics

### Connection Operations

#### Add Connection
```rust
pub async fn add_connection(&self, client_id: String, user_id: Option<String>, session_id: Option<String>) -> Result<(), String>
```
- Adds a new WebSocket connection
- Validates connection limits
- Initializes client state
- Updates metrics

#### Remove Connection
```rust
pub async fn remove_connection(&self, client_id: &str)
```
- Removes a WebSocket connection
- Updates metrics
- Logs connection duration

#### Send to Client
```rust
pub async fn send_to_client(&self, client_id: &str, message: WebSocketMessage) -> Result<(), String>
```
- Sends a message to a specific client
- Validates client state
- Handles delivery errors

### Broadcasting Operations

#### Broadcast to All
```rust
pub async fn broadcast(&self, message: WebSocketMessage) -> Result<usize, String>
```
- Sends message to all connected clients
- Returns number of clients reached
- Updates message metrics

#### Broadcast to Subscribers
```rust
pub async fn broadcast_to_subscribers(&self, event_type: &str, message: WebSocketMessage) -> Result<usize, String>
```
- Sends message to clients subscribed to specific event type
- Efficient filtering based on subscriptions
- Returns number of subscribers reached

### Subscription Management

#### Subscribe to Events
```rust
pub async fn subscribe(&self, client_id: &str, event_types: Vec<String>) -> Result<(), String>
```
- Subscribes client to specific event types
- Prevents duplicate subscriptions
- Updates client state

#### Unsubscribe from Events
```rust
pub async fn unsubscribe(&self, client_id: &str, event_types: Vec<String>) -> Result<(), String>
```
- Removes client subscriptions
- Updates client state
- Logs unsubscription

### Health Monitoring

#### Ping/Pong Mechanism
```rust
pub async fn ping_client(&self, client_id: &str) -> Result<(), String>
pub async fn handle_pong(&self, client_id: &str) -> Result<(), String>
```
- Sends ping messages to clients
- Handles pong responses
- Updates last ping timestamps

#### Timeout Detection
```rust
pub async fn check_timeouts(&self) -> Vec<String>
```
- Identifies clients that haven't responded to pings
- Returns list of timed out clients
- Updates timeout metrics

## WebSocket Handler

### WebSocketHandler
Individual connection handler that:
- Processes incoming messages
- Manages client-specific state
- Handles message routing
- Provides connection-specific operations

#### Message Handling
```rust
pub async fn handle_message(&self, message: WebSocketMessage) -> Result<(), String>
```
- Processes different message types
- Handles pong responses
- Manages custom messages
- Logs message processing

## Event Types

### WebSocketEventType
```rust
pub enum WebSocketEventType {
    TaskCreated,
    TaskUpdated,
    TaskCompleted,
    TaskFailed,
    QueueStatusChanged,
    SystemNotification,
    UserNotification,
}
```

#### Event Type Mapping
- **TaskCreated**: New task created
- **TaskUpdated**: Task status or progress updated
- **TaskCompleted**: Task finished successfully
- **TaskFailed**: Task failed with error
- **QueueStatusChanged**: Overall queue status changed
- **SystemNotification**: System-wide notifications
- **UserNotification**: User-specific notifications

## Metrics and Monitoring

### WebSocketMetrics
```rust
pub struct WebSocketMetrics {
    pub total_connections: u64,
    pub active_connections: u64,
    pub messages_sent: u64,
    pub messages_received: u64,
    pub connection_errors: u64,
    pub ping_timeouts: u64,
    pub average_connection_duration: Duration,
}
```

#### Calculated Metrics
- **Connection Success Rate**: (total_connections - connection_errors) / total_connections
- **Message Throughput**: messages_sent per unit time
- **Average Connection Duration**: Mean connection lifetime

## Background Tasks

### Heartbeat Task
- Sends periodic ping messages to all connected clients
- Monitors connection health
- Identifies and removes timed out connections
- Runs at configurable intervals

### Cleanup Task
- Removes disconnected clients from memory
- Prevents memory leaks
- Updates connection metrics
- Runs at configurable intervals

## WebSocket Factory

### Pre-configured Managers
- **Standard Manager**: Balanced configuration for general use
- **High-Performance Manager**: Optimized for high-volume traffic
- **Lightweight Manager**: Minimal resource usage for simple deployments

### Configuration Examples

#### Standard Configuration
```rust
let config = WebSocketConfig {
    max_connections: 1000,
    ping_interval: Duration::from_secs(30),
    pong_timeout: Duration::from_secs(10),
    max_message_size: 1024 * 1024, // 1MB
    enable_compression: true,
    enable_heartbeat: true,
    cleanup_interval: Duration::from_secs(60),
};
```

#### High-Performance Configuration
```rust
let config = WebSocketConfig {
    max_connections: 10000,
    ping_interval: Duration::from_secs(15),
    pong_timeout: Duration::from_secs(5),
    max_message_size: 4 * 1024 * 1024, // 4MB
    enable_compression: true,
    enable_heartbeat: true,
    cleanup_interval: Duration::from_secs(30),
};
```

#### Lightweight Configuration
```rust
let config = WebSocketConfig {
    max_connections: 100,
    ping_interval: Duration::from_secs(60),
    pong_timeout: Duration::from_secs(30),
    max_message_size: 64 * 1024, // 64KB
    enable_compression: false,
    enable_heartbeat: true,
    cleanup_interval: Duration::from_secs(120),
};
```

## Integration Points

### Task Queue Integration
- Real-time task status updates
- Queue status broadcasting
- Task completion notifications
- Error event propagation

### Notification System Integration
- System-wide notifications
- User-specific notifications
- Alert broadcasting
- Event subscription management

### Authentication Integration
- User-based connection tracking
- Session management
- Access control for subscriptions
- Secure message delivery

## Error Handling

### Connection Errors
- Graceful handling of connection failures
- Automatic cleanup of failed connections
- Error logging and metrics
- Fallback mechanisms

### Message Errors
- Invalid message format handling
- Message size validation
- Delivery failure recovery
- Error message broadcasting

### Resource Management
- Connection limit enforcement
- Memory usage monitoring
- Automatic cleanup of expired connections
- Resource leak prevention

## Performance Considerations

### Memory Management
- Bounded connection tracking prevents memory leaks
- Efficient HashMap usage for O(1) operations
- Automatic cleanup of expired connections
- Minimal allocation overhead

### Async Operations
- Non-blocking WebSocket operations
- Efficient async/await usage
- Concurrent message processing
- Minimal blocking operations

### Scalability
- Configurable connection limits
- Efficient broadcasting algorithms
- Subscription-based filtering
- Background task optimization

## Security Considerations

### Connection Security
- Client authentication and authorization
- Session management
- Secure message delivery
- Access control for subscriptions

### Message Security
- Message validation and sanitization
- Size limits to prevent abuse
- Rate limiting for message frequency
- Secure message content handling

## Testing Strategy

### Unit Tests
- Connection management operations
- Message broadcasting functionality
- Subscription management
- Metrics calculation accuracy

### Integration Tests
- End-to-end WebSocket communication
- Real-time message delivery
- Connection health monitoring
- Performance under load

### Performance Tests
- Connection scalability
- Message throughput
- Memory usage patterns
- Latency measurements

## Monitoring and Observability

### Metrics Collection
- Connection counts for capacity planning
- Message rates for performance tuning
- Error rates for reliability monitoring
- Connection duration for usage analysis

### Health Checks
- WebSocket service availability
- Connection health monitoring
- Message delivery verification
- Performance metrics collection

## Future Enhancements

### Planned Features
- WebSocket compression support
- Advanced message routing
- Clustering and load balancing
- Real-time analytics dashboard

### Scalability Improvements
- Distributed WebSocket support
- Advanced connection pooling
- Message queuing and persistence
- Integration with message brokers

