# TypeScript/JavaScript SDK - Technical Specification

## Overview

This document outlines the comprehensive implementation of a TypeScript/JavaScript SDK for the Task Queue system, providing both Node.js and browser compatibility with modern Promise-based API, full TypeScript definitions, React/Vue components, and WebSocket client support.

## Architecture Decision Records (ADRs)

### ADR 1: TypeScript-First Design
**Decision**: Implement TypeScript as primary language with JavaScript compatibility
**Rationale**: Better type safety, modern development experience, automatic IntelliSense
**Alternatives Considered**: JavaScript-first with TypeScript definitions
**Impact**: All code written in TypeScript, compiled to JavaScript for distribution

### ADR 2: Multi-Target Build System
**Decision**: Support both Node.js and browser environments with separate builds
**Rationale**: Different environments have different requirements and dependencies
**Alternatives Considered**: Single build with runtime detection
**Impact**: Separate build configurations for Node.js and browser targets

### ADR 3: Fetch API with Polyfills
**Decision**: Use native fetch API with polyfills for older browsers
**Rationale**: Modern standard, better performance than XMLHttpRequest
**Alternatives Considered**: Axios, XMLHttpRequest
**Impact**: Smaller bundle size, better tree-shaking, polyfill for IE11+

### ADR 4: Zod for Runtime Validation
**Decision**: Use Zod for runtime type validation and schema generation
**Rationale**: TypeScript-first validation, excellent error messages, schema inference
**Alternatives Considered**: Joi, Yup, io-ts
**Impact**: Type-safe validation with automatic TypeScript type generation

### ADR 5: Rollup for Bundling
**Decision**: Use Rollup for module bundling with multiple output formats
**Rationale**: Better tree-shaking, smaller bundles, multiple output formats
**Alternatives Considered**: Webpack, esbuild, Parcel
**Impact**: Optimized bundles for different environments

## Technical Architecture

### Project Structure

```
sdks/typescript/
├── src/
│   ├── client/
│   │   ├── base-client.ts          # Base client class
│   │   ├── node-client.ts           # Node.js specific client
│   │   ├── browser-client.ts        # Browser specific client
│   │   └── websocket-client.ts      # WebSocket client
│   ├── models/
│   │   ├── task.ts                  # Task models and schemas
│   │   ├── project.ts                # Project models and schemas
│   │   ├── workflow.ts               # Workflow models and schemas
│   │   └── common.ts                 # Common types and enums
│   ├── components/
│   │   ├── react/                   # React components
│   │   │   ├── TaskList.tsx
│   │   │   ├── ProjectDashboard.tsx
│   │   │   └── WorkflowVisualizer.tsx
│   │   └── vue/                     # Vue components
│   │       ├── TaskList.vue
│   │       ├── ProjectDashboard.vue
│   │       └── WorkflowVisualizer.vue
│   ├── utils/
│   │   ├── validation.ts            # Validation utilities
│   │   ├── serialization.ts         # Serialization helpers
│   │   └── error-handling.ts        # Error handling utilities
│   └── index.ts                     # Main entry point
├── dist/                            # Compiled output
├── tests/                           # Test files
├── examples/                        # Usage examples
├── docs/                           # Documentation
├── package.json
├── tsconfig.json
├── rollup.config.js
└── README.md
```

### Core Components

#### 1. Base Client Class

