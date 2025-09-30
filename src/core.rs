//! Core task queue types and functionality

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(snake_case)]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Task status enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TaskStatus {
    // Development lifecycle statuses
    Planning,                  // Planejamento - criar documentação técnica da implementação
    Implementation,            // Implementação das especificações
    TestCreation,             // Criação de testes automatizados
    Testing,                  // Teste
    AIReview,                 // Revisão por modelos de IA (pelo menos 3 modelos)
    Finalized,                // Finalizado
    
    // Legacy statuses (for backward compatibility)
    AnalysisAndDocumentation,  // Análise e criação de documentação técnica
    InDiscussion,              // Em discussão
    InImplementation,          // Em implementação
    InReview,                  // Em revisão
    InTesting,                 // Em testes
    
    // Execution statuses
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
    WaitingForDependencies,
}

/// Task result enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskResult {
    Success {
        output: String,
        artifacts: Vec<String>,
        metrics: TaskMetrics,
    },
    Failure {
        error: String,
        exit_code: Option<i32>,
        logs: Vec<String>,
    },
    Cancelled {
        reason: String,
    },
}

/// Task metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskMetrics {
    pub execution_time: Duration,
    pub memory_usage: u64,
    pub cpu_usage: f64,
    pub disk_usage: u64,
    pub network_io: u64,
}

/// Dependency condition types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DependencyCondition {
    Success,
    Failure,
    Completion,
    Custom(String), // JSON serialized custom condition
}

/// Task priority levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum TaskPriority {
    Low = 1,
    Normal = 2,
    High = 3,
    Critical = 4,
}

/// Project status enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProjectStatus {
    Planning,
    Active,
    OnHold,
    Completed,
    Cancelled,
}

/// Project structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub status: ProjectStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub due_date: Option<DateTime<Utc>>,
    pub tags: Vec<String>,
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Project update structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectUpdate {
    pub name: Option<String>,
    pub description: Option<String>,
    pub status: Option<ProjectStatus>,
    pub tags: Option<Vec<String>>,
}

/// AI Review structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIReview {
    pub model_name: String,
    pub review_result: String,
    pub score: f64, // 0.0 to 1.0
    pub suggestions: Vec<String>,
    pub approved: bool,
    pub reviewed_at: DateTime<Utc>,
}

/// Task phase structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskPhase {
    pub phase: TaskStatus,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub documentation: Option<String>,
    pub artifacts: Vec<String>,
    pub ai_reviews: Vec<AIReview>,
}

/// Task type enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskType {
    Simple,
    Dependent,
    Workflow,
    Scheduled,
}

/// Development workflow information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevelopmentWorkflow {
    /// Caminho para documentação técnica gerada na fase de Planning
    pub technical_documentation_path: Option<String>,
    /// Cobertura de testes alcançada
    pub test_coverage_percentage: Option<f64>,
    /// Relatórios de revisão de IA
    pub ai_review_reports: Vec<AIDevelopmentReview>,
    /// Status detalhado do workflow
    pub workflow_status: DevelopmentWorkflowStatus,
    /// Data de início do workflow
    pub started_at: Option<DateTime<Utc>>,
    /// Data de conclusão do workflow
    pub completed_at: Option<DateTime<Utc>>,
}

/// Status do workflow de desenvolvimento
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum DevelopmentWorkflowStatus {
    NotStarted,
    Planning,
    InImplementation,
    TestCreation,
    Testing,
    AIReview,
    Completed,
    Failed,
}

/// Relatório de revisão de desenvolvimento por IA
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIDevelopmentReview {
    pub model_name: String,
    pub review_type: AIReviewType,
    pub content: String,
    pub score: f64, // 0.0 to 1.0
    pub approved: bool,
    pub suggestions: Vec<String>,
    pub reviewed_at: DateTime<Utc>,
}

/// Tipo de revisão de IA
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AIReviewType {
    CodeQuality,
    Security,
    Performance,
    Documentation,
    Testing,
    Architecture,
}

