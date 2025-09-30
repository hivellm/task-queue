# Task Queue Python SDK

A comprehensive Python SDK for interacting with the Task Queue API, providing both synchronous and asynchronous operations with full type safety and comprehensive error handling.

## Features

- 🚀 **Async/Await Support**: Full async support for high-performance applications
- 📝 **Type Hints**: Complete type annotations for better IDE support
- 🔧 **CLI Tool**: Command-line interface for quick operations
- 🛡️ **Error Handling**: Comprehensive exception handling with custom error types
- 📊 **Retry Logic**: Automatic retry with exponential backoff
- 🔄 **Batch Operations**: Support for multiple task operations
- 🔔 **Webhooks**: Real-time notifications via webhook callbacks
- 📚 **Full API Coverage**: All Task Queue API endpoints supported

## Installation

```bash
pip install taskqueue-sdk
```

## Quick Start

### Basic Usage

```python
from taskqueue import TaskQueueClient

# Initialize client
client = TaskQueueClient(base_url="http://localhost:8080")

# Create a task
task = client.create_task(
    name="Process Data",
    command="python process.py",
    project_id="your-project-id",
    priority="High"
)

# Get task status
status = client.get_task(task.id)
print(f"Task status: {status.status}")
```

### Async Usage

```python
import asyncio
from taskqueue import AsyncTaskQueueClient

async def main():
    async with AsyncTaskQueueClient(base_url="http://localhost:8080") as client:
        # Create multiple tasks
        tasks = await client.create_tasks([
            {
                "name": "Task 1",
                "command": "echo 'Hello'",
                "project_id": "project-1"
            },
            {
                "name": "Task 2",
                "command": "echo 'World'",
                "project_id": "project-1"
            }
        ])

        # Wait for completion
        for task in tasks:
            await client.wait_for_completion(task.id)

asyncio.run(main())
```

### CLI Usage

```bash
# List tasks
taskqueue tasks list

# Create a task
taskqueue tasks create --name "My Task" --command "echo hello" --project-id "proj-123"

# Get task details
taskqueue tasks get task-uuid-here

# Cancel a task
taskqueue tasks cancel task-uuid-here
```

## API Reference

### TaskQueueClient

Main synchronous client for Task Queue operations.

#### Methods

- `create_task(name, command, project_id, **kwargs)` - Create a new task
- `get_task(task_id)` - Get task details
- `list_tasks(filters=None)` - List tasks with optional filters
- `update_task(task_id, **updates)` - Update task properties
- `cancel_task(task_id)` - Cancel a running task
- `delete_task(task_id)` - Delete a task
- `create_project(name, description=None)` - Create a new project
- `get_project(project_id)` - Get project details
- `list_projects()` - List all projects

### AsyncTaskQueueClient

Asynchronous version of the client with the same methods but async.

## Error Handling

The SDK provides custom exceptions for different error scenarios:

```python
from taskqueue import TaskQueueError, TaskNotFoundError, ValidationError

try:
    task = client.create_task(name="", command="echo hello")
except ValidationError as e:
    print(f"Validation failed: {e}")
except TaskQueueError as e:
    print(f"API error: {e}")
```

## Configuration

Configure the client with custom settings:

```python
client = TaskQueueClient(
    base_url="https://api.taskqueue.dev",
    timeout=30.0,
    retry_attempts=3,
    retry_delay=1.0,
    headers={"Authorization": "Bearer your-token"}
)
```

## Webhook Support

Handle real-time notifications:

```python
from taskqueue import WebhookHandler

handler = WebhookHandler(secret="your-webhook-secret")

@handler.on_task_completed
def handle_completion(task_id, result):
    print(f"Task {task_id} completed with result: {result}")

# Start webhook server
handler.run(port=8081)
```

## Development

```bash
# Clone the repository
git clone https://github.com/taskqueue/taskqueue-sdk-python
cd taskqueue-sdk-python

# Install development dependencies
pip install -e ".[dev]"

# Run tests
pytest

# Build documentation
mkdocs build
```

## Contributing

Contributions are welcome! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
