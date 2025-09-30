"""Tests for TaskQueue client - Fixed version"""

import pytest
import pytest_asyncio
import asyncio
from unittest.mock import AsyncMock, MagicMock, patch
from httpx import AsyncClient, Response

from taskqueue.client import AsyncTaskQueueClient, TaskQueueClient
from taskqueue.models import Task, TaskStatus, TaskPriority, TaskCreateRequest
from taskqueue.exceptions import APIError, TaskNotFoundError, ValidationError, TaskQueueError


class TestAsyncTaskQueueClient:
    """Test AsyncTaskQueueClient"""

    @pytest_asyncio.fixture
    async def client(self):
        """Create test client"""
        client = AsyncTaskQueueClient(base_url="http://test.example.com")
        # Mock the _ensure_client method to avoid actual HTTP calls
        client._ensure_client = AsyncMock()
        client._client = AsyncMock()
        yield client
        await client.close()

    @pytest.mark.asyncio
    async def test_create_task_success(self, client):
        """Test successful task creation"""
        # Mock the _make_request method directly
        mock_response_data = {
            "id": "550e8400-e29b-41d4-a716-446655440000",
            "name": "Test Task",
            "command": "echo hello",
            "project_id": "550e8400-e29b-41d4-a716-446655440001",
            "status": "Planning",
            "priority": "Normal",
            "created_at": "2024-01-01T00:00:00Z",
            "updated_at": "2024-01-01T00:00:00Z"
        }
        client._make_request = AsyncMock(return_value=mock_response_data)

        task = await client.create_task(
            name="Test Task",
            command="echo hello",
            project_id="550e8400-e29b-41d4-a716-446655440001"
        )

        assert task.name == "Test Task"
        assert task.command == "echo hello"
        assert task.status == TaskStatus.PLANNING
        assert task.priority == TaskPriority.NORMAL

    @pytest.mark.asyncio
    async def test_create_task_validation_error(self, client):
        """Test task creation with validation error"""
        # Test that Pydantic validation works by trying to create a task with empty name
        from pydantic import ValidationError as PydanticValidationError
        
        with pytest.raises(PydanticValidationError):
            # This should fail at the TaskCreateRequest level due to validation
            TaskCreateRequest(
                name="",  # Empty name should cause validation error
                command="echo hello",
                project_id="550e8400-e29b-41d4-a716-446655440001"
            )

    @pytest.mark.asyncio
    async def test_get_task_success(self, client):
        """Test successful task retrieval"""
        task_id = "550e8400-e29b-41d4-a716-446655440000"
        mock_response_data = {
            "id": task_id,
            "name": "Existing Task",
            "command": "python script.py",
            "project_id": "550e8400-e29b-41d4-a716-446655440001",
            "status": "Running",
            "priority": "High",
            "created_at": "2024-01-01T00:00:00Z",
            "updated_at": "2024-01-01T00:01:00Z"
        }
        client._make_request = AsyncMock(return_value=mock_response_data)

        task = await client.get_task(task_id)

        assert str(task.id) == task_id  # Convert UUID to string for comparison
        assert task.name == "Existing Task"
        assert task.status == TaskStatus.RUNNING
        assert task.priority == TaskPriority.HIGH

    @pytest.mark.asyncio
    async def test_get_task_not_found(self, client):
        """Test task retrieval when task doesn't exist"""
        task_id = "550e8400-e29b-41d4-a716-446655440000"
        client._make_request = AsyncMock(side_effect=TaskNotFoundError("Task not found", 404))

        with pytest.raises(TaskNotFoundError) as exc_info:
            await client.get_task(task_id)

        assert exc_info.value.status_code == 404
        assert "not found" in str(exc_info.value).lower()

    @pytest.mark.asyncio
    async def test_list_tasks_with_filters(self, client):
        """Test listing tasks with filters"""
        mock_response_data = {
            "tasks": [
                {
                    "id": "550e8400-e29b-41d4-a716-446655440000",
                    "name": "Task 1",
                    "command": "echo 1",
                    "project_id": "550e8400-e29b-41d4-a716-446655440001",
                    "status": "Completed",
                    "priority": "Normal",
                    "created_at": "2024-01-01T00:00:00Z",
                    "updated_at": "2024-01-01T00:01:00Z"
                },
                {
                    "id": "550e8400-e29b-41d4-a716-446655440002",
                    "name": "Task 2",
                    "command": "echo 2",
                    "project_id": "550e8400-e29b-41d4-a716-446655440001",
                    "status": "Running",
                    "priority": "High",
                    "created_at": "2024-01-01T00:02:00Z",
                    "updated_at": "2024-01-01T00:03:00Z"
                }
            ]
        }
        client._make_request = AsyncMock(return_value=mock_response_data)

        tasks = await client.list_tasks(
            status=TaskStatus.COMPLETED,
            project_id="550e8400-e29b-41d4-a716-446655440001",
            limit=10
        )

        assert len(tasks) == 2
        assert tasks[0].name == "Task 1"
        assert tasks[0].status == TaskStatus.COMPLETED
        assert tasks[1].name == "Task 2"
        assert tasks[1].status == TaskStatus.RUNNING

    @pytest.mark.asyncio
    async def test_update_task(self, client):
        """Test task update"""
        task_id = "550e8400-e29b-41d4-a716-446655440000"
        mock_response_data = {
            "id": task_id,
            "name": "Updated Task Name",
            "command": "python updated.py",
            "project_id": "550e8400-e29b-41d4-a716-446655440001",
            "status": "Planning",
            "priority": "High",
            "created_at": "2024-01-01T00:00:00Z",
            "updated_at": "2024-01-01T00:05:00Z"
        }
        client._make_request = AsyncMock(return_value=mock_response_data)

        task = await client.update_task(
            task_id,
            name="Updated Task Name",
            command="python updated.py",
            priority=TaskPriority.HIGH
        )

        assert task.name == "Updated Task Name"
        assert task.command == "python updated.py"
        assert task.priority == TaskPriority.HIGH

    @pytest.mark.asyncio
    async def test_cancel_task(self, client):
        """Test task cancellation"""
        task_id = "550e8400-e29b-41d4-a716-446655440000"
        mock_response_data = {
            "id": task_id,
            "name": "Cancelled Task",
            "command": "echo test",  # Add required field
            "project_id": "550e8400-e29b-41d4-a716-446655440001",
            "status": "Cancelled",
            "priority": "Normal",
            "created_at": "2024-01-01T00:00:00Z",  # Add required field
            "updated_at": "2024-01-01T00:10:00Z"
        }
        client._make_request = AsyncMock(return_value=mock_response_data)

        task = await client.cancel_task(task_id)

        assert task.status == TaskStatus.CANCELLED

    @pytest.mark.asyncio
    async def test_delete_task(self, client):
        """Test task deletion"""
        task_id = "550e8400-e29b-41d4-a716-446655440000"
        client._make_request = AsyncMock(return_value={})

        result = await client.delete_task(task_id)

        assert result is True

    @pytest.mark.asyncio
    async def test_delete_task_not_found(self, client):
        """Test task deletion when task doesn't exist"""
        task_id = "550e8400-e29b-41d4-a716-446655440000"
        client._make_request = AsyncMock(side_effect=TaskNotFoundError("Task not found", 404))

        with pytest.raises(TaskNotFoundError):
            await client.delete_task(task_id)

    @pytest.mark.asyncio
    async def test_network_error(self, client):
        """Test network connectivity error"""
        client._make_request = AsyncMock(side_effect=Exception("Connection failed"))

        with pytest.raises(Exception) as exc_info:
            await client.create_task(
                name="Test Task",
                command="echo hello",
                project_id="550e8400-e29b-41d4-a716-446655440001"
            )

        assert "Connection failed" in str(exc_info.value)

    @pytest.mark.asyncio
    async def test_timeout_error(self, client):
        """Test request timeout"""
        import httpx
        client._make_request = AsyncMock(side_effect=httpx.TimeoutException("Request timed out"))

        with pytest.raises(TaskQueueError) as exc_info:
            await client.create_task(
                name="Test Task",
                command="echo hello",
                project_id="550e8400-e29b-41d4-a716-446655440001"
            )

        assert "timed out" in str(exc_info.value).lower()