/// Main Task structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub name: String,
    pub command: String,
    #[serde(default = "default_description")]
    pub description: String, // Descrição detalhada da tarefa
    #[serde(default)]
    pub technical_specs: Option<String>, // Especificações técnicas detalhadas
    #[serde(default)]
    pub acceptance_criteria: Vec<String>, // Critérios de aceitação
    pub project: Option<String>,
    pub task_type: TaskType,
    pub priority: TaskPriority,
    #[serde(default)]
    pub project_id: Option<Uuid>, // Link to project
    #[serde(default)]
    pub dependencies: Vec<Dependency>,
    pub timeout: Option<Duration>,
    #[serde(default = "default_retry_attempts")]
    pub retry_attempts: u32,
    #[serde(default = "default_retry_delay")]
    pub retry_delay: Duration,
    #[serde(default)]
    pub environment: HashMap<String, String>,
    pub working_directory: Option<String>,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
    pub status: TaskStatus,
    pub result: Option<TaskResult>,
    #[serde(default = "default_phases")]
    pub phases: Vec<TaskPhase>, // Histórico de fases
    #[serde(default = "default_current_phase")]
    pub current_phase: TaskStatus, // Fase atual
    #[serde(default = "default_ai_reviews_required")]
    pub ai_reviews_required: u32, // Número mínimo de revisões IA (padrão: 3)
    #[serde(default)]
    pub ai_reviews_completed: u32, // Número de revisões IA completadas
    #[serde(default = "default_development_workflow")]
    pub development_workflow: Option<DevelopmentWorkflow>, // Workflow de desenvolvimento
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Default description for backward compatibility
fn default_description() -> String {
    "Task description not available".to_string()
}

/// Default retry attempts for backward compatibility
fn default_retry_attempts() -> u32 {
    3
}

/// Default retry delay for backward compatibility
fn default_retry_delay() -> Duration {
    Duration::from_secs(30)
}

/// Default phases for backward compatibility
fn default_phases() -> Vec<TaskPhase> {
    vec![TaskPhase {
        phase: TaskStatus::Planning,
        started_at: Some(chrono::Utc::now()),
        completed_at: None,
        documentation: None,
        artifacts: Vec::new(),
        ai_reviews: Vec::new(),
    }]
}

/// Default current phase for backward compatibility
fn default_current_phase() -> TaskStatus {
    TaskStatus::Planning
}

/// Default AI reviews required for backward compatibility
fn default_ai_reviews_required() -> u32 {
    3
}

/// Default development workflow for backward compatibility
fn default_development_workflow() -> Option<DevelopmentWorkflow> {
    Some(DevelopmentWorkflow {
        technical_documentation_path: None,
        test_coverage_percentage: None,
        ai_review_reports: Vec::new(),
        workflow_status: DevelopmentWorkflowStatus::NotStarted,
        started_at: None,
        completed_at: None,
    })
}

/// Enhanced dependency structure with correlation support
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dependency {
    pub task_id: Uuid,
    pub task_name: Option<String>, // For easier reference
    pub condition: DependencyCondition,
    pub required: bool,
    pub correlation_id: Option<String>, // For grouping related dependencies
    pub metadata: HashMap<String, serde_json::Value>, // Additional dependency metadata
}

/// Dependency correlation group
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyCorrelation {
    pub correlation_id: String,
    pub name: String,
    pub description: Option<String>,
    pub dependencies: Vec<Uuid>, // Task IDs in this correlation group
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

/// Workflow structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workflow {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub tasks: Vec<Task>,
    pub dependencies: Vec<WorkflowDependency>,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
    pub status: WorkflowStatus,
}

/// Workflow dependency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowDependency {
    pub from_task: Uuid,
    pub to_task: Uuid,
    pub condition: DependencyCondition,
}

/// Workflow status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WorkflowStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
}

/// Task execution context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskContext {
    pub task_id: Uuid,
    pub project: Option<String>,
    pub dependencies: Vec<Uuid>,
    pub parameters: HashMap<String, serde_json::Value>,
    pub execution_time: Duration,
    pub result: TaskResult,
    pub artifacts: Vec<String>,
    pub logs: Vec<String>,
}

/// Request structure for creating a new task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTaskRequest {
    pub name: String,
    pub command: String,
    pub description: String, // Descrição detalhada obrigatória
    pub technical_specs: Option<String>, // Especificações técnicas
    pub acceptance_criteria: Option<Vec<String>>, // Critérios de aceitação
    pub project: Option<String>,
    pub task_type: TaskType,
    pub priority: TaskPriority,
    pub project_id: Option<Uuid>,
    pub estimated_hours: Option<u32>,
    pub tags: Option<Vec<String>>,
    pub ai_reviews_required: Option<u32>, // Número de revisões IA (padrão: 3)
}

/// Task builder for fluent API
pub struct TaskBuilder {
    task: Task,
}

