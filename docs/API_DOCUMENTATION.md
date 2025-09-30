# Task Queue HTTP API Documentation

## Overview

The Task Queue HTTP API provides RESTful endpoints for managing tasks and workflows in the task orchestration system. The API is built on top of Axum and provides comprehensive functionality for task management, dependency resolution, and workflow orchestration.

## Base URL

```
http://localhost:16080/api/v1
```

## Authentication

Currently, the API does not require authentication. In production environments, consider implementing authentication mechanisms.

## Content Type

All requests and responses use `application/json`.

## Error Handling

The API returns standard HTTP status codes and JSON error responses:

```json
{
  "error": "Error message",
  "code": "ERROR_CODE",
  "details": {
    "additional": "information"
  }
}
```

## Endpoints

### Health Check

#### GET /health

Check the health status of the Task Queue service.

**Response:**
```json
{
  "status": "healthy",
  "version": "0.1.0",
  "timestamp": "2024-01-01T00:00:00Z",
  "uptime": 3600,
  "active_tasks": 5,
  "pending_tasks": 10,
  "completed_tasks": 100,
  "failed_tasks": 2
}
```

### Statistics

#### GET /stats

Get system statistics and metrics.

**Response:**
```json
{
  "total_tasks": 117,
  "total_workflows": 15,
  "active_tasks": 5,
  "pending_tasks": 10,
  "completed_tasks": 100,
  "failed_tasks": 2,
  "uptime_seconds": 3600,
  "memory_usage_mb": 128.5,
  "cpu_usage_percent": 15.2,
  "timestamp": "2024-01-01T00:00:00Z"
}
```

## Task Management

### Create Task

#### POST /tasks

Create a new task.

**Request Body:**
```json
{
  "name": "build-api",
  "command": "npm run build",
  "project": "my-project",
  "task_type": "simple",
  "priority": "normal",
  "dependencies": ["run-tests"],
  "dependency_conditions": {
    "run-tests": "success"
  },
  "timeout": 300,
  "retry_attempts": 3,
  "retry_delay": 5,
  "environment": {
    "NODE_ENV": "production"
  },
  "working_directory": "/app",
  "metadata": {
    "build_type": "production"
  }
}
```

**Response:**
```json
{
  "message": "Task created successfully",
  "task_id": "550e8400-e29b-41d4-a716-446655440000",
  "name": "build-api",
  "status": "pending"
}
```

### Get Task

#### GET /tasks/{task_id}

Get information about a specific task.

