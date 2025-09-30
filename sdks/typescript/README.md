# TypeScript SDK for Task Queue

A modern, type-safe TypeScript/JavaScript SDK for the Task Queue REST API. Built with async-first design, comprehensive validation, and excellent developer experience.

## Features

- 🚀 **Async-First Design** - Promise-based API with full async/await support
- 🔒 **Type Safety** - Complete TypeScript definitions with Zod validation
- 🌐 **Cross-Platform** - Works in Node.js, browsers, and modern JavaScript environments
- 📦 **Zero Dependencies** - Minimal footprint with only essential dependencies
- 🛡️ **Error Handling** - Comprehensive error handling with custom error types
- 🔄 **Retry Logic** - Built-in retry mechanisms for network resilience
- 📊 **Validation** - Runtime validation with Zod schemas
- 🧪 **Well Tested** - Comprehensive test suite with unit and integration tests

## Installation

```bash
npm install @taskqueue/typescript-sdk
```

## Quick Start

```typescript
import { TaskQueueClient } from '@taskqueue/typescript-sdk';

// Initialize the client
const client = new TaskQueueClient({
  baseUrl: 'http://localhost:16080',
  apiKey: 'your-api-key', // Optional
});

// Create a project
const project = await client.createProject({
  name: 'My Project',
  description: 'A sample project',
  tags: ['example', 'demo'],
});

// Create a task
const task = await client.createTask({
  name: 'Hello World Task',
  command: 'echo "Hello, World!"',
  project_id: project.id,
  priority: 'Normal',
});

// Get task status
const taskStatus = await client.getTask(task.id);
console.log(`Task status: ${taskStatus.status}`);

// List tasks
const tasks = await client.listTasks({
  project_id: project.id,
  status: 'Pending',
  limit: 10,
});
```

## API Reference

### TaskQueueClient

The main client class for interacting with the Task Queue API.

#### Constructor

```typescript
new TaskQueueClient(config: ClientConfig)
```

**ClientConfig:**
- `baseUrl: string` - Base URL of the Task Queue API
- `apiKey?: string` - API key for authentication (optional)
- `timeout?: number` - Request timeout in milliseconds (default: 30000)
- `retries?: number` - Number of retry attempts (default: 3)
- `retryDelay?: number` - Delay between retries in milliseconds (default: 1000)

#### Task Operations

```typescript
// Create a task
const task = await client.createTask({
  name: string,
  command: string,
  project_id: string,
  priority?: 'Low' | 'Normal' | 'High' | 'Critical',
  description?: string,
  dependencies?: TaskDependency[],
  metadata?: Record<string, any>,
});

// Get a task by ID
const task = await client.getTask(taskId: string);

// List tasks with filters
const tasks = await client.listTasks({
  status?: 'Pending' | 'Running' | 'Completed' | 'Failed' | 'Cancelled',
  priority?: 'Low' | 'Normal' | 'High' | 'Critical',
  project_id?: string,
  limit?: number,
  offset?: number,
});

// Update a task
const updatedTask = await client.updateTask(taskId: string, updates: Partial<Task>);

// Cancel a task
const cancelledTask = await client.cancelTask(taskId: string, reason?: string);

// Delete a task
await client.deleteTask(taskId: string);
```

#### Project Operations

```typescript
// Create a project
const project = await client.createProject({
  name: string,
  description?: string,
  tags?: string[],
  metadata?: Record<string, any>,
});

// Get a project by ID
const project = await client.getProject(projectId: string);

// List projects with filters
const projects = await client.listProjects({
  status?: 'Planning' | 'Active' | 'Completed' | 'Archived',
  tags?: string[],
  limit?: number,
  offset?: number,
});

// Update a project
const updatedProject = await client.updateProject(projectId: string, updates: Partial<Project>);

// Delete a project
await client.deleteProject(projectId: string);
```

#### Utility Operations

```typescript
// Health check
const health = await client.healthCheck();

// Get server metrics
const metrics = await client.getMetrics();

// Get server status
const status = await client.getServerStatus();
```

## Data Models

### Task

```typescript
interface Task {
  id: string;
  name: string;
  command: string;
  project_id: string;
  priority: 'Low' | 'Normal' | 'High' | 'Critical';
  status: 'Pending' | 'Running' | 'Completed' | 'Failed' | 'Cancelled';
  description?: string;
  dependencies: TaskDependency[];
  metadata?: Record<string, any>;
  created_at: string;
  updated_at: string;
  started_at?: string;
  completed_at?: string;
  result?: TaskResult;
  metrics?: TaskMetrics;
}
```

### Project

```typescript
interface Project {
  id: string;
  name: string;
  description?: string;
  status: 'Planning' | 'Active' | 'Completed' | 'Archived';
  tags: string[];
  metadata?: Record<string, any>;
  created_at: string;
  updated_at: string;
}
```

