# Python SDK Technical Specification

## Implementation Overview

This document provides comprehensive technical specification for the Task Queue Python SDK implementation, including architecture decisions, API contracts, data structures, and integration points.

## Architecture Decision Records (ADRs)

### ADR 1: Async-First Design Pattern
**Decision**: Implement async client as primary interface with sync wrapper
**Context**: Modern Python applications increasingly use async/await patterns for I/O bound operations
**Rationale**:
- Better performance for HTTP operations
- Alignment with modern Python best practices
- Future-proof design for concurrent operations
- Single implementation with sync wrapper reduces maintenance burden
**Alternatives Considered**:
- Sync-first with async wrapper (rejected due to performance implications)
- Separate sync/async implementations (rejected due to code duplication)
**Impact**: All internal operations are async, sync client uses `asyncio.run()`

### ADR 2: Pydantic v2 for Data Validation
**Decision**: Use Pydantic v2 for all data models and validation
**Context**: Strong typing and runtime validation are critical for API reliability
**Rationale**:
- Excellent type safety with full type hints support
- Automatic JSON schema generation
- Runtime validation prevents invalid API calls
- Rich error messages for debugging
- Performance optimized for high-throughput scenarios
**Alternatives Considered**:
- dataclasses with custom validation (rejected due to less robust validation)
- marshmallow (rejected due to slower performance)
- attrs (rejected due to less mature ecosystem)
**Impact**: All data structures use Pydantic BaseModel with strict validation

### ADR 3: HTTPX for HTTP Operations
**Decision**: Use HTTPX as the HTTP client library
**Context**: Need a robust, modern HTTP client supporting both sync and async operations
**Rationale**:
- Same API for sync and async operations (single maintenance point)
- Excellent async performance
- Built-in connection pooling and timeout management
- Modern TLS/SSL support
- Comprehensive error handling
**Alternatives Considered**:
- requests + aiohttp (rejected due to dual dependency management)
- urllib3 (rejected due to less modern API)
- aiohttp only (rejected due to sync operation requirements)
**Impact**: HTTP client is abstracted behind AsyncTaskQueueClient interface

### ADR 4: Custom Exception Hierarchy
**Decision**: Implement comprehensive custom exception classes
**Context**: API consumers need specific, actionable error information
**Rationale**:
- Precise error classification for different failure modes
- HTTP status code preservation in exceptions
- Rich error context for debugging
- Type-safe error handling in consuming applications
- Prevents silent failures through specific exception types
**Alternatives Considered**:
- Standard exceptions with error codes (rejected due to less type safety)
- Single generic exception (rejected due to poor developer experience)
**Impact**: Exception hierarchy mirrors HTTP error types and API-specific errors

### ADR 5: Click Framework for CLI
**Decision**: Use Click for command-line interface implementation
**Context**: Need a professional CLI with good UX and maintainable code
**Rationale**:
- Mature, battle-tested framework
- Automatic help generation and command discovery
- Composable command structure
- Excellent error handling and validation
- Rich ecosystem of extensions
- Professional help formatting
**Alternatives Considered**:
- argparse (rejected due to verbose boilerplate)
- typer (rejected due to less mature ecosystem)
- custom CLI framework (rejected due to maintenance burden)
**Impact**: CLI provides feature parity with programmatic API

## API Contracts

### Core Client Interface

```python
class AsyncTaskQueueClient:
    """Primary async client interface"""

    def __init__(
        self,
        base_url: str = "http://localhost:8080",
        timeout: float = 30.0,
        headers: Optional[Dict[str, str]] = None,
        max_connections: int = 100,
        max_keepalive_connections: int = 20,
        retry_attempts: int = 3,
        retry_backoff: float = 1.0
    ) -> None:
        """Initialize async client with connection pooling and retry logic"""

    # Context manager support
    async def __aenter__(self) -> "AsyncTaskQueueClient":
    async def __aexit__(self, exc_type, exc_val, exc_tb) -> None:

    # Core task operations
    async def create_task(self, name: str, command: str, project_id: str, **kwargs) -> Task:
    async def get_task(self, task_id: str) -> Task:
    async def list_tasks(self, **filters) -> List[Task]:
    async def update_task(self, task_id: str, **updates) -> Task:
    async def cancel_task(self, task_id: str) -> Task:
    async def delete_task(self, task_id: str) -> bool:

    # Project operations
    async def create_project(self, name: str, **kwargs) -> Project:
    async def get_project(self, project_id: str) -> Project:
    async def list_projects(self) -> List[Project]:
    async def update_project(self, project_id: str, **updates) -> Project:
    async def delete_project(self, project_id: str) -> bool:

    # Batch operations
    async def create_tasks(self, tasks_data: List[Dict[str, Any]]) -> List[Task]:
    async def wait_for_completion(self, task_id: str, timeout: int = 300) -> Task:
```

