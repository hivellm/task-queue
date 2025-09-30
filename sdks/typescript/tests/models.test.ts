import { 
  TaskSchema, 
  CreateTaskRequestSchema, 
  TaskFiltersSchema,
  TaskStatusSchema,
  TaskPrioritySchema,
  DependencyConditionSchema,
  TaskDependencySchema,
  TaskMetricsSchema,
  TaskResultSchema,
} from '../src/models/task';
import { 
  ProjectSchema, 
  CreateProjectRequestSchema, 
  ProjectFiltersSchema,
  ProjectStatusSchema,
} from '../src/models/project';
import { 
  WorkflowSchema,
  CreateWorkflowRequestSchema,
  WorkflowStatusSchema,
  WorkflowStepSchema,
} from '../src/models/workflow';

describe('Task Models', () => {
  describe('TaskSchema', () => {
    const validTask = {
      id: '123e4567-e89b-12d3-a456-426614174000',
      name: 'Test Task',
      command: 'echo hello',
      project_id: '123e4567-e89b-12d3-a456-426614174001',
      priority: 'Normal',
      status: 'Pending',
      dependencies: [],
      created_at: '2023-01-01T00:00:00Z',
      updated_at: '2023-01-01T00:00:00Z',
    };

    it('should validate a valid task', () => {
      const result = TaskSchema.safeParse(validTask);
      expect(result.success).toBe(true);
      if (result.success) {
        expect(result.data.name).toBe('Test Task');
        expect(result.data.status).toBe('Pending');
      }
    });

    it('should reject invalid task data', () => {
      const invalidTask = {
        id: 'invalid-uuid',
        name: '',
        command: '',
        project_id: 'invalid-uuid',
        priority: 'InvalidPriority',
        status: 'InvalidStatus',
      };

      const result = TaskSchema.safeParse(invalidTask);
      expect(result.success).toBe(false);
    });

    it('should handle optional fields', () => {
      const taskWithOptionals = {
        ...validTask,
        description: 'Optional description',
        started_at: '2023-01-01T01:00:00Z',
        completed_at: '2023-01-01T02:00:00Z',
        metadata: { key: 'value' },
      };

      const result = TaskSchema.safeParse(taskWithOptionals);
      expect(result.success).toBe(true);
    });
  });

  describe('CreateTaskRequestSchema', () => {
    it('should validate a valid create task request', () => {
      const validRequest = {
        name: 'New Task',
        command: 'python script.py',
        project_id: '123e4567-e89b-12d3-a456-426614174001',
        priority: 'High',
        dependencies: [],
        metadata: { env: 'production' },
      };

      const result = CreateTaskRequestSchema.safeParse(validRequest);
      expect(result.success).toBe(true);
    });

    it('should apply default values', () => {
      const minimalRequest = {
        name: 'Minimal Task',
        command: 'echo hello',
        project_id: '123e4567-e89b-12d3-a456-426614174001',
      };

      const result = CreateTaskRequestSchema.safeParse(minimalRequest);
      expect(result.success).toBe(true);
      if (result.success) {
        expect(result.data.priority).toBe('Normal');
        expect(result.data.dependencies).toEqual([]);
      }
    });

    it('should reject invalid create task request', () => {
      const invalidRequest = {
        name: '',
        command: '',
        project_id: 'invalid-uuid',
        priority: 'InvalidPriority',
      };

      const result = CreateTaskRequestSchema.safeParse(invalidRequest);
      expect(result.success).toBe(false);
    });
  });

  describe('TaskFiltersSchema', () => {
    it('should validate task filters', () => {
      const filters = {
        status: 'Pending',
        priority: 'High',
        project_id: '123e4567-e89b-12d3-a456-426614174001',
        limit: 20,
        offset: 10,
      };

      const result = TaskFiltersSchema.safeParse(filters);
      expect(result.success).toBe(true);
    });

    it('should apply default values for filters', () => {
      const emptyFilters = {};

      const result = TaskFiltersSchema.safeParse(emptyFilters);
      expect(result.success).toBe(true);
      if (result.success) {
        expect(result.data.limit).toBe(50);
        expect(result.data.offset).toBe(0);
      }
    });
  });

  describe('Task Enums', () => {
    it('should validate task status', () => {
      expect(TaskStatusSchema.safeParse('Pending').success).toBe(true);
      expect(TaskStatusSchema.safeParse('Running').success).toBe(true);
      expect(TaskStatusSchema.safeParse('Completed').success).toBe(true);
      expect(TaskStatusSchema.safeParse('InvalidStatus').success).toBe(false);
    });

    it('should validate task priority', () => {
      expect(TaskPrioritySchema.safeParse('Low').success).toBe(true);
      expect(TaskPrioritySchema.safeParse('Normal').success).toBe(true);
      expect(TaskPrioritySchema.safeParse('High').success).toBe(true);
      expect(TaskPrioritySchema.safeParse('Critical').success).toBe(true);
      expect(TaskPrioritySchema.safeParse('InvalidPriority').success).toBe(false);
    });

    it('should validate dependency condition', () => {
      expect(DependencyConditionSchema.safeParse('Success').success).toBe(true);
      expect(DependencyConditionSchema.safeParse('Failure').success).toBe(true);
      expect(DependencyConditionSchema.safeParse('Completion').success).toBe(true);
      expect(DependencyConditionSchema.safeParse('InvalidCondition').success).toBe(false);
    });
  });

  describe('Task Dependencies', () => {
    it('should validate task dependency', () => {
      const dependency = {
        task_id: '123e4567-e89b-12d3-a456-426614174000',
        condition: 'Success',
      };

      const result = TaskDependencySchema.safeParse(dependency);
      expect(result.success).toBe(true);
    });

    it('should reject invalid dependency', () => {
      const invalidDependency = {
        task_id: 'invalid-uuid',
        condition: 'InvalidCondition',
      };

      const result = TaskDependencySchema.safeParse(invalidDependency);
      expect(result.success).toBe(false);
    });
  });

  describe('Task Metrics', () => {
    it('should validate task metrics', () => {
      const metrics = {
        execution_time: 1000,
        memory_usage: 1024,
        cpu_usage: 0.5,
        disk_usage: 2048,
        network_io: 512,
      };

      const result = TaskMetricsSchema.safeParse(metrics);
      expect(result.success).toBe(true);
    });
  });

  describe('Task Results', () => {
    it('should validate success result', () => {
      const successResult = {
        type: 'success',
        output: 'Task completed successfully',
        artifacts: ['output.txt', 'log.txt'],
        metrics: {
          execution_time: 1000,
          memory_usage: 1024,
          cpu_usage: 0.5,
          disk_usage: 2048,
          network_io: 512,
        },
      };

      const result = TaskResultSchema.safeParse(successResult);
      expect(result.success).toBe(true);
    });

    it('should validate failure result', () => {
      const failureResult = {
        type: 'failure',
        error: 'Task failed with error',
        exit_code: 1,
        logs: ['Error: Something went wrong'],
      };

      const result = TaskResultSchema.safeParse(failureResult);
      expect(result.success).toBe(true);
    });

    it('should validate cancelled result', () => {
      const cancelledResult = {
        type: 'cancelled',
        reason: 'User requested cancellation',
      };

      const result = TaskResultSchema.safeParse(cancelledResult);
      expect(result.success).toBe(true);
    });

    it('should reject invalid result type', () => {
      const invalidResult = {
        type: 'invalid',
        output: 'Some output',
      };

      const result = TaskResultSchema.safeParse(invalidResult);
      expect(result.success).toBe(false);
    });

    it('should handle failure result without exit code', () => {
      const failureResult = {
        type: 'failure',
        error: 'Task failed with error',
        logs: ['Error: Something went wrong'],
      };

      const result = TaskResultSchema.safeParse(failureResult);
      expect(result.success).toBe(true);
    });

    it('should handle success result without artifacts', () => {
      const successResult = {
        type: 'success',
        output: 'Task completed successfully',
        artifacts: [],
        metrics: {
          execution_time: 1000,
          memory_usage: 1024,
          cpu_usage: 0.5,
          disk_usage: 2048,
          network_io: 512,
        },
      };

      const result = TaskResultSchema.safeParse(successResult);
      expect(result.success).toBe(true);
    });
  });

  describe('Edge Cases and Error Scenarios', () => {
    it('should handle very long task names', () => {
      const longName = 'a'.repeat(1000);
      const task = {
        ...validTask,
        name: longName,
      };

      const result = TaskSchema.safeParse(task);
      expect(result.success).toBe(true);
    });

    it('should handle empty arrays in dependencies', () => {
      const task = {
        ...validTask,
        dependencies: [],
      };

      const result = TaskSchema.safeParse(task);
      expect(result.success).toBe(true);
    });

    it('should handle complex metadata', () => {
      const complexMetadata = {
        nested: {
          array: [1, 2, 3],
          object: { key: 'value' },
        },
        boolean: true,
        number: 42,
        null_value: null,
      };

      const task = {
        ...validTask,
        metadata: complexMetadata,
      };

      const result = TaskSchema.safeParse(task);
      expect(result.success).toBe(true);
    });

    it('should handle special characters in task names', () => {
      const specialName = 'Task with special chars: !@#$%^&*()_+-=[]{}|;:,.<>?';
      const task = {
        ...validTask,
        name: specialName,
      };

      const result = TaskSchema.safeParse(task);
      expect(result.success).toBe(true);
    });

    it('should handle unicode characters', () => {
      const unicodeName = 'Tarefa com acentos: ção, ão, í, é, ó, ú';
      const task = {
        ...validTask,
        name: unicodeName,
      };

      const result = TaskSchema.safeParse(task);
      expect(result.success).toBe(true);
    });

    it('should reject task with missing required fields', () => {
      const incompleteTask = {
        id: '123e4567-e89b-12d3-a456-426614174000',
        name: 'Test Task',
        // Missing command, project_id, priority, status, etc.
      };

      const result = TaskSchema.safeParse(incompleteTask);
      expect(result.success).toBe(false);
    });

    it('should reject task with invalid UUID format', () => {
      const taskWithInvalidId = {
        ...validTask,
        id: 'not-a-uuid',
      };

      const result = TaskSchema.safeParse(taskWithInvalidId);
      expect(result.success).toBe(false);
    });

    it('should reject task with invalid date format', () => {
      const taskWithInvalidDate = {
        ...validTask,
        created_at: 'not-a-date',
      };

      const result = TaskSchema.safeParse(taskWithInvalidDate);
      expect(result.success).toBe(false);
    });

    it('should handle task with all optional fields', () => {
      const taskWithAllOptionals = {
        ...validTask,
        description: 'Full description',
        started_at: '2023-01-01T01:00:00Z',
        completed_at: '2023-01-01T02:00:00Z',
        metadata: { key: 'value' },
        result: {
          type: 'success',
          output: 'Task completed',
          artifacts: ['output.txt'],
          metrics: {
            execution_time: 1000,
            memory_usage: 1024,
            cpu_usage: 0.5,
            disk_usage: 2048,
            network_io: 512,
          },
        },
      };

      const result = TaskSchema.safeParse(taskWithAllOptionals);
      expect(result.success).toBe(true);
    });
  });
});

