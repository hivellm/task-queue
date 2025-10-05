# Vectorizer Integration Guide

## Overview

The Task Queue system integrates with Vectorizer v0.3.0 to provide advanced semantic search and context persistence capabilities. This integration enables the system to learn from task execution patterns and provide intelligent recommendations.

## Architecture

```
┌─────────────────┐    HTTP/JSON    ┌─────────────────┐
│   Task Queue    │ ◄─────────────► │   Vectorizer    │
│   Server        │   Port 15002    │   v0.3.0        │
└─────────────────┘                 └─────────────────┘
         │                                   │
         ▼                                   ▼
┌─────────────────┐                 ┌─────────────────┐
│   Task Context  │                 │   Vector Store  │
│   Storage       │                 │   (Collection)  │
└─────────────────┘                 └─────────────────┘
```

## Configuration

### Vectorizer Settings

```yaml
vectorizer:
  endpoint: "http://localhost:15002"  # Vectorizer server endpoint
  collection: "task-interactions"     # Collection name for task data
  auto_index: true                     # Automatically index task interactions
```

### Environment Variables

```bash
# Vectorizer Configuration
VECTORIZER_ENDPOINT=http://localhost:15002
TASK_QUEUE_COLLECTION=task-interactions
```

## Features

### 1. Task Context Persistence

Every task execution is automatically stored in the Vectorizer with rich context:

```rust
let context = TaskContext {
    task_id: "build-api",
    project: "hivellm-governance",
    execution_time: Duration::from_secs(45),
    result: TaskResult::Success,
    artifacts: vec!["api-server", "documentation"],
    dependencies: vec!["run-tests"],
    logs: vec!["Build completed successfully"],
    parameters: json!({
        "build_type": "production",
        "optimize": true
    })
};

// Automatically stored in vectorizer
vectorizer.store_task_context(&context).await?;
```

### 2. Semantic Search

Search through task history using natural language queries:

```rust
// Find similar tasks
let similar_tasks = vectorizer.search_task_contexts(
    "API deployment with tests",
    Some(10) // limit
).await?;

// Results include similarity scores and metadata
for task in similar_tasks {
    println!("Task: {} (Score: {})", task.task_id, task.score);
    println!("Text: {}", task.text);
    println!("Metadata: {:?}", task.metadata);
}
```

### 3. Task Recommendations

Get intelligent recommendations based on historical data:

```rust
let recommendations = vectorizer.get_task_recommendations(
    &current_task,
    Some(5) // limit
).await?;

for rec in recommendations {
    println!("Similar Task: {}", rec.task_id);
    println!("Similarity Score: {}", rec.similarity_score);
    println!("Suggested Dependencies: {:?}", rec.suggested_dependencies);
    println!("Suggested Artifacts: {:?}", rec.suggested_artifacts);
    println!("Execution Time Estimate: {:?}", rec.execution_time_estimate);
}
```

## API Integration

### Endpoint Details

The integration uses the Vectorizer's `/insert_texts` endpoint:

```http
POST http://localhost:15002/insert_texts
Content-Type: application/json

{
  "texts": [
    {
      "id": "task-123",
      "text": "Task execution context...",
      "metadata": {
        "task_id": "task-123",
        "project": "my-project",
        "execution_time_ms": 45000,
        "artifacts": ["build-output"],
        "dependencies": ["test-task"],
        "timestamp": "2025-10-05T13:40:00Z",
        "result_type": "success"
      }
    }
  ]
}
```

### Health Check

The system automatically checks Vectorizer connectivity:

```http
GET http://localhost:15002/health
```

Response:
```json
{
  "status": "healthy",
  "timestamp": "2025-10-05T13:38:32.258562269Z",
  "version": "0.3.0"
}
```

## Collection Management

### Automatic Collection Creation

The system automatically creates the `task-interactions` collection:

```http
POST http://localhost:15002/collections
Content-Type: application/json

{
  "name": "task-interactions",
  "dimension": 512,
  "metric": "cosine"
}
```

### Collection Configuration

- **Dimension**: 512 (configurable)
- **Metric**: Cosine similarity
- **Auto-indexing**: Enabled by default
- **Retention**: Configurable through vectorizer settings

## Data Format

### Task Context Structure

```rust
pub struct TaskContext {
    pub task_id: String,
    pub project: Option<String>,
    pub execution_time: Duration,
    pub result: TaskResult,
    pub artifacts: Vec<String>,
    pub dependencies: Vec<String>,
    pub logs: Vec<String>,
    pub parameters: serde_json::Value,
}
```

