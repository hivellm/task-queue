"""Task Queue API client with sync and async support"""

import asyncio
from contextlib import asynccontextmanager
from typing import List, Optional, Dict, Any, Union
from urllib.parse import urljoin

import httpx
from pydantic import ValidationError

from .models import (
    Task, Project, TaskStatus, TaskPriority, TaskType,
    TaskCreateRequest, TaskUpdateRequest, ProjectCreateRequest, TaskFilters
)
from .exceptions import (
    TaskQueueError, ValidationError as TQValidationError, APIError,
    TaskNotFoundError, ProjectNotFoundError, ConnectionError, TimeoutError, RateLimitError
)


class AsyncTaskQueueClient:
    """Async client for Task Queue API operations"""

    def __init__(
        self,
        base_url: str = "http://localhost:8080",
        timeout: float = 30.0,
        headers: Optional[Dict[str, str]] = None,
        max_connections: int = 100,
        max_keepalive_connections: int = 20,
        retry_attempts: int = 3,
        retry_backoff: float = 1.0
    ):
        self.base_url = base_url.rstrip('/')
        self.timeout = timeout
        self.retry_attempts = retry_attempts
        self.retry_backoff = retry_backoff

        # Setup headers
        self.headers = headers or {}
        self.headers.update({
            'Content-Type': 'application/json',
            'User-Agent': f'TaskQueue-Python-SDK/0.1.0'
        })

        # HTTP client configuration
        self._client_kwargs = {
            'timeout': timeout,
            'headers': self.headers,
            'limits': httpx.Limits(
                max_connections=max_connections,
                max_keepalive_connections=max_keepalive_connections
            )
        }

        self._client: Optional[httpx.AsyncClient] = None

    async def __aenter__(self):
        await self._ensure_client()
        return self

    async def __aexit__(self, exc_type, exc_val, exc_tb):
        await self.close()

    async def _ensure_client(self):
        """Ensure HTTP client is initialized"""
        if self._client is None:
            self._client = httpx.AsyncClient(**self._client_kwargs)

    async def close(self):
        """Close the HTTP client"""
        if self._client:
            await self._client.aclose()
            self._client = None

    async def _make_request(
        self,
        method: str,
        endpoint: str,
        data: Optional[Dict[str, Any]] = None,
        params: Optional[Dict[str, Any]] = None
    ) -> Dict[str, Any]:
        """Make HTTP request with retry logic"""
        await self._ensure_client()

        url = urljoin(self.base_url + '/', endpoint.lstrip('/'))

        for attempt in range(self.retry_attempts):
            try:
                if method.upper() == 'GET':
                    response = await self._client.get(url, params=params)
                elif method.upper() == 'POST':
                    response = await self._client.post(url, json=data)
                elif method.upper() == 'PUT':
                    response = await self._client.put(url, json=data)
                elif method.upper() == 'DELETE':
                    response = await self._client.delete(url)
                else:
                    raise ValueError(f"Unsupported HTTP method: {method}")

                return await self._handle_response(response)

            except httpx.TimeoutException as e:
                if attempt == self.retry_attempts - 1:
                    raise TimeoutError(f"Request timeout after {self.retry_attempts} attempts") from e
                await asyncio.sleep(self.retry_backoff * (2 ** attempt))

            except httpx.ConnectError as e:
                if attempt == self.retry_attempts - 1:
                    raise ConnectionError(f"Connection failed after {self.retry_attempts} attempts") from e
                await asyncio.sleep(self.retry_backoff * (2 ** attempt))

            except Exception as e:
                if attempt == self.retry_attempts - 1:
                    raise TaskQueueError(f"Request failed: {str(e)}") from e
                await asyncio.sleep(self.retry_backoff * (2 ** attempt))

    async def _handle_response(self, response: httpx.Response) -> Dict[str, Any]:
        """Handle HTTP response and extract JSON data"""
        try:
            data = response.json()
        except ValueError:
            data = {'message': response.text}

        if response.status_code >= 400:
            await self._handle_error_response(response.status_code, data)

        return data

    async def _handle_error_response(self, status_code: int, data: Dict[str, Any]):
        """Handle API error responses"""
        message = data.get('message', 'Unknown error')

        if status_code == 400:
            raise TQValidationError(message, status_code, data)
        elif status_code == 401:
            from .exceptions import AuthenticationError
            raise AuthenticationError(message, status_code, data)
        elif status_code == 403:
            from .exceptions import AuthorizationError
            raise AuthorizationError(message, status_code, data)
        elif status_code == 404:
            error_message = data.get('message', '')
            if 'task' in error_message.lower():
                raise TaskNotFoundError(message, status_code, data)
            elif 'project' in error_message.lower():
                raise ProjectNotFoundError(message, status_code, data)
            else:
                raise APIError(message, status_code, data)
        elif status_code == 429:
            raise RateLimitError(message, status_code, data)
        else:
            raise APIError(message, status_code, data)

    # Task Operations
    async def create_task(
        self,
        name: str,
        command: str,
        project_id: str,
        description: Optional[str] = None,
        technical_specs: Optional[str] = None,
        acceptance_criteria: Optional[List[str]] = None,
        priority: TaskPriority = TaskPriority.NORMAL,
        timeout: Optional[int] = None,
        retry_attempts: int = 3,
        retry_delay: int = 30,
        environment: Optional[Dict[str, str]] = None,
        working_directory: Optional[str] = None
    ) -> Task:
        """Create a new task"""
        request_data = TaskCreateRequest(
            name=name,
            command=command,
            project_id=project_id,
            description=description,
            technical_specs=technical_specs,
            acceptance_criteria=acceptance_criteria or [],
            priority=priority,
            timeout=timeout,
            retry_attempts=retry_attempts,
            retry_delay=retry_delay,
            environment=environment or {},
            working_directory=working_directory
        )

        try:
            response_data = await self._make_request('POST', '/api/tasks', request_data.model_dump())
            return Task(**response_data)
        except TQValidationError:
            raise
        except Exception as e:
            raise TaskQueueError(f"Failed to create task: {str(e)}") from e

    async def get_task(self, task_id: str) -> Task:
        """Get task by ID"""
        try:
            response_data = await self._make_request('GET', f'/api/tasks/{task_id}')
            return Task(**response_data)
        except APIError as e:
            if e.status_code == 404:
                raise TaskNotFoundError(f"Task {task_id} not found") from e
            raise

    async def list_tasks(
        self,
        status: Optional[TaskStatus] = None,
        project_id: Optional[str] = None,
        priority: Optional[TaskPriority] = None,
        task_type: Optional[TaskType] = None,
        limit: int = 100,
        offset: int = 0
    ) -> List[Task]:
        """List tasks with optional filters"""
        params = {
            'limit': limit,
            'offset': offset
        }

        if status:
            params['status'] = status.value
        if project_id:
            params['project_id'] = project_id
        if priority:
            params['priority'] = priority.value
        if task_type:
            params['task_type'] = task_type.value

        response_data = await self._make_request('GET', '/api/tasks', params=params)

        # Assume response has a 'tasks' key with list of tasks
        tasks_data = response_data.get('tasks', response_data)
        if not isinstance(tasks_data, list):
            tasks_data = [response_data]

        return [Task(**task_data) for task_data in tasks_data]

    async def update_task(
        self,
        task_id: str,
        name: Optional[str] = None,
        command: Optional[str] = None,
        description: Optional[str] = None,
        technical_specs: Optional[str] = None,
        acceptance_criteria: Optional[List[str]] = None,
        priority: Optional[TaskPriority] = None,
        status: Optional[TaskStatus] = None,
        timeout: Optional[int] = None,
        retry_attempts: Optional[int] = None,
        retry_delay: Optional[int] = None,
        environment: Optional[Dict[str, str]] = None,
        working_directory: Optional[str] = None
    ) -> Task:
        """Update an existing task"""
        update_data = TaskUpdateRequest(
            name=name,
            command=command,
            description=description,
            technical_specs=technical_specs,
            acceptance_criteria=acceptance_criteria,
            priority=priority,
            status=status,
            timeout=timeout,
            retry_attempts=retry_attempts,
            retry_delay=retry_delay,
            environment=environment,
            working_directory=working_directory
        )

        # Remove None values
        update_dict = update_data.model_dump(exclude_unset=True)

        try:
            response_data = await self._make_request('PUT', f'/api/tasks/{task_id}', update_dict)
            return Task(**response_data)
        except APIError as e:
            if e.status_code == 404:
                raise TaskNotFoundError(f"Task {task_id} not found") from e
            raise

    async def cancel_task(self, task_id: str) -> Task:
        """Cancel a running task"""
        try:
            response_data = await self._make_request('POST', f'/api/tasks/{task_id}/cancel')
            return Task(**response_data)
        except APIError as e:
            if e.status_code == 404:
                raise TaskNotFoundError(f"Task {task_id} not found") from e
            raise

    async def delete_task(self, task_id: str) -> bool:
        """Delete a task"""
        try:
            await self._make_request('DELETE', f'/api/tasks/{task_id}')
            return True
        except APIError as e:
            if e.status_code == 404:
                raise TaskNotFoundError(f"Task {task_id} not found") from e
            return False

    # Project Operations
    async def create_project(
        self,
        name: str,
        description: Optional[str] = None,
        due_date: Optional[str] = None,
        tags: Optional[List[str]] = None
    ) -> Project:
        """Create a new project"""
        request_data = ProjectCreateRequest(
            name=name,
            description=description,
            due_date=due_date,
            tags=tags or []
        )

        try:
            response_data = await self._make_request('POST', '/api/projects', request_data.model_dump())
            return Project(**response_data)
        except Exception as e:
            raise TaskQueueError(f"Failed to create project: {str(e)}") from e

    async def get_project(self, project_id: str) -> Project:
        """Get project by ID"""
        try:
            response_data = await self._make_request('GET', f'/api/projects/{project_id}')
            return Project(**response_data)
        except APIError as e:
            if e.status_code == 404:
                raise ProjectNotFoundError(f"Project {project_id} not found") from e
            raise

    async def list_projects(self) -> List[Project]:
        """List all projects"""
        response_data = await self._make_request('GET', '/api/projects')

        # Assume response has a 'projects' key or is a list
        projects_data = response_data.get('projects', response_data)
        if not isinstance(projects_data, list):
            projects_data = [response_data]

        return [Project(**project_data) for project_data in projects_data]

    async def update_project(
        self,
        project_id: str,
        name: Optional[str] = None,
        description: Optional[str] = None,
        status: Optional[str] = None,
        due_date: Optional[str] = None,
        tags: Optional[List[str]] = None
    ) -> Project:
        """Update an existing project"""
        update_data = {}
        if name is not None:
            update_data['name'] = name
        if description is not None:
            update_data['description'] = description
        if status is not None:
            update_data['status'] = status
        if due_date is not None:
            update_data['due_date'] = due_date
        if tags is not None:
            update_data['tags'] = tags

        try:
            response_data = await self._make_request('PUT', f'/api/projects/{project_id}', update_data)
            return Project(**response_data)
        except APIError as e:
            if e.status_code == 404:
                raise ProjectNotFoundError(f"Project {project_id} not found") from e
            raise

    async def delete_project(self, project_id: str) -> bool:
        """Delete a project"""
        try:
            await self._make_request('DELETE', f'/api/projects/{project_id}')
            return True
        except APIError as e:
            if e.status_code == 404:
                raise ProjectNotFoundError(f"Project {project_id} not found") from e
            return False

    # Batch Operations
    async def create_tasks(self, tasks_data: List[Dict[str, Any]]) -> List[Task]:
        """Create multiple tasks in batch"""
        created_tasks = []

        # Validate all tasks first
        validated_tasks = []
        for task_data in tasks_data:
            try:
                validated = TaskCreateRequest(**task_data)
                validated_tasks.append(validated)
            except ValidationError as e:
                raise TQValidationError(f"Invalid task data: {e}") from e

        # Create tasks (could be optimized with concurrent requests)
        for task_request in validated_tasks:
            try:
                task = await self.create_task(**task_request.model_dump())
                created_tasks.append(task)
            except Exception as e:
                # Continue with other tasks but log error
                print(f"Failed to create task {task_request.name}: {e}")
                continue

        return created_tasks

    async def wait_for_completion(self, task_id: str, timeout: int = 300) -> Task:
        """Wait for task completion with timeout"""
        import time
        start_time = time.time()

        while time.time() - start_time < timeout:
            task = await self.get_task(task_id)

            if task.status in [TaskStatus.COMPLETED, TaskStatus.FAILED, TaskStatus.CANCELLED]:
                return task

            await asyncio.sleep(2)  # Poll every 2 seconds

        raise TimeoutError(f"Task {task_id} did not complete within {timeout} seconds")


