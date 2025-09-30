//! Rate Limiting and Throttling Module
//!
//! Provides comprehensive rate limiting and throttling capabilities to protect
//! the Task Queue API from abuse and ensure fair resource usage.
//!

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock as AsyncRwLock;
use tracing::{warn, debug, error};

/// Rate limiting algorithms
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RateLimitAlgorithm {
    /// Token bucket algorithm
    TokenBucket,
    /// Sliding window algorithm
    SlidingWindow,
    /// Fixed window algorithm
    FixedWindow,
    /// Leaky bucket algorithm
    LeakyBucket,
}

/// Rate limit configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    pub algorithm: RateLimitAlgorithm,
    pub requests_per_minute: u32,
    pub burst_size: Option<u32>,
    pub window_size: Duration,
    pub cleanup_interval: Duration,
    pub enable_metrics: bool,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            algorithm: RateLimitAlgorithm::TokenBucket,
            requests_per_minute: 60,
            burst_size: Some(10),
            window_size: Duration::from_secs(60),
            cleanup_interval: Duration::from_secs(300), // 5 minutes
            enable_metrics: true,
        }
    }
}

/// Rate limit entry for tracking client requests
#[derive(Debug, Clone)]
pub struct RateLimitEntry {
    pub tokens: u32,
    pub last_refill: Instant,
    pub request_count: u32,
    pub window_start: Instant,
    pub blocked_until: Option<Instant>,
}

impl RateLimitEntry {
    pub fn new(max_tokens: u32) -> Self {
        let now = Instant::now();
        Self {
            tokens: max_tokens,
            last_refill: now,
            request_count: 0,
            window_start: now,
            blocked_until: None,
        }
    }
}

/// Rate limit metrics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RateLimitMetrics {
    pub total_requests: u64,
    pub allowed_requests: u64,
    pub blocked_requests: u64,
    pub current_clients: usize,
    pub average_response_time: Duration,
}

impl RateLimitMetrics {
    pub fn block_rate(&self) -> f64 {
        if self.total_requests == 0 {
            0.0
        } else {
            self.blocked_requests as f64 / self.total_requests as f64
        }
    }

    pub fn allow_rate(&self) -> f64 {
        if self.total_requests == 0 {
            0.0
        } else {
            self.allowed_requests as f64 / self.total_requests as f64
        }
    }
}

/// Main rate limiter implementation
pub struct RateLimiter {
    entries: Arc<AsyncRwLock<HashMap<String, RateLimitEntry>>>,
    config: RateLimitConfig,
    metrics: Arc<RwLock<RateLimitMetrics>>,
}

impl RateLimiter {
    /// Create a new rate limiter with the given configuration
    pub fn new(config: RateLimitConfig) -> Self {
        Self {
            entries: Arc::new(AsyncRwLock::new(HashMap::new())),
            config,
            metrics: Arc::new(RwLock::new(RateLimitMetrics::default())),
        }
    }

    /// Check if a request should be allowed
    pub async fn is_allowed(&self, client_id: &str) -> bool {
        let start_time = Instant::now();
        let mut entries = self.entries.write().await;
        let mut metrics = self.metrics.write().unwrap();

        metrics.total_requests += 1;

        let entry = entries.entry(client_id.to_string()).or_insert_with(|| {
            RateLimitEntry::new(self.config.burst_size.unwrap_or(self.config.requests_per_minute))
        });

        let allowed = match self.config.algorithm {
            RateLimitAlgorithm::TokenBucket => self.check_token_bucket(entry),
            RateLimitAlgorithm::SlidingWindow => self.check_sliding_window(entry),
            RateLimitAlgorithm::FixedWindow => self.check_fixed_window(entry),
            RateLimitAlgorithm::LeakyBucket => self.check_leaky_bucket(entry),
        };

        if allowed {
            metrics.allowed_requests += 1;
            debug!("Request allowed for client: {}", client_id);
        } else {
            metrics.blocked_requests += 1;
            warn!("Request blocked for client: {}", client_id);
        }

        // Update average response time
        let response_time = start_time.elapsed();
        let avg_millis = (metrics.average_response_time.as_millis() + response_time.as_millis()) / 2;
        metrics.average_response_time = Duration::from_millis(avg_millis as u64);

        metrics.current_clients = entries.len();
        allowed
    }