describe('Project Models', () => {
  describe('ProjectSchema', () => {
    const validProject = {
      id: '123e4567-e89b-12d3-a456-426614174001',
      name: 'Test Project',
      description: 'Test project description',
      status: 'Active',
      created_at: '2023-01-01T00:00:00Z',
      updated_at: '2023-01-01T00:00:00Z',
      tags: ['test', 'example'],
    };

    it('should validate a valid project', () => {
      const result = ProjectSchema.safeParse(validProject);
      expect(result.success).toBe(true);
      if (result.success) {
        expect(result.data.name).toBe('Test Project');
        expect(result.data.status).toBe('Active');
      }
    });

    it('should handle optional fields', () => {
      const projectWithOptionals = {
        ...validProject,
        metadata: { key: 'value' },
      };

      const result = ProjectSchema.safeParse(projectWithOptionals);
      expect(result.success).toBe(true);
    });
  });

  describe('CreateProjectRequestSchema', () => {
    it('should validate a valid create project request', () => {
      const validRequest = {
        name: 'New Project',
        description: 'New project description',
        tags: ['new', 'project'],
        metadata: { version: '1.0.0' },
      };

      const result = CreateProjectRequestSchema.safeParse(validRequest);
      expect(result.success).toBe(true);
    });

    it('should apply default values', () => {
      const minimalRequest = {
        name: 'Minimal Project',
      };

      const result = CreateProjectRequestSchema.safeParse(minimalRequest);
      expect(result.success).toBe(true);
      if (result.success) {
        expect(result.data.tags).toEqual([]);
      }
    });
  });

  describe('Project Enums', () => {
    it('should validate project status', () => {
      expect(ProjectStatusSchema.safeParse('Planning').success).toBe(true);
      expect(ProjectStatusSchema.safeParse('Active').success).toBe(true);
      expect(ProjectStatusSchema.safeParse('Completed').success).toBe(true);
      expect(ProjectStatusSchema.safeParse('InvalidStatus').success).toBe(false);
    });
  });

  describe('Project Edge Cases', () => {
    it('should handle project with empty tags', () => {
      const project = {
        ...validProject,
        tags: [],
      };

      const result = ProjectSchema.safeParse(project);
      expect(result.success).toBe(true);
    });

    it('should handle project with many tags', () => {
      const manyTags = Array.from({ length: 100 }, (_, i) => `tag${i}`);
      const project = {
        ...validProject,
        tags: manyTags,
      };

      const result = ProjectSchema.safeParse(project);
      expect(result.success).toBe(true);
    });

    it('should handle project with special characters in name', () => {
      const specialName = 'Project with special chars: !@#$%^&*()_+-=[]{}|;:,.<>?';
      const project = {
        ...validProject,
        name: specialName,
      };

      const result = ProjectSchema.safeParse(project);
      expect(result.success).toBe(true);
    });

    it('should handle project with unicode characters', () => {
      const unicodeName = 'Projeto com acentos: ção, ão, í, é, ó, ú';
      const project = {
        ...validProject,
        name: unicodeName,
      };

      const result = ProjectSchema.safeParse(project);
      expect(result.success).toBe(true);
    });

    it('should reject project with missing required fields', () => {
      const incompleteProject = {
        id: '123e4567-e89b-12d3-a456-426614174001',
        // Missing name, status, created_at, updated_at
      };

      const result = ProjectSchema.safeParse(incompleteProject);
      expect(result.success).toBe(false);
    });

    it('should reject project with invalid UUID format', () => {
      const projectWithInvalidId = {
        ...validProject,
        id: 'not-a-uuid',
      };

      const result = ProjectSchema.safeParse(projectWithInvalidId);
      expect(result.success).toBe(false);
    });

    it('should handle project with complex metadata', () => {
      const complexMetadata = {
        configuration: {
          settings: {
            timeout: 30000,
            retries: 3,
            enabled: true,
          },
          features: ['feature1', 'feature2'],
        },
        statistics: {
          tasks_created: 100,
          tasks_completed: 95,
          success_rate: 0.95,
        },
      };

      const project = {
        ...validProject,
        metadata: complexMetadata,
      };

      const result = ProjectSchema.safeParse(project);
      expect(result.success).toBe(true);
    });
  });
});

