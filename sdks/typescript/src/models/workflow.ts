import { z } from 'zod';

export const WorkflowStatusSchema = z.enum([
  'Pending',
  'Running',
  'Completed',
  'Failed',
  'Cancelled',
]);

export const WorkflowSchema = z.object({
  id: z.string().uuid(),
  name: z.string(),
  description: z.string().optional(),
  status: WorkflowStatusSchema,
  tasks: z.array(z.string().uuid()),
  dependencies: z.array(z.object({
    from_task: z.string().uuid(),
    to_task: z.string().uuid(),
    condition: z.enum(['Success', 'Failure', 'Completion']),
  })),
  created_at: z.string().datetime(),
  updated_at: z.string().datetime(),
  started_at: z.string().datetime().optional(),
  completed_at: z.string().datetime().optional(),
  metadata: z.record(z.any()).optional(),
});

export const CreateWorkflowRequestSchema = z.object({
  name: z.string().min(1),
  description: z.string().optional(),
  tasks: z.array(z.string().uuid()),
  dependencies: z.array(z.object({
    from_task: z.string().uuid(),
    to_task: z.string().uuid(),
    condition: z.enum(['Success', 'Failure', 'Completion']),
  })).default([]),
  metadata: z.record(z.any()).optional(),
});

export const WorkflowFiltersSchema = z.object({
  status: WorkflowStatusSchema.optional(),
  limit: z.number().min(1).max(1000).default(50),
  offset: z.number().min(0).default(0),
});

// TypeScript types inferred from schemas
export type WorkflowStatus = z.infer<typeof WorkflowStatusSchema>;
export type Workflow = z.infer<typeof WorkflowSchema>;
export type CreateWorkflowRequest = z.infer<typeof CreateWorkflowRequestSchema>;
export type WorkflowFilters = z.infer<typeof WorkflowFiltersSchema>;