class TaskQueueClient:
    """Synchronous wrapper for AsyncTaskQueueClient"""

    def __init__(self, **kwargs):
        self._async_client = AsyncTaskQueueClient(**kwargs)

    def __enter__(self):
        # For sync context manager support
        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        # Close async client
        asyncio.run(self._async_client.close())

    def create_task(self, **kwargs) -> Task:
        """Create a new task"""
        return asyncio.run(self._async_client.create_task(**kwargs))

    def get_task(self, task_id: str) -> Task:
        """Get task by ID"""
        return asyncio.run(self._async_client.get_task(task_id))

    def list_tasks(self, **kwargs) -> List[Task]:
        """List tasks with optional filters"""
        return asyncio.run(self._async_client.list_tasks(**kwargs))

    def update_task(self, task_id: str, **kwargs) -> Task:
        """Update an existing task"""
        return asyncio.run(self._async_client.update_task(task_id, **kwargs))

    def cancel_task(self, task_id: str) -> Task:
        """Cancel a running task"""
        return asyncio.run(self._async_client.cancel_task(task_id))

    def delete_task(self, task_id: str) -> bool:
        """Delete a task"""
        return asyncio.run(self._async_client.delete_task(task_id))

    def create_project(self, **kwargs) -> Project:
        """Create a new project"""
        return asyncio.run(self._async_client.create_project(**kwargs))

    def get_project(self, project_id: str) -> Project:
        """Get project by ID"""
        return asyncio.run(self._async_client.get_project(project_id))

    def list_projects(self) -> List[Project]:
        """List all projects"""
        return asyncio.run(self._async_client.list_projects())

    def update_project(self, project_id: str, **kwargs) -> Project:
        """Update an existing project"""
        return asyncio.run(self._async_client.update_project(project_id, **kwargs))

    def delete_project(self, project_id: str) -> bool:
        """Delete a project"""
        return asyncio.run(self._async_client.delete_project(project_id))

    def create_tasks(self, tasks_data: List[Dict[str, Any]]) -> List[Task]:
        """Create multiple tasks in batch"""
        return asyncio.run(self._async_client.create_tasks(tasks_data))

    def wait_for_completion(self, task_id: str, timeout: int = 300) -> Task:
        """Wait for task completion with timeout"""
        return asyncio.run(self._async_client.wait_for_completion(task_id, timeout))