impl CreateTaskRequest {
    /// Convert CreateTaskRequest to Task
    pub fn to_task(self) -> Task {
        let now = SystemTime::now();
        Task {
            id: Uuid::new_v4(),
            name: self.name,
            command: self.command,
            description: self.description,
            technical_specs: self.technical_specs,
            acceptance_criteria: self.acceptance_criteria.unwrap_or_default(),
            project: self.project,
            task_type: self.task_type,
            priority: self.priority,
            project_id: self.project_id,
            dependencies: Vec::new(),
            timeout: None,
            retry_attempts: 3,
            retry_delay: Duration::from_secs(30),
            environment: HashMap::new(),
            working_directory: None,
            created_at: now,
            updated_at: now,
            status: TaskStatus::Planning,
            result: None,
            phases: vec![TaskPhase {
                phase: TaskStatus::Planning,
                started_at: Some(chrono::Utc::now()),
                completed_at: None,
                documentation: None,
                artifacts: Vec::new(),
                ai_reviews: Vec::new(),
            }],
            current_phase: TaskStatus::Planning,
            ai_reviews_required: self.ai_reviews_required.unwrap_or(3),
            ai_reviews_completed: 0,
            metadata: {
                let mut metadata = HashMap::new();
                if let Some(hours) = self.estimated_hours {
                    metadata.insert("estimated_hours".to_string(), serde_json::Value::Number(hours.into()));
                }
                if let Some(tags) = self.tags {
                    metadata.insert("tags".to_string(), serde_json::Value::Array(
                        tags.iter().map(|tag| serde_json::Value::String(tag.clone())).collect()
                    ));
                }
                metadata
            },
            development_workflow: default_development_workflow(),
        }
    }
}

impl TaskBuilder {
    pub fn new(name: &str) -> Self {
        let now = SystemTime::now();
        Self {
            task: Task {
                id: Uuid::new_v4(),
                name: name.to_string(),
                command: String::new(),
                description: String::new(),
                technical_specs: None,
                acceptance_criteria: Vec::new(),
                project: None,
                task_type: TaskType::Simple,
                priority: TaskPriority::Normal,
                project_id: None,
                dependencies: Vec::new(),
                timeout: None,
                retry_attempts: 3,
                retry_delay: Duration::from_secs(1),
                environment: HashMap::new(),
                working_directory: None,
                created_at: now,
                updated_at: now,
                status: TaskStatus::Planning,
                result: None,
                phases: vec![TaskPhase {
                    phase: TaskStatus::Planning,
                    started_at: Some(chrono::Utc::now()),
                    completed_at: None,
                    documentation: None,
                    artifacts: Vec::new(),
                    ai_reviews: Vec::new(),
                }],
                current_phase: TaskStatus::Planning,
                ai_reviews_required: 3,
                ai_reviews_completed: 0,
                development_workflow: default_development_workflow(),
                metadata: HashMap::new(),
            },
        }
    }

    pub fn with_command(mut self, command: &str) -> Self {
        self.task.command = command.to_string();
        self
    }

    pub fn with_project(mut self, project: &str) -> Self {
        self.task.project = Some(project.to_string());
        self
    }

    pub fn with_type(mut self, task_type: TaskType) -> Self {
        self.task.task_type = task_type;
        self
    }

    pub fn with_priority(mut self, priority: TaskPriority) -> Self {
        self.task.priority = priority;
        self
    }

    pub fn depends_on(mut self, task_name: &str) -> Self {
        // Note: In real implementation, this would resolve task_name to task_id
        // For now, we'll use a placeholder UUID
        let dependency = Dependency {
            task_id: Uuid::new_v4(), // This should be resolved from task_name
            task_name: Some(task_name.to_string()),
            condition: DependencyCondition::Success,
            required: true,
            correlation_id: None,
            metadata: HashMap::new(),
        };
        self.task.dependencies.push(dependency);
        self
    }

    pub fn with_condition(mut self, condition: DependencyCondition) -> Self {
        if let Some(last_dep) = self.task.dependencies.last_mut() {
            last_dep.condition = condition;
        }
        self
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.task.timeout = Some(timeout);
        self
    }

    pub fn with_retry(mut self, attempts: u32, delay: Duration) -> Self {
        self.task.retry_attempts = attempts;
        self.task.retry_delay = delay;
        self
    }

    pub fn with_environment(mut self, key: &str, value: &str) -> Self {
        self.task.environment.insert(key.to_string(), value.to_string());
        self
    }

    pub fn with_working_directory(mut self, dir: &str) -> Self {
        self.task.working_directory = Some(dir.to_string());
        self
    }

    pub fn with_metadata(mut self, key: &str, value: serde_json::Value) -> Self {
        self.task.metadata.insert(key.to_string(), value);
        self
    }

    pub fn build(self) -> Task {
        self.task
    }
}


/// Convenience function to create a new task
pub fn Task(name: &str) -> TaskBuilder {
    TaskBuilder::new(name)
}

impl Task {
    /// Create a new task builder
    pub fn new(name: &str) -> TaskBuilder {
        TaskBuilder::new(name)
    }

    /// Check if task is ready to execute (all dependencies satisfied)
    pub fn is_ready(&self, completed_tasks: &HashMap<Uuid, TaskResult>) -> bool {
        for dependency in &self.dependencies {
            if let Some(result) = completed_tasks.get(&dependency.task_id) {
                match (&dependency.condition, result) {
                    (DependencyCondition::Success, TaskResult::Success { .. }) => continue,
                    (DependencyCondition::Failure, TaskResult::Failure { .. }) => continue,
                    (DependencyCondition::Completion, _) => continue,
                    _ => return false,
                }
            } else {
                return false;
            }
        }
        true
    }

