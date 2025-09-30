//! Cache System Module
//!
//! Provides comprehensive caching capabilities with different eviction strategies,
//! TTL support, and performance optimization for the task queue system.
//!
//! ## 🚨 CRITICAL TESTING REQUIREMENTS
//!
//! **BEFORE ANY CHANGES TO THIS MODULE:**
//! 1. Execute tests: `cargo test --lib cache`
//! 2. Verify all tests pass: `cargo test --lib cache -- --nocapture`
//! 3. Check coverage: `cargo test --lib cache -- --nocapture --test-threads=1`
//!
//! **TESTING COMMANDS:**
//! ```bash
//! # Run Cache module tests
//! cargo test --lib cache
//!
//! # Run with verbose output
//! cargo test --lib cache -- --nocapture
//!
//! # Run specific test
//! cargo test --lib cache test_in_memory_cache_put_get
//!
//! # Run all Cache tests with coverage
//! cargo test --lib cache -- --nocapture --test-threads=1
//! ```
//!
//! **⚠️ NO COMMITS WITHOUT PASSING TESTS!**

use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock as AsyncRwLock;
use tracing::{info, debug};

/// Cache entry with metadata
#[derive(Debug, Clone)]
pub struct CacheEntry<V> {
    pub value: V,
    pub created_at: Instant,
    pub last_accessed: Instant,
    pub access_count: u64,
    pub ttl: Option<Duration>,
}

impl<V> CacheEntry<V> {
    pub fn new(value: V, ttl: Option<Duration>) -> Self {
        let now = Instant::now();
        Self {
            value,
            created_at: now,
            last_accessed: now,
            access_count: 1,
            ttl,
        }
    }

    pub fn is_expired(&self) -> bool {
        if let Some(ttl) = self.ttl {
            self.created_at.elapsed() > ttl
        } else {
            false
        }
    }

    pub fn touch(&mut self) {
        self.last_accessed = Instant::now();
        self.access_count += 1;
    }
}

/// Cache eviction strategies
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvictionStrategy {
    /// Least Recently Used
    Lru,
    /// Least Frequently Used
    Lfu,
    /// First In First Out
    Fifo,
    /// Time To Live based
    Ttl,
    /// Random eviction
    Random,
}

/// Cache configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    pub max_size: usize,
    pub eviction_strategy: EvictionStrategy,
    pub default_ttl: Option<Duration>,
    pub cleanup_interval: Duration,
    pub enable_metrics: bool,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            max_size: 1000,
            eviction_strategy: EvictionStrategy::Lru,
            default_ttl: Some(Duration::from_secs(300)), // 5 minutes
            cleanup_interval: Duration::from_secs(60), // 1 minute
            enable_metrics: true,
        }
    }
}

/// Cache metrics for monitoring
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CacheMetrics {
    pub hits: u64,
    pub misses: u64,
    pub evictions: u64,
    pub inserts: u64,
    pub removals: u64,
    pub current_size: usize,
    pub total_requests: u64,
}

impl CacheMetrics {
    pub fn hit_rate(&self) -> f64 {
        if self.total_requests == 0 {
            0.0
        } else {
            self.hits as f64 / self.total_requests as f64
        }
    }

    pub fn miss_rate(&self) -> f64 {
        if self.total_requests == 0 {
            0.0
        } else {
            self.misses as f64 / self.total_requests as f64
        }
    }
}

/// Main cache implementation
pub struct Cache<K, V> 
where
    K: Hash + Eq + Clone + Send + Sync + std::fmt::Debug + 'static,
    V: Clone + Send + Sync + 'static,
{
    data: Arc<AsyncRwLock<HashMap<K, CacheEntry<V>>>>,
    config: CacheConfig,
    metrics: Arc<RwLock<CacheMetrics>>,
}