### Sync Client Wrapper

```python
class TaskQueueClient:
    """Synchronous wrapper for AsyncTaskQueueClient"""

    def __init__(self, **kwargs) -> None:
        """Initialize sync client - delegates to async implementation"""

    def __enter__(self) -> "TaskQueueClient":
    def __exit__(self, exc_type, exc_val, exc_tb) -> None:

    # Synchronous versions of all async methods
    def create_task(self, **kwargs) -> Task:
    def get_task(self, task_id: str) -> Task:
    def list_tasks(self, **kwargs) -> List[Task]:
    def update_task(self, task_id: str, **kwargs) -> Task:
    def cancel_task(self, task_id: str) -> Task:
    def delete_task(self, task_id: str) -> bool:

    # Project operations (sync versions)
    def create_project(self, **kwargs) -> Project:
    def get_project(self, project_id: str) -> Project:
    def list_projects(self) -> List[Project]:
    def update_project(self, project_id: str, **kwargs) -> Project:
    def delete_project(self, project_id: str) -> bool:

    # Batch operations (sync versions)
    def create_tasks(self, tasks_data: List[Dict[str, Any]]) -> List[Task]:
    def wait_for_completion(self, task_id: str, timeout: int = 300) -> Task:
```

## Data Structures

### Core Models

```python
# Enums
class TaskStatus(str, Enum):
    PLANNING = "Planning"
    IMPLEMENTATION = "Implementation"
    TEST_CREATION = "TestCreation"
    TESTING = "Testing"
    AI_REVIEW = "AIReview"
    FINALIZED = "Finalized"
    PENDING = "Pending"
    RUNNING = "Running"
    COMPLETED = "Completed"
    FAILED = "Failed"
    CANCELLED = "Cancelled"

class TaskPriority(str, Enum):
    LOW = "Low"
    NORMAL = "Normal"
    HIGH = "High"
    CRITICAL = "Critical"

class TaskType(str, Enum):
    SIMPLE = "Simple"
    WORKFLOW = "Workflow"
    SCHEDULED = "Scheduled"

# Core entities
@dataclass
class Task:
    id: UUID4
    name: str
    command: str
    description: str = ""
    technical_specs: Optional[str] = None
    acceptance_criteria: List[str] = field(default_factory=list)
    project: Optional[str] = None
    task_type: TaskType = TaskType.SIMPLE
    priority: TaskPriority = TaskPriority.NORMAL
    project_id: Optional[UUID4] = None
    dependencies: List[Dict[str, Any]] = field(default_factory=list)
    timeout: Optional[int] = None
    retry_attempts: int = 3
    retry_delay: int = 30
    environment: Dict[str, str] = field(default_factory=dict)
    working_directory: Optional[str] = None
    created_at: datetime
    updated_at: datetime
    status: TaskStatus = TaskStatus.PLANNING
    result: Optional[Dict[str, Any]] = None
    phases: List[TaskPhase] = field(default_factory=list)
    current_phase: TaskStatus = TaskStatus.PLANNING
    ai_reviews_required: int = 3
    ai_reviews_completed: int = 0
    development_workflow: Optional[DevelopmentWorkflow] = None
    metadata: Dict[str, Any] = field(default_factory=dict)

@dataclass
class Project:
    id: UUID4
    name: str
    description: Optional[str] = None
    status: str = "Planning"
    created_at: datetime
    updated_at: datetime
    due_date: Optional[datetime] = None
    tags: List[str] = field(default_factory=list)
    metadata: Dict[str, Any] = field(default_factory=dict)
```

### Request/Response Models

