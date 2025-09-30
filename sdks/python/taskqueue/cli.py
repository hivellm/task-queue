"""Command-line interface for Task Queue"""

import json
import sys
from typing import Optional

import click
from rich.console import Console
from rich.table import Table
from rich.panel import Panel
from rich.text import Text

from .client import TaskQueueClient
from .models import TaskStatus, TaskPriority
from .exceptions import TaskQueueError


console = Console()


class CLIContext:
    """CLI context object"""

    def __init__(self, base_url: str = "http://localhost:8080", verbose: bool = False):
        self.base_url = base_url
        self.verbose = verbose
        self.client = TaskQueueClient(base_url=base_url)


@click.group()
@click.option('--base-url', default='http://localhost:8080', help='Task Queue API base URL')
@click.option('--verbose', '-v', is_flag=True, help='Enable verbose output')
@click.pass_context
def cli(ctx, base_url, verbose):
    """Task Queue Command Line Interface"""
    ctx.obj = CLIContext(base_url=base_url, verbose=verbose)


@cli.group()
def tasks():
    """Task management commands"""
    pass


@cli.group()
def projects():
    """Project management commands"""
    pass


# Task Commands
@tasks.command()
@click.option('--name', required=True, help='Task name')
@click.option('--command', required=True, help='Command to execute')
@click.option('--project-id', required=True, help='Project ID')
@click.option('--description', help='Task description')
@click.option('--priority', type=click.Choice(['Low', 'Normal', 'High', 'Critical']),
              default='Normal', help='Task priority')
@click.option('--timeout', type=int, help='Task timeout in seconds')
@click.pass_obj
def create(cli_ctx, name, command, project_id, description, priority, timeout):
    """Create a new task"""
    try:
        task = cli_ctx.client.create_task(
            name=name,
            command=command,
            project_id=project_id,
            description=description,
            priority=TaskPriority(priority),
            timeout=timeout
        )

        if cli_ctx.verbose:
            console.print(Panel.fit(f"✅ Task created successfully!\n\n{format_task_details(task)}"))
        else:
            console.print(f"✅ Task '{task.name}' created with ID: {task.id}")

    except Exception as e:
        console.print(f"❌ Failed to create task: {e}", style="red")
        sys.exit(1)


@tasks.command()
@click.argument('task_id')
@click.pass_obj
def get(cli_ctx, task_id):
    """Get task details"""
    try:
        task = cli_ctx.client.get_task(task_id)
        console.print(Panel.fit(format_task_details(task)))

    except Exception as e:
        console.print(f"❌ Failed to get task: {e}", style="red")
        sys.exit(1)


@tasks.command()
@click.option('--status', type=click.Choice(['Planning', 'Implementation', 'TestCreation',
                                           'Testing', 'AIReview', 'Finalized', 'Pending',
                                           'Running', 'Completed', 'Failed', 'Cancelled']),
              help='Filter by status')
@click.option('--project-id', help='Filter by project ID')
@click.option('--priority', type=click.Choice(['Low', 'Normal', 'High', 'Critical']),
              help='Filter by priority')
@click.option('--limit', type=int, default=50, help='Maximum number of tasks to show')
@click.option('--format', type=click.Choice(['table', 'json']), default='table',
              help='Output format')
@click.pass_obj
def list(cli_ctx, status, project_id, priority, limit, format):
    """List tasks"""
    try:
        filters = {}
        if status:
            filters['status'] = TaskStatus(status)
        if project_id:
            filters['project_id'] = project_id
        if priority:
            filters['priority'] = TaskPriority(priority)

        tasks = cli_ctx.client.list_tasks(limit=limit, **filters)

        if not tasks:
            console.print("No tasks found.")
            return

        if format == 'json':
            # Convert tasks to dict for JSON output
            tasks_data = []
            for task in tasks:
                task_dict = task.dict()
                # Convert enums to strings
                task_dict['status'] = task.status.value
                task_dict['priority'] = task.priority.value
                task_dict['task_type'] = task.task_type.value
                tasks_data.append(task_dict)

            console.print_json(json.dumps(tasks_data, indent=2, default=str))
        else:
            table = Table(title=f"Tasks ({len(tasks)} found)")
            table.add_column("ID", style="cyan", no_wrap=True)
            table.add_column("Name", style="white", max_width=30)
            table.add_column("Status", style="green")
            table.add_column("Priority", style="yellow")
            table.add_column("Project", style="blue", max_width=20)
            table.add_column("Created", style="dim", no_wrap=True)

            for task in tasks:
                table.add_row(
                    str(task.id)[:8] + "...",
                    task.name,
                    task.status.value,
                    task.priority.value,
                    task.project_id[:8] + "..." if task.project_id else "N/A",
                    task.created_at.strftime("%Y-%m-%d %H:%M")
                )

            console.print(table)

    except Exception as e:
        console.print(f"❌ Failed to list tasks: {e}", style="red")
        sys.exit(1)


