//! WebSocket Support Module
//!
//! Provides comprehensive WebSocket support for real-time communication
//! in the Task Queue system, including connection management, message
//! handling, and event broadcasting.
//!

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};
use tokio::sync::{RwLock as AsyncRwLock, broadcast};
// use tokio_tungstenite::{WebSocketStream, MaybeTlsStream};
// use tokio::net::TcpStream;
// use futures_util::{SinkExt, StreamExt};
use tracing::{info, warn, debug};

/// WebSocket message types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WebSocketMessage {
    /// Task status update
    TaskUpdate {
        task_id: String,
        status: String,
        progress: Option<f32>,
        message: Option<String>,
    },
    /// Task queue status
    QueueStatus {
        total_tasks: u32,
        pending_tasks: u32,
        running_tasks: u32,
        completed_tasks: u32,
        failed_tasks: u32,
    },
    /// System notification
    Notification {
        level: String,
        title: String,
        message: String,
        timestamp: String,
    },
    /// Heartbeat/ping message
    Ping,
    /// Heartbeat/pong response
    Pong,
    /// Error message
    Error {
        code: String,
        message: String,
    },
    /// Custom message
    Custom {
        event_type: String,
        data: serde_json::Value,
    },
}

/// WebSocket connection state
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConnectionState {
    Connecting,
    Connected,
    Disconnecting,
    Disconnected,
    Error,
}

/// WebSocket client information
#[derive(Debug, Clone)]
pub struct WebSocketClient {
    pub id: String,
    pub user_id: Option<String>,
    pub session_id: Option<String>,
    pub connected_at: Instant,
    pub last_ping: Instant,
    pub state: ConnectionState,
    pub subscriptions: Vec<String>,
}

/// WebSocket configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSocketConfig {
    pub max_connections: u32,
    pub ping_interval: Duration,
    pub pong_timeout: Duration,
    pub max_message_size: usize,
    pub enable_compression: bool,
    pub enable_heartbeat: bool,
    pub cleanup_interval: Duration,
}

impl Default for WebSocketConfig {
    fn default() -> Self {
        Self {
            max_connections: 1000,
            ping_interval: Duration::from_secs(30),
            pong_timeout: Duration::from_secs(10),
            max_message_size: 1024 * 1024, // 1MB
            enable_compression: true,
            enable_heartbeat: true,
            cleanup_interval: Duration::from_secs(60),
        }
    }
}

/// WebSocket metrics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WebSocketMetrics {
    pub total_connections: u64,
    pub active_connections: u64,
    pub messages_sent: u64,
    pub messages_received: u64,
    pub connection_errors: u64,
    pub ping_timeouts: u64,
    pub average_connection_duration: Duration,
}

impl WebSocketMetrics {
    pub fn connection_success_rate(&self) -> f64 {
        if self.total_connections == 0 {
            0.0
        } else {
            (self.total_connections - self.connection_errors) as f64 / self.total_connections as f64
        }
    }
}

/// WebSocket connection manager
pub struct WebSocketManager {
    clients: Arc<AsyncRwLock<HashMap<String, WebSocketClient>>>,
    config: WebSocketConfig,
    metrics: Arc<RwLock<WebSocketMetrics>>,
    message_sender: broadcast::Sender<WebSocketMessage>,
    message_receiver: broadcast::Receiver<WebSocketMessage>,
}

impl WebSocketManager {
    /// Create a new WebSocket manager
    pub fn new(config: WebSocketConfig) -> Self {
        let (sender, receiver) = broadcast::channel(1000);
        
        Self {
            clients: Arc::new(AsyncRwLock::new(HashMap::new())),
            config,
            metrics: Arc::new(RwLock::new(WebSocketMetrics::default())),
            message_sender: sender,
            message_receiver: receiver,
        }
    }

    /// Add a new WebSocket connection
    pub async fn add_connection(&self, client_id: String, user_id: Option<String>, session_id: Option<String>) -> Result<(), String> {
        let mut clients = self.clients.write().await;
        let mut metrics = self.metrics.write().unwrap();

        // Check connection limit
        if clients.len() >= self.config.max_connections as usize {
            return Err("Maximum connections reached".to_string());
        }

        let client = WebSocketClient {
            id: client_id.clone(),
            user_id,
            session_id,
            connected_at: Instant::now(),
            last_ping: Instant::now(),
            state: ConnectionState::Connected,
            subscriptions: Vec::new(),
        };

        clients.insert(client_id.clone(), client);
        metrics.total_connections += 1;
        metrics.active_connections += 1;

        info!("WebSocket client {} connected", client_id);
        Ok(())
    }

