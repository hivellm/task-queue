# Task Queue - Advanced Task Orchestration System

## Overview

Task Queue is an advanced task orchestration system built in Rust, specifically designed for the HiveLLM ecosystem. It provides advanced features beyond traditional message queues like Redis/RabbitMQ.

## Key Features

### 🎯 **Dependency Management**
- **Task Dependencies**: Tasks can depend on other tasks completing successfully
- **Conditional Execution**: Tasks execute only when dependencies are satisfied
- **Dependency Graphs**: Visual representation of task relationships
- **Circular Dependency Detection**: Prevents infinite loops

### 📊 **Vectorizer Integration**
- **Interaction Persistence**: All task interactions are stored in vectorizer
- **Semantic Search**: Search task history and patterns
- **Context Awareness**: Tasks have rich context from previous executions
- **Learning System**: System learns from task execution patterns
- **Vectorizer v0.3.0**: Compatible with latest Vectorizer interface
- **Collection Management**: Automatic collection creation and management

### ⚡ **High Performance**
- **Rust Performance**: Native speed with memory safety
- **Local Storage**: Embedded database for persistence
- **Concurrent Processing**: Parallel task execution
- **Resource Management**: Intelligent resource allocation

### 🔄 **Advanced Orchestration**
- **Workflow Management**: Complex multi-step workflows
- **Retry Logic**: Intelligent retry with exponential backoff
- **Timeout Handling**: Configurable timeouts per task type
- **Priority Queues**: Task prioritization system

### 🛡️ **Reliability**
- **Task Integrity**: Cryptographic task validation
- **Crash Recovery**: Automatic recovery from failures
- **Audit Trail**: Complete execution history
- **Health Monitoring**: System health checks

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                    Task Queue Server                           │
├─────────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐             │
│  │   Task      │  │ Dependency  │  │ Vectorizer  │             │
│  │  Manager    │  │  Resolver   │  │ Integration │             │
│  └─────────────┘  └─────────────┘  └─────────────┘             │
├─────────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐             │
│  │   Storage   │  │   Metrics   │  │   Health    │             │
│  │   Engine    │  │  Collector  │  │  Monitor    │             │
│  └─────────────┘  └─────────────┘  └─────────────┘             │
├─────────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐             │
│  │   GRPC      │  │   HTTP      │  │   MCP       │             │
│  │   Server    │  │   Server    │  │   Server    │             │
│  └─────────────┘  └─────────────┘  └─────────────┘             │
└─────────────────────────────────────────────────────────────────┘
```

## Task Types

### 1. **Simple Tasks**
- Independent tasks with no dependencies
- Immediate execution when resources available
- Basic retry logic

### 2. **Dependent Tasks**
- Tasks that depend on other tasks completing
- Automatic dependency resolution
- Conditional execution based on dependency results

### 3. **Workflow Tasks**
- Complex multi-step workflows
- Branching and merging logic
- Parallel and sequential execution

### 4. **Scheduled Tasks**
- Time-based task execution
- Cron-like scheduling
- Recurring task support

## Dependency System

### Dependency Types

#### **Success Dependency**
```rust
Task::new("build-api")
    .depends_on("run-tests")
    .with_condition(DependencyCondition::Success)
```

#### **Failure Dependency**
```rust
Task::new("notify-failure")
    .depends_on("deploy")
    .with_condition(DependencyCondition::Failure)
```

#### **Completion Dependency**
```rust
Task::new("cleanup")
    .depends_on("deploy")
    .with_condition(DependencyCondition::Completion)
```

#### **Custom Condition**
```rust
Task::new("conditional-task")
    .depends_on("test-coverage")
    .with_condition(DependencyCondition::Custom(|result| {
        result.coverage_percentage >= 80.0
    }))
```

### Dependency Resolution

```rust
// Example: Dashboard depends on API being ready
let dashboard_task = Task::new("create-dashboard")
    .depends_on("api-ready")
    .with_condition(DependencyCondition::Success)
    .with_timeout(Duration::from_secs(300));

// API task must complete successfully before dashboard starts
let api_task = Task::new("api-ready")
    .with_command("start-api-server")
    .with_health_check("/api/health");
```

## Vectorizer Integration

### Interaction Persistence

Every task interaction is automatically stored in the vectorizer:

```rust
// Task execution context
let context = TaskContext {
    task_id: "build-api",
    project: "hivellm-governance",
    dependencies: vec!["run-tests"],
    parameters: json!({
        "build_type": "production",
        "optimize": true
    }),
    execution_time: Duration::from_secs(45),
    result: TaskResult::Success,
    artifacts: vec!["api-server", "documentation"]
};

// Automatically stored in vectorizer
vectorizer.store_task_interaction(context).await?;
```

### Semantic Search

Search task history and patterns:

```rust
// Find similar tasks
let similar_tasks = vectorizer.search_tasks(
    "API deployment with tests",
    SearchOptions {
        project: Some("hivellm-governance"),
        task_type: Some("deployment"),
        limit: 10
    }
).await?;

// Get task recommendations
let recommendations = vectorizer.get_task_recommendations(
    "build-api",
    RecommendationOptions {
        include_dependencies: true,
        include_artifacts: true
    }
).await?;
```

## Usage Examples

### Basic Task Creation

```rust
use task_queue::{TaskQueue, Task, TaskType};