```python
@dataclass
class TaskCreateRequest:
    name: str = field(..., metadata={"validate": "min_length(1)"})
    command: str = field(..., metadata={"validate": "min_length(1)"})
    project_id: UUID4
    description: Optional[str] = None
    technical_specs: Optional[str] = None
    acceptance_criteria: List[str] = field(default_factory=list)
    priority: TaskPriority = TaskPriority.NORMAL
    timeout: Optional[int] = None
    retry_attempts: int = 3
    retry_delay: int = 30
    environment: Dict[str, str] = field(default_factory=dict)
    working_directory: Optional[str] = None

@dataclass
class TaskFilters:
    status: Optional[TaskStatus] = None
    project_id: Optional[UUID4] = None
    priority: Optional[TaskPriority] = None
    task_type: Optional[TaskType] = None
    created_after: Optional[datetime] = None
    created_before: Optional[datetime] = None
    limit: int = 100
    offset: int = 0
```

## Exception Hierarchy

```python
class TaskQueueError(Exception):
    """Base exception for all Task Queue operations"""

    def __init__(
        self,
        message: str,
        status_code: Optional[int] = None,
        response_data: Optional[Dict[str, Any]] = None
    ):
        super().__init__(message)
        self.message = message
        self.status_code = status_code
        self.response_data = response_data or {}

    def __str__(self) -> str:
        if self.status_code:
            return f"[{self.status_code}] {self.message}"
        return self.message

# Specific exceptions
class ValidationError(TaskQueueError): pass
class AuthenticationError(TaskQueueError): pass
class AuthorizationError(TaskQueueError): pass
class TaskNotFoundError(TaskQueueError): pass
class ProjectNotFoundError(TaskQueueError): pass
class APIError(TaskQueueError): pass
class ConnectionError(TaskQueueError): pass
class TimeoutError(TaskQueueError): pass
class RateLimitError(TaskQueueError): pass
```

## Integration Points

### HTTP API Endpoints

The SDK integrates with the following Task Queue HTTP API endpoints:

#### Task Management
- `POST /api/tasks` - Create task
- `GET /api/tasks/{id}` - Get task details
- `GET /api/tasks` - List tasks with filtering
- `PUT /api/tasks/{id}` - Update task
- `POST /api/tasks/{id}/cancel` - Cancel task
- `DELETE /api/tasks/{id}` - Delete task

#### Project Management
- `POST /api/projects` - Create project
- `GET /api/projects/{id}` - Get project details
- `GET /api/projects` - List projects
- `PUT /api/projects/{id}` - Update project
- `DELETE /api/projects/{id}` - Delete project

### Authentication Integration

```python
# Future authentication support
class AuthConfig:
    token: Optional[str] = None
    api_key: Optional[str] = None
    username: Optional[str] = None
    password: Optional[str] = None

    def get_headers(self) -> Dict[str, str]:
        headers = {}
        if self.token:
            headers["Authorization"] = f"Bearer {self.token}"
        elif self.api_key:
            headers["X-API-Key"] = self.api_key
        return headers
```

## Implementation Details

### Connection Management

```python
class ConnectionConfig:
    """HTTP connection configuration"""

    base_url: str
    timeout: float = 30.0
    max_connections: int = 100
    max_keepalive: int = 20
    retry_attempts: int = 3
    retry_backoff: float = 1.0

    @property
    def httpx_limits(self) -> httpx.Limits:
        return httpx.Limits(
            max_connections=self.max_connections,
            max_keepalive_connections=self.max_keepalive
        )

    @property
    def httpx_timeout(self) -> httpx.Timeout:
        return httpx.Timeout(self.timeout)
```

### Request/Response Handling

```python
class APIResponse:
    """Standardized API response wrapper"""

    status_code: int
    data: Dict[str, Any]
    headers: Dict[str, str]
    success: bool

    @classmethod
    def from_httpx_response(cls, response: httpx.Response) -> "APIResponse":
        try:
            data = response.json()
        except ValueError:
            data = {"message": response.text}

        return cls(
            status_code=response.status_code,
            data=data,
            headers=dict(response.headers),
            success=response.status_code < 400
        )

    def raise_for_status(self) -> None:
        """Raise appropriate exception for error responses"""
        if not self.success:
            # Map status codes to specific exceptions
            raise APIError(
                self.data.get("message", "API request failed"),
                self.status_code,
                self.data
            )
```