    /// Remove a WebSocket connection
    pub async fn remove_connection(&self, client_id: &str) {
        let mut clients = self.clients.write().await;
        let mut metrics = self.metrics.write().unwrap();

        if let Some(client) = clients.remove(client_id) {
            metrics.active_connections = metrics.active_connections.saturating_sub(1);
            
            let duration = client.connected_at.elapsed();
            let avg_millis = (metrics.average_connection_duration.as_millis() + duration.as_millis()) / 2;
            metrics.average_connection_duration = Duration::from_millis(avg_millis as u64);

            info!("WebSocket client {} disconnected after {:?}", client_id, duration);
        }
    }

    /// Send a message to a specific client
    pub async fn send_to_client(&self, client_id: &str, message: WebSocketMessage) -> Result<(), String> {
        let clients = self.clients.read().await;
        
        if let Some(client) = clients.get(client_id) {
            if client.state == ConnectionState::Connected {
                // In a real implementation, this would send through the WebSocket connection
                debug!("Sending message to client {}: {:?}", client_id, message);
                return Ok(());
            }
        }
        
        Err("Client not found or not connected".to_string())
    }

    /// Broadcast a message to all connected clients
    pub async fn broadcast(&self, message: WebSocketMessage) -> Result<usize, String> {
        let clients = self.clients.read().await;
        let mut metrics = self.metrics.write().unwrap();
        
        let connected_clients: Vec<String> = clients
            .values()
            .filter(|client| client.state == ConnectionState::Connected)
            .map(|client| client.id.clone())
            .collect();

        let count = connected_clients.len();
        
        // In a real implementation, this would send to all connected clients
        debug!("Broadcasting message to {} clients: {:?}", count, message);
        
        metrics.messages_sent += count as u64;
        Ok(count)
    }

    /// Broadcast a message to clients with specific subscriptions
    pub async fn broadcast_to_subscribers(&self, event_type: &str, _message: WebSocketMessage) -> Result<usize, String> {
        let clients = self.clients.read().await;
        let mut metrics = self.metrics.write().unwrap();
        
        let subscribed_clients: Vec<String> = clients
            .values()
            .filter(|client| {
                client.state == ConnectionState::Connected && 
                client.subscriptions.contains(&event_type.to_string())
            })
            .map(|client| client.id.clone())
            .collect();

        let count = subscribed_clients.len();
        
        debug!("Broadcasting {} message to {} subscribed clients", event_type, count);
        
        metrics.messages_sent += count as u64;
        Ok(count)
    }

    /// Subscribe a client to specific event types
    pub async fn subscribe(&self, client_id: &str, event_types: Vec<String>) -> Result<(), String> {
        let mut clients = self.clients.write().await;
        
        if let Some(client) = clients.get_mut(client_id) {
            for event_type in event_types {
                if !client.subscriptions.contains(&event_type) {
                    client.subscriptions.push(event_type);
                }
            }
            debug!("Client {} subscribed to events", client_id);
            Ok(())
        } else {
            Err("Client not found".to_string())
        }
    }

    /// Unsubscribe a client from specific event types
    pub async fn unsubscribe(&self, client_id: &str, event_types: Vec<String>) -> Result<(), String> {
        let mut clients = self.clients.write().await;
        
        if let Some(client) = clients.get_mut(client_id) {
            for event_type in event_types {
                client.subscriptions.retain(|sub| sub != &event_type);
            }
            debug!("Client {} unsubscribed from events", client_id);
            Ok(())
        } else {
            Err("Client not found".to_string())
        }
    }

    /// Send a ping to a client
    pub async fn ping_client(&self, client_id: &str) -> Result<(), String> {
        let mut clients = self.clients.write().await;
        
        if let Some(client) = clients.get_mut(client_id) {
            client.last_ping = Instant::now();
            self.send_to_client(client_id, WebSocketMessage::Ping).await?;
            debug!("Ping sent to client {}", client_id);
            Ok(())
        } else {
            Err("Client not found".to_string())
        }
    }

    /// Handle a pong from a client
    pub async fn handle_pong(&self, client_id: &str) -> Result<(), String> {
        let mut clients = self.clients.write().await;
        
        if let Some(client) = clients.get_mut(client_id) {
            client.last_ping = Instant::now();
            debug!("Pong received from client {}", client_id);
            Ok(())
        } else {
            Err("Client not found".to_string())
        }
    }