```typescript
// src/client/base-client.ts
import { z } from 'zod';
import { Task, Project, Workflow } from '../models';

export interface ClientConfig {
  baseUrl: string;
  apiKey?: string;
  timeout?: number;
  retries?: number;
  retryDelay?: number;
}

export abstract class BaseClient {
  protected config: ClientConfig;
  protected fetch: typeof fetch;

  constructor(config: ClientConfig, fetchImpl?: typeof fetch) {
    this.config = {
      timeout: 30000,
      retries: 3,
      retryDelay: 1000,
      ...config,
    };
    this.fetch = fetchImpl || fetch;
  }

  protected async request<T>(
    endpoint: string,
    options: RequestInit = {}
  ): Promise<T> {
    const url = `${this.config.baseUrl}${endpoint}`;
    const headers = {
      'Content-Type': 'application/json',
      ...(this.config.apiKey && { 'Authorization': `Bearer ${this.config.apiKey}` }),
      ...options.headers,
    };

    const controller = new AbortController();
    const timeoutId = setTimeout(() => controller.abort(), this.config.timeout);

    try {
      const response = await this.fetch(url, {
        ...options,
        headers,
        signal: controller.signal,
      });

      clearTimeout(timeoutId);

      if (!response.ok) {
        throw new ClientError(`HTTP ${response.status}: ${response.statusText}`);
      }

      return await response.json();
    } catch (error) {
      clearTimeout(timeoutId);
      throw error;
    }
  }

  // Abstract methods to be implemented by subclasses
  abstract createTask(task: CreateTaskRequest): Promise<Task>;
  abstract getTask(id: string): Promise<Task>;
  abstract listTasks(filters?: TaskFilters): Promise<Task[]>;
  abstract updateTask(id: string, updates: Partial<Task>): Promise<Task>;
  abstract deleteTask(id: string): Promise<void>;
  abstract cancelTask(id: string, reason?: string): Promise<Task>;
}
```

#### 2. Node.js Client

```typescript
// src/client/node-client.ts
import { BaseClient } from './base-client';
import { Task, Project, Workflow } from '../models';
import { WebSocket } from 'ws';

export class NodeClient extends BaseClient {
  private ws?: WebSocket;

  constructor(config: ClientConfig) {
    super(config);
  }

  async createTask(task: CreateTaskRequest): Promise<Task> {
    return this.request<Task>('/api/tasks', {
      method: 'POST',
      body: JSON.stringify(task),
    });
  }

  async getTask(id: string): Promise<Task> {
    return this.request<Task>(`/api/tasks/${id}`);
  }

  async listTasks(filters?: TaskFilters): Promise<Task[]> {
    const params = new URLSearchParams();
    if (filters) {
      Object.entries(filters).forEach(([key, value]) => {
        if (value !== undefined) {
          params.append(key, String(value));
        }
      });
    }
    
    return this.request<Task[]>(`/api/tasks?${params.toString()}`);
  }

  async updateTask(id: string, updates: Partial<Task>): Promise<Task> {
    return this.request<Task>(`/api/tasks/${id}`, {
      method: 'PUT',
      body: JSON.stringify(updates),
    });
  }

  async deleteTask(id: string): Promise<void> {
    await this.request<void>(`/api/tasks/${id}`, {
      method: 'DELETE',
    });
  }

  async cancelTask(id: string, reason?: string): Promise<Task> {
    return this.request<Task>(`/api/tasks/${id}/cancel`, {
      method: 'POST',
      body: JSON.stringify({ reason }),
    });
  }

  // WebSocket support for Node.js
  connectWebSocket(): Promise<void> {
    return new Promise((resolve, reject) => {
      const wsUrl = this.config.baseUrl.replace('http', 'ws') + '/ws';
      this.ws = new WebSocket(wsUrl);

      this.ws.on('open', () => resolve());
      this.ws.on('error', reject);
    });
  }

  disconnectWebSocket(): void {
    this.ws?.close();
  }
}
```

#### 3. Browser Client

