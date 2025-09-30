"""Tests for data models"""

import pytest
from datetime import datetime
from uuid import uuid4

from taskqueue.models import (
    Task, Project, TaskStatus, TaskPriority, TaskType,
    TaskCreateRequest, TaskUpdateRequest, ProjectCreateRequest
)


class TestEnums:
    """Test enum definitions"""

    def test_task_status_values(self):
        """Test TaskStatus enum has all expected values"""
        expected_values = [
            'Planning', 'Implementation', 'TestCreation', 'Testing',
            'AIReview', 'Finalized', 'Pending', 'Running', 'Completed',
            'Failed', 'Cancelled'
        ]
        assert [status.value for status in TaskStatus] == expected_values

    def test_task_priority_values(self):
        """Test TaskPriority enum has all expected values"""
        expected_values = ['Low', 'Normal', 'High', 'Critical']
        assert [priority.value for priority in TaskPriority] == expected_values

    def test_task_type_values(self):
        """Test TaskType enum has all expected values"""
        expected_values = ['Simple', 'Workflow', 'Scheduled']
        assert [task_type.value for task_type in TaskType] == expected_values


class TestTaskModel:
    """Test Task model"""

    def test_task_creation_minimal(self):
        """Test creating a task with minimal required fields"""
        task_data = {
            "id": uuid4(),
            "name": "Test Task",
            "command": "echo hello",
            "project_id": uuid4(),
            "created_at": datetime.now(),
            "updated_at": datetime.now()
        }

        task = Task(**task_data)
        assert task.name == "Test Task"
        assert task.command == "echo hello"
        assert task.status == TaskStatus.PLANNING  # default
        assert task.priority == TaskPriority.NORMAL  # default

    def test_task_creation_complete(self):
        """Test creating a task with all fields"""
        task_id = uuid4()
        project_id = uuid4()
        now = datetime.now()

        task_data = {
            "id": task_id,
            "name": "Complete Test Task",
            "command": "python script.py",
            "description": "A complete test task",
            "technical_specs": "Python 3.8+, pandas",
            "acceptance_criteria": ["Must run without errors", "Output valid JSON"],
            "project": "Test Project",
            "task_type": TaskType.SIMPLE,
            "priority": TaskPriority.HIGH,
            "project_id": project_id,
            "dependencies": [],
            "timeout": 300,
            "retry_attempts": 3,
            "retry_delay": 30,
            "environment": {"ENV": "test"},
            "working_directory": "/tmp",
            "created_at": now,
            "updated_at": now,
            "status": TaskStatus.PENDING,
            "result": None,
            "phases": [],
            "current_phase": TaskStatus.PENDING,
            "ai_reviews_required": 3,
            "ai_reviews_completed": 0,
            "metadata": {"key": "value"}
        }

        task = Task(**task_data)
        assert task.id == task_id
        assert task.name == "Complete Test Task"
        assert task.description == "A complete test task"
        assert task.technical_specs == "Python 3.8+, pandas"
        assert len(task.acceptance_criteria) == 2
        assert task.priority == TaskPriority.HIGH
        assert task.status == TaskStatus.PENDING

    def test_task_validation_name_required(self):
        """Test that name is required"""
        with pytest.raises(ValueError):
            Task(
                id=uuid4(),
                name="",  # empty name should fail
                command="echo hello",
                project_id=uuid4(),
                created_at=datetime.now(),
                updated_at=datetime.now()
            )

    def test_task_validation_command_required(self):
        """Test that command is required"""
        with pytest.raises(ValueError):
            Task(
                id=uuid4(),
                name="Test Task",
                command="",  # empty command should fail
                project_id=uuid4(),
                created_at=datetime.now(),
                updated_at=datetime.now()
            )


class TestProjectModel:
    """Test Project model"""

    def test_project_creation_minimal(self):
        """Test creating a project with minimal required fields"""
        project_data = {
            "id": uuid4(),
            "name": "Test Project",
            "created_at": datetime.now(),
            "updated_at": datetime.now()
        }

        project = Project(**project_data)
        assert project.name == "Test Project"
        assert project.status.name == "PLANNING"  # default enum

    def test_project_creation_complete(self):
        """Test creating a project with all fields"""
        project_id = uuid4()
        now = datetime.now()

        project_data = {
            "id": project_id,
            "name": "Complete Test Project",
            "description": "A complete test project",
            "status": "Active",
            "created_at": now,
            "updated_at": now,
            "due_date": now,
            "tags": ["test", "important"],
            "metadata": {"priority": "high"}
        }

        project = Project(**project_data)
        assert project.id == project_id
        assert project.name == "Complete Test Project"
        assert project.description == "A complete test project"
        assert project.status.value == "Active"
        assert len(project.tags) == 2


class TestRequestModels:
    """Test request model validation"""

    def test_task_create_request_validation(self):
        """Test TaskCreateRequest validation"""
        # Valid request
        request = TaskCreateRequest(
            name="Valid Task",
            command="echo hello",
            project_id=uuid4()
        )
        assert request.name == "Valid Task"

        # Invalid - empty name
        with pytest.raises(ValueError):
            TaskCreateRequest(
                name="",
                command="echo hello",
                project_id=uuid4()
            )

        # Invalid - empty command
        with pytest.raises(ValueError):
            TaskCreateRequest(
                name="Valid Task",
                command="",
                project_id=uuid4()
            )

    def test_task_update_request_partial(self):
        """Test TaskUpdateRequest allows partial updates"""
        # Should allow partial updates
        request = TaskUpdateRequest(name="New Name")
        assert request.name == "New Name"
        assert request.command is None  # not provided

        # Should allow empty dict (no updates)
        request = TaskUpdateRequest()
        assert request.name is None

    def test_project_create_request_validation(self):
        """Test ProjectCreateRequest validation"""
        # Valid request
        request = ProjectCreateRequest(name="Valid Project")
        assert request.name == "Valid Project"

        # Invalid - empty name
        with pytest.raises(ValueError):
            ProjectCreateRequest(name="")
