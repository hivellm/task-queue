import { ClientError } from '../utils/error-handling';
import { Task, CreateTaskRequest, TaskFilters, TaskSchema } from '../models/task';
import { Project, CreateProjectRequest, ProjectFilters, ProjectSchema } from '../models/project';

export interface ClientConfig {
  baseUrl: string;
  apiKey?: string;
  timeout?: number;
  retries?: number;
  retryDelay?: number;
}

export class TaskQueueClient {
  private config: ClientConfig;
  private fetch: typeof fetch;

  constructor(config: ClientConfig, fetchImpl?: typeof fetch) {
    this.config = {
      timeout: 30000,
      retries: 3,
      retryDelay: 1000,
      ...config,
    };
    this.fetch = fetchImpl || fetch;
  }

  private async request<T>(
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
        const errorText = await response.text();
        throw new ClientError(
          `HTTP ${response.status}: ${response.statusText}${errorText ? ` - ${errorText}` : ''}`,
          response.status
        );
      }

      const contentType = response.headers.get('content-type');
      if (contentType && contentType.includes('application/json')) {
        return await response.json();
      } else {
        return await response.text() as T;
      }
    } catch (error) {
      clearTimeout(timeoutId);
      if (error instanceof ClientError) {
        throw error;
      }
      throw new ClientError(`Network error: ${error instanceof Error ? error.message : 'Unknown error'}`);
    }
  }

  // Task operations
  async createTask(task: CreateTaskRequest): Promise<Task> {
    const response = await this.request<any>('/api/tasks', {
      method: 'POST',
      body: JSON.stringify(task),
    });
    return TaskSchema.parse(response);
  }

  async getTask(id: string): Promise<Task> {
    const response = await this.request<any>(`/api/tasks/${id}`);
    return TaskSchema.parse(response);
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
    
    const response = await this.request<any[]>(`/api/tasks?${params.toString()}`);
    return response.map(task => TaskSchema.parse(task));
  }

  async updateTask(id: string, updates: Partial<Task>): Promise<Task> {
    const response = await this.request<any>(`/api/tasks/${id}`, {
      method: 'PUT',
      body: JSON.stringify(updates),
    });
    return TaskSchema.parse(response);
  }

  async deleteTask(id: string): Promise<void> {
    await this.request<void>(`/api/tasks/${id}`, {
      method: 'DELETE',
    });
  }

  async cancelTask(id: string, reason?: string): Promise<Task> {
    const response = await this.request<any>(`/api/tasks/${id}/cancel`, {
      method: 'POST',
      body: JSON.stringify({ reason }),
    });
    return TaskSchema.parse(response);
  }

  // Project operations
  async createProject(project: CreateProjectRequest): Promise<Project> {
    const response = await this.request<any>('/api/projects', {
      method: 'POST',
      body: JSON.stringify(project),
    });
    return ProjectSchema.parse(response);
  }

  async getProject(id: string): Promise<Project> {
    const response = await this.request<any>(`/api/projects/${id}`);
    return ProjectSchema.parse(response);
  }

  async listProjects(filters?: ProjectFilters): Promise<Project[]> {
    const params = new URLSearchParams();
    if (filters) {
      Object.entries(filters).forEach(([key, value]) => {
        if (value !== undefined) {
          params.append(key, String(value));
        }
      });
    }
    
    const response = await this.request<any[]>(`/api/projects?${params.toString()}`);
    return response.map(project => ProjectSchema.parse(project));
  }

  async updateProject(id: string, updates: Partial<Project>): Promise<Project> {
    const response = await this.request<any>(`/api/projects/${id}`, {
      method: 'PUT',
      body: JSON.stringify(updates),
    });
    return ProjectSchema.parse(response);
  }

  async deleteProject(id: string): Promise<void> {
    await this.request<void>(`/api/projects/${id}`, {
      method: 'DELETE',
    });
  }

  // Utility methods
  async healthCheck(): Promise<any> {
    return this.request<any>('/health');
  }

  async getMetrics(): Promise<any> {
    return this.request<any>('/metrics');
  }

  async getServerStatus(): Promise<any> {
    return this.request<any>('/api/status');
  }
}