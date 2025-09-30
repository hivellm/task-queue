// Test setup file
import 'jest';

// Mock fetch globally for all tests
global.fetch = jest.fn();

// Mock console methods to reduce noise in tests
global.console = {
  ...console,
  log: jest.fn(),
  debug: jest.fn(),
  info: jest.fn(),
  warn: jest.fn(),
  error: jest.fn(),
};

// Setup test environment variables
process.env.NODE_ENV = 'test';
process.env.TASK_QUEUE_URL = 'http://localhost:16080';
process.env.TASK_QUEUE_API_KEY = 'test-api-key';

// Global test utilities
declare global {
  namespace jest {
    interface Matchers<R> {
      toBeValidTask(): R;
      toBeValidProject(): R;
    }
  }
}

// Custom matchers
expect.extend({
  toBeValidTask(received) {
    const pass = received && 
                 typeof received.id === 'string' &&
                 typeof received.name === 'string' &&
                 typeof received.command === 'string' &&
                 typeof received.status === 'string' &&
                 typeof received.priority === 'string';
    
    if (pass) {
      return {
        message: () => `expected ${received} not to be a valid task`,
        pass: true,
      };
    } else {
      return {
        message: () => `expected ${received} to be a valid task`,
        pass: false,
      };
    }
  },
  
  toBeValidProject(received) {
    const pass = received && 
                 typeof received.id === 'string' &&
                 typeof received.name === 'string' &&
                 typeof received.status === 'string';
    
    if (pass) {
      return {
        message: () => `expected ${received} not to be a valid project`,
        pass: true,
      };
    } else {
      return {
        message: () => `expected ${received} to be a valid project`,
        pass: false,
      };
    }
  },
});