**Response:**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "name": "build-api",
  "command": "npm run build",
  "project": "my-project",
  "task_type": "simple",
  "priority": "normal",
  "status": "completed",
  "dependencies": ["run-tests"],
  "dependency_conditions": {
    "run-tests": "success"
  },
  "timeout": 300,
  "retry_attempts": 3,
  "retry_delay": 5,
  "environment": {
    "NODE_ENV": "production"
  },
  "working_directory": "/app",
  "metadata": {
    "build_type": "production"
  },
  "created_at": "2024-01-01T00:00:00Z",
  "started_at": "2024-01-01T00:01:00Z",
  "completed_at": "2024-01-01T00:05:00Z",
  "execution_time": 240,
  "exit_code": 0,
  "output": "Build completed successfully",
  "error_output": null
}
```

### List Tasks

#### GET /tasks

List tasks with optional filtering.

**Query Parameters:**
- `limit` (optional): Maximum number of tasks to return (default: 100)
- `offset` (optional): Number of tasks to skip (default: 0)
- `status` (optional): Filter by task status
- `project` (optional): Filter by project name
- `task_type` (optional): Filter by task type

**Response:**
```json
{
  "tasks": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "name": "build-api",
      "command": "npm run build",
      "project": "my-project",
      "task_type": "simple",
      "priority": "normal",
      "status": "completed",
      "dependencies": ["run-tests"],
      "dependency_conditions": {
        "run-tests": "success"
      },
      "timeout": 300,
      "retry_attempts": 3,
      "retry_delay": 5,
      "environment": {
        "NODE_ENV": "production"
      },
      "working_directory": "/app",
      "metadata": {
        "build_type": "production"
      },
      "created_at": "2024-01-01T00:00:00Z",
      "started_at": "2024-01-01T00:01:00Z",
      "completed_at": "2024-01-01T00:05:00Z",
      "execution_time": 240,
      "exit_code": 0,
      "output": "Build completed successfully",
      "error_output": null
    }
  ],
  "total": 1,
  "limit": 100,
  "offset": 0
}
```

### Cancel Task

#### POST /tasks/{task_id}/cancel

Cancel a running or pending task.

**Request Body:**
```json
{
  "reason": "User requested cancellation"
}
```

**Response:**
```json
{
  "message": "Task cancelled successfully",
  "task_id": "550e8400-e29b-41d4-a716-446655440000"
}
```

### Retry Task

#### POST /tasks/{task_id}/retry

Retry a failed task.

**Request Body:**
```json
{
  "reset_retry_count": true
}
```

**Response:**
```json
{
  "message": "Task retry initiated successfully",
  "task_id": "550e8400-e29b-41d4-a716-446655440000"
}
```

### Update Task Priority

#### PUT /tasks/{task_id}/priority

Update the priority of a task.

**Request Body:**
```json
{
  "priority": "high"
}
```

**Response:**
```json
{
  "message": "Task priority updated successfully",
  "task_id": "550e8400-e29b-41d4-a716-446655440000",
  "priority": "high"
}
```

### Get Task Result

#### GET /tasks/{task_id}/result

Get the result of a completed task.

**Response:**
```json
{
  "task_id": "550e8400-e29b-41d4-a716-446655440000",
  "status": "completed",
  "exit_code": 0,
  "output": "Build completed successfully",
  "error_output": null,
  "execution_time": 240,
  "artifacts": ["dist/app.js", "dist/app.css"],
  "metadata": {
    "build_type": "production"
  }
}
```

## Workflow Management

### Create Workflow

#### POST /workflows

Create a new workflow with multiple tasks.

**Request Body:**
```json
{
  "name": "production-deployment",
  "description": "Deploy application to production",
  "tasks": [
    {
      "name": "run-tests",
      "command": "npm test",
      "project": "my-project",
      "task_type": "simple",
      "priority": "normal",
      "timeout": 300,
      "retry_attempts": 3,
      "retry_delay": 5
    },
    {
      "name": "build-docker",
      "command": "docker build -t my-app .",
      "project": "my-project",
      "task_type": "dependent",
      "priority": "normal",
      "dependencies": ["run-tests"],
      "dependency_conditions": {
        "run-tests": "success"
      },
      "timeout": 600,
      "retry_attempts": 2,
      "retry_delay": 10
    },
    {
      "name": "deploy-production",
      "command": "kubectl apply -f k8s/",
      "project": "my-project",
      "task_type": "dependent",
      "priority": "high",
      "dependencies": ["build-docker"],
      "dependency_conditions": {
        "build-docker": "success"
      },
      "timeout": 300,
      "retry_attempts": 1,
      "retry_delay": 30
    }
  ],
  "dependencies": {
    "build-docker": ["run-tests"],
    "deploy-production": ["build-docker"]
  },
  "timeout": 1800,
  "parallel_execution": false,
  "manual_approval": true,
  "metadata": {
    "environment": "production",
    "version": "1.0.0"
  }
}
```

**Response:**
```json
{
  "message": "Workflow created successfully",
  "workflow_id": "550e8400-e29b-41d4-a716-446655440001",
  "name": "production-deployment",
  "task_count": 3
}
```

### Get Workflow

#### GET /workflows/{workflow_id}

Get information about a specific workflow.

**Response:**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440001",
  "name": "production-deployment",
  "description": "Deploy application to production",
  "status": "running",
  "tasks": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440002",
      "name": "run-tests",
      "command": "npm test",
      "project": "my-project",
      "task_type": "simple",
      "priority": "normal",
      "status": "completed",
      "dependencies": [],
      "dependency_conditions": {},
      "timeout": 300,
      "retry_attempts": 3,
      "retry_delay": 5,
      "environment": {},
      "working_directory": null,
      "metadata": {},
      "created_at": "2024-01-01T00:00:00Z",
      "started_at": "2024-01-01T00:01:00Z",
      "completed_at": "2024-01-01T00:05:00Z",
      "execution_time": 240,
      "exit_code": 0,
      "output": "Tests passed",
      "error_output": null
    }
  ],
  "dependencies": {
    "build-docker": ["run-tests"],
    "deploy-production": ["build-docker"]
  },
  "timeout": 1800,
  "parallel_execution": false,
  "manual_approval": true,
  "metadata": {
    "environment": "production",
    "version": "1.0.0"
  },
  "created_at": "2024-01-01T00:00:00Z",
  "started_at": "2024-01-01T00:00:00Z",
  "completed_at": null,
  "execution_time": null
}
```

### List Workflows

#### GET /workflows

List workflows with optional filtering.

**Query Parameters:**
- `limit` (optional): Maximum number of workflows to return (default: 100)
- `offset` (optional): Number of workflows to skip (default: 0)
- `status` (optional): Filter by workflow status

