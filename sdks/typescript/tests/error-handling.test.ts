import { ClientError, ValidationError, NetworkError, handleError } from '../src/utils/error-handling';

describe('Error Handling', () => {
  describe('ClientError', () => {
    it('should create a client error with message', () => {
      const error = new ClientError('Test error');
      expect(error.message).toBe('Test error');
      expect(error.name).toBe('ClientError');
      expect(error.status).toBeUndefined();
      expect(error.code).toBeUndefined();
    });

    it('should create a client error with status', () => {
      const error = new ClientError('Not found', 404);
      expect(error.message).toBe('Not found');
      expect(error.status).toBe(404);
      expect(error.code).toBeUndefined();
    });

    it('should create a client error with status and code', () => {
      const error = new ClientError('Validation failed', 400, 'VALIDATION_ERROR');
      expect(error.message).toBe('Validation failed');
      expect(error.status).toBe(400);
      expect(error.code).toBe('VALIDATION_ERROR');
    });

    it('should be an instance of Error', () => {
      const error = new ClientError('Test error');
      expect(error).toBeInstanceOf(Error);
    });
  });

  describe('ValidationError', () => {
    it('should create a validation error with message', () => {
      const error = new ValidationError('Invalid input');
      expect(error.message).toBe('Invalid input');
      expect(error.name).toBe('ValidationError');
      expect(error.field).toBeUndefined();
    });

    it('should create a validation error with field', () => {
      const error = new ValidationError('Invalid email', 'email');
      expect(error.message).toBe('Invalid email');
      expect(error.field).toBe('email');
    });

    it('should be an instance of Error', () => {
      const error = new ValidationError('Test error');
      expect(error).toBeInstanceOf(Error);
    });
  });

  describe('NetworkError', () => {
    it('should create a network error with message', () => {
      const error = new NetworkError('Connection failed');
      expect(error.message).toBe('Connection failed');
      expect(error.name).toBe('NetworkError');
      expect(error.originalError).toBeUndefined();
    });

    it('should create a network error with original error', () => {
      const originalError = new Error('Original error');
      const error = new NetworkError('Network failed', originalError);
      expect(error.message).toBe('Network failed');
      expect(error.originalError).toBe(originalError);
    });

    it('should be an instance of Error', () => {
      const error = new NetworkError('Test error');
      expect(error).toBeInstanceOf(Error);
    });
  });

  describe('handleError', () => {
    it('should re-throw ClientError', () => {
      const clientError = new ClientError('Client error');
      
      expect(() => handleError(clientError)).toThrow(ClientError);
      expect(() => handleError(clientError)).toThrow('Client error');
    });

    it('should wrap Error in NetworkError', () => {
      const error = new Error('Test error');
      
      expect(() => handleError(error)).toThrow(NetworkError);
      expect(() => handleError(error)).toThrow('Test error');
    });

    it('should wrap unknown errors in NetworkError', () => {
      const unknownError = 'String error';
      
      expect(() => handleError(unknownError)).toThrow(NetworkError);
      expect(() => handleError(unknownError)).toThrow('Unknown error occurred');
    });

    it('should handle null/undefined errors', () => {
      expect(() => handleError(null)).toThrow(NetworkError);
      expect(() => handleError(undefined)).toThrow(NetworkError);
    });
  });

  describe('Error Inheritance', () => {
    it('should maintain proper inheritance chain', () => {
      const clientError = new ClientError('Test');
      const validationError = new ValidationError('Test');
      const networkError = new NetworkError('Test');

      expect(clientError).toBeInstanceOf(Error);
      expect(validationError).toBeInstanceOf(Error);
      expect(networkError).toBeInstanceOf(Error);

      expect(clientError).toBeInstanceOf(ClientError);
      expect(validationError).toBeInstanceOf(ValidationError);
      expect(networkError).toBeInstanceOf(NetworkError);
    });
  });

  describe('Error Properties', () => {
    it('should preserve error properties', () => {
      const originalError = new Error('Original');
      const networkError = new NetworkError('Network', originalError);
      
      expect(networkError.originalError).toBe(originalError);
      expect(networkError.message).toBe('Network');
    });

    it('should preserve client error properties', () => {
      const clientError = new ClientError('Client', 404, 'NOT_FOUND');
      
      expect(clientError.status).toBe(404);
      expect(clientError.code).toBe('NOT_FOUND');
      expect(clientError.message).toBe('Client');
    });

    it('should preserve validation error properties', () => {
      const validationError = new ValidationError('Invalid field', 'fieldName');
      
      expect(validationError.field).toBe('fieldName');
      expect(validationError.message).toBe('Invalid field');
    });
  });
});