    /// Check for timed out connections
    pub async fn check_timeouts(&self) -> Vec<String> {
        let mut clients = self.clients.write().await;
        let mut metrics = self.metrics.write().unwrap();
        let mut timed_out_clients = Vec::new();
        let now = Instant::now();

        for (client_id, client) in clients.iter_mut() {
            if now.duration_since(client.last_ping) > self.config.pong_timeout {
                client.state = ConnectionState::Error;
                timed_out_clients.push(client_id.clone());
                metrics.ping_timeouts += 1;
            }
        }

        if !timed_out_clients.is_empty() {
            warn!("Found {} timed out WebSocket connections", timed_out_clients.len());
        }

        timed_out_clients
    }

    /// Get WebSocket metrics
    pub fn metrics(&self) -> WebSocketMetrics {
        self.metrics.read().unwrap().clone()
    }

    /// Get current number of active connections
    pub async fn active_connections(&self) -> usize {
        self.clients.read().await.len()
    }

    /// Get client information
    pub async fn get_client(&self, client_id: &str) -> Option<WebSocketClient> {
        self.clients.read().await.get(client_id).cloned()
    }

    /// Start heartbeat task
    pub async fn start_heartbeat_task(&self) {
        let manager = self.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(manager.config.ping_interval);
            loop {
                interval.tick().await;
                
                // Ping all connected clients
                let clients: Vec<String> = {
                    let clients = manager.clients.read().await;
                    clients
                        .values()
                        .filter(|client| client.state == ConnectionState::Connected)
                        .map(|client| client.id.clone())
                        .collect()
                };

                for client_id in clients {
                    if let Err(e) = manager.ping_client(&client_id).await {
                        warn!("Failed to ping client {}: {}", client_id, e);
                    }
                }

                // Check for timeouts
                let timed_out = manager.check_timeouts().await;
                for client_id in timed_out {
                    manager.remove_connection(&client_id).await;
                }
            }
        });
    }

    /// Start cleanup task
    pub async fn start_cleanup_task(&self) {
        let manager = self.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(manager.config.cleanup_interval);
            loop {
                interval.tick().await;
                
                // Remove disconnected clients
                let disconnected_clients: Vec<String> = {
                    let clients = manager.clients.read().await;
                    clients
                        .iter()
                        .filter(|(_, client)| client.state == ConnectionState::Disconnected)
                        .map(|(id, _)| id.clone())
                        .collect()
                };

                for client_id in disconnected_clients {
                    manager.remove_connection(&client_id).await;
                }
            }
        });
    }
}

impl Clone for WebSocketManager {
    fn clone(&self) -> Self {
        Self {
            clients: Arc::clone(&self.clients),
            config: self.config.clone(),
            metrics: Arc::clone(&self.metrics),
            message_sender: self.message_sender.clone(),
            message_receiver: self.message_receiver.resubscribe(),
        }
    }
}

/// WebSocket handler for individual connections
pub struct WebSocketHandler {
    client_id: String,
    manager: Arc<WebSocketManager>,
}

impl WebSocketHandler {
    /// Create a new WebSocket handler
    pub fn new(client_id: String, manager: Arc<WebSocketManager>) -> Self {
        Self {
            client_id,
            manager,
        }
    }

    /// Handle incoming WebSocket messages
    pub async fn handle_message(&self, message: WebSocketMessage) -> Result<(), String> {
        match message {
            WebSocketMessage::Pong => {
                self.manager.handle_pong(&self.client_id).await?;
            }
            WebSocketMessage::Custom { event_type, data } => {
                debug!("Received custom message from {}: {} - {:?}", self.client_id, event_type, data);
                // Handle custom message logic here
            }
            _ => {
                warn!("Received unexpected message type from {}", self.client_id);
            }
        }
        Ok(())
    }

    /// Send a message to this client
    pub async fn send_message(&self, message: WebSocketMessage) -> Result<(), String> {
        self.manager.send_to_client(&self.client_id, message).await
    }
}

/// WebSocket event types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WebSocketEventType {
    TaskCreated,
    TaskUpdated,
    TaskCompleted,
    TaskFailed,
    QueueStatusChanged,
    SystemNotification,
    UserNotification,
}