**Response:**
```json
{
  "workflows": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440001",
      "name": "production-deployment",
      "description": "Deploy application to production",
      "status": "running",
      "tasks": [...],
      "dependencies": {...},
      "timeout": 1800,
      "parallel_execution": false,
      "manual_approval": true,
      "metadata": {...},
      "created_at": "2024-01-01T00:00:00Z",
      "started_at": "2024-01-01T00:00:00Z",
      "completed_at": null,
      "execution_time": null
    }
  ],
  "total": 1,
  "limit": 100,
  "offset": 0
}
```

### Cancel Workflow

#### POST /workflows/{workflow_id}/cancel

Cancel a running workflow.

**Request Body:**
```json
{
  "reason": "User requested cancellation"
}
```

**Response:**
```json
{
  "message": "Workflow cancelled successfully",
  "workflow_id": "550e8400-e29b-41d4-a716-446655440001"
}
```

### Approve Workflow

#### POST /workflows/{workflow_id}/approve

Approve a workflow that requires manual approval.

**Request Body:**
```json
{
  "message": "Approved for production deployment"
}
```

**Response:**
```json
{
  "message": "Workflow approved successfully",
  "workflow_id": "550e8400-e29b-41d4-a716-446655440001"
}
```

### Get Workflow Status

#### GET /workflows/{workflow_id}/status

Get the current status of a workflow.

**Response:**
```json
"running"
```

### Update Workflow Status

#### PUT /workflows/{workflow_id}/status

Update the status of a workflow.

**Request Body:**
```json
{
  "status": "completed",
  "message": "Workflow completed successfully"
}
```

**Response:**
```json
{
  "message": "Workflow status updated successfully",
  "workflow_id": "550e8400-e29b-41d4-a716-446655440001",
  "status": "completed"
}
```

### Get Workflow Result

#### GET /workflows/{workflow_id}/result

Get the result of a completed workflow.

**Response:**
```json
{
  "workflow_id": "550e8400-e29b-41d4-a716-446655440001",
  "status": "completed",
  "task_results": [
    {
      "task_id": "550e8400-e29b-41d4-a716-446655440002",
      "status": "completed",
      "exit_code": 0,
      "output": "Tests passed",
      "error_output": null,
      "execution_time": 240,
      "artifacts": [],
      "metadata": {}
    }
  ],
  "execution_time": 1800,
  "success_count": 3,
  "failure_count": 0,
  "artifacts": ["dist/app.js", "dist/app.css", "docker-image:latest"],
  "metadata": {
    "environment": "production",
    "version": "1.0.0"
  }
}
```

## Status Codes

- `200 OK`: Request successful
- `201 Created`: Resource created successfully
- `400 Bad Request`: Invalid request data
- `404 Not Found`: Resource not found
- `409 Conflict`: Resource conflict (e.g., duplicate task name)
- `500 Internal Server Error`: Server error

## Rate Limiting

Currently, there are no rate limits implemented. In production environments, consider implementing rate limiting to prevent abuse.

## Examples

### Complete Workflow Example

```bash
# Create a workflow
curl -X POST http://localhost:16080/api/v1/workflows \
  -H "Content-Type: application/json" \
  -d '{
    "name": "ci-cd-pipeline",
    "description": "Continuous Integration and Deployment pipeline",
    "tasks": [
      {
        "name": "install-deps",
        "command": "npm install",
        "project": "my-app",
        "task_type": "simple",
        "priority": "normal",
        "timeout": 300
      },
      {
        "name": "run-tests",
        "command": "npm test",
        "project": "my-app",
        "task_type": "dependent",
        "priority": "normal",
        "dependencies": ["install-deps"],
        "dependency_conditions": {
          "install-deps": "success"
        },
        "timeout": 600
      },
      {
        "name": "build-app",
        "command": "npm run build",
        "project": "my-app",
        "task_type": "dependent",
        "priority": "normal",
        "dependencies": ["run-tests"],
        "dependency_conditions": {
          "run-tests": "success"
        },
        "timeout": 300
      },
      {
        "name": "deploy-staging",
        "command": "npm run deploy:staging",
        "project": "my-app",
        "task_type": "dependent",
        "priority": "high",
        "dependencies": ["build-app"],
        "dependency_conditions": {
          "build-app": "success"
        },
        "timeout": 600
      }
    ],
    "timeout": 3600,
    "parallel_execution": false,
    "manual_approval": false
  }'

# Check workflow status
curl http://localhost:16080/api/v1/workflows/{workflow_id}/status

# Get workflow result when completed
curl http://localhost:16080/api/v1/workflows/{workflow_id}/result
```

This API provides comprehensive functionality for managing tasks and workflows in the Task Queue system, with full support for dependency management, retry logic, and workflow orchestration.