describe('Workflow Models', () => {
  describe('WorkflowSchema', () => {
    const validWorkflow = {
      id: '123e4567-e89b-12d3-a456-426614174002',
      name: 'Test Workflow',
      description: 'Test workflow description',
      project_id: '123e4567-e89b-12d3-a456-426614174001',
      status: 'Active',
      steps: [
        {
          id: 'step1',
          name: 'Step 1',
          task_id: '123e4567-e89b-12d3-a456-426614174000',
          order: 1,
          dependencies: [],
        },
      ],
      created_at: '2023-01-01T00:00:00Z',
      updated_at: '2023-01-01T00:00:00Z',
    };

    it('should validate a valid workflow', () => {
      const result = WorkflowSchema.safeParse(validWorkflow);
      expect(result.success).toBe(true);
      if (result.success) {
        expect(result.data.name).toBe('Test Workflow');
        expect(result.data.status).toBe('Active');
      }
    });

    it('should handle workflow with multiple steps', () => {
      const workflowWithSteps = {
        ...validWorkflow,
        steps: [
          {
            id: 'step1',
            name: 'Step 1',
            task_id: '123e4567-e89b-12d3-a456-426614174000',
            order: 1,
            dependencies: [],
          },
          {
            id: 'step2',
            name: 'Step 2',
            task_id: '123e4567-e89b-12d3-a456-426614174001',
            order: 2,
            dependencies: ['step1'],
          },
        ],
      };

      const result = WorkflowSchema.safeParse(workflowWithSteps);
      expect(result.success).toBe(true);
    });

    it('should handle workflow with optional fields', () => {
      const workflowWithOptionals = {
        ...validWorkflow,
        metadata: { key: 'value' },
        tags: ['workflow', 'test'],
      };

      const result = WorkflowSchema.safeParse(workflowWithOptionals);
      expect(result.success).toBe(true);
    });

    it('should reject workflow with invalid step order', () => {
      const workflowWithInvalidOrder = {
        ...validWorkflow,
        steps: [
          {
            id: 'step1',
            name: 'Step 1',
            task_id: '123e4567-e89b-12d3-a456-426614174000',
            order: 2, // Invalid order
            dependencies: [],
          },
        ],
      };

      const result = WorkflowSchema.safeParse(workflowWithInvalidOrder);
      expect(result.success).toBe(false);
    });
  });

  describe('CreateWorkflowRequestSchema', () => {
    it('should validate a valid create workflow request', () => {
      const validRequest = {
        name: 'New Workflow',
        description: 'New workflow description',
        project_id: '123e4567-e89b-12d3-a456-426614174001',
        steps: [
          {
            id: 'step1',
            name: 'Step 1',
            task_id: '123e4567-e89b-12d3-a456-426614174000',
            order: 1,
            dependencies: [],
          },
        ],
        tags: ['new', 'workflow'],
      };

      const result = CreateWorkflowRequestSchema.safeParse(validRequest);
      expect(result.success).toBe(true);
    });

    it('should apply default values', () => {
      const minimalRequest = {
        name: 'Minimal Workflow',
        project_id: '123e4567-e89b-12d3-a456-426614174001',
        steps: [],
      };

      const result = CreateWorkflowRequestSchema.safeParse(minimalRequest);
      expect(result.success).toBe(true);
      if (result.success) {
        expect(result.data.tags).toEqual([]);
      }
    });
  });

  describe('Workflow Enums', () => {
    it('should validate workflow status', () => {
      expect(WorkflowStatusSchema.safeParse('Planning').success).toBe(true);
      expect(WorkflowStatusSchema.safeParse('Active').success).toBe(true);
      expect(WorkflowStatusSchema.safeParse('Completed').success).toBe(true);
      expect(WorkflowStatusSchema.safeParse('Failed').success).toBe(true);
      expect(WorkflowStatusSchema.safeParse('InvalidStatus').success).toBe(false);
    });
  });

  describe('WorkflowStepSchema', () => {
    it('should validate a valid workflow step', () => {
      const validStep = {
        id: 'step1',
        name: 'Step 1',
        task_id: '123e4567-e89b-12d3-a456-426614174000',
        order: 1,
        dependencies: [],
      };

      const result = WorkflowStepSchema.safeParse(validStep);
      expect(result.success).toBe(true);
    });

    it('should handle step with dependencies', () => {
      const stepWithDependencies = {
        id: 'step2',
        name: 'Step 2',
        task_id: '123e4567-e89b-12d3-a456-426614174001',
        order: 2,
        dependencies: ['step1'],
      };

      const result = WorkflowStepSchema.safeParse(stepWithDependencies);
      expect(result.success).toBe(true);
    });

    it('should reject step with invalid order', () => {
      const stepWithInvalidOrder = {
        id: 'step1',
        name: 'Step 1',
        task_id: '123e4567-e89b-12d3-a456-426614174000',
        order: 0, // Invalid order (should be >= 1)
        dependencies: [],
      };

      const result = WorkflowStepSchema.safeParse(stepWithInvalidOrder);
      expect(result.success).toBe(false);
    });
  });
});
