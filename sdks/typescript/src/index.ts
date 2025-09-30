// Main entry point for the SDK
export { TaskQueueClient } from './client';
export * from './models/task';
export * from './models/project';
export * from './utils/error-handling';

// Re-export commonly used types
export type { ClientConfig } from './client';
export type { Task, CreateTaskRequest, TaskFilters, TaskStatus, TaskPriority } from './models/task';
export type { Project, CreateProjectRequest, ProjectFilters, ProjectStatus } from './models/project';
export type { ClientError, ValidationError, NetworkError } from './utils/error-handling';