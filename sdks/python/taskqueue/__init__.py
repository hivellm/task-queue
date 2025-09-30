"""Task Queue Python SDK"""

__version__ = "0.1.0"
__author__ = "Task Queue Team"
__email__ = "team@taskqueue.dev"

from .client import TaskQueueClient, AsyncTaskQueueClient
from .exceptions import TaskQueueError, ValidationError, TaskNotFoundError, APIError
from .models import Task, Project, TaskStatus, TaskPriority

__all__ = [
    "TaskQueueClient",
    "AsyncTaskQueueClient",
    "TaskQueueError",
    "ValidationError",
    "TaskNotFoundError",
    "APIError",
    "Task",
    "Project",
    "TaskStatus",
    "TaskPriority",
    "__version__",
]
