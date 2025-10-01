import { TaskQueueClient } from '../src/client';
import { Task, Project } from '../src/models/task';

// Performance test configuration
const PERFORMANCE_CONFIG = {
  baseUrl: process.env.TASK_QUEUE_URL || 'http://localhost:16080',
  apiKey: process.env.TASK_QUEUE_API_KEY || 'test-api-key',
  timeout: 30000, // 30 seconds timeout for performance tests
};

describe('TaskQueueClient Performance Tests', () => {
  let client: TaskQueueClient;
  let testProjectId: string;
  let testTasks: string[] = [];

  beforeAll(async () => {
    client = new TaskQueueClient(PERFORMANCE_CONFIG);
    
    // Create a test project for performance tests
    const project = await client.createProject({
      name: `Performance Test Project ${Date.now()}`,
      description: 'Project for performance testing',
    });
    testProjectId = project.id;
  });

  afterAll(async () => {
    // Cleanup test data
    try {
      await Promise.all(testTasks.map(taskId => client.deleteTask(taskId)));
      await client.deleteProject(testProjectId);
    } catch (error) {
      // Ignore cleanup errors
    }
  });

  describe('Concurrent Operations', () => {
    it('should handle multiple concurrent task creations efficiently', async () => {
      const startTime = Date.now();
      const taskCount = 10;
      
      const taskPromises = Array.from({ length: taskCount }, (_, i) => 
        client.createTask({
          name: `Concurrent Task ${i}`,
          command: `echo "Task ${i}"`,
          project_id: testProjectId,
          priority: 'Normal' as const,
        })
      );

      const tasks = await Promise.all(taskPromises);
      const endTime = Date.now();
      const duration = endTime - startTime;
      
      expect(tasks).toHaveLength(taskCount);
      expect(duration).toBeLessThan(10000); // Should complete within 10 seconds
      
      // Store task IDs for cleanup
      testTasks.push(...tasks.map(task => task.id));
    }, 30000);

    it('should handle multiple concurrent task retrievals efficiently', async () => {
      // Create a test task first
      const task = await client.createTask({
        name: 'Performance Test Task',
        command: 'echo "Performance test"',
        project_id: testProjectId,
        priority: 'Normal' as const,
      });
      testTasks.push(task.id);

      const startTime = Date.now();
      const retrievalCount = 20;
      
      // Perform multiple concurrent retrievals
      const retrievalPromises = Array.from({ length: retrievalCount }, () => 
        client.getTask(task.id)
      );

      const retrievedTasks = await Promise.all(retrievalPromises);
      const endTime = Date.now();
      const duration = endTime - startTime;
      
      expect(retrievedTasks).toHaveLength(retrievalCount);
      expect(duration).toBeLessThan(5000); // Should complete within 5 seconds
      
      // All retrieved tasks should be identical
      retrievedTasks.forEach(retrievedTask => {
        expect(retrievedTask.id).toBe(task.id);
        expect(retrievedTask.name).toBe(task.name);
      });
    }, 30000);

    it('should handle mixed concurrent operations efficiently', async () => {
      const startTime = Date.now();
      const operationCount = 15;
      
      const operations = Array.from({ length: operationCount }, (_, i) => {
        if (i % 3 === 0) {
          // Create task
          return client.createTask({
            name: `Mixed Operation Task ${i}`,
            command: `echo "Mixed task ${i}"`,
            project_id: testProjectId,
            priority: 'Normal' as const,
          });
        } else if (i % 3 === 1) {
          // List tasks
          return client.listTasks({ project_id: testProjectId, limit: 5 });
        } else {
          // Health check
          return client.healthCheck();
        }
      });

      const results = await Promise.all(operations);
      const endTime = Date.now();
      const duration = endTime - startTime;
      
      expect(results).toHaveLength(operationCount);
      expect(duration).toBeLessThan(15000); // Should complete within 15 seconds
      
      // Store created task IDs for cleanup
      results.forEach((result, index) => {
        if (index % 3 === 0 && typeof result === 'object' && 'id' in result) {
          testTasks.push((result as Task).id);
        }
      });
    }, 30000);
  });

  describe('Large Data Handling', () => {
    it('should handle large task lists efficiently', async () => {
      // Create multiple tasks first
      const taskPromises = Array.from({ length: 20 }, (_, i) => 
        client.createTask({
          name: `Large List Task ${i}`,
          command: `echo "Large list task ${i}"`,
          project_id: testProjectId,
          priority: 'Normal' as const,
        })
      );

      const createdTasks = await Promise.all(taskPromises);
      testTasks.push(...createdTasks.map(task => task.id));

      const startTime = Date.now();
      
      const tasks = await client.listTasks({ 
        project_id: testProjectId,
        limit: 100 
      });
      
      const endTime = Date.now();
      const duration = endTime - startTime;
      
      expect(Array.isArray(tasks)).toBe(true);
      expect(tasks.length).toBeGreaterThanOrEqual(20);
      expect(duration).toBeLessThan(5000); // Should complete within 5 seconds
    }, 30000);

    it('should handle large project lists efficiently', async () => {
      // Create multiple projects first
      const projectPromises = Array.from({ length: 10 }, (_, i) => 
        client.createProject({
          name: `Large List Project ${i}`,
          description: `Project for large list testing ${i}`,
        })
      );

      const createdProjects = await Promise.all(projectPromises);

      const startTime = Date.now();
      
      const projects = await client.listProjects({ limit: 50 });
      
      const endTime = Date.now();
      const duration = endTime - startTime;
      
      expect(Array.isArray(projects)).toBe(true);
      expect(projects.length).toBeGreaterThanOrEqual(10);
      expect(duration).toBeLessThan(3000); // Should complete within 3 seconds
      
      // Cleanup created projects
      await Promise.all(createdProjects.map(project => client.deleteProject(project.id)));
    }, 30000);
  });

  describe('Memory Usage', () => {
    it('should not leak memory during repeated operations', async () => {
      const iterations = 50;
      const initialMemory = process.memoryUsage();
      
      for (let i = 0; i < iterations; i++) {
        // Perform various operations
        await client.healthCheck();
        await client.listTasks({ limit: 10 });
        await client.listProjects({ limit: 10 });
        
        // Force garbage collection if available
        if (global.gc) {
          global.gc();
        }
      }
      
      const finalMemory = process.memoryUsage();
      const memoryIncrease = finalMemory.heapUsed - initialMemory.heapUsed;
      
      // Memory increase should be reasonable (less than 50MB)
      expect(memoryIncrease).toBeLessThan(50 * 1024 * 1024);
    }, 60000);
  });

  describe('Response Time Benchmarks', () => {
    it('should meet response time benchmarks for common operations', async () => {
      const benchmarks = {
        healthCheck: 1000, // 1 second
        listTasks: 2000,  // 2 seconds
        listProjects: 2000, // 2 seconds
        createTask: 3000, // 3 seconds
        createProject: 3000, // 3 seconds
      };

      // Test health check
      const healthStart = Date.now();
      await client.healthCheck();
      const healthDuration = Date.now() - healthStart;
      expect(healthDuration).toBeLessThan(benchmarks.healthCheck);

      // Test list operations
      const listTasksStart = Date.now();
      await client.listTasks({ limit: 10 });
      const listTasksDuration = Date.now() - listTasksStart;
      expect(listTasksDuration).toBeLessThan(benchmarks.listTasks);

      const listProjectsStart = Date.now();
      await client.listProjects({ limit: 10 });
      const listProjectsDuration = Date.now() - listProjectsStart;
      expect(listProjectsDuration).toBeLessThan(benchmarks.listProjects);

      // Test create operations
      const createTaskStart = Date.now();
      const task = await client.createTask({
        name: 'Benchmark Task',
        command: 'echo "benchmark"',
        project_id: testProjectId,
        priority: 'Normal' as const,
      });
      const createTaskDuration = Date.now() - createTaskStart;
      expect(createTaskDuration).toBeLessThan(benchmarks.createTask);
      testTasks.push(task.id);

      const createProjectStart = Date.now();
      const project = await client.createProject({
        name: 'Benchmark Project',
        description: 'Project for benchmarking',
      });
      const createProjectDuration = Date.now() - createProjectStart;
      expect(createProjectDuration).toBeLessThan(benchmarks.createProject);
      
      // Cleanup
      await client.deleteProject(project.id);
    }, 30000);
  });

  describe('Error Recovery Performance', () => {
    it('should recover quickly from network errors', async () => {
      // Create a client with very short timeout to simulate network issues
      const timeoutClient = new TaskQueueClient({
        ...PERFORMANCE_CONFIG,
        timeout: 100, // 100ms timeout
        retries: 3,
        retryDelay: 50,
      });

      const startTime = Date.now();
      
      try {
        await timeoutClient.healthCheck();
      } catch (error) {
        // Expected to fail due to timeout
      }
      
      const endTime = Date.now();
      const duration = endTime - startTime;
      
      // Should fail quickly (within 1 second including retries)
      expect(duration).toBeLessThan(1000);
    }, 10000);
  });
});
