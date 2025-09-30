//! CLI argument parsing and command structure

use clap::{Parser, Subcommand, Args, ValueEnum};
use std::path::PathBuf;

use crate::OutputFormat;

#[derive(Parser)]
#[command(name = "task-queue")]
#[command(about = "Task Queue CLI - Manage tasks, projects, and workflows")]
#[command(version)]
pub struct Cli {
    #[command(flatten)]
    pub global: GlobalArgs,
    
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Args)]
pub struct GlobalArgs {
    /// Configuration file path
    #[arg(long, global = true)]
    pub config: Option<PathBuf>,
    
    /// Server URL
    #[arg(long, global = true, default_value = "http://localhost:16080")]
    pub server_url: String,
    
    /// API key for authentication
    #[arg(long, global = true)]
    pub api_key: Option<String>,
    
    /// Enable verbose logging
    #[arg(short, long, global = true)]
    pub verbose: bool,
    
    /// Suppress output except errors
    #[arg(short, long, global = true)]
    pub quiet: bool,
    
    /// Output format
    #[arg(long, global = true, value_enum, default_value = "table")]
    pub format: OutputFormat,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Task management commands
    Tasks(TasksCommand),
    /// Project management commands
    Projects(ProjectsCommand),
    /// Workflow management commands
    Workflows(WorkflowsCommand),
    /// Server operations
    Server(ServerCommand),
    /// Configuration management
    Config(ConfigCommand),
    /// Interactive TUI mode
    Interactive,
    /// Generate shell completion scripts
    Completions {
        shell: clap_complete::Shell,
    },
}

#[derive(Args)]
pub struct TasksCommand {
    #[command(subcommand)]
    pub action: TasksAction,
}

#[derive(Subcommand)]
pub enum TasksAction {
    /// List tasks
    List {
        /// Filter by status
        #[arg(long)]
        status: Option<String>,
        /// Filter by project
        #[arg(long)]
        project: Option<String>,
        /// Filter by priority
        #[arg(long)]
        priority: Option<String>,
    },
    /// Create a new task
    Create {
        /// Task name
        #[arg(short, long)]
        name: String,
        /// Command to execute
        #[arg(short, long)]
        command: String,
        /// Project ID
        #[arg(short, long)]
        project: String,
        /// Task description
        #[arg(long)]
        description: Option<String>,
        /// Task priority
        #[arg(long, value_enum)]
        priority: Option<TaskPriority>,
        /// Working directory
        #[arg(long)]
        working_directory: Option<String>,
    },
    /// Get task details
    Get {
        /// Task ID
        task_id: String,
    },
    /// Update task
    Update {
        /// Task ID
        task_id: String,
        /// New task name
        #[arg(long)]
        name: Option<String>,
        /// New command
        #[arg(long)]
        command: Option<String>,
        /// New priority
        #[arg(long, value_enum)]
        priority: Option<TaskPriority>,
    },
    /// Cancel task
    Cancel {
        /// Task ID
        task_id: String,
        /// Cancellation reason
        #[arg(long)]
        reason: Option<String>,
    },
    /// Delete task
    Delete {
        /// Task ID
        task_id: String,
        /// Force deletion without confirmation
        #[arg(short, long)]
        force: bool,
    },
    /// Wait for task completion
    Wait {
        /// Task ID
        task_id: String,
        /// Timeout in seconds
        #[arg(long, default_value = "300")]
        timeout: u64,
    },
}

#[derive(Args)]
pub struct ProjectsCommand {
    #[command(subcommand)]
    pub action: ProjectsAction,
}

#[derive(Subcommand)]
pub enum ProjectsAction {
    /// List projects
    List,
    /// Create a new project
    Create {
        /// Project name
        #[arg(short, long)]
        name: String,
        /// Project description
        #[arg(long)]
        description: Option<String>,
    },
    /// Get project details
    Get {
        /// Project ID
        project_id: String,
    },
    /// Update project
    Update {
        /// Project ID
        project_id: String,
        /// New project name
        #[arg(long)]
        name: Option<String>,
        /// New description
        #[arg(long)]
        description: Option<String>,
    },
    /// Delete project
    Delete {
        /// Project ID
        project_id: String,
        /// Force deletion without confirmation
        #[arg(short, long)]
        force: bool,
    },
    /// List project tasks
    Tasks {
        /// Project ID
        project_id: String,
    },
}

#[derive(Args)]
pub struct WorkflowsCommand {
    #[command(subcommand)]
    pub action: WorkflowsAction,
}

#[derive(Subcommand)]
pub enum WorkflowsAction {
    /// List workflows
    List,
    /// Create a new workflow
    Create {
        /// Workflow name
        #[arg(short, long)]
        name: String,
        /// Task IDs (comma-separated)
        #[arg(short, long)]
        tasks: String,
        /// Workflow description
        #[arg(long)]
        description: Option<String>,
    },
    /// Get workflow details
    Get {
        /// Workflow ID
        workflow_id: String,
    },
    /// Start workflow
    Start {
        /// Workflow ID
        workflow_id: String,
    },
    /// Cancel workflow
    Cancel {
        /// Workflow ID
        workflow_id: String,
        /// Cancellation reason
        #[arg(long)]
        reason: Option<String>,
    },
    /// Get workflow status
    Status {
        /// Workflow ID
        workflow_id: String,
    },
}

#[derive(Args)]
pub struct ServerCommand {
    #[command(subcommand)]
    pub action: ServerAction,
}

#[derive(Subcommand)]
pub enum ServerAction {
    /// Show server status
    Status,
    /// Check server health
    Health,
    /// Show server metrics
    Metrics,
    /// Show server statistics
    Stats,
}

#[derive(Args)]
pub struct ConfigCommand {
    #[command(subcommand)]
    pub action: ConfigAction,
}

#[derive(Subcommand)]
pub enum ConfigAction {
    /// Show current configuration
    Show,
    /// Set configuration value
    Set {
        /// Configuration key
        key: String,
        /// Configuration value
        value: String,
    },
    /// Reset configuration to defaults
    Reset,
}

#[derive(Clone, Debug, ValueEnum)]
pub enum TaskPriority {
    Low,
    Normal,
    High,
    Critical,
}
