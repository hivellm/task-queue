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