```typescript
// src/client/browser-client.ts
import { BaseClient } from './base-client';
import { Task, Project, Workflow } from '../models';

export class BrowserClient extends BaseClient {
  private ws?: WebSocket;

  constructor(config: ClientConfig) {
    super(config);
  }

  async createTask(task: CreateTaskRequest): Promise<Task> {
    return this.request<Task>('/api/tasks', {
      method: 'POST',
      body: JSON.stringify(task),
    });
  }

  async getTask(id: string): Promise<Task> {
    return this.request<Task>(`/api/tasks/${id}`);
  }

  async listTasks(filters?: TaskFilters): Promise<Task[]> {
    const params = new URLSearchParams();
    if (filters) {
      Object.entries(filters).forEach(([key, value]) => {
        if (value !== undefined) {
          params.append(key, String(value));
        }
      });
    }
    
    return this.request<Task[]>(`/api/tasks?${params.toString()}`);
  }

  async updateTask(id: string, updates: Partial<Task>): Promise<Task> {
    return this.request<Task>(`/api/tasks/${id}`, {
      method: 'PUT',
      body: JSON.stringify(updates),
    });
  }

  async deleteTask(id: string): Promise<void> {
    await this.request<void>(`/api/tasks/${id}`, {
      method: 'DELETE',
    });
  }

  async cancelTask(id: string, reason?: string): Promise<Task> {
    return this.request<Task>(`/api/tasks/${id}/cancel`, {
      method: 'POST',
      body: JSON.stringify({ reason }),
    });
  }

  // WebSocket support for browser
  connectWebSocket(): Promise<void> {
    return new Promise((resolve, reject) => {
      const wsUrl = this.config.baseUrl.replace('http', 'ws') + '/ws';
      this.ws = new WebSocket(wsUrl);

      this.ws.onopen = () => resolve();
      this.ws.onerror = reject;
    });
  }

  disconnectWebSocket(): void {
    this.ws?.close();
  }
}
```

#### 4. Data Models with Zod Schemas

```typescript
// src/models/task.ts
import { z } from 'zod';

export const TaskStatusSchema = z.enum([
  'Planning',
  'Implementation',
  'TestCreation',
  'Testing',
  'AIReview',
  'Finalized',
  'Pending',
  'Running',
  'Completed',
  'Failed',
  'Cancelled',
  'WaitingForDependencies',
]);

export const TaskPrioritySchema = z.enum(['Low', 'Normal', 'High', 'Critical']);

export const DependencyConditionSchema = z.enum(['Success', 'Failure', 'Completion']);

export const TaskDependencySchema = z.object({
  task_id: z.string().uuid(),
  condition: DependencyConditionSchema,
});

export const TaskMetricsSchema = z.object({
  execution_time: z.number(),
  memory_usage: z.number(),
  cpu_usage: z.number(),
  disk_usage: z.number(),
  network_io: z.number(),
});

export const TaskResultSchema = z.discriminatedUnion('type', [
  z.object({
    type: z.literal('success'),
    output: z.string(),
    artifacts: z.array(z.string()),
    metrics: TaskMetricsSchema,
  }),
  z.object({
    type: z.literal('failure'),
    error: z.string(),
    exit_code: z.number().optional(),
    logs: z.array(z.string()),
  }),
  z.object({
    type: z.literal('cancelled'),
    reason: z.string(),
  }),
]);

export const TaskSchema = z.object({
  id: z.string().uuid(),
  name: z.string(),
  command: z.string(),
  description: z.string().optional(),
  project_id: z.string().uuid(),
  priority: TaskPrioritySchema,
  status: TaskStatusSchema,
  dependencies: z.array(TaskDependencySchema),
  result: TaskResultSchema.optional(),
  created_at: z.string().datetime(),
  updated_at: z.string().datetime(),
  started_at: z.string().datetime().optional(),
  completed_at: z.string().datetime().optional(),
  metadata: z.record(z.any()).optional(),
});

export const CreateTaskRequestSchema = z.object({
  name: z.string().min(1),
  command: z.string().min(1),
  description: z.string().optional(),
  project_id: z.string().uuid(),
  priority: TaskPrioritySchema.default('Normal'),
  dependencies: z.array(TaskDependencySchema).default([]),
  metadata: z.record(z.any()).optional(),
});

export const TaskFiltersSchema = z.object({
  status: TaskStatusSchema.optional(),
  priority: TaskPrioritySchema.optional(),
  project_id: z.string().uuid().optional(),
  limit: z.number().min(1).max(1000).default(50),
  offset: z.number().min(0).default(0),
});

// TypeScript types inferred from schemas
export type TaskStatus = z.infer<typeof TaskStatusSchema>;
export type TaskPriority = z.infer<typeof TaskPrioritySchema>;
export type DependencyCondition = z.infer<typeof DependencyConditionSchema>;
export type TaskDependency = z.infer<typeof TaskDependencySchema>;
export type TaskMetrics = z.infer<typeof TaskMetricsSchema>;
export type TaskResult = z.infer<typeof TaskResultSchema>;
export type Task = z.infer<typeof TaskSchema>;
export type CreateTaskRequest = z.infer<typeof CreateTaskRequestSchema>;
export type TaskFilters = z.infer<typeof TaskFiltersSchema>;
```

