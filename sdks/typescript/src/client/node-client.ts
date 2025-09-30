import { BaseClient, ClientConfig } from './base-client';
import { Task, CreateTaskRequest, TaskFilters } from '../models/task';
import { Project, CreateProjectRequest, ProjectFilters } from '../models/project';
import { Workflow, CreateWorkflowRequest, WorkflowFilters } from '../models/workflow';

export class NodeClient extends BaseClient {
  private ws?: any; // WebSocket will be imported when needed

  constructor(config: ClientConfig) {
    super(config);
  }

  async createTask(task: CreateTaskRequest): Promise<Task> {
    return this.request<Task>('/api/tasks', {
      method: 'POST',
      body: JSON.stringify(task),
    });
  }

  async getTask(id: string): Promise<Task> {
    return this.request<Task>(`/api/tasks/${id}`);
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
    
    return this.request<Task[]>(`/api/tasks?${params.toString()}`);
  }

  async updateTask(id: string, updates: Partial<Task>): Promise<Task> {
    return this.request<Task>(`/api/tasks/${id}`, {
      method: 'PUT',
      body: JSON.stringify(updates),
    });
  }

  async deleteTask(id: string): Promise<void> {
    await this.request<void>(`/api/tasks/${id}`, {
      method: 'DELETE',
    });
  }

  async cancelTask(id: string, reason?: string): Promise<Task> {
    return this.request<Task>(`/api/tasks/${id}/cancel`, {
      method: 'POST',
      body: JSON.stringify({ reason }),
    });
  }

  // Project operations
  async createProject(project: CreateProjectRequest): Promise<Project> {
    return this.request<Project>('/api/projects', {
      method: 'POST',
      body: JSON.stringify(project),
    });
  }

  async getProject(id: string): Promise<Project> {
    return this.request<Project>(`/api/projects/${id}`);
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
    
    return this.request<Project[]>(`/api/projects?${params.toString()}`);
  }

  // Workflow operations
  async createWorkflow(workflow: CreateWorkflowRequest): Promise<Workflow> {
    return this.request<Workflow>('/api/workflows', {
      method: 'POST',
      body: JSON.stringify(workflow),
    });
  }

  async getWorkflow(id: string): Promise<Workflow> {
    return this.request<Workflow>(`/api/workflows/${id}`);
  }

  async listWorkflows(filters?: WorkflowFilters): Promise<Workflow[]> {
    const params = new URLSearchParams();
    if (filters) {
      Object.entries(filters).forEach(([key, value]) => {
        if (value !== undefined) {
          params.append(key, String(value));
        }
      });
    }
    
    return this.request<Workflow[]>(`/api/workflows?${params.toString()}`);
  }

  // WebSocket support for Node.js
  async connectWebSocket(): Promise<void> {
    // WebSocket implementation will be added when ws dependency is available
    return Promise.resolve();
  }

  disconnectWebSocket(): void {
    this.ws?.close();
  }
}