let mut queue = TaskQueue::new().await?;

// Simple task
let simple_task = Task::new("hello-world")
    .with_command("echo 'Hello, World!'")
    .with_type(TaskType::Simple);

queue.submit_task(simple_task).await?;
```

### Dependency Chain

```rust
// Test coverage must be >= 80% before deployment
let test_task = Task::new("run-tests")
    .with_command("npm test -- --coverage")
    .with_validation(|result| {
        result.coverage_percentage >= 80.0
    });

let deploy_task = Task::new("deploy")
    .depends_on("run-tests")
    .with_condition(DependencyCondition::Success)
    .with_command("npm run deploy");

queue.submit_tasks(vec![test_task, deploy_task]).await?;
```

### Complex Workflow

```rust
// Multi-step deployment workflow
let workflow = Workflow::new("production-deployment")
    .add_task(Task::new("run-tests")
        .with_command("npm test")
        .with_parallel_execution(true))
    .add_task(Task::new("build-docker")
        .depends_on("run-tests")
        .with_condition(DependencyCondition::Success))
    .add_task(Task::new("deploy-staging")
        .depends_on("build-docker")
        .with_condition(DependencyCondition::Success))
    .add_task(Task::new("run-integration-tests")
        .depends_on("deploy-staging")
        .with_condition(DependencyCondition::Success))
    .add_task(Task::new("deploy-production")
        .depends_on("run-integration-tests")
        .with_condition(DependencyCondition::Success)
        .with_manual_approval(true));

queue.submit_workflow(workflow).await?;
```

## Configuration

### Server Configuration

```yaml
# config.yml
server:
  host: "0.0.0.0"
  port: 16080
  grpc_port: 16081
  mcp_port: 16082

storage:
  database_path: "./data/task-queue.db"
  backup_interval: "1h"
  retention_days: 30

vectorizer:
  endpoint: "http://localhost:15002"
  collection: "task-interactions"
  auto_index: true

execution:
  max_concurrent_tasks: 10
  default_timeout: "5m"
  retry_attempts: 3
  retry_delay: "1s"

monitoring:
  metrics_enabled: true
  metrics_port: 9090
  health_check_interval: "30s"
```

## Client SDKs

### Rust Client

```rust
use task_queue_client::TaskQueueClient;

let client = TaskQueueClient::new("http://localhost:16080").await?;

// Submit task
let task_id = client.submit_task(Task::new("build")
    .with_command("cargo build")
    .with_project("my-project")).await?;

// Check status
let status = client.get_task_status(&task_id).await?;

// Get result
let result = client.get_task_result(&task_id).await?;
```

### TypeScript Client

```typescript
import { TaskQueueClient } from '@hivellm/task-queue-client';

const client = new TaskQueueClient('http://localhost:16080');

// Submit task
const taskId = await client.submitTask({
  name: 'build',
  command: 'npm run build',
  project: 'my-project',
  dependencies: ['test']
});

// Monitor progress
const status = await client.getTaskStatus(taskId);
```

### Python Client

```python
from task_queue_client import TaskQueueClient

client = TaskQueueClient('http://localhost:16080')

# Submit task
task_id = client.submit_task(
    name='build',
    command='python setup.py build',
    project='my-project',
    dependencies=['test']
)

# Get result
result = client.get_task_result(task_id)
```

## Dev Tools Integration

The dev-tools MCP server will use task-queue for orchestration:

```typescript
// Dev-tools integration
class DevToolsMCPServer {
  private taskQueue: TaskQueueClient;

  async executeComplexWorkflow(workflow: WorkflowDefinition) {
    // Submit to task-queue
    const workflowId = await this.taskQueue.submitWorkflow({
      name: workflow.name,
      tasks: workflow.tasks,
      dependencies: workflow.dependencies
    });

    // Monitor execution
    const status = await this.taskQueue.getWorkflowStatus(workflowId);
    
    return {
      workflowId,
      status,
      results: await this.taskQueue.getWorkflowResults(workflowId)
    };
  }
}
```

## Performance Characteristics

- **Throughput**: 10,000+ tasks/second
- **Latency**: < 1ms for task submission
- **Concurrency**: 100+ concurrent tasks
- **Storage**: Efficient binary serialization
- **Memory**: < 100MB base usage
- **CPU**: Optimized for multi-core systems

## Security Features

- **Task Integrity**: SHA-256 task validation
- **Access Control**: Role-based permissions
- **Audit**: Complete execution trail
- **Sandboxing**: Isolated task execution
- **Resource Limits**: CPU, memory and time limits

This task-queue system provides the advanced orchestration capabilities needed for the HiveLLM ecosystem, maintaining high performance and reliability.

## Documentation

- [Complete Documentation](docs/COMPLETE_DOCUMENTATION.md) - Comprehensive system documentation
- [API Documentation](docs/API_DOCUMENTATION.md) - HTTP API reference
- [Vectorizer Integration](docs/VECTORIZER_INTEGRATION.md) - Advanced semantic search integration
- [Development Roadmap](docs/DEVELOPMENT_ROADMAP.md) - Future development plans
- [Testing Guide](docs/TESTING_EXECUTION_GUIDE.md) - Testing and validation procedures

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for a detailed list of changes and updates.