class TestTaskQueueClient:
    """Test synchronous TaskQueueClient"""

    @pytest.fixture
    def sync_client(self):
        """Create test sync client"""
        with patch('taskqueue.client.AsyncTaskQueueClient') as mock_async_client:
            mock_instance = AsyncMock()
            mock_async_client.return_value = mock_instance

            client = TaskQueueClient(base_url="http://test.example.com")
            client._async_client = mock_instance

            yield client

    def test_sync_client_delegation(self, sync_client):
        """Test that sync client delegates to async client"""
        # Mock the async client's create_task method
        mock_task = MagicMock()
        mock_task.name = "Test Task"
        sync_client._async_client.create_task.return_value = mock_task

        # Call sync method
        result = sync_client.create_task(
            name="Test Task",
            command="echo hello",
            project_id="550e8400-e29b-41d4-a716-446655440001"
        )

        # Verify async method was called
        sync_client._async_client.create_task.assert_called_once()
        assert result == mock_task

    def test_sync_client_context_manager(self, sync_client):
        """Test sync client context manager"""
        with sync_client as client:
            assert client is sync_client

        # Verify close was called on async client
        sync_client._async_client.close.assert_called_once()


class TestBatchOperations:
    """Test batch operations"""

    @pytest_asyncio.fixture
    async def client(self):
        """Create test client for batch operations"""
        client = AsyncTaskQueueClient(base_url="http://test.example.com")
        client._ensure_client = AsyncMock()
        client._client = AsyncMock()
        yield client
        await client.close()

    @pytest.mark.asyncio
    async def test_create_tasks_batch(self, client):
        """Test batch task creation"""
        # Mock successful responses for each task
        mock_responses = []
        for i in range(3):
            mock_response_data = {
                "id": f"550e8400-e29b-41d4-a716-44665544000{i}",
                "name": f"Task {i}",
                "command": f"echo {i}",
                "project_id": "550e8400-e29b-41d4-a716-446655440001",
                "status": "Planning",
                "priority": "Normal",
                "created_at": "2024-01-01T00:00:00Z",
                "updated_at": "2024-01-01T00:00:00Z"
            }
            mock_responses.append(mock_response_data)

        # Mock the create_task method to return the responses
        client.create_task = AsyncMock(side_effect=[
            Task(**mock_responses[0]),
            Task(**mock_responses[1]),
            Task(**mock_responses[2])
        ])

        tasks_data = [
            {"name": "Task 0", "command": "echo 0", "project_id": "550e8400-e29b-41d4-a716-446655440001"},
            {"name": "Task 1", "command": "echo 1", "project_id": "550e8400-e29b-41d4-a716-446655440001"},
            {"name": "Task 2", "command": "echo 2", "project_id": "550e8400-e29b-41d4-a716-446655440001"},
        ]

        tasks = await client.create_tasks(tasks_data)

        assert len(tasks) == 3
        assert tasks[0].name == "Task 0"
        assert tasks[1].name == "Task 1"
        assert tasks[2].name == "Task 2"

    @pytest.mark.asyncio
    async def test_wait_for_completion(self, client):
        """Test waiting for task completion"""
        task_id = "550e8400-e29b-41d4-a716-446655440000"

        # Mock task in running state first, then completed
        running_task = Task(
            id=task_id,
            name="Test Task",
            command="echo test",
            project_id="550e8400-e29b-41d4-a716-446655440001",
            status=TaskStatus.RUNNING,
            created_at="2024-01-01T00:00:00Z",
            updated_at="2024-01-01T00:01:00Z"
        )

        completed_task = Task(
            id=task_id,
            name="Test Task",
            command="echo test",
            project_id="550e8400-e29b-41d4-a716-446655440001",
            status=TaskStatus.COMPLETED,
            created_at="2024-01-01T00:00:00Z",
            updated_at="2024-01-01T00:05:00Z"
        )

        client.get_task = AsyncMock(side_effect=[running_task, completed_task])

        task = await client.wait_for_completion(task_id, timeout=10)

        assert task.status == TaskStatus.COMPLETED
        assert client.get_task.call_count == 2  # Called twice: once running, once completed
