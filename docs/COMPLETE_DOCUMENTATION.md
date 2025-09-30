# Task Queue System - Complete Documentation

## Table of Contents
1. [Overview](#overview)
2. [Architecture](#architecture)
3. [API Reference](#api-reference)
4. [Configuration](#configuration)
5. [Deployment](#deployment)
6. [Development Guide](#development-guide)
7. [User Guide](#user-guide)
8. [Troubleshooting](#troubleshooting)
9. [Contributing](#contributing)

## Overview

The Task Queue System is a comprehensive, enterprise-grade task management platform designed to handle complex workflows, project management, and task orchestration. Built with Rust for performance and reliability, it provides both HTTP API and MCP (Model Context Protocol) interfaces for seamless integration with AI systems and external applications.

### Key Features

- **Task Management**: Create, update, delete, and monitor tasks with full lifecycle tracking
- **Project Organization**: Group related tasks into projects with hierarchical management
- **Workflow Engine**: Advanced workflow management with dependency resolution
- **Development Workflow**: Structured 5-phase development process (Planning → Implementation → Test Creation → Testing → AI Review)
- **MCP Integration**: Native support for AI model interaction through Model Context Protocol
- **Multiple Interfaces**: HTTP REST API, CLI tool, Python SDK, and interactive dashboard
- **Real-time Monitoring**: Live task status updates and progress tracking
- **Extensible Architecture**: Plugin system for custom functionality

### Technology Stack

- **Backend**: Rust with Axum web framework
- **Database**: Sled embedded key-value store
- **Frontend**: Vue.js dashboard with real-time updates
- **CLI**: Rust-based command-line interface with clap
- **SDK**: Python client library with async/sync support
- **Protocol**: MCP (Model Context Protocol) for AI integration

## Architecture

### System Architecture

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Web Dashboard │    │   CLI Tool      │    │   Python SDK    │
│   (Vue.js)      │    │   (Rust)        │    │   (Python)      │
└─────────┬───────┘    └─────────┬───────┘    └─────────┬───────┘
          │                      │                      │
          └──────────────────────┼──────────────────────┘
                                 │
                    ┌─────────────▼─────────────┐
                    │     HTTP REST API         │
                    │     (Axum Framework)      │
                    └─────────────┬─────────────┘
                                 │
                    ┌─────────────▼─────────────┐
                    │     MCP Server            │
                    │     (Model Context        │
                    │      Protocol)             │
                    └─────────────┬─────────────┘
                                 │
                    ┌─────────────▼─────────────┐
                    │   Task Queue Core         │
                    │   (Rust Business Logic)   │
                    └─────────────┬─────────────┘
                                 │
                    ┌─────────────▼─────────────┐
                    │   Sled Database           │
                    │   (Persistent Storage)    │
                    └───────────────────────────┘
```

### Core Components

#### 1. Task Queue Core (`src/core.rs`)
- **Task Management**: Core task data structures and business logic
- **Project Management**: Project organization and hierarchy
- **Workflow Engine**: Development workflow state management
- **Validation**: Input validation and business rules

#### 2. HTTP Server (`src/server.rs`)
- **REST API**: HTTP endpoints for all operations
- **Request Handling**: Axum-based request processing
- **Error Handling**: Comprehensive error management
- **Middleware**: Authentication, logging, and CORS

#### 3. MCP Server (`src/mcp.rs`)
- **AI Integration**: Model Context Protocol implementation
- **Tool Definitions**: MCP tool schemas and handlers
- **Workflow Instructions**: Dynamic workflow guidance for AI models
- **Context Management**: AI model context and state

#### 4. Storage Layer (`src/storage.rs`)
- **Database Operations**: Sled database interactions
- **Data Persistence**: Task, project, and workflow storage
- **Transaction Management**: ACID compliance and data integrity

### Data Models

#### Task Structure
```rust
pub struct Task {
    pub id: Uuid,
    pub name: String,
    pub command: String,
    pub description: String,
    pub project_id: Option<Uuid>,
    pub priority: TaskPriority,
    pub status: TaskStatus,
    pub development_workflow: Option<DevelopmentWorkflow>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

#### Development Workflow
```rust
pub struct DevelopmentWorkflow {
    pub technical_documentation_path: Option<String>,
    pub test_coverage_percentage: Option<f64>,
    pub ai_review_reports: Vec<AIDevelopmentReview>,
    pub workflow_status: DevelopmentWorkflowStatus,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
}
```

## API Reference

### HTTP REST API

#### Base URL
```
http://localhost:16080
```

#### Authentication
Currently, the API operates without authentication. Future versions will support:
- API Key authentication
- JWT token-based authentication
- OAuth 2.0 integration

#### Endpoints

##### Tasks

**GET /tasks**
- List all tasks with optional filtering
- Query Parameters:
  - `status`: Filter by task status
  - `project`: Filter by project ID
  - `priority`: Filter by priority level
- Response: Array of Task objects

**POST /tasks**
- Create a new task
- Request Body: Task creation data
- Response: Created Task object

**GET /tasks/{id}**
- Get task details by ID
- Response: Task object

**PUT /tasks/{id}**
- Update task by ID
- Request Body: Task update data
- Response: Updated Task object

**DELETE /tasks/{id}**
- Delete task by ID
- Response: Success confirmation

**POST /tasks/{id}/cancel**
- Cancel a running task
- Request Body: `{"reason": "cancellation reason"}`
- Response: Success confirmation

##### Projects

**GET /projects**
- List all projects
- Response: Array of Project objects

**POST /projects**
- Create a new project
- Request Body: Project creation data
- Response: Created Project object

**GET /projects/{id}**
- Get project details by ID
- Response: Project object

**PUT /projects/{id}**
- Update project by ID
- Request Body: Project update data
- Response: Updated Project object

**DELETE /projects/{id}**
- Delete project by ID
- Response: Success confirmation

##### Workflows

**GET /workflows**
- List all workflows
- Response: Array of Workflow objects

**POST /workflows**
- Create a new workflow
- Request Body: Workflow creation data
- Response: Created Workflow object

**GET /workflows/{id}**
- Get workflow details by ID
- Response: Workflow object

##### System

**GET /stats**
- Get system statistics
- Response: ServerStats object

**GET /health**
- Health check endpoint
- Response: Health status

**GET /metrics**
- System metrics (if enabled)
- Response: Metrics data

### MCP (Model Context Protocol) API

The Task Queue system implements MCP to enable AI model integration. AI models can interact with the system through standardized MCP tools.

#### Available MCP Tools

##### Task Management Tools

**submit_task**
- Submit a new task to the queue
- Parameters: `name`, `command`, `project_id`, `description`, `priority`
- Returns: Task creation confirmation with workflow instructions

**get_task**
- Retrieve task details and current workflow status
- Parameters: `task_id`
- Returns: Task object with workflow instructions

**list_tasks**
- List tasks with filtering options
- Parameters: `limit`, `status`, `project_id`
- Returns: Array of tasks with workflow status

**update_task**
- Update existing task
- Parameters: `task_id`, `name`, `command`, `priority`, `status`
- Returns: Updated task confirmation

**cancel_task**
- Cancel a running task
- Parameters: `task_id`, `reason`
- Returns: Cancellation confirmation

**delete_task**
- Delete a task
- Parameters: `task_id`
- Returns: Deletion confirmation

##### Project Management Tools

**create_project**
- Create a new project
- Parameters: `name`, `description`
- Returns: Project creation confirmation

**get_project**
- Get project details
- Parameters: `project_id`
- Returns: Project object

**list_projects**
- List all projects
- Returns: Array of projects

##### Workflow Management Tools

**advance_workflow_phase**
- Advance task to next development phase
- Parameters: `task_id`
- Returns: New workflow status and instructions

**set_technical_documentation**
- Mark planning phase complete with documentation
- Parameters: `task_id`, `doc_path`
- Returns: Confirmation and next phase instructions

**set_test_coverage**
- Report test coverage for testing phase
- Parameters: `task_id`, `coverage`
- Returns: Confirmation and next phase instructions

**add_ai_review_report**
- Add AI review report for AI Review phase
- Parameters: `task_id`, `model_name`, `review_type`, `content`, `score`, `approved`, `suggestions`
- Returns: Confirmation and next phase instructions

## Configuration

### Server Configuration

The Task Queue server can be configured through environment variables or configuration files.

#### Environment Variables

```bash
# Server Configuration
TASK_QUEUE_HOST=0.0.0.0
TASK_QUEUE_PORT=16080
TASK_QUEUE_DATABASE_PATH=./task-queue-data/task-queue.db

# Logging Configuration
RUST_LOG=info
LOG_LEVEL=info

# Development Configuration
TASK_QUEUE_DEV_MODE=false
TASK_QUEUE_ENABLE_METRICS=true
```

#### Configuration File

Create a `config.yml` file in the project root:

```yaml
server:
  host: "0.0.0.0"
  port: 16080
  database_path: "./task-queue-data/task-queue.db"

logging:
  level: "info"
  format: "json"

development:
  dev_mode: false
  enable_metrics: true
  enable_cors: true
```

### CLI Configuration

The CLI tool supports configuration through:

1. **Command-line arguments**: Override any setting
2. **Configuration file**: `~/.config/task-queue/config.yaml`
3. **Environment variables**: `TASK_QUEUE_*` prefixed variables

#### CLI Configuration File

```yaml
server:
  url: "http://localhost:16080"
  api_key: null
  timeout: 30
  retry_attempts: 3

ui:
  theme: "Default"
  refresh_interval: 1
  show_progress: true

output:
  default_format: "Table"
  table_style: "Default"
  colors: true
```

## Deployment

### Docker Deployment

#### Dockerfile
```dockerfile
FROM rust:1.75-slim as builder

WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/task-queue /usr/local/bin/task-queue
COPY --from=builder /app/dashboard /usr/local/share/task-queue/dashboard

EXPOSE 16080
CMD ["task-queue"]
```

#### Docker Compose
```yaml
version: '3.8'
services:
  task-queue:
    build: .
    ports:
      - "16080:16080"
    volumes:
      - task-queue-data:/app/data
    environment:
      - TASK_QUEUE_HOST=0.0.0.0
      - TASK_QUEUE_PORT=16080
      - RUST_LOG=info

volumes:
  task-queue-data:
```

### Manual Deployment

#### Prerequisites
- Rust 1.75+ installed
- System dependencies (if any)

#### Build Steps
```bash
# Clone repository
git clone <repository-url>
cd task-queue

# Build release version
cargo build --release

# Run server
./target/release/task-queue
```

#### System Service (systemd)

Create `/etc/systemd/system/task-queue.service`:

```ini
[Unit]
Description=Task Queue Server
After=network.target

[Service]
Type=simple
User=taskqueue
WorkingDirectory=/opt/task-queue
ExecStart=/opt/task-queue/task-queue
Restart=always
RestartSec=5
Environment=RUST_LOG=info

[Install]
WantedBy=multi-user.target
```

Enable and start:
```bash
sudo systemctl enable task-queue
sudo systemctl start task-queue
```

## Development Guide

### Setting Up Development Environment

#### Prerequisites
- Rust 1.75+
- Node.js 18+ (for dashboard development)
- Python 3.8+ (for SDK development)

#### Development Setup
```bash
# Clone repository
git clone <repository-url>
cd task-queue

# Install Rust dependencies
cargo build

# Install Python SDK dependencies
cd sdks/python
pip install -e .

# Install CLI dependencies
cd ../../cli
cargo build
```

### Project Structure

```
task-queue/
├── src/                    # Rust source code
│   ├── core.rs            # Core data structures
│   ├── server.rs          # HTTP server implementation
│   ├── mcp.rs             # MCP server implementation
│   ├── storage.rs         # Database operations
│   └── error.rs           # Error definitions
├── cli/                   # CLI tool
│   ├── src/
│   └── Cargo.toml
├── dashboard/             # Web dashboard
│   ├── public/
│   └── src/
├── sdks/                  # Client SDKs
│   └── python/
├── docs/                  # Documentation
├── tests/                 # Integration tests
└── Cargo.toml
```

### Development Workflow

#### 1. Planning Phase
- Create technical documentation in `/docs`
- Define API contracts and data structures
- Document implementation details and edge cases

#### 2. Implementation Phase
- Implement code according to documentation
- Follow established patterns and conventions
- Ensure code quality and maintainability

#### 3. Test Creation Phase
- Create comprehensive test suite
- Aim for 90%+ code coverage
- Include unit, integration, and end-to-end tests

#### 4. Testing Phase
- Execute all tests
- Fix any failing tests
- Validate test coverage requirements

#### 5. AI Review Phase
- Get reviews from 3 different AI models
- Address critical issues found
- Document review results and improvements

### Code Standards

#### Rust Code Standards
- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting
- Follow Rust naming conventions
- Document all public APIs
- Use meaningful variable and function names

#### Testing Standards
- Unit tests for all public functions
- Integration tests for API endpoints
- End-to-end tests for critical workflows
- Mock external dependencies
- Test error conditions and edge cases

### Contributing Guidelines

#### Pull Request Process
1. Create feature branch from `main`
2. Implement changes following development workflow
3. Add comprehensive tests
4. Update documentation
5. Submit pull request with detailed description

#### Code Review Process
1. Automated tests must pass
2. Code coverage must meet requirements
3. Manual review by maintainers
4. Address all feedback
5. Merge after approval

## User Guide

### Getting Started

#### Installation

**Using Cargo (Recommended)**
```bash
cargo install task-queue-cli
```

**From Source**
```bash
git clone <repository-url>
cd task-queue/cli
cargo build --release
sudo cp target/release/task-queue /usr/local/bin/
```

#### First Steps

1. **Start the Server**
   ```bash
   task-queue
   ```

2. **Access the Dashboard**
   Open http://localhost:16080 in your browser

3. **Use the CLI**
   ```bash
   # List tasks
   task-queue tasks list
   
   # Create a project
   task-queue projects create --name "My Project"
   
   # Create a task
   task-queue tasks create --name "My Task" --command "echo hello" --project <project-id>
   ```

### CLI Usage

#### Basic Commands

**Task Management**
```bash
# List all tasks
task-queue tasks list

# Create a task
task-queue tasks create --name "Task Name" --command "command to run" --project <project-id>

# Get task details
task-queue tasks get <task-id>

# Update a task
task-queue tasks update <task-id> --name "New Name"

# Cancel a task
task-queue tasks cancel <task-id> --reason "Reason for cancellation"

# Delete a task
task-queue tasks delete <task-id>
```

**Project Management**
```bash
# List projects
task-queue projects list

# Create a project
task-queue projects create --name "Project Name" --description "Project description"

# Get project details
task-queue projects get <project-id>

# List project tasks
task-queue projects tasks <project-id>
```

**Server Operations**
```bash
# Check server status
task-queue server status

# Health check
task-queue server health

# View metrics
task-queue server metrics
```

#### Advanced Usage

**Output Formats**
```bash
# JSON output
task-queue tasks list --format json

# YAML output
task-queue tasks list --format yaml

# Table output (default)
task-queue tasks list --format table
```

**Filtering**
```bash
# Filter by status
task-queue tasks list --status pending

# Filter by project
task-queue tasks list --project <project-id>

# Filter by priority
task-queue tasks list --priority high
```

**Configuration**
```bash
# Use custom server
task-queue --server-url http://custom-server:8080 tasks list

# Enable verbose logging
task-queue --verbose tasks list

# Use API key
task-queue --api-key <api-key> tasks list
```

### Python SDK Usage

#### Installation
```bash
pip install task-queue-sdk
```

#### Basic Usage

**Synchronous Client**
```python
from taskqueue import TaskQueueClient

# Create client
client = TaskQueueClient("http://localhost:16080")

# Create a project
project = client.create_project("My Project", "Project description")

# Create a task
task = client.create_task(
    name="My Task",
    command="echo hello",
    project_id=project.id,
    description="Task description"
)

# List tasks
tasks = client.list_tasks()
for task in tasks:
    print(f"Task: {task.name} - Status: {task.status}")
```

**Asynchronous Client**
```python
import asyncio
from taskqueue import AsyncTaskQueueClient

async def main():
    async with AsyncTaskQueueClient("http://localhost:16080") as client:
        # Create a project
        project = await client.create_project("My Project", "Project description")
        
        # Create a task
        task = await client.create_task(
            name="My Task",
            command="echo hello",
            project_id=project.id,
            description="Task description"
        )
        
        # List tasks
        tasks = await client.list_tasks()
        for task in tasks:
            print(f"Task: {task.name} - Status: {task.status}")

# Run async function
asyncio.run(main())
```

#### Error Handling
```python
from taskqueue import TaskQueueClient, TaskQueueError, TaskNotFoundError

client = TaskQueueClient("http://localhost:16080")

try:
    task = client.get_task("invalid-id")
except TaskNotFoundError:
    print("Task not found")
except TaskQueueError as e:
    print(f"Task queue error: {e}")
```

### Dashboard Usage

#### Overview
The web dashboard provides a user-friendly interface for managing tasks and projects.

#### Features
- **Task Management**: Create, edit, and monitor tasks
- **Project Organization**: Manage projects and their associated tasks
- **Real-time Updates**: Live status updates and progress tracking
- **Workflow Visualization**: Visual representation of development workflows
- **Statistics**: System statistics and performance metrics

#### Navigation
- **Dashboard**: Overview of system status and recent activity
- **Tasks**: Task management interface
- **Projects**: Project management interface
- **Workflows**: Workflow management and monitoring
- **Settings**: System configuration and preferences

## Troubleshooting

### Common Issues

#### Server Won't Start

**Port Already in Use**
```bash
# Check what's using port 16080
netstat -tulpn | grep 16080

# Kill the process
sudo kill -9 <process-id>

# Or use a different port
TASK_QUEUE_PORT=16081 task-queue
```

**Database Locked**
```bash
# Check for existing processes
ps aux | grep task-queue

# Kill existing processes
sudo pkill task-queue

# Remove lock files (if safe)
rm -f task-queue-data/task-queue.db-lock
```

#### CLI Connection Issues

**Connection Refused**
```bash
# Check if server is running
curl http://localhost:16080/health

# Check server logs
journalctl -u task-queue -f
```

**Authentication Errors**
```bash
# Check API key configuration
task-queue config show

# Reset configuration
task-queue config reset
```

#### Performance Issues

**High Memory Usage**
- Check for memory leaks in long-running tasks
- Monitor database size and consider cleanup
- Review logging configuration

**Slow Response Times**
- Check database performance
- Monitor network latency
- Review server resource usage

### Debugging

#### Enable Debug Logging
```bash
# Set debug log level
export RUST_LOG=debug
task-queue
```

#### Database Inspection
```bash
# Install sled tools
cargo install sled-tools

# Inspect database
sled-inspect task-queue-data/task-queue.db
```

#### Network Debugging
```bash
# Test API endpoints
curl -v http://localhost:16080/tasks
curl -v http://localhost:16080/projects
curl -v http://localhost:16080/stats
```

### Getting Help

#### Documentation
- Check this documentation for detailed information
- Review API documentation for endpoint details
- Consult CLI help: `task-queue --help`

#### Community Support
- GitHub Issues: Report bugs and request features
- Discussions: Ask questions and share ideas
- Discord: Real-time community support

#### Professional Support
- Enterprise support available
- Custom development services
- Training and consulting

## Contributing

### How to Contribute

#### Reporting Issues
1. Check existing issues first
2. Use the issue template
3. Provide detailed reproduction steps
4. Include system information and logs

#### Suggesting Features
1. Check existing feature requests
2. Describe the use case clearly
3. Explain the expected behavior
4. Consider implementation complexity

#### Code Contributions
1. Fork the repository
2. Create a feature branch
3. Follow the development workflow
4. Submit a pull request

### Development Setup

#### Prerequisites
- Rust 1.75+
- Git
- Basic understanding of Rust and web development

#### Setup Steps
```bash
# Fork and clone repository
git clone https://github.com/your-username/task-queue.git
cd task-queue

# Install dependencies
cargo build

# Run tests
cargo test

# Start development server
cargo run
```

### Code Standards

#### Rust Standards
- Follow Rust naming conventions
- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting
- Write comprehensive tests
- Document public APIs

#### Commit Standards
- Use conventional commit messages
- Keep commits focused and atomic
- Include tests for new features
- Update documentation as needed

### Release Process

#### Version Numbering
- Follow semantic versioning (SemVer)
- Major: Breaking changes
- Minor: New features (backward compatible)
- Patch: Bug fixes (backward compatible)

#### Release Checklist
- [ ] All tests pass
- [ ] Documentation updated
- [ ] Changelog updated
- [ ] Version bumped
- [ ] Release notes prepared
- [ ] Tagged and published

---

## Conclusion

This documentation provides comprehensive coverage of the Task Queue System, from basic usage to advanced development and deployment scenarios. The system is designed to be robust, scalable, and easy to use while providing powerful features for task management and workflow orchestration.

For additional information or support, please refer to the community resources or contact the development team.