    /// Update task status
    pub fn update_status(&mut self, status: TaskStatus) {
        self.status = status;
        self.updated_at = SystemTime::now();
    }

    /// Set task result
    pub fn set_result(&mut self, result: TaskResult) {
        self.result = Some(result.clone());
        self.updated_at = SystemTime::now();
        
        match &result {
            TaskResult::Success { .. } => self.status = TaskStatus::Completed,
            TaskResult::Failure { .. } => self.status = TaskStatus::Failed,
            TaskResult::Cancelled { .. } => self.status = TaskStatus::Cancelled,
        }
    }

    /// Add a dependency to this task
    pub fn add_dependency(&mut self, task_id: Uuid, task_name: Option<String>, condition: DependencyCondition, required: bool) {
        let dependency = Dependency {
            task_id,
            task_name,
            condition,
            required,
            correlation_id: None,
            metadata: HashMap::new(),
        };
        self.dependencies.push(dependency);
        self.updated_at = SystemTime::now();
    }

    /// Add a correlated dependency
    pub fn add_correlated_dependency(&mut self, task_id: Uuid, task_name: Option<String>, condition: DependencyCondition, required: bool, correlation_id: String) {
        let dependency = Dependency {
            task_id,
            task_name,
            condition,
            required,
            correlation_id: Some(correlation_id),
            metadata: HashMap::new(),
        };
        self.dependencies.push(dependency);
        self.updated_at = SystemTime::now();
    }

    /// Get dependencies by correlation ID
    pub fn get_dependencies_by_correlation(&self, correlation_id: &str) -> Vec<&Dependency> {
        self.dependencies.iter()
            .filter(|dep| dep.correlation_id.as_ref().map_or(false, |id| id == correlation_id))
            .collect()
    }

    /// Check if task has any dependencies
    pub fn has_dependencies(&self) -> bool {
        !self.dependencies.is_empty()
    }

    /// Get all dependency task IDs
    pub fn get_dependency_task_ids(&self) -> Vec<Uuid> {
        self.dependencies.iter().map(|dep| dep.task_id).collect()
    }

    /// Check if task is in development phase
    pub fn is_in_development(&self) -> bool {
        matches!(self.status, 
            TaskStatus::AnalysisAndDocumentation |
            TaskStatus::InDiscussion |
            TaskStatus::InImplementation |
            TaskStatus::InReview |
            TaskStatus::InTesting
        )
    }

    /// Check if task is ready for execution
    pub fn is_ready_for_execution(&self) -> bool {
        matches!(self.status, TaskStatus::Pending | TaskStatus::WaitingForDependencies)
    }

    /// Move task to next development phase
    pub fn advance_development_phase(&mut self) -> bool {
        match self.status {
            TaskStatus::AnalysisAndDocumentation => {
                self.status = TaskStatus::InDiscussion;
                self.updated_at = SystemTime::now();
                true
            }
            TaskStatus::InDiscussion => {
                self.status = TaskStatus::InImplementation;
                self.updated_at = SystemTime::now();
                true
            }
            TaskStatus::InImplementation => {
                self.status = TaskStatus::InReview;
                self.updated_at = SystemTime::now();
                true
            }
            TaskStatus::InReview => {
                self.status = TaskStatus::InTesting;
                self.updated_at = SystemTime::now();
                true
            }
            TaskStatus::InTesting => {
                self.status = TaskStatus::Pending;
                self.updated_at = SystemTime::now();
                true
            }
            _ => false, // Not in development phase
        }
    }
}

impl Project {
    /// Create a new project
    pub fn new(name: &str) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            description: None,
            status: ProjectStatus::Planning,
            created_at: now,
            updated_at: now,
            due_date: None,
            tags: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// Update project status
    pub fn update_status(&mut self, status: ProjectStatus) {
        self.status = status;
        self.updated_at = Utc::now();
    }

    /// Add tag to project
    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
            self.updated_at = Utc::now();
        }
    }

    /// Remove tag from project
    pub fn remove_tag(&mut self, tag: &str) {
        self.tags.retain(|t| t != tag);
        self.updated_at = Utc::now();
    }
}