#### 5. React Components

```typescript
// src/components/react/TaskList.tsx
import React, { useState, useEffect } from 'react';
import { Task, TaskFilters, BrowserClient } from '../../models';

interface TaskListProps {
  client: BrowserClient;
  filters?: TaskFilters;
  onTaskSelect?: (task: Task) => void;
}

export const TaskList: React.FC<TaskListProps> = ({
  client,
  filters = {},
  onTaskSelect,
}) => {
  const [tasks, setTasks] = useState<Task[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const loadTasks = async () => {
      try {
        setLoading(true);
        setError(null);
        const taskList = await client.listTasks(filters);
        setTasks(taskList);
      } catch (err) {
        setError(err instanceof Error ? err.message : 'Failed to load tasks');
      } finally {
        setLoading(false);
      }
    };

    loadTasks();
  }, [client, filters]);

  const handleTaskClick = (task: Task) => {
    onTaskSelect?.(task);
  };

  if (loading) {
    return <div className="task-list-loading">Loading tasks...</div>;
  }

  if (error) {
    return <div className="task-list-error">Error: {error}</div>;
  }

  return (
    <div className="task-list">
      <h3>Tasks ({tasks.length})</h3>
      <div className="task-list-items">
        {tasks.map((task) => (
          <div
            key={task.id}
            className={`task-item task-item--${task.status.toLowerCase()}`}
            onClick={() => handleTaskClick(task)}
          >
            <div className="task-item-header">
              <span className="task-item-name">{task.name}</span>
              <span className={`task-item-priority task-item-priority--${task.priority.toLowerCase()}`}>
                {task.priority}
              </span>
            </div>
            <div className="task-item-status">
              Status: {task.status}
            </div>
            {task.description && (
              <div className="task-item-description">
                {task.description}
              </div>
            )}
          </div>
        ))}
      </div>
    </div>
  );
};
```

#### 6. Vue Components

```vue
<!-- src/components/vue/TaskList.vue -->
<template>
  <div class="task-list">
    <h3>Tasks ({{ tasks.length }})</h3>
    <div v-if="loading" class="task-list-loading">
      Loading tasks...
    </div>
    <div v-else-if="error" class="task-list-error">
      Error: {{ error }}
    </div>
    <div v-else class="task-list-items">
      <div
        v-for="task in tasks"
        :key="task.id"
        :class="[
          'task-item',
          `task-item--${task.status.toLowerCase()}`
        ]"
        @click="handleTaskClick(task)"
      >
        <div class="task-item-header">
          <span class="task-item-name">{{ task.name }}</span>
          <span :class="[
            'task-item-priority',
            `task-item-priority--${task.priority.toLowerCase()}`
          ]">
            {{ task.priority }}
          </span>
        </div>
        <div class="task-item-status">
          Status: {{ task.status }}
        </div>
        <div v-if="task.description" class="task-item-description">
          {{ task.description }}
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch } from 'vue';
import { Task, TaskFilters, BrowserClient } from '../../models';

interface Props {
  client: BrowserClient;
  filters?: TaskFilters;
}

interface Emits {
  (e: 'task-select', task: Task): void;
}

const props = defineProps<Props>();
const emit = defineEmits<Emits>();

const tasks = ref<Task[]>([]);
const loading = ref(true);
const error = ref<string | null>(null);

const loadTasks = async () => {
  try {
    loading.value = true;
    error.value = null;
    const taskList = await props.client.listTasks(props.filters);
    tasks.value = taskList;
  } catch (err) {
    error.value = err instanceof Error ? err.message : 'Failed to load tasks';
  } finally {
    loading.value = false;
  }
};

const handleTaskClick = (task: Task) => {
  emit('task-select', task);
};

onMounted(() => {
  loadTasks();
});

watch(() => props.filters, () => {
  loadTasks();
}, { deep: true });
</script>
```