### Vectorizer Payload

```json
{
  "texts": [
    {
      "id": "task-123",
      "text": "Task ID: task-123\nProject: my-project\nExecution Time: 45000ms\nDependencies: test-task\nArtifacts: build-output\nLogs: Build completed successfully\nResult: SUCCESS\nOutput: Build successful\nArtifacts: [\"build-output\"]\nMetrics: {\"duration\": 45000}",
      "metadata": {
        "task_id": "task-123",
        "project": "my-project",
        "execution_time_ms": 45000,
        "artifacts": ["build-output"],
        "dependencies": ["test-task"],
        "timestamp": "2025-10-05T13:40:00Z",
        "result_type": "success"
      }
    }
  ]
}
```

## Error Handling

### Connection Errors

If the Vectorizer is unavailable, the system continues to function without vectorization:

```
⚠️  Vectorizer not available - running without vectorization
```

### API Errors

The system handles various error scenarios gracefully:

```rust
match response {
    Ok(resp) => {
        if !resp.status().is_success() {
            eprintln!("⚠️  Vectorizer returned error status: {} - Task context not stored", resp.status());
        } else {
            println!("✅ Task context stored in vectorizer successfully");
        }
    }
    Err(e) => {
        eprintln!("⚠️  Failed to connect to vectorizer: {} - Task context not stored", e);
    }
}
```

## Performance Considerations

### Batch Operations

For high-volume scenarios, consider implementing batch operations:

```rust
// Batch multiple task contexts
let contexts = vec![context1, context2, context3];
vectorizer.store_batch_contexts(&contexts).await?;
```

### Caching

The system includes intelligent caching to reduce Vectorizer API calls:

- **Search Results**: Cached for 5 minutes
- **Recommendations**: Cached for 10 minutes
- **Collection Info**: Cached for 1 hour

## Monitoring

### Health Checks

Monitor Vectorizer connectivity:

```bash
# Check Vectorizer health
curl http://localhost:15002/health

# Check collection status
curl http://localhost:15002/collections/task-interactions
```

### Metrics

The integration provides metrics for:

- **Connection Status**: Vectorizer connectivity
- **Insertion Rate**: Tasks stored per minute
- **Search Performance**: Query response times
- **Error Rate**: Failed operations

## Troubleshooting

### Common Issues

1. **Connection Refused**
   - Verify Vectorizer is running on port 15002
   - Check firewall settings
   - Verify endpoint configuration

2. **Collection Not Found**
   - Ensure collection is created
   - Check collection name in configuration
   - Verify Vectorizer permissions

3. **Insertion Failures**
   - Check Vectorizer logs
   - Verify payload format
   - Check collection capacity

### Debug Mode

Enable debug logging for detailed integration information:

```bash
RUST_LOG=debug ./target/release/task-queue
```

## Future Enhancements

### Planned Features

- **Real-time Indexing**: Live updates to vector store
- **Advanced Analytics**: Task execution pattern analysis
- **ML Integration**: Machine learning-based recommendations
- **Multi-collection Support**: Separate collections for different project types

### API Improvements

- **Batch Operations**: Efficient bulk operations
- **Streaming Updates**: Real-time context updates
- **Advanced Search**: Complex query capabilities
- **Custom Embeddings**: Support for custom embedding models

## Support

For issues related to Vectorizer integration:

1. Check the [Vectorizer Documentation](../vectorizer/docs/)
2. Review the [Task Queue Logs](../logs/)
3. Verify configuration settings
4. Check network connectivity

## Examples

### Complete Integration Example

```rust
use task_queue::{TaskQueue, VectorizerIntegration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize task queue with vectorizer integration
    let mut queue = TaskQueue::new().await?;
    
    // Create a task
    let task = Task::new("build-api")
        .with_command("cargo build --release")
        .with_project("my-project");
    
    // Submit task
    let task_id = queue.submit_task(task).await?;
    
    // Task execution context is automatically stored in vectorizer
    
    // Search for similar tasks
    let similar_tasks = queue.search_similar_tasks(
        "Rust build with optimization",
        Some(5)
    ).await?;
    
    // Get recommendations for next tasks
    let recommendations = queue.get_task_recommendations(&task_id).await?;
    
    Ok(())
}
```

This integration provides powerful semantic search and learning capabilities for the Task Queue system, enabling intelligent task management and workflow optimization.