    /// Check rate limit using token bucket algorithm
    fn check_token_bucket(&self, entry: &mut RateLimitEntry) -> bool {
        let now = Instant::now();
        let time_passed = now.duration_since(entry.last_refill);
        
        // Refill tokens based on time passed
        let tokens_to_add = (time_passed.as_secs() * self.config.requests_per_minute as u64) / 60;
        entry.tokens = (entry.tokens + tokens_to_add as u32).min(
            self.config.burst_size.unwrap_or(self.config.requests_per_minute)
        );
        entry.last_refill = now;

        if entry.tokens > 0 {
            entry.tokens -= 1;
            true
        } else {
            false
        }
    }

    /// Check rate limit using sliding window algorithm
    fn check_sliding_window(&self, entry: &mut RateLimitEntry) -> bool {
        let now = Instant::now();
        
        // Reset window if it has expired
        if now.duration_since(entry.window_start) >= self.config.window_size {
            entry.window_start = now;
            entry.request_count = 0;
        }

        if entry.request_count < self.config.requests_per_minute {
            entry.request_count += 1;
            true
        } else {
            false
        }
    }

    /// Check rate limit using fixed window algorithm
    fn check_fixed_window(&self, entry: &mut RateLimitEntry) -> bool {
        let now = Instant::now();
        let window_duration = self.config.window_size;
        
        // Check if we're in a new window
        if now.duration_since(entry.window_start) >= window_duration {
            entry.window_start = now;
            entry.request_count = 0;
        }

        if entry.request_count < self.config.requests_per_minute {
            entry.request_count += 1;
            true
        } else {
            false
        }
    }

    /// Check rate limit using leaky bucket algorithm
    fn check_leaky_bucket(&self, entry: &mut RateLimitEntry) -> bool {
        let now = Instant::now();
        let time_passed = now.duration_since(entry.last_refill);
        
        // Leak tokens based on time passed
        let tokens_to_leak = (time_passed.as_secs() * self.config.requests_per_minute as u64) / 60;
        entry.tokens = entry.tokens.saturating_sub(tokens_to_leak as u32);
        entry.last_refill = now;

        let max_tokens = self.config.burst_size.unwrap_or(self.config.requests_per_minute);
        
        if entry.tokens < max_tokens {
            entry.tokens += 1;
            true
        } else {
            false
        }
    }

    /// Block a client for a specific duration
    pub async fn block_client(&self, client_id: &str, duration: Duration) {
        let mut entries = self.entries.write().await;
        if let Some(entry) = entries.get_mut(client_id) {
            entry.blocked_until = Some(Instant::now() + duration);
            warn!("Blocked client {} for {:?}", client_id, duration);
        }
    }

    /// Check if a client is currently blocked
    pub async fn is_blocked(&self, client_id: &str) -> bool {
        let entries = self.entries.read().await;
        if let Some(entry) = entries.get(client_id) {
            if let Some(blocked_until) = entry.blocked_until {
                if Instant::now() < blocked_until {
                    return true;
                }
            }
        }
        false
    }

    /// Get rate limit metrics
    pub fn metrics(&self) -> RateLimitMetrics {
        self.metrics.read().unwrap().clone()
    }

    /// Get current number of tracked clients
    pub async fn client_count(&self) -> usize {
        self.entries.read().await.len()
    }

    /// Clean up expired entries
    pub async fn cleanup_expired(&self) -> usize {
        let mut entries = self.entries.write().await;
        let now = Instant::now();
        let mut removed_count = 0;

        entries.retain(|_, entry| {
            // Remove entries that haven't been accessed for a long time
            let should_keep = now.duration_since(entry.last_refill) < Duration::from_secs(3600); // 1 hour
            if !should_keep {
                removed_count += 1;
            }
            should_keep
        });

        if removed_count > 0 {
            debug!("Cleaned up {} expired rate limit entries", removed_count);
        }

        removed_count
    }