impl std::fmt::Display for WebSocketEventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WebSocketEventType::TaskCreated => write!(f, "task.created"),
            WebSocketEventType::TaskUpdated => write!(f, "task.updated"),
            WebSocketEventType::TaskCompleted => write!(f, "task.completed"),
            WebSocketEventType::TaskFailed => write!(f, "task.failed"),
            WebSocketEventType::QueueStatusChanged => write!(f, "queue.status_changed"),
            WebSocketEventType::SystemNotification => write!(f, "system.notification"),
            WebSocketEventType::UserNotification => write!(f, "user.notification"),
        }
    }
}

/// WebSocket factory for creating different types of managers
pub struct WebSocketFactory;

impl WebSocketFactory {
    /// Create a standard WebSocket manager
    pub fn create_standard_manager() -> WebSocketManager {
        let config = WebSocketConfig::default();
        WebSocketManager::new(config)
    }

    /// Create a high-performance WebSocket manager
    pub fn create_high_performance_manager() -> WebSocketManager {
        let config = WebSocketConfig {
            max_connections: 10000,
            ping_interval: Duration::from_secs(15),
            pong_timeout: Duration::from_secs(5),
            max_message_size: 4 * 1024 * 1024, // 4MB
            enable_compression: true,
            enable_heartbeat: true,
            cleanup_interval: Duration::from_secs(30),
        };
        WebSocketManager::new(config)
    }

    /// Create a lightweight WebSocket manager
    pub fn create_lightweight_manager() -> WebSocketManager {
        let config = WebSocketConfig {
            max_connections: 100,
            ping_interval: Duration::from_secs(60),
            pong_timeout: Duration::from_secs(30),
            max_message_size: 64 * 1024, // 64KB
            enable_compression: false,
            enable_heartbeat: true,
            cleanup_interval: Duration::from_secs(120),
        };
        WebSocketManager::new(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_websocket_manager_creation() {
        let config = WebSocketConfig::default();
        let manager = WebSocketManager::new(config);
        
        assert_eq!(manager.active_connections().await, 0);
    }

    #[tokio::test]
    async fn test_add_remove_connection() {
        let manager = WebSocketManager::new(WebSocketConfig::default());
        
        // Add connection
        assert!(manager.add_connection("client1".to_string(), None, None).await.is_ok());
        assert_eq!(manager.active_connections().await, 1);
        
        // Remove connection
        manager.remove_connection("client1").await;
        assert_eq!(manager.active_connections().await, 0);
    }

    #[tokio::test]
    async fn test_subscribe_unsubscribe() {
        let manager = WebSocketManager::new(WebSocketConfig::default());
        
        // Add connection
        manager.add_connection("client1".to_string(), None, None).await.unwrap();
        
        // Subscribe to events
        let events = vec!["task.created".to_string(), "task.updated".to_string()];
        assert!(manager.subscribe("client1", events.clone()).await.is_ok());
        
        // Check subscription
        let client = manager.get_client("client1").await.unwrap();
        assert_eq!(client.subscriptions.len(), 2);
        
        // Unsubscribe
        assert!(manager.unsubscribe("client1", events).await.is_ok());
        
        let client = manager.get_client("client1").await.unwrap();
        assert_eq!(client.subscriptions.len(), 0);
    }

    #[tokio::test]
    async fn test_broadcast_message() {
        let manager = WebSocketManager::new(WebSocketConfig::default());
        
        // Add multiple connections
        manager.add_connection("client1".to_string(), None, None).await.unwrap();
        manager.add_connection("client2".to_string(), None, None).await.unwrap();
        
        // Broadcast message
        let message = WebSocketMessage::Notification {
            level: "info".to_string(),
            title: "Test".to_string(),
            message: "Test message".to_string(),
            timestamp: "2023-01-01T00:00:00Z".to_string(),
        };
        
        let count = manager.broadcast(message).await.unwrap();
        assert_eq!(count, 2);
    }

    #[tokio::test]
    async fn test_websocket_metrics() {
        let manager = WebSocketManager::new(WebSocketConfig::default());
        
        // Add some connections
        manager.add_connection("client1".to_string(), None, None).await.unwrap();
        manager.add_connection("client2".to_string(), None, None).await.unwrap();
        
        let metrics = manager.metrics();
        assert_eq!(metrics.total_connections, 2);
        assert_eq!(metrics.active_connections, 2);
        assert_eq!(metrics.connection_success_rate(), 1.0);
    }
}
