import { TaskQueueClient } from '../src/client';
import { TaskSchema, ProjectSchema, CreateTaskRequestSchema } from '../src/models/task';
import { CreateProjectRequestSchema } from '../src/models/project';
import { ClientError, ValidationError, NetworkError } from '../src/utils/error-handling';

// Mock fetch for testing
const mockFetch = jest.fn();
global.fetch = mockFetch;

describe('TaskQueueClient', () => {
  let client: TaskQueueClient;

  beforeEach(() => {
    client = new TaskQueueClient({
      baseUrl: 'http://localhost:16080',
      apiKey: 'test-api-key',
    });
    mockFetch.mockClear();
  });

  describe('Configuration', () => {
    it('should initialize with default configuration', () => {
      const defaultClient = new TaskQueueClient({
        baseUrl: 'http://localhost:16080',
      });
      expect(defaultClient).toBeDefined();
    });

    it('should initialize with custom configuration', () => {
      const customClient = new TaskQueueClient({
        baseUrl: 'http://localhost:16080',
        apiKey: 'custom-key',
        timeout: 60000,
        retries: 5,
        retryDelay: 2000,
      });
      expect(customClient).toBeDefined();
    });
  });

  describe('Task Operations', () => {
    const mockTask = {
      id: '123e4567-e89b-12d3-a456-426614174000',
      name: 'Test Task',
      command: 'echo hello',
      project_id: '123e4567-e89b-12d3-a456-426614174001',
      priority: 'Normal',
      status: 'Pending',
      dependencies: [],
      created_at: '2023-01-01T00:00:00Z',
      updated_at: '2023-01-01T00:00:00Z',
    };

    it('should create a task successfully', async () => {
      mockFetch.mockResolvedValueOnce({
        ok: true,
        json: () => Promise.resolve(mockTask),
        headers: { get: () => 'application/json' },
      });

      const taskData = {
        name: 'Test Task',
        command: 'echo hello',
        project_id: '123e4567-e89b-12d3-a456-426614174001',
        priority: 'Normal' as const,
      };

      const task = await client.createTask(taskData);

      expect(TaskSchema.parse(task)).toBeDefined();
      expect(task.name).toBe('Test Task');
      expect(task.command).toBe('echo hello');
      expect(mockFetch).toHaveBeenCalledWith(
        'http://localhost:16080/api/tasks',
        expect.objectContaining({
          method: 'POST',
          headers: expect.objectContaining({
            'Authorization': 'Bearer test-api-key',
            'Content-Type': 'application/json',
          }),
          body: JSON.stringify(taskData),
        })
      );
    });

    it('should get a task by ID', async () => {
      mockFetch.mockResolvedValueOnce({
        ok: true,
        json: () => Promise.resolve(mockTask),
        headers: { get: () => 'application/json' },
      });

      const task = await client.getTask(mockTask.id);

      expect(TaskSchema.parse(task)).toBeDefined();
      expect(task.id).toBe(mockTask.id);
      expect(mockFetch).toHaveBeenCalledWith(
        `http://localhost:16080/api/tasks/${mockTask.id}`,
        expect.objectContaining({
          method: 'GET',
        })
      );
    });

    it('should list tasks with filters', async () => {
      const mockTasks = [mockTask];
      mockFetch.mockResolvedValueOnce({
        ok: true,
        json: () => Promise.resolve(mockTasks),
        headers: { get: () => 'application/json' },
      });

      const filters = {
        status: 'Pending' as const,
        priority: 'Normal' as const,
        limit: 10,
        offset: 0,
      };

      const tasks = await client.listTasks(filters);

      expect(Array.isArray(tasks)).toBe(true);
      expect(tasks.length).toBe(1);
      expect(tasks[0].name).toBe('Test Task');
      expect(mockFetch).toHaveBeenCalledWith(
        'http://localhost:16080/api/tasks?status=Pending&priority=Normal&limit=10&offset=0',
        expect.objectContaining({
          method: 'GET',
        })
      );
    });

    it('should update a task', async () => {
      const updatedTask = { ...mockTask, description: 'Updated description' };
      mockFetch.mockResolvedValueOnce({
        ok: true,
        json: () => Promise.resolve(updatedTask),
        headers: { get: () => 'application/json' },
      });

      const updates = { description: 'Updated description' };
      const task = await client.updateTask(mockTask.id, updates);

      expect(task.description).toBe('Updated description');
      expect(mockFetch).toHaveBeenCalledWith(
        `http://localhost:16080/api/tasks/${mockTask.id}`,
        expect.objectContaining({
          method: 'PUT',
          body: JSON.stringify(updates),
        })
      );
    });

    it('should cancel a task', async () => {
      const cancelledTask = { ...mockTask, status: 'Cancelled' };
      mockFetch.mockResolvedValueOnce({
        ok: true,
        json: () => Promise.resolve(cancelledTask),
        headers: { get: () => 'application/json' },
      });

      const reason = 'Test cancellation';
      const task = await client.cancelTask(mockTask.id, reason);

      expect(task.status).toBe('Cancelled');
      expect(mockFetch).toHaveBeenCalledWith(
        `http://localhost:16080/api/tasks/${mockTask.id}/cancel`,
        expect.objectContaining({
          method: 'POST',
          body: JSON.stringify({ reason }),
        })
      );
    });

    it('should delete a task', async () => {
      mockFetch.mockResolvedValueOnce({
        ok: true,
        json: () => Promise.resolve({}),
        headers: { get: () => 'application/json' },
      });

      await client.deleteTask(mockTask.id);

      expect(mockFetch).toHaveBeenCalledWith(
        `http://localhost:16080/api/tasks/${mockTask.id}`,
        expect.objectContaining({
          method: 'DELETE',
        })
      );
    });
  });

  describe('Project Operations', () => {
    const mockProject = {
      id: '123e4567-e89b-12d3-a456-426614174001',
      name: 'Test Project',
      description: 'Test project description',
      status: 'Active',
      created_at: '2023-01-01T00:00:00Z',
      updated_at: '2023-01-01T00:00:00Z',
      tags: ['test'],
    };

    it('should create a project', async () => {
      mockFetch.mockResolvedValueOnce({
        ok: true,
        json: () => Promise.resolve(mockProject),
        headers: { get: () => 'application/json' },
      });

      const projectData = {
        name: 'Test Project',
        description: 'Test project description',
        tags: ['test'],
      };

      const project = await client.createProject(projectData);

      expect(ProjectSchema.parse(project)).toBeDefined();
      expect(project.name).toBe('Test Project');
      expect(mockFetch).toHaveBeenCalledWith(
        'http://localhost:16080/api/projects',
        expect.objectContaining({
          method: 'POST',
          body: JSON.stringify(projectData),
        })
      );
    });

    it('should get a project by ID', async () => {
      mockFetch.mockResolvedValueOnce({
        ok: true,
        json: () => Promise.resolve(mockProject),
        headers: { get: () => 'application/json' },
      });

      const project = await client.getProject(mockProject.id);

      expect(ProjectSchema.parse(project)).toBeDefined();
      expect(project.id).toBe(mockProject.id);
    });

    it('should list projects with filters', async () => {
      const mockProjects = [mockProject];
      mockFetch.mockResolvedValueOnce({
        ok: true,
        json: () => Promise.resolve(mockProjects),
        headers: { get: () => 'application/json' },
      });

      const filters = {
        status: 'Active' as const,
        tags: ['test'],
        limit: 10,
        offset: 0,
      };

      const projects = await client.listProjects(filters);

      expect(Array.isArray(projects)).toBe(true);
      expect(projects.length).toBe(1);
      expect(projects[0].name).toBe('Test Project');
    });
  });

  describe('Error Handling', () => {
    it('should handle HTTP errors', async () => {
      mockFetch.mockResolvedValueOnce({
        ok: false,
        status: 404,
        statusText: 'Not Found',
        text: () => Promise.resolve('Task not found'),
      });

      await expect(client.getTask('invalid-id')).rejects.toThrow(ClientError);
    });

    it('should handle network errors', async () => {
      mockFetch.mockRejectedValueOnce(new Error('Network error'));

      await expect(client.getTask('test-id')).rejects.toThrow(ClientError);
    });

    it('should handle timeout errors', async () => {
      mockFetch.mockImplementationOnce(() => 
        new Promise((_, reject) => {
          setTimeout(() => reject(new Error('Timeout')), 100);
        })
      );

      await expect(client.getTask('test-id')).rejects.toThrow();
    });
  });

  describe('Utility Methods', () => {
    it('should perform health check', async () => {
      const mockHealth = { 
        status: 'healthy', 
        timestamp: '2023-01-01T00:00:00Z',
        version: '1.0.0',
      };

      mockFetch.mockResolvedValueOnce({
        ok: true,
        json: () => Promise.resolve(mockHealth),
        headers: { get: () => 'application/json' },
      });

      const health = await client.healthCheck();

      expect(health.status).toBe('healthy');
      expect(mockFetch).toHaveBeenCalledWith(
        'http://localhost:16080/health',
        expect.objectContaining({
          method: 'GET',
        })
      );
    });

    it('should get server metrics', async () => {
      const mockMetrics = {
        tasks_total: 100,
        tasks_pending: 10,
        tasks_running: 5,
        tasks_completed: 80,
        tasks_failed: 5,
      };

      mockFetch.mockResolvedValueOnce({
        ok: true,
        json: () => Promise.resolve(mockMetrics),
        headers: { get: () => 'application/json' },
      });

      const metrics = await client.getMetrics();

      expect(metrics.tasks_total).toBe(100);
      expect(mockFetch).toHaveBeenCalledWith(
        'http://localhost:16080/metrics',
        expect.objectContaining({
          method: 'GET',
        })
      );
    });

    it('should get server status', async () => {
      const mockStatus = {
        status: 'running',
        uptime: 3600,
        memory_usage: 0.5,
        cpu_usage: 0.2,
      };

      mockFetch.mockResolvedValueOnce({
        ok: true,
        json: () => Promise.resolve(mockStatus),
        headers: { get: () => 'application/json' },
      });

      const status = await client.getServerStatus();

      expect(status.status).toBe('running');
      expect(mockFetch).toHaveBeenCalledWith(
        'http://localhost:16080/api/status',
        expect.objectContaining({
          method: 'GET',
        })
      );
    });
  });

  describe('Request Configuration', () => {
    it('should include API key in headers when provided', async () => {
      mockFetch.mockResolvedValueOnce({
        ok: true,
        json: () => Promise.resolve({}),
        headers: { get: () => 'application/json' },
      });

      await client.healthCheck();

      expect(mockFetch).toHaveBeenCalledWith(
        expect.any(String),
        expect.objectContaining({
          headers: expect.objectContaining({
            'Authorization': 'Bearer test-api-key',
          }),
        })
      );
    });

    it('should handle requests without API key', async () => {
      const clientWithoutKey = new TaskQueueClient({
        baseUrl: 'http://localhost:16080',
      });

      mockFetch.mockResolvedValueOnce({
        ok: true,
        json: () => Promise.resolve({}),
        headers: { get: () => 'application/json' },
      });

      await clientWithoutKey.healthCheck();

      expect(mockFetch).toHaveBeenCalledWith(
        expect.any(String),
        expect.objectContaining({
          headers: expect.not.objectContaining({
            'Authorization': expect.any(String),
          }),
        })
      );
    });

    it('should handle custom headers', async () => {
      const customClient = new TaskQueueClient({
        baseUrl: 'http://localhost:16080',
        apiKey: 'test-api-key',
        headers: {
          'X-Custom-Header': 'custom-value',
          'User-Agent': 'TaskQueueSDK/1.0.0',
        },
      });

      mockFetch.mockResolvedValueOnce({
        ok: true,
        json: () => Promise.resolve({}),
        headers: { get: () => 'application/json' },
      });

      await customClient.healthCheck();

      expect(mockFetch).toHaveBeenCalledWith(
        expect.any(String),
        expect.objectContaining({
          headers: expect.objectContaining({
            'Authorization': 'Bearer test-api-key',
            'X-Custom-Header': 'custom-value',
            'User-Agent': 'TaskQueueSDK/1.0.0',
          }),
        })
      );
    });

    it('should handle timeout configuration', async () => {
      const timeoutClient = new TaskQueueClient({
        baseUrl: 'http://localhost:16080',
        timeout: 5000,
      });

      mockFetch.mockImplementationOnce(() => 
        new Promise((_, reject) => {
          setTimeout(() => reject(new Error('Timeout')), 100);
        })
      );

      await expect(timeoutClient.healthCheck()).rejects.toThrow();
    });

    it('should handle retry configuration', async () => {
      const retryClient = new TaskQueueClient({
        baseUrl: 'http://localhost:16080',
        retries: 2,
        retryDelay: 100,
      });

      mockFetch
        .mockRejectedValueOnce(new Error('Network error'))
        .mockRejectedValueOnce(new Error('Network error'))
        .mockResolvedValueOnce({
          ok: true,
          json: () => Promise.resolve({}),
          headers: { get: () => 'application/json' },
        });

      await retryClient.healthCheck();

      expect(mockFetch).toHaveBeenCalledTimes(3);
    });
  });

  describe('Advanced Error Handling', () => {
    it('should handle 400 Bad Request', async () => {
      mockFetch.mockResolvedValueOnce({
        ok: false,
        status: 400,
        statusText: 'Bad Request',
        json: () => Promise.resolve({ error: 'Invalid request data' }),
      });

      await expect(client.createTask({
        name: '',
        command: '',
        project_id: 'invalid-id',
      })).rejects.toThrow(ClientError);
    });

    it('should handle 401 Unauthorized', async () => {
      mockFetch.mockResolvedValueOnce({
        ok: false,
        status: 401,
        statusText: 'Unauthorized',
        json: () => Promise.resolve({ error: 'Invalid API key' }),
      });

      await expect(client.getTask('test-id')).rejects.toThrow(ClientError);
    });

    it('should handle 403 Forbidden', async () => {
      mockFetch.mockResolvedValueOnce({
        ok: false,
        status: 403,
        statusText: 'Forbidden',
        json: () => Promise.resolve({ error: 'Access denied' }),
      });

      await expect(client.deleteTask('test-id')).rejects.toThrow(ClientError);
    });

    it('should handle 429 Too Many Requests', async () => {
      mockFetch.mockResolvedValueOnce({
        ok: false,
        status: 429,
        statusText: 'Too Many Requests',
        headers: { get: (name: string) => name === 'Retry-After' ? '60' : null },
        json: () => Promise.resolve({ error: 'Rate limit exceeded' }),
      });

      await expect(client.listTasks()).rejects.toThrow(ClientError);
    });

    it('should handle 500 Internal Server Error', async () => {
      mockFetch.mockResolvedValueOnce({
        ok: false,
        status: 500,
        statusText: 'Internal Server Error',
        json: () => Promise.resolve({ error: 'Internal server error' }),
      });

      await expect(client.healthCheck()).rejects.toThrow(ClientError);
    });

    it('should handle 502 Bad Gateway', async () => {
      mockFetch.mockResolvedValueOnce({
        ok: false,
        status: 502,
        statusText: 'Bad Gateway',
        json: () => Promise.resolve({ error: 'Bad gateway' }),
      });

      await expect(client.getMetrics()).rejects.toThrow(ClientError);
    });

    it('should handle 503 Service Unavailable', async () => {
      mockFetch.mockResolvedValueOnce({
        ok: false,
        status: 503,
        statusText: 'Service Unavailable',
        json: () => Promise.resolve({ error: 'Service unavailable' }),
      });

      await expect(client.getServerStatus()).rejects.toThrow(ClientError);
    });

    it('should handle malformed JSON response', async () => {
      mockFetch.mockResolvedValueOnce({
        ok: true,
        json: () => Promise.reject(new Error('Invalid JSON')),
        headers: { get: () => 'application/json' },
      });

      await expect(client.healthCheck()).rejects.toThrow();
    });

    it('should handle empty response body', async () => {
      mockFetch.mockResolvedValueOnce({
        ok: true,
        json: () => Promise.resolve(null),
        headers: { get: () => 'application/json' },
      });

      const result = await client.healthCheck();
      expect(result).toBeNull();
    });
  });

  describe('Concurrent Operations', () => {
    it('should handle multiple concurrent requests', async () => {
      const mockResponse = {
        ok: true,
        json: () => Promise.resolve({}),
        headers: { get: () => 'application/json' },
      };

      mockFetch.mockResolvedValue(mockResponse);

      const promises = [
        client.healthCheck(),
        client.getMetrics(),
        client.getServerStatus(),
        client.listTasks(),
        client.listProjects(),
      ];

      const results = await Promise.all(promises);
      expect(results).toHaveLength(5);
      expect(mockFetch).toHaveBeenCalledTimes(5);
    });

    it('should handle concurrent task operations', async () => {
      const mockTask = {
        id: '123e4567-e89b-12d3-a456-426614174000',
        name: 'Test Task',
        command: 'echo hello',
        project_id: '123e4567-e89b-12d3-a456-426614174001',
        priority: 'Normal',
        status: 'Pending',
        dependencies: [],
        created_at: '2023-01-01T00:00:00Z',
        updated_at: '2023-01-01T00:00:00Z',
      };

      mockFetch.mockResolvedValue({
        ok: true,
        json: () => Promise.resolve(mockTask),
        headers: { get: () => 'application/json' },
      });

      const promises = [
        client.createTask({
          name: 'Task 1',
          command: 'echo task1',
          project_id: '123e4567-e89b-12d3-a456-426614174001',
        }),
        client.createTask({
          name: 'Task 2',
          command: 'echo task2',
          project_id: '123e4567-e89b-12d3-a456-426614174001',
        }),
        client.createTask({
          name: 'Task 3',
          command: 'echo task3',
          project_id: '123e4567-e89b-12d3-a456-426614174001',
        }),
      ];

      const results = await Promise.all(promises);
      expect(results).toHaveLength(3);
      expect(mockFetch).toHaveBeenCalledTimes(3);
    });
  });

  describe('Data Validation', () => {
    it('should validate task data before sending', async () => {
      mockFetch.mockResolvedValueOnce({
        ok: true,
        json: () => Promise.resolve({}),
        headers: { get: () => 'application/json' },
      });

      await expect(client.createTask({
        name: '', // Invalid empty name
        command: 'echo hello',
        project_id: '123e4567-e89b-12d3-a456-426614174001',
      })).rejects.toThrow(ValidationError);
    });

    it('should validate project data before sending', async () => {
      mockFetch.mockResolvedValueOnce({
        ok: true,
        json: () => Promise.resolve({}),
        headers: { get: () => 'application/json' },
      });

      await expect(client.createProject({
        name: '', // Invalid empty name
      })).rejects.toThrow(ValidationError);
    });

    it('should validate UUID format in requests', async () => {
      await expect(client.getTask('invalid-uuid')).rejects.toThrow(ValidationError);
      await expect(client.getProject('invalid-uuid')).rejects.toThrow(ValidationError);
      await expect(client.updateTask('invalid-uuid', {})).rejects.toThrow(ValidationError);
      await expect(client.deleteTask('invalid-uuid')).rejects.toThrow(ValidationError);
    });
  });

  describe('Response Processing', () => {
    it('should process successful responses correctly', async () => {
      const mockTask = {
        id: '123e4567-e89b-12d3-a456-426614174000',
        name: 'Test Task',
        command: 'echo hello',
        project_id: '123e4567-e89b-12d3-a456-426614174001',
        priority: 'Normal',
        status: 'Pending',
        dependencies: [],
        created_at: '2023-01-01T00:00:00Z',
        updated_at: '2023-01-01T00:00:00Z',
      };

      mockFetch.mockResolvedValueOnce({
        ok: true,
        json: () => Promise.resolve(mockTask),
        headers: { get: () => 'application/json' },
      });

      const task = await client.createTask({
        name: 'Test Task',
        command: 'echo hello',
        project_id: '123e4567-e89b-12d3-a456-426614174001',
      });

      expect(task).toEqual(mockTask);
      expect(TaskSchema.parse(task)).toBeDefined();
    });

    it('should handle array responses', async () => {
      const mockTasks = [
        {
          id: '123e4567-e89b-12d3-a456-426614174000',
          name: 'Task 1',
          command: 'echo task1',
          project_id: '123e4567-e89b-12d3-a456-426614174001',
          priority: 'Normal',
          status: 'Pending',
          dependencies: [],
          created_at: '2023-01-01T00:00:00Z',
          updated_at: '2023-01-01T00:00:00Z',
        },
        {
          id: '123e4567-e89b-12d3-a456-426614174001',
          name: 'Task 2',
          command: 'echo task2',
          project_id: '123e4567-e89b-12d3-a456-426614174001',
          priority: 'High',
          status: 'Running',
          dependencies: [],
          created_at: '2023-01-01T00:00:00Z',
          updated_at: '2023-01-01T00:00:00Z',
        },
      ];

      mockFetch.mockResolvedValueOnce({
        ok: true,
        json: () => Promise.resolve(mockTasks),
        headers: { get: () => 'application/json' },
      });

      const tasks = await client.listTasks();
      expect(Array.isArray(tasks)).toBe(true);
      expect(tasks).toHaveLength(2);
      expect(tasks[0].name).toBe('Task 1');
      expect(tasks[1].name).toBe('Task 2');
    });

    it('should handle empty array responses', async () => {
      mockFetch.mockResolvedValueOnce({
        ok: true,
        json: () => Promise.resolve([]),
        headers: { get: () => 'application/json' },
      });

      const tasks = await client.listTasks();
      expect(Array.isArray(tasks)).toBe(true);
      expect(tasks).toHaveLength(0);
    });
  });
});