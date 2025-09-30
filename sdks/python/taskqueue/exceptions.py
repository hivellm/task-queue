"""Exception classes for Task Queue SDK"""

from typing import Optional, Dict, Any


class TaskQueueError(Exception):
    """Base exception for Task Queue operations"""

    def __init__(self, message: str, status_code: Optional[int] = None, response_data: Optional[Dict[str, Any]] = None):
        super().__init__(message)
        self.message = message
        self.status_code = status_code
        self.response_data = response_data or {}

    def __str__(self):
        if self.status_code:
            return f"[{self.status_code}] {self.message}"
        return self.message


class ValidationError(TaskQueueError):
    """Raised when request validation fails"""
    pass


class AuthenticationError(TaskQueueError):
    """Raised when authentication fails"""
    pass


class AuthorizationError(TaskQueueError):
    """Raised when authorization fails"""
    pass


class TaskNotFoundError(TaskQueueError):
    """Raised when a requested task is not found"""
    pass


class ProjectNotFoundError(TaskQueueError):
    """Raised when a requested project is not found"""
    pass


class APIError(TaskQueueError):
    """Raised when API returns an error response"""
    pass


class ConnectionError(TaskQueueError):
    """Raised when connection to API fails"""
    pass


class TimeoutError(TaskQueueError):
    """Raised when request times out"""
    pass


class RateLimitError(TaskQueueError):
    """Raised when rate limit is exceeded"""
    pass


class WebhookError(TaskQueueError):
    """Raised when webhook processing fails"""
    pass