    /// Start background cleanup task
    pub async fn start_cleanup_task(&self) {
        let limiter = self.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(limiter.config.cleanup_interval);
            loop {
                interval.tick().await;
                limiter.cleanup_expired().await;
            }
        });
    }
}

impl Clone for RateLimiter {
    fn clone(&self) -> Self {
        Self {
            entries: Arc::clone(&self.entries),
            config: self.config.clone(),
            metrics: Arc::clone(&self.metrics),
        }
    }
}

/// Throttling configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThrottleConfig {
    pub max_concurrent_requests: u32,
    pub queue_size: u32,
    pub timeout: Duration,
    pub enable_priority: bool,
}

impl Default for ThrottleConfig {
    fn default() -> Self {
        Self {
            max_concurrent_requests: 100,
            queue_size: 1000,
            timeout: Duration::from_secs(30),
            enable_priority: false,
        }
    }
}

/// Request priority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RequestPriority {
    Low = 1,
    Normal = 2,
    High = 3,
    Critical = 4,
}

/// Throttled request
#[derive(Debug, Clone)]
pub struct ThrottledRequest {
    pub id: String,
    pub client_id: String,
    pub priority: RequestPriority,
    pub created_at: Instant,
    pub timeout: Duration,
}

/// Throttler implementation
pub struct Throttler {
    config: ThrottleConfig,
    active_requests: Arc<AsyncRwLock<HashMap<String, ThrottledRequest>>>,
    request_queue: Arc<AsyncRwLock<Vec<ThrottledRequest>>>,
}

impl Throttler {
    /// Create a new throttler with the given configuration
    pub fn new(config: ThrottleConfig) -> Self {
        Self {
            config,
            active_requests: Arc::new(AsyncRwLock::new(HashMap::new())),
            request_queue: Arc::new(AsyncRwLock::new(Vec::new())),
        }
    }

    /// Submit a request for throttling
    pub async fn submit_request(
        &self,
        client_id: &str,
        priority: RequestPriority,
    ) -> Result<String, String> {
        let request_id = format!("{}-{}", client_id, SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis());
        
        let request = ThrottledRequest {
            id: request_id.clone(),
            client_id: client_id.to_string(),
            priority,
            created_at: Instant::now(),
            timeout: self.config.timeout,
        };

        let mut active_requests = self.active_requests.write().await;
        let mut request_queue = self.request_queue.write().await;

        // Check if we can process immediately
        if active_requests.len() < self.config.max_concurrent_requests as usize {
            active_requests.insert(request_id.clone(), request);
            debug!("Request {} processed immediately", request_id);
            return Ok(request_id);
        }

        // Check if queue has space
        if request_queue.len() < self.config.queue_size as usize {
            request_queue.push(request);
            debug!("Request {} queued", request_id);
            return Ok(request_id);
        }

        error!("Request queue is full, rejecting request {}", request_id);
        Err("Request queue is full".to_string())
    }

    /// Complete a request
    pub async fn complete_request(&self, request_id: &str) -> bool {
        let mut active_requests = self.active_requests.write().await;
        
        if active_requests.remove(request_id).is_some() {
            debug!("Request {} completed", request_id);
            
            // Process next request from queue
            self.process_next_request().await;
            return true;
        }
        
        false
    }

    /// Process the next request from the queue
    async fn process_next_request(&self) {
        let mut request_queue = self.request_queue.write().await;
        let mut active_requests = self.active_requests.write().await;

        if active_requests.len() < self.config.max_concurrent_requests as usize {
            if let Some(request) = request_queue.pop() {
                let request_id = request.id.clone();
                active_requests.insert(request_id.clone(), request);
                debug!("Processed queued request {}", request_id);
            }
        }
    }

    /// Get current queue status
    pub async fn get_queue_status(&self) -> (usize, usize) {
        let active_requests = self.active_requests.read().await;
        let request_queue = self.request_queue.read().await;
        (active_requests.len(), request_queue.len())
    }

