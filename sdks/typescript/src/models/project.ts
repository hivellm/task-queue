import { z } from 'zod';

export const ProjectStatusSchema = z.enum([
  'Planning',
  'Active',
  'OnHold',
  'Completed',
  'Cancelled',
]);

export const ProjectSchema = z.object({
  id: z.string().uuid(),
  name: z.string(),
  description: z.string().optional(),
  status: ProjectStatusSchema,
  created_at: z.string().datetime(),
  updated_at: z.string().datetime(),
  tags: z.array(z.string()).default([]),
  metadata: z.record(z.any()).optional(),
});

export const CreateProjectRequestSchema = z.object({
  name: z.string().min(1),
  description: z.string().optional(),
  tags: z.array(z.string()).default([]),
  metadata: z.record(z.any()).optional(),
});

export const ProjectFiltersSchema = z.object({
  status: ProjectStatusSchema.optional(),
  tags: z.array(z.string()).optional(),
  limit: z.number().min(1).max(1000).default(50),
  offset: z.number().min(0).default(0),
});

// TypeScript types inferred from schemas
export type ProjectStatus = z.infer<typeof ProjectStatusSchema>;
export type Project = z.infer<typeof ProjectSchema>;
export type CreateProjectRequest = z.infer<typeof CreateProjectRequestSchema>;
export type ProjectFilters = z.infer<typeof ProjectFiltersSchema>;