### Build Configuration

#### Package.json

```json
{
  "name": "@taskqueue/sdk",
  "version": "1.0.0",
  "description": "TypeScript/JavaScript SDK for Task Queue",
  "main": "dist/node/index.js",
  "module": "dist/browser/index.esm.js",
  "browser": "dist/browser/index.umd.js",
  "types": "dist/index.d.ts",
  "files": [
    "dist",
    "README.md"
  ],
  "scripts": {
    "build": "rollup -c",
    "build:watch": "rollup -c -w",
    "test": "jest",
    "test:watch": "jest --watch",
    "test:coverage": "jest --coverage",
    "lint": "eslint src/**/*.ts",
    "lint:fix": "eslint src/**/*.ts --fix",
    "type-check": "tsc --noEmit",
    "prepare": "npm run build"
  },
  "keywords": [
    "task-queue",
    "typescript",
    "javascript",
    "sdk",
    "api-client"
  ],
  "author": "Task Queue Team",
  "license": "MIT",
  "dependencies": {
    "zod": "^3.22.0"
  },
  "devDependencies": {
    "@rollup/plugin-commonjs": "^25.0.0",
    "@rollup/plugin-node-resolve": "^15.0.0",
    "@rollup/plugin-typescript": "^11.0.0",
    "@types/jest": "^29.5.0",
    "@types/node": "^20.0.0",
    "@types/react": "^18.0.0",
    "@types/react-dom": "^18.0.0",
    "@typescript-eslint/eslint-plugin": "^6.0.0",
    "@typescript-eslint/parser": "^6.0.0",
    "eslint": "^8.0.0",
    "jest": "^29.5.0",
    "react": "^18.0.0",
    "react-dom": "^18.0.0",
    "rollup": "^3.0.0",
    "ts-jest": "^29.1.0",
    "tslib": "^2.6.0",
    "typescript": "^5.0.0",
    "vue": "^3.3.0",
    "ws": "^8.14.0"
  },
  "peerDependencies": {
    "react": ">=16.8.0",
    "vue": ">=3.0.0"
  },
  "peerDependenciesMeta": {
    "react": {
      "optional": true
    },
    "vue": {
      "optional": true
    }
  }
}
```

#### Rollup Configuration

```javascript
// rollup.config.js
import resolve from '@rollup/plugin-node-resolve';
import commonjs from '@rollup/plugin-commonjs';
import typescript from '@rollup/plugin-typescript';
import { defineConfig } from 'rollup';

const baseConfig = {
  input: 'src/index.ts',
  plugins: [
    resolve({
      browser: true,
      preferBuiltins: false,
    }),
    commonjs(),
    typescript({
      tsconfig: './tsconfig.json',
    }),
  ],
  external: ['react', 'vue', 'ws'],
};

export default defineConfig([
  // Node.js build
  {
    ...baseConfig,
    output: {
      file: 'dist/node/index.js',
      format: 'cjs',
      sourcemap: true,
    },
    external: [...baseConfig.external, 'ws'],
  },
  // Browser ESM build
  {
    ...baseConfig,
    output: {
      file: 'dist/browser/index.esm.js',
      format: 'esm',
      sourcemap: true,
    },
    external: [...baseConfig.external, 'ws'],
  },
  // Browser UMD build
  {
    ...baseConfig,
    output: {
      file: 'dist/browser/index.umd.js',
      format: 'umd',
      name: 'TaskQueueSDK',
      sourcemap: true,
    },
    external: [...baseConfig.external, 'ws'],
  },
]);
```

### Error Handling

```typescript
// src/utils/error-handling.ts
export class ClientError extends Error {
  constructor(
    message: string,
    public status?: number,
    public code?: string
  ) {
    super(message);
    this.name = 'ClientError';
  }
}

export class ValidationError extends Error {
  constructor(message: string, public field?: string) {
    super(message);
    this.name = 'ValidationError';
  }
}

export class NetworkError extends Error {
  constructor(message: string, public originalError?: Error) {
    super(message);
    this.name = 'NetworkError';
  }
}

export function handleError(error: unknown): never {
  if (error instanceof ClientError) {
    throw error;
  }
  
  if (error instanceof Error) {
    throw new NetworkError(error.message, error);
  }
  
  throw new NetworkError('Unknown error occurred');
}
```

