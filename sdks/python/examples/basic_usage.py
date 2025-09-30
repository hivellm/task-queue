#!/usr/bin/env python3
"""
Basic usage examples for Task Queue Python SDK
"""

import asyncio
from taskqueue import TaskQueueClient, AsyncTaskQueueClient


def sync_example():
    """Synchronous client example"""
    print("🚀 Synchronous Client Example")
    print("=" * 50)

    # Initialize client
    client = TaskQueueClient(base_url="http://localhost:8080")

    try:
        # Create a task
        task = client.create_task(
            name="Data Processing Example",
            command="python process_data.py --input data.csv --output results.json",
            project_id="550e8400-e29b-41d4-a716-446655440000",  # Example UUID
            description="Process customer data and generate analytics report",
            technical_specs="Python 3.8+, pandas 1.5+, numpy",
            acceptance_criteria=[
                "Process completes without errors",
                "Output file is valid JSON",
                "All data transformations applied correctly"
            ],
            priority="High"
        )

        print(f"✅ Task created: {task.name} (ID: {task.id})")
        print(f"📊 Status: {task.status.value}")
        print(f"⭐ Priority: {task.priority.value}")

        # List tasks
        tasks = client.list_tasks(limit=5)
        print(f"\n📋 Found {len(tasks)} tasks")

        for task in tasks[:3]:  # Show first 3
            print(f"  - {task.name} ({task.status.value})")

    except Exception as e:
        print(f"❌ Error: {e}")


async def async_example():
    """Asynchronous client example"""
    print("\n🚀 Asynchronous Client Example")
    print("=" * 50)

    async with AsyncTaskQueueClient(base_url="http://localhost:8080") as client:
        try:
            # Create multiple tasks
            tasks_data = [
                {
                    "name": "Data Ingestion",
                    "command": "python ingest.py --source api --table users",
                    "project_id": "550e8400-e29b-41d4-a716-446655440000",
                    "description": "Ingest user data from external API",
                    "priority": "Normal"
                },
                {
                    "name": "Data Validation",
                    "command": "python validate.py --input users.json --schema user_schema.json",
                    "project_id": "550e8400-e29b-41d4-a716-446655440000",
                    "description": "Validate ingested data against schema",
                    "priority": "High"
                }
            ]

            created_tasks = await client.create_tasks(tasks_data)

            print(f"✅ Created {len(created_tasks)} tasks:")
            for task in created_tasks:
                print(f"  - {task.name} (ID: {task.id})")

            # Wait for first task to complete (if running)
            if created_tasks:
                first_task = created_tasks[0]
                print(f"\n⏳ Waiting for task '{first_task.name}' to complete...")
                try:
                    completed_task = await client.wait_for_completion(str(first_task.id), timeout=60)
                    print(f"✅ Task completed with status: {completed_task.status.value}")
                except TimeoutError:
                    print("⏰ Task did not complete within timeout")

        except Exception as e:
            print(f"❌ Error: {e}")


async def project_example():
    """Project management example"""
    print("\n🚀 Project Management Example")
    print("=" * 50)

    async with AsyncTaskQueueClient(base_url="http://localhost:8080") as client:
        try:
            # Create a project
            project = await client.create_project(
                name="Customer Analytics Platform",
                description="Platform for analyzing customer behavior and generating insights"
            )

            print(f"✅ Project created: {project.name} (ID: {project.id})")

            # List projects
            projects = await client.list_projects()
            print(f"\n📁 Found {len(projects)} projects:")
            for proj in projects:
                print(f"  - {proj.name} ({proj.status.value})")

        except Exception as e:
            print(f"❌ Error: {e}")


def error_handling_example():
    """Error handling examples"""
    print("\n🚀 Error Handling Example")
    print("=" * 50)

    client = TaskQueueClient(base_url="http://localhost:8080")

    # Example of different error types
    try:
        # This will fail if project doesn't exist
        task = client.create_task(
            name="Test Task",
            command="echo hello",
            project_id="00000000-0000-0000-0000-000000000000"  # Invalid project
        )
    except Exception as e:
        print(f"❌ Caught error: {type(e).__name__}: {e}")

    try:
        # Try to get non-existent task
        task = client.get_task("00000000-0000-0000-0000-000000000000")
    except Exception as e:
        print(f"❌ Caught error: {type(e).__name__}: {e}")


def main():
    """Run all examples"""
    print("Task Queue Python SDK Examples")
    print("=" * 60)

    # Sync examples
    sync_example()

    # Async examples
    asyncio.run(async_example())
    asyncio.run(project_example())

    # Error handling
    error_handling_example()

    print("\n" + "=" * 60)
    print("✅ All examples completed!")


if __name__ == "__main__":
    main()
