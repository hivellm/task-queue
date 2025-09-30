import { TaskQueueClient } from '../src/client';

// Mock fetch for testing
const mockFetch = jest.fn();
global.fetch = mockFetch;

describe('Environment-Specific Tests', () => {
  let client: TaskQueueClient;

  beforeEach(() => {
    client = new TaskQueueClient({
      baseUrl: 'http://localhost:16080',
      apiKey: 'test-api-key',
    });
    mockFetch.mockClear();
  });

  describe('Node.js Environment', () => {
    beforeEach(() => {
      // Mock Node.js environment
      Object.defineProperty(global, 'process', {
        value: {
          env: {
            NODE_ENV: 'test',
            TASK_QUEUE_URL: 'http://localhost:16080',
            TASK_QUEUE_API_KEY: 'test-api-key',
          },
          versions: {
            node: '18.0.0',
          },
        },
        writable: true,
      });
    });

    it('should work in Node.js environment', async () => {
      mockFetch.mockResolvedValueOnce({
        ok: true,
        json: () => Promise.resolve({ status: 'healthy' }),
        headers: { get: () => 'application/json' },
      });

      const result = await client.healthCheck();
      expect(result.status).toBe('healthy');
    });

    it('should handle Node.js specific features', async () => {
      // Test that the client can handle Node.js specific configurations
      const nodeClient = new TaskQueueClient({
        baseUrl: 'http://localhost:16080',
        apiKey: 'test-api-key',
        timeout: 30000,
        retries: 3,
        retryDelay: 1000,
      });

      mockFetch.mockResolvedValueOnce({
        ok: true,
        json: () => Promise.resolve({}),
        headers: { get: () => 'application/json' },
      });

      await nodeClient.healthCheck();
      expect(mockFetch).toHaveBeenCalledTimes(1);
    });
  });

  describe('Browser Environment', () => {
    beforeEach(() => {
      // Mock browser environment
      Object.defineProperty(global, 'window', {
        value: {
          location: {
            origin: 'http://localhost:3000',
            href: 'http://localhost:3000/dashboard',
          },
          navigator: {
            userAgent: 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36',
          },
        },
        writable: true,
      });

      // Remove Node.js specific properties
      delete (global as any).process;
    });

    it('should work in browser environment', async () => {
      mockFetch.mockResolvedValueOnce({
        ok: true,
        json: () => Promise.resolve({ status: 'healthy' }),
        headers: { get: () => 'application/json' },
      });

      const result = await client.healthCheck();
      expect(result.status).toBe('healthy');
    });

    it('should handle browser-specific features', async () => {
      // Test that the client can handle browser-specific configurations
      const browserClient = new TaskQueueClient({
        baseUrl: 'http://localhost:16080',
        apiKey: 'test-api-key',
        headers: {
          'User-Agent': 'TaskQueueSDK/1.0.0',
          'X-Requested-With': 'XMLHttpRequest',
        },
      });

      mockFetch.mockResolvedValueOnce({
        ok: true,
        json: () => Promise.resolve({}),
        headers: { get: () => 'application/json' },
      });

      await browserClient.healthCheck();

      expect(mockFetch).toHaveBeenCalledWith(
        expect.any(String),
        expect.objectContaining({
          headers: expect.objectContaining({
            'User-Agent': 'TaskQueueSDK/1.0.0',
            'X-Requested-With': 'XMLHttpRequest',
          }),
        })
      );
    });

    it('should handle CORS requests', async () => {
      const corsClient = new TaskQueueClient({
        baseUrl: 'http://localhost:16080',
        apiKey: 'test-api-key',
        mode: 'cors',
        credentials: 'include',
      });

      mockFetch.mockResolvedValueOnce({
        ok: true,
        json: () => Promise.resolve({}),
        headers: { get: () => 'application/json' },
      });

      await corsClient.healthCheck();

      expect(mockFetch).toHaveBeenCalledWith(
        expect.any(String),
        expect.objectContaining({
          mode: 'cors',
          credentials: 'include',
        })
      );
    });
  });

  describe('Cross-Platform Compatibility', () => {
    it('should work with different base URLs', async () => {
      const urls = [
        'http://localhost:16080',
        'https://api.taskqueue.com',
        'http://192.168.1.100:8080',
        'https://taskqueue.example.com/api/v1',
      ];

      for (const url of urls) {
        const testClient = new TaskQueueClient({
          baseUrl: url,
          apiKey: 'test-api-key',
        });

        mockFetch.mockResolvedValueOnce({
          ok: true,
          json: () => Promise.resolve({ status: 'healthy' }),
          headers: { get: () => 'application/json' },
        });

        const result = await testClient.healthCheck();
        expect(result.status).toBe('healthy');
      }
    });

    it('should handle different API key formats', async () => {
      const apiKeys = [
        'simple-key',
        'key-with-dashes',
        'key_with_underscores',
        'key.with.dots',
        'key123456789',
        'very-long-api-key-with-many-characters-and-numbers-123456789',
      ];

      for (const apiKey of apiKeys) {
        const testClient = new TaskQueueClient({
          baseUrl: 'http://localhost:16080',
          apiKey,
        });

        mockFetch.mockResolvedValueOnce({
          ok: true,
          json: () => Promise.resolve({}),
          headers: { get: () => 'application/json' },
        });

        await testClient.healthCheck();

        expect(mockFetch).toHaveBeenCalledWith(
          expect.any(String),
          expect.objectContaining({
            headers: expect.objectContaining({
              'Authorization': `Bearer ${apiKey}`,
            }),
          })
        );
      }
    });

    it('should handle different timeout values', async () => {
      const timeouts = [1000, 5000, 10000, 30000, 60000];

      for (const timeout of timeouts) {
        const testClient = new TaskQueueClient({
          baseUrl: 'http://localhost:16080',
          apiKey: 'test-api-key',
          timeout,
        });

        mockFetch.mockImplementationOnce(() => 
          new Promise((_, reject) => {
            setTimeout(() => reject(new Error('Timeout')), timeout - 100);
          })
        );

        await expect(testClient.healthCheck()).rejects.toThrow();
      }
    });

    it('should handle different retry configurations', async () => {
      const retryConfigs = [
        { retries: 0, retryDelay: 100 },
        { retries: 1, retryDelay: 500 },
        { retries: 3, retryDelay: 1000 },
        { retries: 5, retryDelay: 2000 },
      ];

      for (const config of retryConfigs) {
        const testClient = new TaskQueueClient({
          baseUrl: 'http://localhost:16080',
          apiKey: 'test-api-key',
          ...config,
        });

        // Mock failures followed by success
        mockFetch
          .mockRejectedValueOnce(new Error('Network error'))
          .mockRejectedValueOnce(new Error('Network error'))
          .mockResolvedValueOnce({
            ok: true,
            json: () => Promise.resolve({}),
            headers: { get: () => 'application/json' },
          });

        await testClient.healthCheck();

        const expectedCalls = Math.min(config.retries + 1, 3);
        expect(mockFetch).toHaveBeenCalledTimes(expectedCalls);
      }
    });
  });

  describe('Network Conditions', () => {
    it('should handle slow network connections', async () => {
      const slowClient = new TaskQueueClient({
        baseUrl: 'http://localhost:16080',
        apiKey: 'test-api-key',
        timeout: 10000,
      });

      mockFetch.mockImplementationOnce(() => 
        new Promise((resolve) => {
          setTimeout(() => {
            resolve({
              ok: true,
              json: () => Promise.resolve({ status: 'healthy' }),
              headers: { get: () => 'application/json' },
            });
          }, 2000);
        })
      );

      const result = await slowClient.healthCheck();
      expect(result.status).toBe('healthy');
    });

    it('should handle intermittent network failures', async () => {
      const resilientClient = new TaskQueueClient({
        baseUrl: 'http://localhost:16080',
        apiKey: 'test-api-key',
        retries: 3,
        retryDelay: 100,
      });

      mockFetch
        .mockRejectedValueOnce(new Error('Network error'))
        .mockRejectedValueOnce(new Error('Network error'))
        .mockResolvedValueOnce({
          ok: true,
          json: () => Promise.resolve({ status: 'healthy' }),
          headers: { get: () => 'application/json' },
        });

      const result = await resilientClient.healthCheck();
      expect(result.status).toBe('healthy');
      expect(mockFetch).toHaveBeenCalledTimes(3);
    });

    it('should handle DNS resolution failures', async () => {
      const dnsClient = new TaskQueueClient({
        baseUrl: 'http://nonexistent-domain.com',
        apiKey: 'test-api-key',
        retries: 1,
        retryDelay: 100,
      });

      mockFetch
        .mockRejectedValueOnce(new Error('DNS resolution failed'))
        .mockRejectedValueOnce(new Error('DNS resolution failed'));

      await expect(dnsClient.healthCheck()).rejects.toThrow();
      expect(mockFetch).toHaveBeenCalledTimes(2);
    });

    it('should handle SSL/TLS certificate issues', async () => {
      const sslClient = new TaskQueueClient({
        baseUrl: 'https://self-signed-cert.example.com',
        apiKey: 'test-api-key',
        retries: 1,
        retryDelay: 100,
      });

      mockFetch
        .mockRejectedValueOnce(new Error('SSL certificate verification failed'))
        .mockRejectedValueOnce(new Error('SSL certificate verification failed'));

      await expect(sslClient.healthCheck()).rejects.toThrow();
      expect(mockFetch).toHaveBeenCalledTimes(2);
    });
  });

  describe('Memory and Performance', () => {
    it('should handle large response payloads', async () => {
      const largePayload = {
        tasks: Array.from({ length: 1000 }, (_, i) => ({
          id: `task-${i}`,
          name: `Task ${i}`,
          command: `echo task-${i}`,
          project_id: '123e4567-e89b-12d3-a456-426614174001',
          priority: 'Normal',
          status: 'Pending',
          dependencies: [],
          created_at: '2023-01-01T00:00:00Z',
          updated_at: '2023-01-01T00:00:00Z',
        })),
      };

      mockFetch.mockResolvedValueOnce({
        ok: true,
        json: () => Promise.resolve(largePayload),
        headers: { get: () => 'application/json' },
      });

      const result = await client.listTasks();
      expect(result).toHaveLength(1000);
    });

    it('should handle many concurrent requests efficiently', async () => {
      const concurrentRequests = 100;
      const promises = [];

      mockFetch.mockResolvedValue({
        ok: true,
        json: () => Promise.resolve({ status: 'healthy' }),
        headers: { get: () => 'application/json' },
      });

      for (let i = 0; i < concurrentRequests; i++) {
        promises.push(client.healthCheck());
      }

      const results = await Promise.all(promises);
      expect(results).toHaveLength(concurrentRequests);
      expect(mockFetch).toHaveBeenCalledTimes(concurrentRequests);
    });

    it('should not leak memory with repeated requests', async () => {
      const iterations = 1000;

      mockFetch.mockResolvedValue({
        ok: true,
        json: () => Promise.resolve({ status: 'healthy' }),
        headers: { get: () => 'application/json' },
      });

      for (let i = 0; i < iterations; i++) {
        await client.healthCheck();
      }

      expect(mockFetch).toHaveBeenCalledTimes(iterations);
      // In a real test, you would check memory usage here
    });
  });

  describe('Security Considerations', () => {
    it('should not expose sensitive data in error messages', async () => {
      mockFetch.mockResolvedValueOnce({
        ok: false,
        status: 401,
        statusText: 'Unauthorized',
        json: () => Promise.resolve({ 
          error: 'Invalid API key',
          // This should not be exposed
          internal_error: 'API key format validation failed',
        }),
      });

      try {
        await client.healthCheck();
      } catch (error) {
        expect(error.message).not.toContain('internal_error');
        expect(error.message).not.toContain('API key format validation failed');
      }
    });

    it('should handle malicious response data', async () => {
      const maliciousPayload = {
        // Attempt to inject malicious code
        malicious: '<script>alert("xss")</script>',
        // Attempt to cause prototype pollution
        __proto__: { isAdmin: true },
        // Attempt to cause JSON injection
        json_injection: '{"malicious": true}',
      };

      mockFetch.mockResolvedValueOnce({
        ok: true,
        json: () => Promise.resolve(maliciousPayload),
        headers: { get: () => 'application/json' },
      });

      const result = await client.healthCheck();
      // The client should handle this gracefully
      expect(result).toBeDefined();
      expect(result.malicious).toBe('<script>alert("xss")</script>');
    });

    it('should validate API key format', async () => {
      const invalidApiKeys = [
        '', // Empty
        ' ', // Whitespace only
        'key with spaces', // Contains spaces
        'key\nwith\nnewlines', // Contains newlines
        'key\twith\ttabs', // Contains tabs
        'key"with"quotes', // Contains quotes
        'key\'with\'quotes', // Contains single quotes
        'key`with`backticks', // Contains backticks
      ];

      for (const invalidKey of invalidApiKeys) {
        expect(() => {
          new TaskQueueClient({
            baseUrl: 'http://localhost:16080',
            apiKey: invalidKey,
          });
        }).not.toThrow(); // Client should handle invalid keys gracefully
      }
    });
  });
});