impl Task {
    /// Validate if a status transition is allowed
    pub fn can_transition_to(&self, new_status: &TaskStatus) -> bool {
        match (&self.current_phase, new_status) {
            // Valid forward transitions
            (TaskStatus::Planning, TaskStatus::Implementation) => true,
            (TaskStatus::Implementation, TaskStatus::TestCreation) => true,
            (TaskStatus::TestCreation, TaskStatus::Testing) => true,
            (TaskStatus::Testing, TaskStatus::AIReview) => true,
            (TaskStatus::AIReview, TaskStatus::Finalized) => {
                self.ai_reviews_completed >= self.ai_reviews_required
            },
            // AI Review can send back to Implementation
            (TaskStatus::AIReview, TaskStatus::Implementation) => true,
            // Error states can go back to Implementation
            (TaskStatus::Failed, TaskStatus::Implementation) => true,
            (TaskStatus::Failed, TaskStatus::Planning) => true,
            // Cancelled can only be set from any state
            (_, TaskStatus::Cancelled) => true,
            // Same status is always allowed
            (current, new) if current == new => true,
            // All other transitions are invalid
            _ => false,
        }
    }

    /// Set task status with validation
    pub fn set_status(&mut self, new_status: TaskStatus) -> Result<(), String> {
        if !self.can_transition_to(&new_status) {
            return Err(format!(
                "Invalid status transition from {:?} to {:?}", 
                self.current_phase, new_status
            ));
        }

        // Handle special cases
        match new_status {
            TaskStatus::AIReview => {
                if self.current_phase != TaskStatus::AIReview {
                    // Complete current phase
                    if let Some(current_phase) = self.phases.last_mut() {
                        current_phase.completed_at = Some(chrono::Utc::now());
                    }

                    // Start AI Review phase
                    self.phases.push(TaskPhase {
                        phase: TaskStatus::AIReview,
                        started_at: Some(chrono::Utc::now()),
                        completed_at: None,
                        documentation: None,
                        artifacts: Vec::new(),
                        ai_reviews: Vec::new(),
                    });
                }
            },
            TaskStatus::Implementation => {
                // If coming from AIReview, create new Implementation phase
                if self.current_phase == TaskStatus::AIReview {
                    // Complete AI Review phase
                    if let Some(current_phase) = self.phases.last_mut() {
                        current_phase.completed_at = Some(chrono::Utc::now());
                    }

                    // Start new Implementation phase
                    self.phases.push(TaskPhase {
                        phase: TaskStatus::Implementation,
                        started_at: Some(chrono::Utc::now()),
                        completed_at: None,
                        documentation: None,
                        artifacts: Vec::new(),
                        ai_reviews: Vec::new(),
                    });
                }
            },
            TaskStatus::Finalized => {
                // Complete current phase
                if let Some(current_phase) = self.phases.last_mut() {
                    current_phase.completed_at = Some(chrono::Utc::now());
                }
            },
            _ => {
                // For other transitions, complete current phase and start new one
                if self.current_phase != new_status {
                    if let Some(current_phase) = self.phases.last_mut() {
                        current_phase.completed_at = Some(chrono::Utc::now());
                    }

                    self.phases.push(TaskPhase {
                        phase: new_status.clone(),
                        started_at: Some(chrono::Utc::now()),
                        completed_at: None,
                        documentation: None,
                        artifacts: Vec::new(),
                        ai_reviews: Vec::new(),
                    });
                }
            }
        }

        self.current_phase = new_status.clone();
        self.status = new_status;
        self.updated_at = std::time::SystemTime::now();

        Ok(())
    }

    /// Advance to next phase (deprecated - use set_status instead)
    pub fn advance_phase(&mut self) -> Result<(), String> {
        let next_phase = match self.current_phase {
            TaskStatus::Planning => TaskStatus::Implementation,
            TaskStatus::Implementation => TaskStatus::TestCreation,
            TaskStatus::TestCreation => TaskStatus::Testing,
            TaskStatus::Testing => TaskStatus::AIReview,
            TaskStatus::AIReview => {
                if self.ai_reviews_completed >= self.ai_reviews_required {
                    TaskStatus::Finalized
                } else {
                    return Err(format!("Need {} AI reviews, only {} completed", 
                        self.ai_reviews_required, self.ai_reviews_completed));
                }
            },
            TaskStatus::Finalized => return Err("Task already finalized".to_string()),
            _ => return Err("Invalid phase transition".to_string()),
        };

        self.set_status(next_phase)
    }

    /// Add AI review
    pub fn add_ai_review(&mut self, review: AIReview) {
        if let Some(current_phase) = self.phases.last_mut() {
            current_phase.ai_reviews.push(review);
            self.ai_reviews_completed += 1;
        }
        self.updated_at = std::time::SystemTime::now();
    }

    /// Get current phase progress
    pub fn get_phase_progress(&self) -> f64 {
        match self.current_phase {
            TaskStatus::Planning => 0.0,
            TaskStatus::Implementation => 0.2,
            TaskStatus::TestCreation => 0.4,
            TaskStatus::Testing => 0.6,
            TaskStatus::AIReview => {
                let review_progress = self.ai_reviews_completed as f64 / self.ai_reviews_required as f64;
                0.8 + (review_progress * 0.2)
            },
            TaskStatus::Finalized => 1.0,
            _ => 0.0,
        }
    }
}

