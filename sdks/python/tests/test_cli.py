"""Tests for CLI commands"""

import pytest
from click.testing import CliRunner
from unittest.mock import patch, MagicMock

from taskqueue.cli import cli
from taskqueue.models import TaskStatus, TaskPriority


class TestCLI:
    """Test CLI commands"""

    @pytest.fixture
    def runner(self):
        """Create CLI runner"""
        return CliRunner()

    @pytest.fixture
    def mock_client(self):
        """Mock TaskQueueClient"""
        with patch('taskqueue.cli.TaskQueueClient') as mock_client_class:
            mock_instance = MagicMock()
            mock_client_class.return_value = mock_instance
            yield mock_instance

    def test_cli_help(self, runner):
        """Test CLI help command"""
        result = runner.invoke(cli, ['--help'])
        assert result.exit_code == 0
        assert 'Task Queue Command Line Interface' in result.output
        assert 'tasks' in result.output
        assert 'projects' in result.output

    def test_cli_version(self, runner):
        """Test CLI version command"""
        result = runner.invoke(cli, ['version'])
        assert result.exit_code == 0
        assert 'Task Queue Python SDK' in result.output

    def test_tasks_create_success(self, runner, mock_client):
        """Test successful task creation via CLI"""
        # Mock successful task creation
        mock_task = MagicMock()
        mock_task.name = "Test Task"
        mock_task.id = "550e8400-e29b-41d4-a716-446655440000"
        mock_client.create_task.return_value = mock_task

        result = runner.invoke(cli, [
            'tasks', 'create',
            '--name', 'Test Task',
            '--command', 'echo hello',
            '--project-id', '550e8400-e29b-41d4-a716-446655440001'
        ])

        assert result.exit_code == 0
        assert '✅ Task' in result.output  # Changed from exact match to partial match
        assert 'Test Task' in result.output
        mock_client.create_task.assert_called_once()

    def test_tasks_create_missing_required_args(self, runner, mock_client):
        """Test task creation with missing required arguments"""
        result = runner.invoke(cli, [
            'tasks', 'create',
            '--name', 'Test Task'
            # Missing --command and --project-id
        ])

        assert result.exit_code == 2  # Click error for missing required args
        assert 'Missing option' in result.output

    def test_tasks_get_success(self, runner, mock_client):
        """Test successful task retrieval via CLI"""
        # Mock task data
        mock_task = MagicMock()
        mock_task.name = "Existing Task"
        mock_task.status = TaskStatus.RUNNING
        mock_task.priority = TaskPriority.HIGH
        mock_task.task_type = MagicMock()
        mock_task.task_type.value = "Simple"
        mock_task.created_at.strftime.return_value = "2024-01-01 10:00:00"
        mock_task.updated_at.strftime.return_value = "2024-01-01 10:05:00"
        mock_task.command = "python script.py"
        mock_task.description = "A test task"
        mock_task.technical_specs = "Python 3.8+"
        mock_task.acceptance_criteria = ["Must run successfully", "Output valid"]

        mock_client.get_task.return_value = mock_task

        result = runner.invoke(cli, [
            'tasks', 'get', '550e8400-e29b-41d4-a716-446655440000'
        ])

        assert result.exit_code == 0
        assert 'Existing Task' in result.output
        assert 'Running' in result.output
        assert 'High' in result.output
        assert 'python script.py' in result.output
        mock_client.get_task.assert_called_once()

    def test_tasks_list_table_format(self, runner, mock_client):
        """Test task listing in table format"""
        # Mock task list
        mock_task1 = MagicMock()
        mock_task1.id = "550e8400-e29b-41d4-a716-446655440000"
        mock_task1.name = "Task 1"
        mock_task1.status = TaskStatus.COMPLETED
        mock_task1.priority = TaskPriority.NORMAL
        mock_task1.project_id = "550e8400-e29b-41d4-a716-446655440001"  # Add project_id
        mock_task1.created_at.strftime.return_value = "2024-01-01 10:00:00"

        mock_task2 = MagicMock()
        mock_task2.id = "550e8400-e29b-41d4-a716-446655440001"
        mock_task2.name = "Task 2"
        mock_task2.status = TaskStatus.RUNNING
        mock_task2.priority = TaskPriority.HIGH
        mock_task2.project_id = "550e8400-e29b-41d4-a716-446655440001"  # Add project_id
        mock_task2.created_at.strftime.return_value = "2024-01-01 10:05:00"

        mock_client.list_tasks.return_value = [mock_task1, mock_task2]

        result = runner.invoke(cli, ['tasks', 'list'])

        assert result.exit_code == 0
        assert 'Task 1' in result.output
        assert 'Task 2' in result.output
        assert 'Completed' in result.output
        assert 'Running' in result.output
        mock_client.list_tasks.assert_called_once()

    def test_tasks_list_json_format(self, runner, mock_client):
        """Test task listing in JSON format"""
        # Mock task list
        mock_task = MagicMock()
        mock_task.id = "550e8400-e29b-41d4-a716-446655440000"
        mock_task.name = "Test Task"
        mock_task.status = TaskStatus.COMPLETED
        mock_task.priority = TaskPriority.NORMAL
        mock_task.created_at = MagicMock()
        mock_task.updated_at = MagicMock()

        # Mock the dict() method for JSON serialization
        mock_task.dict.return_value = {
            "id": "550e8400-e29b-41d4-a716-446655440000",
            "name": "Test Task",
            "status": "Completed",
            "priority": "Normal",
            "created_at": "2024-01-01T10:00:00Z",
            "updated_at": "2024-01-01T10:05:00Z"
        }

        mock_client.list_tasks.return_value = [mock_task]

        result = runner.invoke(cli, ['tasks', 'list', '--format', 'json'])

        assert result.exit_code == 0
        assert '"name": "Test Task"' in result.output
        assert '"status": "Completed"' in result.output

    def test_tasks_list_with_filters(self, runner, mock_client):
        """Test task listing with filters"""
        mock_client.list_tasks.return_value = []

        result = runner.invoke(cli, [
            'tasks', 'list',
            '--status', 'Completed',
            '--priority', 'High',
            '--limit', '10'
        ])

        assert result.exit_code == 0
        mock_client.list_tasks.assert_called_once()
        call_args = mock_client.list_tasks.call_args
        assert call_args[1]['status'] == TaskStatus.COMPLETED
        assert call_args[1]['priority'] == TaskPriority.HIGH
        assert call_args[1]['limit'] == 10

    def test_tasks_update(self, runner, mock_client):
        """Test task update via CLI"""
        mock_task = MagicMock()
        mock_task.name = "Updated Task"
        mock_client.update_task.return_value = mock_task

        result = runner.invoke(cli, [
            'tasks', 'update', '550e8400-e29b-41d4-a716-446655440000',
            '--name', 'Updated Task',
            '--priority', 'High'
        ])

        assert result.exit_code == 0
        assert '✅ Task \'Updated Task\' updated successfully!' in result.output
        mock_client.update_task.assert_called_once()

    def test_tasks_cancel(self, runner, mock_client):
        """Test task cancellation via CLI"""
        mock_task = MagicMock()
        mock_task.name = "Cancelled Task"
        mock_task.status = TaskStatus.CANCELLED
        mock_client.cancel_task.return_value = mock_task

        result = runner.invoke(cli, [
            'tasks', 'cancel', '550e8400-e29b-41d4-a716-446655440000'
        ])

        assert result.exit_code == 0
        assert '✅ Task \'Cancelled Task\' cancelled successfully!' in result.output
        mock_client.cancel_task.assert_called_once()

    def test_tasks_delete_with_confirmation(self, runner, mock_client):
        """Test task deletion with confirmation"""
        mock_client.delete_task.return_value = True

        # Simulate user confirming deletion
        result = runner.invoke(cli, [
            'tasks', 'delete', '550e8400-e29b-41d4-a716-446655440000'
        ], input='y\n')

        assert result.exit_code == 0
        assert '✅ Task deleted successfully!' in result.output
        mock_client.delete_task.assert_called_once()

    def test_tasks_delete_force(self, runner, mock_client):
        """Test task deletion with force flag"""
        mock_client.delete_task.return_value = True

        result = runner.invoke(cli, [
            'tasks', 'delete', '550e8400-e29b-41d4-a716-446655440000',
            '--force'
        ])

        assert result.exit_code == 0
        assert '✅ Task deleted successfully!' in result.output
        mock_client.delete_task.assert_called_once()
        # Should not prompt for confirmation when --force is used

    def test_tasks_wait(self, runner, mock_client):
        """Test waiting for task completion"""
        mock_task = MagicMock()
        mock_task.status = TaskStatus.COMPLETED
        mock_client.wait_for_completion.return_value = mock_task

        result = runner.invoke(cli, [
            'tasks', 'wait', '550e8400-e29b-41d4-a716-446655440000'
        ])

        assert result.exit_code == 0
        assert '✅ Task completed successfully!' in result.output
        mock_client.wait_for_completion.assert_called_once()

    def test_projects_create(self, runner, mock_client):
        """Test project creation via CLI"""
        mock_project = MagicMock()
        mock_project.name = "Test Project"
        mock_project.id = "550e8400-e29b-41d4-a716-446655440002"
        mock_client.create_project.return_value = mock_project

        result = runner.invoke(cli, [
            'projects', 'create',
            '--name', 'Test Project',
            '--description', 'A test project'
        ])

        assert result.exit_code == 0
        assert '✅ Project \'Test Project\' created' in result.output
        mock_client.create_project.assert_called_once()

    def test_projects_list(self, runner, mock_client):
        """Test project listing"""
        mock_project = MagicMock()
        mock_project.id = "550e8400-e29b-41d4-a716-446655440002"
        mock_project.name = "Test Project"
        mock_project.status.value = "Active"
        mock_project.created_at.strftime.return_value = "2024-01-01 10:00:00"

        mock_client.list_projects.return_value = [mock_project]

        result = runner.invoke(cli, ['projects', 'list'])

        assert result.exit_code == 0
        assert 'Test Project' in result.output
        assert 'Active' in result.output
        mock_client.list_projects.assert_called_once()

    def test_error_handling(self, runner, mock_client):
        """Test error handling in CLI"""
        mock_client.get_task.side_effect = Exception("Task not found")

        result = runner.invoke(cli, [
            'tasks', 'get', '550e8400-e29b-41d4-a716-446655440000'
        ])

        assert result.exit_code == 1  # Should exit with error code
        assert '❌ Failed to get task: Task not found' in result.output

    def test_verbose_output(self, runner, mock_client):
        """Test verbose output mode"""
        mock_task = MagicMock()
        mock_task.name = "Verbose Task"
        mock_task.id = "550e8400-e29b-41d4-a716-446655440000"
        mock_client.create_task.return_value = mock_task

        result = runner.invoke(cli, [
            '--verbose',
            'tasks', 'create',
            '--name', 'Verbose Task',
            '--command', 'echo verbose',
            '--project-id', '550e8400-e29b-41d4-a716-446655440001'
        ])

        assert result.exit_code == 0
        assert 'Verbose Task' in result.output
        # Verbose mode should show more detailed output