### Retry Logic Implementation

```python
class RetryConfig:
    """Configuration for retry behavior"""

    max_attempts: int = 3
    backoff_factor: float = 1.0
    status_codes: Set[int] = {429, 500, 502, 503, 504}
    retry_on_methods: Set[str] = {"GET", "POST", "PUT", "DELETE"}

    def should_retry(self, response: APIResponse, attempt: int) -> bool:
        """Determine if request should be retried"""
        if attempt >= self.max_attempts:
            return False

        if response.status_code not in self.status_codes:
            return False

        return True

    def get_backoff_delay(self, attempt: int) -> float:
        """Calculate backoff delay for retry attempt"""
        return self.backoff_factor * (2 ** attempt)
```

## CLI Implementation

### Command Structure

```bash
taskqueue [OPTIONS] COMMAND [ARGS]...

Commands:
  tasks     Task management operations
  projects  Project management operations

Options:
  --base-url TEXT     Task Queue API base URL
  --verbose          Enable verbose output
  --help             Show this message and exit
```

### Task Commands

```bash
taskqueue tasks create [OPTIONS] NAME COMMAND

Options:
  --project-id TEXT   Project ID  [required]
  --description TEXT  Task description
  --priority [Low|Normal|High|Critical]
  --timeout INTEGER   Timeout in seconds
  --help              Show this message and exit

taskqueue tasks list [OPTIONS]

Options:
  --status TEXT       Filter by status
  --project-id TEXT   Filter by project ID
  --priority TEXT     Filter by priority
  --limit INTEGER     Maximum tasks to show
  --format [table|json]  Output format
  --help              Show this message and exit
```

## Testing Strategy

### Unit Tests
- Model validation tests
- Exception handling tests
- Utility function tests
- CLI argument parsing tests

### Integration Tests
- HTTP client tests with mock server
- End-to-end API flow tests
- Error scenario tests
- Rate limiting tests

### Performance Tests
- Concurrent request handling
- Memory usage benchmarks
- Connection pool efficiency tests

## Packaging and Distribution

### Package Structure
```
taskqueue/
├── __init__.py          # Package exports
├── client.py            # Main client implementations
├── models.py            # Data models and enums
├── exceptions.py        # Exception hierarchy
├── cli.py              # Command-line interface
├── py.typed            # Type hint marker
└── pyproject.toml      # Package configuration

tests/
├── test_models.py      # Model tests
├── test_client.py      # Client tests
└── test_cli.py        # CLI tests

examples/
└── basic_usage.py      # Usage examples

docs/
└── *.md               # Documentation
```

### Build Configuration

```toml
[build-system]
requires = ["setuptools>=61.0", "wheel"]
build-backend = "setuptools.build_meta"

[project]
name = "taskqueue-sdk"
version = "0.1.0"
description = "Python SDK for Task Queue API"
dependencies = [
    "httpx>=0.24.0",
    "pydantic>=2.0.0",
    "click>=8.0.0",
    "rich>=13.0.0",
    "typing-extensions>=4.0.0"
]

[project.scripts]
taskqueue = "taskqueue.cli:main"
```

## Security Considerations

### Input Validation
- All user inputs validated via Pydantic models
- SQL injection prevention through parameterized queries
- XSS prevention in CLI output formatting

### Authentication
- Token-based authentication support (future)
- API key authentication support (future)
- Secure credential storage guidelines

### Network Security
- TLS/SSL certificate validation
- Timeout protections against DoS
- Rate limiting awareness
- Secure header handling

## Future Enhancements

### Planned Features
- [ ] Webhook callback support
- [ ] Streaming response handling
- [ ] Bulk operation optimizations
- [ ] Advanced retry strategies
- [ ] Caching layer integration
- [ ] Metrics and monitoring hooks

### Performance Optimizations
- [ ] HTTP/2 support
- [ ] Connection multiplexing
- [ ] Request batching
- [ ] Response compression
- [ ] Memory-efficient streaming

### Developer Experience
- [ ] Interactive CLI mode
- [ ] Configuration file support
- [ ] Plugin architecture
- [ ] IDE integration helpers
- [ ] Comprehensive logging

---

This technical specification provides the complete blueprint for implementing the Task Queue Python SDK, ensuring consistency, reliability, and maintainability across the entire codebase.