### Usage Examples

#### Node.js Usage

```typescript
// examples/node-usage.ts
import { NodeClient } from '@taskqueue/sdk';

async function main() {
  const client = new NodeClient({
    baseUrl: 'http://localhost:16080',
    apiKey: 'your-api-key',
  });

  try {
    // Create a task
    const task = await client.createTask({
      name: 'Process Data',
      command: 'python process.py',
      project_id: 'your-project-id',
      priority: 'High',
    });

    console.log('Created task:', task);

    // List tasks
    const tasks = await client.listTasks({
      status: 'Pending',
      limit: 10,
    });

    console.log('Tasks:', tasks);

    // Connect to WebSocket for real-time updates
    await client.connectWebSocket();
    console.log('Connected to WebSocket');

  } catch (error) {
    console.error('Error:', error);
  }
}

main();
```

#### Browser Usage

```typescript
// examples/browser-usage.ts
import { BrowserClient } from '@taskqueue/sdk';

async function main() {
  const client = new BrowserClient({
    baseUrl: 'http://localhost:16080',
    apiKey: 'your-api-key',
  });

  try {
    // Create a task
    const task = await client.createTask({
      name: 'Process Data',
      command: 'python process.py',
      project_id: 'your-project-id',
      priority: 'High',
    });

    console.log('Created task:', task);

    // Connect to WebSocket for real-time updates
    await client.connectWebSocket();
    console.log('Connected to WebSocket');

  } catch (error) {
    console.error('Error:', error);
  }
}

main();
```

#### React Usage

```tsx
// examples/react-usage.tsx
import React, { useEffect, useState } from 'react';
import { BrowserClient, Task } from '@taskqueue/sdk';
import { TaskList } from '@taskqueue/sdk/react';

function App() {
  const [client] = useState(() => new BrowserClient({
    baseUrl: 'http://localhost:16080',
    apiKey: 'your-api-key',
  }));

  const [selectedTask, setSelectedTask] = useState<Task | null>(null);

  useEffect(() => {
    // Connect to WebSocket on component mount
    client.connectWebSocket().catch(console.error);

    return () => {
      client.disconnectWebSocket();
    };
  }, [client]);

  return (
    <div className="app">
      <h1>Task Queue Dashboard</h1>
      <TaskList
        client={client}
        onTaskSelect={setSelectedTask}
      />
      {selectedTask && (
        <div className="task-details">
          <h3>Task Details</h3>
          <pre>{JSON.stringify(selectedTask, null, 2)}</pre>
        </div>
      )}
    </div>
  );
}

export default App;
```

#### Vue Usage

```vue
<!-- examples/vue-usage.vue -->
<template>
  <div class="app">
    <h1>Task Queue Dashboard</h1>
    <TaskList
      :client="client"
      @task-select="setSelectedTask"
    />
    <div v-if="selectedTask" class="task-details">
      <h3>Task Details</h3>
      <pre>{{ JSON.stringify(selectedTask, null, 2) }}</pre>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import { BrowserClient, Task } from '@taskqueue/sdk';
import { TaskList } from '@taskqueue/sdk/vue';

const client = new BrowserClient({
  baseUrl: 'http://localhost:16080',
  apiKey: 'your-api-key',
});

const selectedTask = ref<Task | null>(null);

const setSelectedTask = (task: Task) => {
  selectedTask.value = task;
};

onMounted(async () => {
  try {
    await client.connectWebSocket();
  } catch (error) {
    console.error('Failed to connect to WebSocket:', error);
  }
});

onUnmounted(() => {
  client.disconnectWebSocket();
});
</script>
```

## Testing Strategy

### Unit Tests

