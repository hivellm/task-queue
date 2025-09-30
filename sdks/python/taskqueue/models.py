"""Data models for Task Queue API"""

from datetime import datetime
from enum import Enum
from typing import List, Optional, Dict, Any
from pydantic import BaseModel, Field, UUID4


class TaskStatus(str, Enum):
    """Task status enumeration"""
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
    """Task priority enumeration"""
    LOW = "Low"
    NORMAL = "Normal"
    HIGH = "High"
    CRITICAL = "Critical"


class TaskType(str, Enum):
    """Task type enumeration"""
    SIMPLE = "Simple"
    WORKFLOW = "Workflow"
    SCHEDULED = "Scheduled"


class TaskPhase(BaseModel):
    """Task phase model"""
    phase: TaskStatus
    started_at: Optional[datetime] = None
    completed_at: Optional[datetime] = None
    documentation: Optional[str] = None
    artifacts: List[str] = Field(default_factory=list)
    ai_reviews: List[Dict[str, Any]] = Field(default_factory=list)


class Task(BaseModel):
    """Task model"""
    id: UUID4
    name: str = Field(..., min_length=1)
    command: str = Field(..., min_length=1)
    description: Optional[str] = None
    technical_specs: Optional[str] = None
    acceptance_criteria: List[str] = Field(default_factory=list)
    project: Optional[str] = None
    task_type: TaskType = TaskType.SIMPLE
    priority: TaskPriority = TaskPriority.NORMAL
    project_id: Optional[UUID4] = None
    dependencies: List[Dict[str, Any]] = Field(default_factory=list)
    timeout: Optional[int] = None
    retry_attempts: int = 3
    retry_delay: int = 30
    environment: Dict[str, str] = Field(default_factory=dict)
    working_directory: Optional[str] = None
    created_at: datetime
    updated_at: datetime
    status: TaskStatus = TaskStatus.PLANNING
    result: Optional[Dict[str, Any]] = None
    phases: List[TaskPhase] = Field(default_factory=list)
    current_phase: TaskStatus = TaskStatus.PLANNING
    ai_reviews_required: int = 3
    ai_reviews_completed: int = 0
    metadata: Dict[str, Any] = Field(default_factory=dict)


class ProjectStatus(str, Enum):
    """Project status enumeration"""
    PLANNING = "Planning"
    ACTIVE = "Active"
    COMPLETED = "Completed"
    CANCELLED = "Cancelled"
    ON_HOLD = "OnHold"


class Project(BaseModel):
    """Project model"""
    id: UUID4
    name: str = Field(..., min_length=1)
    description: Optional[str] = None
    status: ProjectStatus = ProjectStatus.PLANNING
    created_at: datetime
    updated_at: datetime
    due_date: Optional[datetime] = None
    tags: List[str] = Field(default_factory=list)
    metadata: Dict[str, Any] = Field(default_factory=dict)


class TaskCreateRequest(BaseModel):
    """Request model for creating tasks"""
    name: str = Field(..., min_length=1)
    command: str = Field(..., min_length=1)
    project_id: UUID4
    description: Optional[str] = None
    technical_specs: Optional[str] = None
    acceptance_criteria: List[str] = Field(default_factory=list)
    priority: TaskPriority = TaskPriority.NORMAL
    timeout: Optional[int] = None
    retry_attempts: int = 3
    retry_delay: int = 30
    environment: Dict[str, str] = Field(default_factory=dict)
    working_directory: Optional[str] = None


class TaskUpdateRequest(BaseModel):
    """Request model for updating tasks"""
    name: Optional[str] = None
    command: Optional[str] = None
    description: Optional[str] = None
    technical_specs: Optional[str] = None
    acceptance_criteria: Optional[List[str]] = None
    priority: Optional[TaskPriority] = None
    status: Optional[TaskStatus] = None
    timeout: Optional[int] = None
    retry_attempts: Optional[int] = None
    retry_delay: Optional[int] = None
    environment: Optional[Dict[str, str]] = None
    working_directory: Optional[str] = None


class ProjectCreateRequest(BaseModel):
    """Request model for creating projects"""
    name: str = Field(..., min_length=1)
    description: Optional[str] = None
    due_date: Optional[datetime] = None
    tags: List[str] = Field(default_factory=list)


class TaskFilters(BaseModel):
    """Filters for task listing"""
    status: Optional[TaskStatus] = None
    project_id: Optional[UUID4] = None
    priority: Optional[TaskPriority] = None
    task_type: Optional[TaskType] = None
    created_after: Optional[datetime] = None
    created_before: Optional[datetime] = None