@tasks.command()
@click.argument('task_id')
@click.option('--name', help='New task name')
@click.option('--command', help='New command')
@click.option('--description', help='New description')
@click.option('--priority', type=click.Choice(['Low', 'Normal', 'High', 'Critical']),
              help='New priority')
@click.option('--status', type=click.Choice(['Planning', 'Implementation', 'TestCreation',
                                           'Testing', 'AIReview', 'Finalized', 'Pending',
                                           'Running', 'Completed', 'Failed', 'Cancelled']),
              help='New status')
@click.pass_obj
def update(cli_ctx, task_id, name, command, description, priority, status):
    """Update an existing task"""
    try:
        update_data = {}
        if name:
            update_data['name'] = name
        if command:
            update_data['command'] = command
        if description:
            update_data['description'] = description
        if priority:
            update_data['priority'] = TaskPriority(priority)
        if status:
            update_data['status'] = TaskStatus(status)

        if not update_data:
            console.print("❌ No update parameters provided", style="yellow")
            return

        task = cli_ctx.client.update_task(task_id, **update_data)
        console.print(f"✅ Task '{task.name}' updated successfully!")

        if cli_ctx.verbose:
            console.print(Panel.fit(format_task_details(task)))

    except Exception as e:
        console.print(f"❌ Failed to update task: {e}", style="red")
        sys.exit(1)


@tasks.command()
@click.argument('task_id')
@click.pass_obj
def cancel(cli_ctx, task_id):
    """Cancel a running task"""
    try:
        task = cli_ctx.client.cancel_task(task_id)
        console.print(f"✅ Task '{task.name}' cancelled successfully!")
        console.print(f"Status: {task.status.value}")

    except Exception as e:
        console.print(f"❌ Failed to cancel task: {e}", style="red")
        sys.exit(1)


@tasks.command()
@click.argument('task_id')
@click.option('--force', '-f', is_flag=True, help='Skip confirmation')
@click.pass_obj
def delete(cli_ctx, task_id, force):
    """Delete a task"""
    if not force:
        if not click.confirm(f"Are you sure you want to delete task '{task_id}'?"):
            return

    try:
        success = cli_ctx.client.delete_task(task_id)
        if success:
            console.print(f"✅ Task deleted successfully!")
        else:
            console.print("⚠️ Task not found or already deleted", style="yellow")

    except Exception as e:
        console.print(f"❌ Failed to delete task: {e}", style="red")
        sys.exit(1)


@tasks.command()
@click.argument('task_id')
@click.option('--timeout', type=int, default=300, help='Timeout in seconds')
@click.pass_obj
def wait(cli_ctx, task_id, timeout):
    """Wait for task completion"""
    try:
        console.print(f"⏳ Waiting for task {task_id} to complete...")
        task = cli_ctx.client.wait_for_completion(task_id, timeout)

        if task.status == TaskStatus.COMPLETED:
            console.print(f"✅ Task completed successfully!")
        elif task.status == TaskStatus.FAILED:
            console.print(f"❌ Task failed!", style="red")
        elif task.status == TaskStatus.CANCELLED:
            console.print(f"⚠️ Task was cancelled", style="yellow")
        else:
            console.print(f"⚠️ Task finished with status: {task.status.value}", style="yellow")

        if cli_ctx.verbose and task.result:
            console.print("\n📄 Result:")
            console.print(Panel(task.result.get('message', 'No result message')))

    except Exception as e:
        console.print(f"❌ Failed to wait for task: {e}", style="red")
        sys.exit(1)