```typescript
// tests/client.test.ts
import { NodeClient, BrowserClient } from '../src/client';
import { TaskSchema } from '../src/models';

describe('Client Tests', () => {
  let client: NodeClient;

  beforeEach(() => {
    client = new NodeClient({
      baseUrl: 'http://localhost:16080',
    });
  });

  describe('Task Operations', () => {
    it('should create a task', async () => {
      const taskData = {
        name: 'Test Task',
        command: 'echo hello',
        project_id: 'test-project-id',
        priority: 'Normal' as const,
      };

      const task = await client.createTask(taskData);
      
      expect(TaskSchema.parse(task)).toBeDefined();
      expect(task.name).toBe(taskData.name);
      expect(task.command).toBe(taskData.command);
    });

    it('should list tasks', async () => {
      const tasks = await client.listTasks();
      
      expect(Array.isArray(tasks)).toBe(true);
      tasks.forEach(task => {
        expect(TaskSchema.parse(task)).toBeDefined();
      });
    });

    it('should handle errors gracefully', async () => {
      const invalidClient = new NodeClient({
        baseUrl: 'http://invalid-url',
      });

      await expect(invalidClient.listTasks()).rejects.toThrow();
    });
  });
});
```

### Integration Tests

```typescript
// tests/integration.test.ts
import { NodeClient } from '../src/client';

describe('Integration Tests', () => {
  let client: NodeClient;

  beforeAll(() => {
    client = new NodeClient({
      baseUrl: 'http://localhost:16080',
    });
  });

  it('should perform complete task lifecycle', async () => {
    // Create task
    const task = await client.createTask({
      name: 'Integration Test Task',
      command: 'echo integration test',
      project_id: 'test-project-id',
    });

    expect(task.id).toBeDefined();

    // Get task
    const retrievedTask = await client.getTask(task.id);
    expect(retrievedTask.id).toBe(task.id);

    // Update task
    const updatedTask = await client.updateTask(task.id, {
      description: 'Updated description',
    });
    expect(updatedTask.description).toBe('Updated description');

    // Cancel task
    const cancelledTask = await client.cancelTask(task.id, 'Test cancellation');
    expect(cancelledTask.status).toBe('Cancelled');

    // Delete task
    await client.deleteTask(task.id);
  });
});
```

## Performance Considerations

### Bundle Size Optimization

- Tree-shaking enabled for unused code elimination
- Separate builds for Node.js and browser environments
- Minimal dependencies (only Zod for validation)
- WebSocket support as optional dependency

### Memory Management

- Proper cleanup of WebSocket connections
- Request timeout handling
- Retry logic with exponential backoff
- Connection pooling for HTTP requests

### Browser Compatibility

- Fetch API polyfill for older browsers
- WebSocket polyfill for IE11+
- ES5/ES6 builds for different browser targets
- TypeScript compilation to ES5 for maximum compatibility

## Security Considerations

### API Key Management

- Secure storage of API keys
- Environment variable support
- No hardcoded credentials in code

### Request Validation

- Input validation using Zod schemas
- Output validation for API responses
- Type safety throughout the application

### Error Handling

- No sensitive information in error messages
- Proper error logging without exposing internals
- Graceful degradation on network failures

## Deployment and Distribution

### NPM Package

- Published to npm registry as `@taskqueue/sdk`
- Semantic versioning (semver)
- Proper package.json configuration
- TypeScript definitions included

### CDN Distribution

- UMD build for CDN usage
- Minified and gzipped versions
- Version-specific URLs for caching

### Documentation

- Comprehensive README with examples
- API documentation with TypeScript
- Component documentation for React/Vue
- Migration guides for version updates

## Future Enhancements

### Planned Features

1. **Offline Support**: Local storage and sync capabilities
2. **Real-time Subscriptions**: WebSocket event subscriptions
3. **Batch Operations**: Bulk task operations
4. **Plugin System**: Extensible client architecture
5. **Advanced Caching**: Intelligent response caching
6. **Metrics Collection**: Performance and usage metrics

### Versioning Strategy

- Major versions for breaking changes
- Minor versions for new features
- Patch versions for bug fixes
- Deprecation warnings for removed features
- Migration guides for major version updates

This technical specification provides a comprehensive foundation for implementing the TypeScript/JavaScript SDK with modern best practices, excellent developer experience, and robust functionality across both Node.js and browser environments.