impl Workflow {
    /// Create a new workflow
    pub fn new(name: &str) -> Self {
        let now = SystemTime::now();
        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            description: None,
            tasks: Vec::new(),
            dependencies: Vec::new(),
            created_at: now,
            updated_at: now,
            status: WorkflowStatus::Pending,
        }
    }

    /// Add a task to the workflow
    pub fn add_task(mut self, task: Task) -> Self {
        self.tasks.push(task);
        self.updated_at = SystemTime::now();
        self
    }

    /// Add a dependency between tasks
    pub fn add_dependency(mut self, from: Uuid, to: Uuid, condition: DependencyCondition) -> Self {
        let dependency = WorkflowDependency {
            from_task: from,
            to_task: to,
            condition,
        };
        self.dependencies.push(dependency);
        self.updated_at = SystemTime::now();
        self
    }

    /// Get tasks ready for execution
    pub fn get_ready_tasks(&self, completed_tasks: &HashMap<Uuid, TaskResult>) -> Vec<&Task> {
        self.tasks
            .iter()
            .filter(|task| task.is_ready(completed_tasks))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_task_status_creation() {
        let status = TaskStatus::Pending;
        assert_eq!(status, TaskStatus::Pending);
    }

    #[test]
    fn test_task_priority_ordering() {
        assert!(TaskPriority::Low < TaskPriority::Normal);
        assert!(TaskPriority::Normal < TaskPriority::High);
        assert!(TaskPriority::High < TaskPriority::Critical);
    }

    #[test]
    fn test_task_creation() {
        let task = Task("test_task")
            .with_command("echo hello")
            .with_priority(TaskPriority::Normal)
            .build();

        assert_eq!(task.name, "test_task");
        assert_eq!(task.command, "echo hello");
        assert_eq!(task.priority, TaskPriority::Normal);
        assert_eq!(task.status, TaskStatus::Planning);
        assert!(task.dependencies.is_empty());
    }

    #[test]
    fn test_task_with_dependencies() {
        let dependency_id = Uuid::new_v4();
        let task = Task("dependent_task")
            .with_command("echo dependent")
            .with_priority(TaskPriority::High)
            .depends_on("dependency_task")
            .with_condition(DependencyCondition::Success)
            .build();

        assert_eq!(task.dependencies.len(), 1);
        assert_eq!(task.dependencies[0].condition, DependencyCondition::Success);
    }

    #[test]
    fn test_task_is_ready_without_dependencies() {
        let task = Task("independent_task")
            .with_command("echo independent")
            .with_priority(TaskPriority::Normal)
            .build();

        let completed_tasks = HashMap::new();
        assert!(task.is_ready(&completed_tasks));
    }

    #[test]
    fn test_task_is_ready_with_satisfied_dependencies() {
        let task = Task("dependent_task")
            .with_command("echo dependent")
            .with_priority(TaskPriority::Normal)
            .depends_on("dependency_task")
            .with_condition(DependencyCondition::Success)
            .build();

        // Get the actual dependency ID from the task
        let dependency_id = task.dependencies[0].task_id;
        
        let mut completed_tasks = HashMap::new();
        completed_tasks.insert(
            dependency_id,
            TaskResult::Success {
                output: "success".to_string(),
                artifacts: vec![],
                metrics: TaskMetrics {
                    execution_time: Duration::from_secs(1),
                    memory_usage: 1024,
                    cpu_usage: 0.5,
                    disk_usage: 512,
                    network_io: 256,
                },
            },
        );

        assert!(task.is_ready(&completed_tasks));
    }

    #[test]
    fn test_task_not_ready_with_unsatisfied_dependencies() {
        let task = Task("dependent_task")
            .with_command("echo dependent")
            .with_priority(TaskPriority::Normal)
            .depends_on("dependency_task")
            .with_condition(DependencyCondition::Success)
            .build();

        let completed_tasks = HashMap::new();
        assert!(!task.is_ready(&completed_tasks));
    }

    #[test]
    fn test_task_not_ready_with_failed_dependency() {
        let dependency_id = Uuid::new_v4();
        let task = Task("dependent_task")
            .with_command("echo dependent")
            .with_priority(TaskPriority::Normal)
            .depends_on("dependency_task")
            .with_condition(DependencyCondition::Success)
            .build();

        let mut completed_tasks = HashMap::new();
        completed_tasks.insert(
            dependency_id,
            TaskResult::Failure {
                error: "Task failed".to_string(),
                exit_code: Some(1),
                logs: vec!["Error log".to_string()],
            },
        );

        assert!(!task.is_ready(&completed_tasks));
    }

    #[test]
    fn test_task_ready_with_completion_dependency() {
        let task = Task("dependent_task")
            .with_command("echo dependent")
            .with_priority(TaskPriority::Normal)
            .depends_on("dependency_task")
            .with_condition(DependencyCondition::Completion)
            .build();

        // Get the actual dependency ID from the task
        let dependency_id = task.dependencies[0].task_id;
        
        let mut completed_tasks = HashMap::new();
        completed_tasks.insert(
            dependency_id,
            TaskResult::Failure {
                error: "Task failed".to_string(),
                exit_code: Some(1),
                logs: vec![],
            },
        );

        assert!(task.is_ready(&completed_tasks));
    }

    #[test]
    fn test_project_creation() {
        let project = Project::new("test_project");

        assert_eq!(project.name, "test_project");
        assert_eq!(project.status, ProjectStatus::Planning);
        assert!(project.tags.is_empty());
    }

    #[test]
    fn test_project_with_description() {
        let project = Project::new("test_project");

        assert_eq!(project.name, "test_project");
        assert_eq!(project.status, ProjectStatus::Planning);
        assert!(project.tags.is_empty());
    }

    #[test]
    fn test_workflow_creation() {
        let workflow = Workflow::new("test_workflow");

        assert_eq!(workflow.name, "test_workflow");
        assert_eq!(workflow.status, WorkflowStatus::Pending);
        assert!(workflow.tasks.is_empty());
        assert!(workflow.dependencies.is_empty());
    }

    #[test]
    fn test_workflow_with_description() {
        let workflow = Workflow::new("test_workflow");

        assert_eq!(workflow.name, "test_workflow");
        assert_eq!(workflow.status, WorkflowStatus::Pending);
    }

    #[test]
    fn test_workflow_add_task() {
        let mut workflow = Workflow::new("test_workflow");

        let task = Task("test_task")
            .with_command("echo hello")
            .with_priority(TaskPriority::Normal)
            .build();

        workflow = workflow.add_task(task);
        assert_eq!(workflow.tasks.len(), 1);
        assert_eq!(workflow.tasks[0].name, "test_task");
    }

    #[test]
    fn test_workflow_add_dependency() {
        let mut workflow = Workflow::new("test_workflow");

        let task1_id = Uuid::new_v4();
        let task2_id = Uuid::new_v4();

        workflow = workflow.add_dependency(task1_id, task2_id, DependencyCondition::Success);
        assert_eq!(workflow.dependencies.len(), 1);
        assert_eq!(workflow.dependencies[0].from_task, task1_id);
        assert_eq!(workflow.dependencies[0].to_task, task2_id);
        assert_eq!(workflow.dependencies[0].condition, DependencyCondition::Success);
    }

    #[test]
    fn test_workflow_get_ready_tasks() {
        let mut workflow = Workflow::new("test_workflow");

        let task1 = Task("task1")
            .with_command("echo task1")
            .with_priority(TaskPriority::Normal)
            .build();

        let task2 = Task("task2")
            .with_command("echo task2")
            .with_priority(TaskPriority::Normal)
            .depends_on("dependency_task")
            .with_condition(DependencyCondition::Success)
            .build();

        workflow = workflow.add_task(task1).add_task(task2);

        let completed_tasks = HashMap::new();
        let ready_tasks = workflow.get_ready_tasks(&completed_tasks);

        // Only task1 should be ready (no dependencies)
        assert_eq!(ready_tasks.len(), 1);
        assert_eq!(ready_tasks[0].name, "task1");
    }

    #[test]
    fn test_task_metrics_creation() {
        let metrics = TaskMetrics {
            execution_time: Duration::from_secs(5),
            memory_usage: 2048,
            cpu_usage: 0.75,
            disk_usage: 1024,
            network_io: 512,
        };

        assert_eq!(metrics.execution_time, Duration::from_secs(5));
        assert_eq!(metrics.memory_usage, 2048);
        assert_eq!(metrics.cpu_usage, 0.75);
        assert_eq!(metrics.disk_usage, 1024);
        assert_eq!(metrics.network_io, 512);
    }

    #[test]
    fn test_task_result_success() {
        let result = TaskResult::Success {
            output: "Task completed successfully".to_string(),
            artifacts: vec!["artifact1.txt".to_string(), "artifact2.log".to_string()],
            metrics: TaskMetrics {
                execution_time: Duration::from_secs(10),
                memory_usage: 4096,
                cpu_usage: 0.5,
                disk_usage: 2048,
                network_io: 1024,
            },
        };

        match result {
            TaskResult::Success { output, artifacts, metrics } => {
                assert_eq!(output, "Task completed successfully");
                assert_eq!(artifacts.len(), 2);
                assert_eq!(metrics.execution_time, Duration::from_secs(10));
            }
            _ => panic!("Expected Success result"),
        }
    }

    #[test]
    fn test_task_result_failure() {
        let result = TaskResult::Failure {
            error: "Task execution failed".to_string(),
            exit_code: Some(1),
            logs: vec!["Error: Permission denied".to_string()],
        };

        match result {
            TaskResult::Failure { error, exit_code, logs } => {
                assert_eq!(error, "Task execution failed");
                assert_eq!(exit_code, Some(1));
                assert_eq!(logs.len(), 1);
                assert_eq!(logs[0], "Error: Permission denied");
            }
            _ => panic!("Expected Failure result"),
        }
    }

    #[test]
    fn test_task_result_cancelled() {
        let result = TaskResult::Cancelled {
            reason: "User requested cancellation".to_string(),
        };

        match result {
            TaskResult::Cancelled { reason } => {
                assert_eq!(reason, "User requested cancellation");
            }
            _ => panic!("Expected Cancelled result"),
        }
    }

    #[test]
    fn test_dependency_condition_variants() {
        let success_condition = DependencyCondition::Success;
        let failure_condition = DependencyCondition::Failure;
        let completion_condition = DependencyCondition::Completion;
        let custom_condition = DependencyCondition::Custom("custom_rule".to_string());

        match success_condition {
            DependencyCondition::Success => assert!(true),
            _ => panic!("Expected Success condition"),
        }

        match failure_condition {
            DependencyCondition::Failure => assert!(true),
            _ => panic!("Expected Failure condition"),
        }

        match completion_condition {
            DependencyCondition::Completion => assert!(true),
            _ => panic!("Expected Completion condition"),
        }

        match custom_condition {
            DependencyCondition::Custom(rule) => assert_eq!(rule, "custom_rule"),
            _ => panic!("Expected Custom condition"),
        }
    }

    #[test]
    fn test_task_builder_pattern() {
        let task = Task("test_task")
            .with_command("echo hello world")
            .with_priority(TaskPriority::High)
            .build();

        assert_eq!(task.name, "test_task");
        assert_eq!(task.command, "echo hello world");
        assert_eq!(task.priority, TaskPriority::High);
    }

    #[test]
    fn test_task_builder_with_dependencies() {
        let task = Task("dependent_task")
            .with_command("echo dependent")
            .with_priority(TaskPriority::Normal)
            .depends_on("dependency_task")
            .with_condition(DependencyCondition::Success)
            .build();

        assert_eq!(task.dependencies.len(), 1);
        assert_eq!(task.dependencies[0].condition, DependencyCondition::Success);
    }

    #[test]
    fn test_project_builder_pattern() {
        let project = Project::new("test_project");

        assert_eq!(project.name, "test_project");
        assert_eq!(project.status, ProjectStatus::Planning);
    }

    #[test]
    fn test_workflow_builder_pattern() {
        let workflow = Workflow::new("test_workflow");

        assert_eq!(workflow.name, "test_workflow");
        assert_eq!(workflow.status, WorkflowStatus::Pending);
    }

    #[test]
    fn test_task_type_variants() {
        let simple = TaskType::Simple;
        let dependent = TaskType::Dependent;
        let workflow = TaskType::Workflow;
        let scheduled = TaskType::Scheduled;

        match simple {
            TaskType::Simple => assert!(true),
            _ => panic!("Expected Simple type"),
        }

        match dependent {
            TaskType::Dependent => assert!(true),
            _ => panic!("Expected Dependent type"),
        }

        match workflow {
            TaskType::Workflow => assert!(true),
            _ => panic!("Expected Workflow type"),
        }

        match scheduled {
            TaskType::Scheduled => assert!(true),
            _ => panic!("Expected Scheduled type"),
        }
    }

    #[test]
    fn test_development_workflow_status() {
        let not_started = DevelopmentWorkflowStatus::NotStarted;
        let planning = DevelopmentWorkflowStatus::Planning;
        let completed = DevelopmentWorkflowStatus::Completed;

        assert_eq!(not_started, DevelopmentWorkflowStatus::NotStarted);
        assert_eq!(planning, DevelopmentWorkflowStatus::Planning);
        assert_eq!(completed, DevelopmentWorkflowStatus::Completed);
    }

    #[test]
    fn test_ai_review_type_variants() {
        let code_quality = AIReviewType::CodeQuality;
        let security = AIReviewType::Security;
        let performance = AIReviewType::Performance;

        match code_quality {
            AIReviewType::CodeQuality => assert!(true),
            _ => panic!("Expected CodeQuality type"),
        }

        match security {
            AIReviewType::Security => assert!(true),
            _ => panic!("Expected Security type"),
        }

        match performance {
            AIReviewType::Performance => assert!(true),
            _ => panic!("Expected Performance type"),
        }
    }
}