# Project Commands
@projects.command()
@click.option('--name', required=True, help='Project name')
@click.option('--description', help='Project description')
@click.pass_obj
def create(cli_ctx, name, description):
    """Create a new project"""
    try:
        project = cli_ctx.client.create_project(
            name=name,
            description=description
        )

        console.print(f"✅ Project '{project.name}' created with ID: {project.id}")

        if cli_ctx.verbose:
            console.print(Panel.fit(format_project_details(project)))

    except Exception as e:
        console.print(f"❌ Failed to create project: {e}", style="red")
        sys.exit(1)


@projects.command()
@click.argument('project_id')
@click.pass_obj
def get(cli_ctx, project_id):
    """Get project details"""
    try:
        project = cli_ctx.client.get_project(project_id)
        console.print(Panel.fit(format_project_details(project)))

    except Exception as e:
        console.print(f"❌ Failed to get project: {e}", style="red")
        sys.exit(1)


@projects.command()
@click.option('--format', type=click.Choice(['table', 'json']), default='table',
              help='Output format')
@click.pass_obj
def list(cli_ctx, format):
    """List all projects"""
    try:
        projects = cli_ctx.client.list_projects()

        if not projects:
            console.print("No projects found.")
            return

        if format == 'json':
            projects_data = [project.dict() for project in projects]
            console.print_json(json.dumps(projects_data, indent=2, default=str))
        else:
            table = Table(title=f"Projects ({len(projects)} found)")
            table.add_column("ID", style="cyan", no_wrap=True)
            table.add_column("Name", style="white", max_width=30)
            table.add_column("Status", style="green")
            table.add_column("Tasks", style="blue", justify="right")
            table.add_column("Created", style="dim", no_wrap=True)

            for project in projects:
                table.add_row(
                    str(project.id)[:8] + "...",
                    project.name,
                    project.status.value,
                    "N/A",  # Would need to fetch task count
                    project.created_at.strftime("%Y-%m-%d %H:%M")
                )

            console.print(table)

    except Exception as e:
        console.print(f"❌ Failed to list projects: {e}", style="red")
        sys.exit(1)


@cli.command()
def version():
    """Show version information"""
    from . import __version__
    console.print(f"Task Queue Python SDK v{__version__}")


def format_task_details(task):
    """Format task details for display"""
    details = f"""
📋 Name: {task.name}
🆔 ID: {task.id}
📊 Status: {task.status.value}
⭐ Priority: {task.priority.value}
📁 Project: {task.project_id or 'None'}
⚙️ Type: {task.task_type.value}
⏰ Created: {task.created_at.strftime('%Y-%m-%d %H:%M:%S')}
🔄 Updated: {task.updated_at.strftime('%Y-%m-%d %H:%M:%S')}
"""

    if task.description:
        details += f"\n📝 Description:\n{task.description}\n"

    if task.technical_specs:
        details += f"\n🔧 Technical Specs:\n{task.technical_specs}\n"

    if task.acceptance_criteria:
        details += "\n✅ Acceptance Criteria:\n"
        for i, criterion in enumerate(task.acceptance_criteria, 1):
            details += f"{i}. {criterion}\n"

    details += f"\n💻 Command:\n{task.command}"

    return details.strip()


def format_project_details(project):
    """Format project details for display"""
    details = f"""
📁 Name: {project.name}
🆔 ID: {project.id}
📊 Status: {project.status.value}
⏰ Created: {project.created_at.strftime('%Y-%m-%d %H:%M:%S')}
🔄 Updated: {project.updated_at.strftime('%Y-%m-%d %H:%M:%S')}
"""

    if project.description:
        details += f"\n📝 Description:\n{project.description}\n"

    if project.due_date:
        details += f"\n📅 Due Date: {project.due_date.strftime('%Y-%m-%d')}\n"

    if project.tags:
        details += f"\n🏷️ Tags: {', '.join(project.tags)}\n"

    return details.strip()


def main():
    """Main CLI entry point"""
    try:
        cli()
    except KeyboardInterrupt:
        console.print("\n👋 Goodbye!", style="yellow")
        sys.exit(0)
    except Exception as e:
        console.print(f"💥 Unexpected error: {e}", style="red")
        if '--verbose' in sys.argv:
            import traceback
            traceback.print_exc()
        sys.exit(1)


if __name__ == '__main__':
    main()