### TaskDependency

```typescript
interface TaskDependency {
  task_id: string;
  condition: 'Success' | 'Failure' | 'Completion';
}
```

### TaskResult

```typescript
interface TaskResult {
  type: 'success' | 'failure' | 'cancelled';
  output?: string;
  error?: string;
  exit_code?: number;
  logs?: string[];
  artifacts?: string[];
  metrics?: TaskMetrics;
  reason?: string;
}
```

## Error Handling

The SDK provides comprehensive error handling with custom error types:

```typescript
import { 
  ClientError, 
  ValidationError, 
  NetworkError,
  NotFoundError,
  UnauthorizedError,
  ForbiddenError,
  ConflictError,
  ServerError 
} from '@taskqueue/typescript-sdk';

try {
  const task = await client.createTask(taskData);
} catch (error) {
  if (error instanceof ValidationError) {
    console.error('Validation failed:', error.message);
    console.error('Field:', error.field);
  } else if (error instanceof NetworkError) {
    console.error('Network error:', error.message);
    console.error('Original error:', error.originalError);
  } else if (error instanceof ClientError) {
    console.error('Client error:', error.message);
    console.error('Status:', error.status);
    console.error('Code:', error.code);
  }
}
```

## Validation

All data is validated using Zod schemas for runtime type safety:

```typescript
import { TaskSchema, ProjectSchema } from '@taskqueue/typescript-sdk';

// Validate task data
const result = TaskSchema.safeParse(taskData);
if (result.success) {
  const task = result.data; // Fully typed and validated
} else {
  console.error('Validation errors:', result.error.errors);
}
```

## Configuration

### Environment Variables

You can configure the client using environment variables:

```bash
TASK_QUEUE_URL=http://localhost:16080
TASK_QUEUE_API_KEY=your-api-key
```

```typescript
const client = new TaskQueueClient({
  baseUrl: process.env.TASK_QUEUE_URL || 'http://localhost:16080',
  apiKey: process.env.TASK_QUEUE_API_KEY,
});
```

### Custom Configuration

```typescript
const client = new TaskQueueClient({
  baseUrl: 'https://api.taskqueue.com',
  apiKey: 'your-api-key',
  timeout: 60000, // 60 seconds
  retries: 5,
  retryDelay: 2000, // 2 seconds
});
```

## Examples

### Basic Task Management

```typescript
import { TaskQueueClient } from '@taskqueue/typescript-sdk';

const client = new TaskQueueClient({
  baseUrl: 'http://localhost:16080',
});

async function manageTasks() {
  // Create a project
  const project = await client.createProject({
    name: 'Data Processing',
    description: 'Process large datasets',
    tags: ['data', 'processing'],
  });

  // Create multiple tasks
  const tasks = await Promise.all([
    client.createTask({
      name: 'Download Data',
      command: 'wget https://example.com/data.csv',
      project_id: project.id,
      priority: 'High',
    }),
    client.createTask({
      name: 'Process Data',
      command: 'python process.py data.csv',
      project_id: project.id,
      priority: 'Normal',
      dependencies: [{ task_id: 'download-task-id', condition: 'Success' }],
    }),
  ]);

  // Monitor task progress
  for (const task of tasks) {
    const status = await client.getTask(task.id);
    console.log(`${task.name}: ${status.status}`);
  }
}
```

### Error Handling Example

```typescript
async function robustTaskCreation() {
  try {
    const task = await client.createTask({
      name: 'Robust Task',
      command: 'python script.py',
      project_id: 'valid-project-id',
    });
    
    console.log('Task created successfully:', task.id);
  } catch (error) {
    if (error instanceof ValidationError) {
      console.error('Invalid task data:', error.message);
    } else if (error instanceof NetworkError) {
      console.error('Network issue:', error.message);
      // Retry logic could be implemented here
    } else {
      console.error('Unexpected error:', error);
    }
  }
}
```

## Testing

The SDK includes comprehensive tests:

```bash
# Run all tests
npm test

# Run tests in watch mode
npm run test:watch

# Run tests with coverage
npm run test:coverage

# Run integration tests
npm run test:integration
```

## Building

```bash
# Build the SDK
npm run build

# Build in watch mode
npm run build:watch

# Type check
npm run type-check
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Run the test suite
6. Submit a pull request

## License

MIT License - see LICENSE file for details.

## Support

- 📖 [Documentation](https://github.com/taskqueue/typescript-sdk#readme)
- 🐛 [Issue Tracker](https://github.com/taskqueue/typescript-sdk/issues)
- 💬 [Discussions](https://github.com/taskqueue/typescript-sdk/discussions)