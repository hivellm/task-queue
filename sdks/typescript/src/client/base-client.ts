import { z } from 'zod';

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
  abstract createTask(task: any): Promise<any>;
  abstract getTask(id: string): Promise<any>;
  abstract listTasks(filters?: any): Promise<any[]>;
  abstract updateTask(id: string, updates: any): Promise<any>;
  abstract deleteTask(id: string): Promise<void>;
  abstract cancelTask(id: string, reason?: string): Promise<any>;
}