    /// Clean up expired requests
    pub async fn cleanup_expired(&self) -> usize {
        let mut active_requests = self.active_requests.write().await;
        let now = Instant::now();
        let mut expired_count = 0;

        active_requests.retain(|_, request| {
            let is_expired = now.duration_since(request.created_at) > request.timeout;
            if is_expired {
                expired_count += 1;
            }
            !is_expired
        });

        if expired_count > 0 {
            warn!("Cleaned up {} expired requests", expired_count);
        }

        expired_count
    }
}

/// Rate limiter factory
pub struct RateLimiterFactory;

impl RateLimiterFactory {
    /// Create a standard API rate limiter
    pub fn create_api_limiter() -> RateLimiter {
        let config = RateLimitConfig {
            algorithm: RateLimitAlgorithm::TokenBucket,
            requests_per_minute: 100,
            burst_size: Some(20),
            window_size: Duration::from_secs(60),
            cleanup_interval: Duration::from_secs(300),
            enable_metrics: true,
        };
        
        RateLimiter::new(config)
    }

    /// Create a strict rate limiter for sensitive endpoints
    pub fn create_strict_limiter() -> RateLimiter {
        let config = RateLimitConfig {
            algorithm: RateLimitAlgorithm::SlidingWindow,
            requests_per_minute: 10,
            burst_size: Some(2),
            window_size: Duration::from_secs(60),
            cleanup_interval: Duration::from_secs(300),
            enable_metrics: true,
        };
        
        RateLimiter::new(config)
    }

    /// Create a high-throughput rate limiter
    pub fn create_high_throughput_limiter() -> RateLimiter {
        let config = RateLimitConfig {
            algorithm: RateLimitAlgorithm::LeakyBucket,
            requests_per_minute: 1000,
            burst_size: Some(100),
            window_size: Duration::from_secs(60),
            cleanup_interval: Duration::from_secs(300),
            enable_metrics: true,
        };
        
        RateLimiter::new(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn test_token_bucket_rate_limiting() {
        let config = RateLimitConfig {
            algorithm: RateLimitAlgorithm::TokenBucket,
            requests_per_minute: 60,
            burst_size: Some(10),
            window_size: Duration::from_secs(60),
            cleanup_interval: Duration::from_secs(300),
            enable_metrics: true,
        };
        
        let limiter = RateLimiter::new(config);
        
        // Should allow burst requests
        for _ in 0..10 {
            assert!(limiter.is_allowed("client1").await);
        }
        
        // Should block after burst
        assert!(!limiter.is_allowed("client1").await);
    }

    #[tokio::test]
    async fn test_sliding_window_rate_limiting() {
        let config = RateLimitConfig {
            algorithm: RateLimitAlgorithm::SlidingWindow,
            requests_per_minute: 5,
            burst_size: None,
            window_size: Duration::from_secs(60),
            cleanup_interval: Duration::from_secs(300),
            enable_metrics: true,
        };
        
        let limiter = RateLimiter::new(config);
        
        // Should allow requests up to limit
        for _ in 0..5 {
            assert!(limiter.is_allowed("client1").await);
        }
        
        // Should block after limit
        assert!(!limiter.is_allowed("client1").await);
    }

    #[tokio::test]
    async fn test_throttler_basic_operations() {
        let config = ThrottleConfig::default();
        let throttler = Throttler::new(config);
        
        // Submit requests
        let request_id1 = throttler.submit_request("client1", RequestPriority::Normal).await.unwrap();
        let request_id2 = throttler.submit_request("client2", RequestPriority::High).await.unwrap();
        
        // Complete requests
        assert!(throttler.complete_request(&request_id1).await);
        assert!(throttler.complete_request(&request_id2).await);
    }

    #[tokio::test]
    async fn test_rate_limiter_metrics() {
        let limiter = RateLimiterFactory::create_api_limiter();
        
        // Make some requests
        limiter.is_allowed("client1").await;
        limiter.is_allowed("client1").await;
        limiter.is_allowed("client2").await;
        
        let metrics = limiter.metrics();
        assert_eq!(metrics.total_requests, 3);
        assert_eq!(metrics.allowed_requests, 3);
        assert_eq!(metrics.blocked_requests, 0);
    }
}
