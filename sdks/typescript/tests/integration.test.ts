import { TaskQueueClient } from '../src/client';
import { TaskSchema, ProjectSchema } from '../src/models/task';

// Integration test configuration
const TEST_CONFIG = {
  baseUrl: process.env.TASK_QUEUE_URL || 'http://localhost:16080',
  apiKey: process.env.TASK_QUEUE_API_KEY || 'test-api-key',
};

describe('TaskQueueClient Integration Tests', () => {
  let client: TaskQueueClient;
  let testProjectId: string;
  let testTaskId: string;

  beforeAll(() => {
    client = new TaskQueueClient(TEST_CONFIG);
  });

  describe('Project Operations', () => {
    it('should create a test project', async () => {
      const projectData = {
        name: `Integration Test Project ${Date.now()}`,
        description: 'Project for integration testing',
        tags: ['integration', 'test'],
      };

      const project = await client.createProject(projectData);
      expect(ProjectSchema.parse(project)).toBeDefined();
      expect(project.name).toBe(projectData.name);
      expect(project.description).toBe(projectData.description);
      
      testProjectId = project.id;
    });

    it('should get the created project', async () => {
      const project = await client.getProject(testProjectId);
      expect(project.id).toBe(testProjectId);
      expect(project.name).toContain('Integration Test Project');
    });

    it('should list projects', async () => {
      const projects = await client.listProjects({ limit: 10 });
      expect(Array.isArray(projects)).toBe(true);
      expect(projects.length).toBeGreaterThan(0);
      
      const testProject = projects.find(p => p.id === testProjectId);
      expect(testProject).toBeDefined();
    });
  });

  describe('Task Operations', () => {
    it('should create a test task', async () => {
      const taskData = {
        name: `Integration Test Task ${Date.now()}`,
        command: 'echo "Hello from integration test"',
        project_id: testProjectId,
        priority: 'Normal' as const,
        description: 'Task for integration testing',
      };

      const task = await client.createTask(taskData);
      expect(TaskSchema.parse(task)).toBeDefined();
      expect(task.name).toBe(taskData.name);
      expect(task.command).toBe(taskData.command);
      expect(task.project_id).toBe(testProjectId);
      
      testTaskId = task.id;
    });

    it('should get the created task', async () => {
      const task = await client.getTask(testTaskId);
      expect(task.id).toBe(testTaskId);
      expect(task.name).toContain('Integration Test Task');
    });

    it('should list tasks', async () => {
      const tasks = await client.listTasks({ 
        project_id: testProjectId,
        limit: 10 
      });
      expect(Array.isArray(tasks)).toBe(true);
      expect(tasks.length).toBeGreaterThan(0);
      
      const testTask = tasks.find(t => t.id === testTaskId);
      expect(testTask).toBeDefined();
    });

    it('should update the task', async () => {
      const updates = {
        description: 'Updated description for integration test',
        priority: 'High' as const,
      };

      const updatedTask = await client.updateTask(testTaskId, updates);
      expect(updatedTask.description).toBe(updates.description);
      expect(updatedTask.priority).toBe(updates.priority);
    });

    it('should cancel the task', async () => {
      const cancelledTask = await client.cancelTask(testTaskId, 'Integration test cancellation');
      expect(cancelledTask.status).toBe('Cancelled');
    });
  });

  describe('Utility Operations', () => {
    it('should perform health check', async () => {
      const health = await client.healthCheck();
      expect(health).toBeDefined();
      expect(health.status).toBeDefined();
    });

    it('should get server metrics', async () => {
      const metrics = await client.getMetrics();
      expect(metrics).toBeDefined();
      expect(typeof metrics.tasks_total).toBe('number');
    });

    it('should get server status', async () => {
      const status = await client.getServerStatus();
      expect(status).toBeDefined();
      expect(status.status).toBeDefined();
    });
  });

  describe('Error Handling', () => {
    it('should handle non-existent task', async () => {
      const nonExistentId = '123e4567-e89b-12d3-a456-426614174999';
      
      await expect(client.getTask(nonExistentId)).rejects.toThrow();
    });

    it('should handle non-existent project', async () => {
      const nonExistentId = '123e4567-e89b-12d3-a456-426614174999';
      
      await expect(client.getProject(nonExistentId)).rejects.toThrow();
    });

    it('should handle invalid task data', async () => {
      const invalidTaskData = {
        name: '', // Invalid: empty name
        command: '', // Invalid: empty command
        project_id: 'invalid-uuid', // Invalid: not a UUID
        priority: 'InvalidPriority' as any, // Invalid: not a valid priority
      };

      await expect(client.createTask(invalidTaskData)).rejects.toThrow();
    });
  });

  describe('Cleanup', () => {
    it('should delete the test task', async () => {
      await client.deleteTask(testTaskId);
      
      // Verify task is deleted
      await expect(client.getTask(testTaskId)).rejects.toThrow();
    });

    it('should delete the test project', async () => {
      await client.deleteProject(testProjectId);
      
      // Verify project is deleted
      await expect(client.getProject(testProjectId)).rejects.toThrow();
    });
  });
});

describe('TaskQueueClient Performance Tests', () => {
  let client: TaskQueueClient;
  let testProjectId: string;

  beforeAll(async () => {
    client = new TaskQueueClient(TEST_CONFIG);
    
    // Create a test project for performance tests
    const project = await client.createProject({
      name: `Performance Test Project ${Date.now()}`,
      description: 'Project for performance testing',
    });
    testProjectId = project.id;
  });

  afterAll(async () => {
    // Cleanup test project
    try {
      await client.deleteProject(testProjectId);
    } catch (error) {
      // Ignore cleanup errors
    }
  });

  it('should handle multiple concurrent task creations', async () => {
    const taskPromises = Array.from({ length: 5 }, (_, i) => 
      client.createTask({
        name: `Concurrent Task ${i}`,
        command: `echo "Task ${i}"`,
        project_id: testProjectId,
        priority: 'Normal' as const,
      })
    );

    const tasks = await Promise.all(taskPromises);
    expect(tasks).toHaveLength(5);
    
    // Cleanup created tasks
    await Promise.all(tasks.map(task => client.deleteTask(task.id)));
  });

  it('should handle multiple concurrent task retrievals', async () => {
    // Create a test task first
    const task = await client.createTask({
      name: 'Performance Test Task',
      command: 'echo "Performance test"',
      project_id: testProjectId,
      priority: 'Normal' as const,
    });

    // Perform multiple concurrent retrievals
    const retrievalPromises = Array.from({ length: 10 }, () => 
      client.getTask(task.id)
    );

    const retrievedTasks = await Promise.all(retrievalPromises);
    expect(retrievedTasks).toHaveLength(10);
    
    // All retrieved tasks should be identical
    retrievedTasks.forEach(retrievedTask => {
      expect(retrievedTask.id).toBe(task.id);
      expect(retrievedTask.name).toBe(task.name);
    });

    // Cleanup
    await client.deleteTask(task.id);
  });

  it('should handle large task lists efficiently', async () => {
    const startTime = Date.now();
    
    const tasks = await client.listTasks({ 
      project_id: testProjectId,
      limit: 100 
    });
    
    const endTime = Date.now();
    const duration = endTime - startTime;
    
    expect(Array.isArray(tasks)).toBe(true);
    expect(duration).toBeLessThan(5000); // Should complete within 5 seconds
  });
});