impl<K, V> Cache<K, V>
where
    K: Hash + Eq + Clone + Send + Sync + std::fmt::Debug + 'static,
    V: Clone + Send + Sync + 'static,
{
    /// Create a new cache with the given configuration
    pub fn new(config: CacheConfig) -> Self {
        Self {
            data: Arc::new(AsyncRwLock::new(HashMap::new())),
            config,
            metrics: Arc::new(RwLock::new(CacheMetrics::default())),
        }
    }

    /// Get a value from the cache
    pub async fn get(&self, key: &K) -> Option<V> {
        let mut data = self.data.write().await;
        let mut metrics = self.metrics.write().unwrap();
        
        metrics.total_requests += 1;

        if let Some(entry) = data.get_mut(key) {
            if entry.is_expired() {
                data.remove(key);
                metrics.misses += 1;
                metrics.current_size = data.len();
                debug!("Cache miss due to expiration for key: {:?}", key);
                return None;
            }

            entry.touch();
            metrics.hits += 1;
            debug!("Cache hit for key: {:?}", key);
            Some(entry.value.clone())
        } else {
            metrics.misses += 1;
            debug!("Cache miss for key: {:?}", key);
            None
        }
    }

    /// Insert a value into the cache
    pub async fn insert(&self, key: K, value: V) {
        self.insert_with_ttl(key, value, self.config.default_ttl).await;
    }

    /// Insert a value with custom TTL
    pub async fn insert_with_ttl(&self, key: K, value: V, ttl: Option<Duration>) {
        let mut data = self.data.write().await;
        let mut metrics = self.metrics.write().unwrap();

        // Check if we need to evict
        if data.len() >= self.config.max_size && !data.contains_key(&key) {
            self.evict_entry(&mut data, &mut metrics).await;
        }

        let entry = CacheEntry::new(value, ttl);
        data.insert(key.clone(), entry);
        metrics.inserts += 1;
        metrics.current_size = data.len();
        
        debug!("Inserted cache entry for key: {:?}", key);
    }

    /// Remove a value from the cache
    pub async fn remove(&self, key: &K) -> Option<V> {
        let mut data = self.data.write().await;
        let mut metrics = self.metrics.write().unwrap();

        if let Some(entry) = data.remove(key) {
            metrics.removals += 1;
            metrics.current_size = data.len();
            debug!("Removed cache entry for key: {:?}", key);
            Some(entry.value)
        } else {
            None
        }
    }

    /// Clear all entries from the cache
    pub async fn clear(&self) {
        let mut data = self.data.write().await;
        let mut metrics = self.metrics.write().unwrap();

        data.clear();
        metrics.current_size = 0;
        info!("Cleared all cache entries");
    }

    /// Get cache metrics
    pub fn metrics(&self) -> CacheMetrics {
        self.metrics.read().unwrap().clone()
    }

    /// Get current cache size
    pub async fn size(&self) -> usize {
        self.data.read().await.len()
    }

    /// Check if cache contains a key
    pub async fn contains_key(&self, key: &K) -> bool {
        let data = self.data.read().await;
        if let Some(entry) = data.get(key) {
            !entry.is_expired()
        } else {
            false
        }
    }

    /// Evict an entry based on the configured strategy
    async fn evict_entry(&self, data: &mut HashMap<K, CacheEntry<V>>, metrics: &mut CacheMetrics) {
        if data.is_empty() {
            return;
        }

        let key_to_evict = match self.config.eviction_strategy {
            EvictionStrategy::Lru => {
                data.iter()
                    .min_by_key(|(_, entry)| entry.last_accessed)
                    .map(|(key, _)| key.clone())
            }
            EvictionStrategy::Lfu => {
                data.iter()
                    .min_by_key(|(_, entry)| entry.access_count)
                    .map(|(key, _)| key.clone())
            }
            EvictionStrategy::Fifo => {
                data.iter()
                    .min_by_key(|(_, entry)| entry.created_at)
                    .map(|(key, _)| key.clone())
            }
            EvictionStrategy::Ttl => {
                data.iter()
                    .filter(|(_, entry)| entry.is_expired())
                    .min_by_key(|(_, entry)| entry.created_at)
                    .map(|(key, _)| key.clone())
            }
            EvictionStrategy::Random => {
                data.keys().next().cloned()
            }
        };

        if let Some(key) = key_to_evict {
            data.remove(&key);
            metrics.evictions += 1;
            debug!("Evicted cache entry for key: {:?} using strategy: {:?}", key, self.config.eviction_strategy);
        }
    }

    /// Clean up expired entries
    pub async fn cleanup_expired(&self) -> usize {
        let mut data = self.data.write().await;
        let mut metrics = self.metrics.write().unwrap();

        let initial_size = data.len();
        data.retain(|_, entry| !entry.is_expired());
        let removed_count = initial_size - data.len();
        
        if removed_count > 0 {
            metrics.current_size = data.len();
            debug!("Cleaned up {} expired cache entries", removed_count);
        }

        removed_count
    }

    /// Start background cleanup task
    pub async fn start_cleanup_task(&self) {
        let cache = self.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(cache.config.cleanup_interval);
            loop {
                interval.tick().await;
                cache.cleanup_expired().await;
            }
        });
    }
}

impl<K, V> Clone for Cache<K, V>
where
    K: Hash + Eq + Clone + Send + Sync + std::fmt::Debug + 'static,
    V: Clone + Send + Sync + 'static,
{
    fn clone(&self) -> Self {
        Self {
            data: Arc::clone(&self.data),
            config: self.config.clone(),
            metrics: Arc::clone(&self.metrics),
        }
    }
}

/// Task-specific cache for storing task data
pub type TaskCache = Cache<String, serde_json::Value>;

/// Cache factory for creating different types of caches
pub struct CacheFactory;

