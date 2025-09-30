import { z } from 'zod';

export const TaskStatusSchema = z.enum([
  'Planning',
  'Implementation', 
  'TestCreation',
  'Testing',
  'AIReview',
  'Finalized',
  'Pending',
  'Running',
  'Completed',
  'Failed',
  'Cancelled',
  'WaitingForDependencies',
]);

export const TaskPrioritySchema = z.enum(['Low', 'Normal', 'High', 'Critical']);

export const DependencyConditionSchema = z.enum(['Success', 'Failure', 'Completion']);

export const TaskDependencySchema = z.object({
  task_id: z.string().uuid(),
  condition: DependencyConditionSchema,
});

export const TaskMetricsSchema = z.object({
  execution_time: z.number(),
  memory_usage: z.number(),
  cpu_usage: z.number(),
  disk_usage: z.number(),
  network_io: z.number(),
});

export const TaskResultSchema = z.discriminatedUnion('type', [
  z.object({
    type: z.literal('success'),
    output: z.string(),
    artifacts: z.array(z.string()),
    metrics: TaskMetricsSchema,
  }),
  z.object({
    type: z.literal('failure'),
    error: z.string(),
    exit_code: z.number().optional(),
    logs: z.array(z.string()),
  }),
  z.object({
    type: z.literal('cancelled'),
    reason: z.string(),
  }),
]);

export const TaskSchema = z.object({
  id: z.string().uuid(),
  name: z.string(),
  command: z.string(),
  description: z.string().optional(),
  project_id: z.string().uuid(),
  priority: TaskPrioritySchema,
  status: TaskStatusSchema,
  dependencies: z.array(TaskDependencySchema),
  result: TaskResultSchema.optional(),
  created_at: z.string().datetime(),
  updated_at: z.string().datetime(),
  started_at: z.string().datetime().optional(),
  completed_at: z.string().datetime().optional(),
  metadata: z.record(z.any()).optional(),
});

export const CreateTaskRequestSchema = z.object({
  name: z.string().min(1),
  command: z.string().min(1),
  description: z.string().optional(),
  project_id: z.string().uuid(),
  priority: TaskPrioritySchema.default('Normal'),
  dependencies: z.array(TaskDependencySchema).default([]),
  metadata: z.record(z.any()).optional(),
});

export const TaskFiltersSchema = z.object({
  status: TaskStatusSchema.optional(),
  priority: TaskPrioritySchema.optional(),
  project_id: z.string().uuid().optional(),
  limit: z.number().min(1).max(1000).default(50),
  offset: z.number().min(0).default(0),
});

// TypeScript types inferred from schemas
export type TaskStatus = z.infer<typeof TaskStatusSchema>;
export type TaskPriority = z.infer<typeof TaskPrioritySchema>;
export type DependencyCondition = z.infer<typeof DependencyConditionSchema>;
export type TaskDependency = z.infer<typeof TaskDependencySchema>;
export type TaskMetrics = z.infer<typeof TaskMetricsSchema>;
export type TaskResult = z.infer<typeof TaskResultSchema>;
export type Task = z.infer<typeof TaskSchema>;
export type CreateTaskRequest = z.infer<typeof CreateTaskRequestSchema>;
export type TaskFilters = z.infer<typeof TaskFiltersSchema>;