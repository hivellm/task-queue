# Cache System - Technical Documentation

## Overview
The Cache System provides comprehensive caching capabilities with different eviction strategies, TTL support, and performance optimization for the Task Queue system. It supports multiple cache types and configurations for different use cases.

## Architecture

### Core Components

#### 1. CacheEntry Structure
```rust
pub struct CacheEntry<V> {
    pub value: V,
    pub created_at: Instant,
    pub last_accessed: Instant,
    pub access_count: u64,
    pub ttl: Option<Duration>,
}
```
- Stores cached value with metadata
- Tracks creation time, last access, and access count
- Supports TTL (Time To Live) expiration

#### 2. Eviction Strategies
```rust
pub enum EvictionStrategy {
    Lru,    // Least Recently Used
    Lfu,    // Least Frequently Used
    Fifo,   // First In First Out
    Ttl,    // Time To Live based
    Random, // Random eviction
}
```

#### 3. Cache Configuration
```rust
pub struct CacheConfig {
    pub max_size: usize,
    pub eviction_strategy: EvictionStrategy,
    pub default_ttl: Option<Duration>,
    pub cleanup_interval: Duration,
    pub enable_metrics: bool,
}
```

### Cache Implementation

#### Generic Cache<K, V>
- Thread-safe async cache implementation
- Supports any key-value types that implement required traits
- Built on top of tokio::sync::RwLock for async operations
- Includes comprehensive metrics tracking

#### Key Features
- **Async Operations**: All cache operations are async and non-blocking
- **Thread Safety**: Uses Arc<RwLock> for safe concurrent access
- **TTL Support**: Automatic expiration of entries based on TTL
- **Eviction Policies**: Multiple eviction strategies for different use cases
- **Metrics**: Built-in performance and usage metrics
- **Cleanup**: Background cleanup of expired entries

## Cache Types

### 1. TaskCache
```rust
pub type TaskCache = Cache<String, serde_json::Value>;
```
- Specialized cache for task data
- Uses JSON values for flexible task storage
- Optimized for task queue operations

### 2. SessionCache
```rust
Cache<String, String>
```
- Cache for session data
- Shorter TTL for security
- Optimized for user session management

### 3. ResultCache
```rust
Cache<String, serde_json::Value>
```
- Cache for API response results
- Uses LFU eviction strategy
- Optimized for API response caching

## Implementation Details

### Cache Operations

#### Get Operation
```rust
pub async fn get(&self, key: &K) -> Option<V>
```
- Retrieves value from cache
- Updates access metadata
- Checks for expiration
- Updates hit/miss metrics

#### Insert Operation
```rust
pub async fn insert(&self, key: K, value: V)
pub async fn insert_with_ttl(&self, key: K, value: V, ttl: Option<Duration>)
```
- Inserts value into cache
- Handles eviction when cache is full
- Updates metrics
- Supports custom TTL

#### Remove Operation
```rust
pub async fn remove(&self, key: &K) -> Option<V>
```
- Removes entry from cache
- Updates metrics
- Returns removed value

### Eviction Logic

#### LRU (Least Recently Used)
- Evicts entry with oldest last_accessed time
- Good for temporal locality patterns

#### LFU (Least Frequently Used)
- Evicts entry with lowest access_count
- Good for frequency-based access patterns

#### FIFO (First In First Out)
- Evicts entry with oldest created_at time
- Simple and predictable eviction

#### TTL-based
- Evicts expired entries first
- Good for time-sensitive data

#### Random
- Evicts random entry
- Simple implementation
- Good for uniform access patterns

### Metrics System

#### CacheMetrics Structure
```rust
pub struct CacheMetrics {
    pub hits: u64,
    pub misses: u64,
    pub evictions: u64,
    pub inserts: u64,
    pub removals: u64,
    pub current_size: usize,
    pub total_requests: u64,
}
```

#### Calculated Metrics
- **Hit Rate**: hits / total_requests
- **Miss Rate**: misses / total_requests
- **Current Size**: Number of active entries

## Cache Factory

### CacheFactory
Provides factory methods for creating different cache types:

```rust
impl CacheFactory {
    pub fn create_task_cache() -> TaskCache
    pub fn create_session_cache() -> Cache<String, String>
    pub fn create_result_cache() -> Cache<String, serde_json::Value>
}
```

## Cache Middleware

### CacheMiddleware
HTTP middleware for caching API responses:

```rust
pub struct CacheMiddleware {
    cache: TaskCache,
}
```

#### Features
- Request-based cache key generation
- Response caching with TTL
- Integration with HTTP handlers

## Performance Considerations

### Memory Management
- Bounded cache size prevents memory leaks
- Efficient HashMap usage for O(1) operations
- Automatic cleanup of expired entries

### Async Operations
- Non-blocking cache operations
- Efficient async/await usage
- Minimal allocation overhead

### Concurrency
- Read-write lock for optimal concurrent access
- Atomic metrics updates
- Thread-safe operations

## Configuration Examples

### Basic Task Cache
```rust
let config = CacheConfig {
    max_size: 5000,
    eviction_strategy: EvictionStrategy::Lru,
    default_ttl: Some(Duration::from_secs(600)), // 10 minutes
    cleanup_interval: Duration::from_secs(30),
    enable_metrics: true,
};
```

### Session Cache
```rust
let config = CacheConfig {
    max_size: 1000,
    eviction_strategy: EvictionStrategy::Ttl,
    default_ttl: Some(Duration::from_secs(1800)), // 30 minutes
    cleanup_interval: Duration::from_secs(60),
    enable_metrics: true,
};
```

### High-Performance Cache
```rust
let config = CacheConfig {
    max_size: 10000,
    eviction_strategy: EvictionStrategy::Lfu,
    default_ttl: Some(Duration::from_secs(300)), // 5 minutes
    cleanup_interval: Duration::from_secs(15),
    enable_metrics: true,
};
```

## Error Handling

### Graceful Degradation
- Cache misses return None instead of panicking
- Expired entries are automatically removed
- Metrics continue to work even with errors

### Resource Management
- Automatic cleanup prevents resource leaks
- Bounded size prevents memory exhaustion
- Background tasks are properly managed

## Testing Strategy

### Unit Tests
- Cache operations (get, insert, remove)
- Eviction strategies
- TTL expiration
- Metrics accuracy

### Integration Tests
- Cache middleware functionality
- Concurrent access patterns
- Performance under load

### Performance Tests
- Cache hit/miss ratios
- Memory usage patterns
- Cleanup efficiency

## Security Considerations

### Data Protection
- No sensitive data logging in debug messages
- Secure key generation for cache keys
- Proper cleanup of sensitive cached data

### Access Control
- Cache access is internal to the application
- No external cache access points
- Secure cache key generation

## Monitoring and Observability

### Metrics Collection
- Hit/miss ratios for performance tuning
- Cache size monitoring for memory management
- Eviction rates for capacity planning

### Health Checks
- Cache accessibility verification
- Memory usage monitoring
- Performance metrics collection

## Future Enhancements

### Planned Features
- Distributed caching support
- Cache warming strategies
- Advanced eviction algorithms
- Cache compression

### Scalability Improvements
- Multi-level caching
- Cache partitioning
- Advanced metrics and monitoring
- Integration with external cache systems