impl CacheFactory {
    /// Create a task cache with default configuration
    pub fn create_task_cache() -> TaskCache {
        let config = CacheConfig {
            max_size: 5000,
            eviction_strategy: EvictionStrategy::Lru,
            default_ttl: Some(Duration::from_secs(600)), // 10 minutes
            cleanup_interval: Duration::from_secs(30),
            enable_metrics: true,
        };
        
        Cache::new(config)
    }

    /// Create a session cache with shorter TTL
    pub fn create_session_cache() -> Cache<String, String> {
        let config = CacheConfig {
            max_size: 1000,
            eviction_strategy: EvictionStrategy::Ttl,
            default_ttl: Some(Duration::from_secs(1800)), // 30 minutes
            cleanup_interval: Duration::from_secs(60),
            enable_metrics: true,
        };
        
        Cache::new(config)
    }

    /// Create a result cache for API responses
    pub fn create_result_cache() -> Cache<String, serde_json::Value> {
        let config = CacheConfig {
            max_size: 2000,
            eviction_strategy: EvictionStrategy::Lfu,
            default_ttl: Some(Duration::from_secs(300)), // 5 minutes
            cleanup_interval: Duration::from_secs(45),
            enable_metrics: true,
        };
        
        Cache::new(config)
    }
}

/// Cache middleware for HTTP requests
pub struct CacheMiddleware {
    cache: TaskCache,
}

impl CacheMiddleware {
    pub fn new(cache: TaskCache) -> Self {
        Self { cache }
    }

    /// Get cached response for a request
    pub async fn get_cached_response(&self, key: &str) -> Option<serde_json::Value> {
        self.cache.get(&key.to_string()).await
    }

    /// Cache a response
    pub async fn cache_response(&self, key: String, response: serde_json::Value, ttl: Option<Duration>) {
        self.cache.insert_with_ttl(key, response, ttl).await;
    }

    /// Generate cache key from request
    pub fn generate_cache_key(&self, method: &str, path: &str, query: Option<&str>) -> String {
        let query_str = query.unwrap_or("");
        format!("{}:{}:{}", method.to_uppercase(), path, query_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[tokio::test]
    async fn test_cache_basic_operations() {
        let cache = Cache::new(CacheConfig::default());
        
        // Test insert and get
        cache.insert("key1".to_string(), "value1".to_string()).await;
        assert_eq!(cache.get(&"key1".to_string()).await, Some("value1".to_string()));
        
        // Test miss
        assert_eq!(cache.get(&"key2".to_string()).await, None);
        
        // Test remove
        assert_eq!(cache.remove(&"key1".to_string()).await, Some("value1".to_string()));
        assert_eq!(cache.get(&"key1".to_string()).await, None);
    }

    #[tokio::test]
    async fn test_cache_ttl() {
        let config = CacheConfig {
            max_size: 100,
            eviction_strategy: EvictionStrategy::Lru,
            default_ttl: Some(Duration::from_millis(100)),
            cleanup_interval: Duration::from_millis(50),
            enable_metrics: true,
        };
        
        let cache = Cache::new(config);
        
        cache.insert("key1".to_string(), "value1".to_string()).await;
        assert_eq!(cache.get(&"key1".to_string()).await, Some("value1".to_string()));
        
        // Wait for expiration
        thread::sleep(Duration::from_millis(150));
        assert_eq!(cache.get(&"key1".to_string()).await, None);
    }

    #[tokio::test]
    async fn test_cache_metrics() {
        let cache = Cache::new(CacheConfig::default());
        
        cache.insert("key1".to_string(), "value1".to_string()).await;
        cache.get(&"key1".to_string()).await;
        cache.get(&"key2".to_string()).await;
        
        let metrics = cache.metrics();
        assert_eq!(metrics.hits, 1);
        assert_eq!(metrics.misses, 1);
        assert_eq!(metrics.total_requests, 2);
        assert_eq!(metrics.inserts, 1);
    }

    #[tokio::test]
    async fn test_cache_eviction() {
        let config = CacheConfig {
            max_size: 2,
            eviction_strategy: EvictionStrategy::Lru,
            default_ttl: None,
            cleanup_interval: Duration::from_secs(60),
            enable_metrics: true,
        };
        
        let cache = Cache::new(config);
        
        cache.insert("key1".to_string(), "value1".to_string()).await;
        cache.insert("key2".to_string(), "value2".to_string()).await;
        cache.insert("key3".to_string(), "value3".to_string()).await;
        
        // key1 should be evicted (LRU)
        assert_eq!(cache.get(&"key1".to_string()).await, None);
        assert_eq!(cache.get(&"key2".to_string()).await, Some("value2".to_string()));
        assert_eq!(cache.get(&"key3".to_string()).await, Some("value3".to_string()));
    }
}
