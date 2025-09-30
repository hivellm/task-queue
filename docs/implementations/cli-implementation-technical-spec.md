# CLI Implementation - Technical Specification

## Overview

This document outlines the comprehensive implementation of a Command-Line Interface (CLI) for the Task Queue system. The CLI will provide a powerful, user-friendly interface for managing tasks, projects, workflows, and system operations.

## Architecture

### Core Components

1. **CLI Framework**: Built on `clap` v4.x for argument parsing and command structure
2. **Interactive TUI**: Powered by `ratatui` for rich terminal user interface
3. **Configuration Management**: Using `serde` for configuration serialization
4. **Async Operations**: Leveraging `tokio` for asynchronous task operations
5. **Progress Indicators**: Using `indicatif` for progress bars and spinners
6. **Shell Completion**: Auto-generated completion scripts for bash, zsh, fish, and PowerShell

### Command Structure

```
task-queue [GLOBAL_OPTIONS] <COMMAND> [COMMAND_OPTIONS]
```

## Commands Specification

### Global Options

- `--config <PATH>`: Specify custom configuration file path
- `--server-url <URL>`: Override server URL (default: http://localhost:16080)
- `--api-key <KEY>`: API authentication key
- `--verbose, -v`: Enable verbose logging
- `--quiet, -q`: Suppress output except errors
- `--json`: Output in JSON format
- `--yaml`: Output in YAML format
- `--table`: Output in table format (default)

### Core Commands

#### 1. Task Management

```bash
# List tasks
task-queue tasks list [--status <STATUS>] [--project <PROJECT>] [--priority <PRIORITY>]

# Create task
task-queue tasks create --name <NAME> --command <COMMAND> --project <PROJECT_ID> [OPTIONS]

# Get task details
task-queue tasks get <TASK_ID>

# Update task
task-queue tasks update <TASK_ID> [--name <NAME>] [--command <COMMAND>] [--priority <PRIORITY>]

# Cancel task
task-queue tasks cancel <TASK_ID> [--reason <REASON>]

# Delete task
task-queue tasks delete <TASK_ID> [--force]

# Wait for task completion
task-queue tasks wait <TASK_ID> [--timeout <SECONDS>]
```

#### 2. Project Management

```bash
# List projects
task-queue projects list

# Create project
task-queue projects create --name <NAME> [--description <DESCRIPTION>]

# Get project details
task-queue projects get <PROJECT_ID>

# Update project
task-queue projects update <PROJECT_ID> [--name <NAME>] [--description <DESCRIPTION>]

# Delete project
task-queue projects delete <PROJECT_ID> [--force]

# List project tasks
task-queue projects tasks <PROJECT_ID>
```

#### 3. Workflow Management

```bash
# List workflows
task-queue workflows list

# Create workflow
task-queue workflows create --name <NAME> --tasks <TASK_IDS> [OPTIONS]

# Get workflow details
task-queue workflows get <WORKFLOW_ID>

# Start workflow
task-queue workflows start <WORKFLOW_ID>

# Cancel workflow
task-queue workflows cancel <WORKFLOW_ID> [--reason <REASON>]

# Get workflow status
task-queue workflows status <WORKFLOW_ID>
```

#### 4. System Operations

```bash
# Server status
task-queue server status

# Server health check
task-queue server health

# Server metrics
task-queue server metrics

# Server stats
task-queue server stats

# Configuration management
task-queue config show
task-queue config set <KEY> <VALUE>
task-queue config reset
```

#### 5. Interactive Mode

```bash
# Launch interactive TUI
task-queue interactive
task-queue i  # Short alias
```

## Technical Implementation

### 1. CLI Framework Setup

```rust
use clap::{Parser, Subcommand, Args};
use clap_complete::{generate, Generator, Shell};

#[derive(Parser)]
#[command(name = "task-queue")]
#[command(about = "Task Queue CLI - Manage tasks, projects, and workflows")]
#[command(version)]
struct Cli {
    #[command(flatten)]
    global: GlobalArgs,
    
    #[command(subcommand)]
    command: Commands,
}

#[derive(Args)]
struct GlobalArgs {
    /// Configuration file path
    #[arg(long, global = true)]
    config: Option<PathBuf>,
    
    /// Server URL
    #[arg(long, global = true, default_value = "http://localhost:16080")]
    server_url: String,
    
    /// API key for authentication
    #[arg(long, global = true)]
    api_key: Option<String>,
    
    /// Enable verbose logging
    #[arg(short, long, global = true)]
    verbose: bool,
    
    /// Suppress output except errors
    #[arg(short, long, global = true)]
    quiet: bool,
    
    /// Output format
    #[arg(long, global = true, value_enum, default_value = "table")]
    format: OutputFormat,
}

#[derive(Subcommand)]
enum Commands {
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
        shell: Shell,
    },
}
```

### 2. Interactive TUI Implementation

```rust
use ratatui::{
    prelude::*,
    widgets::*,
    Terminal,
};

pub struct TuiApp {
    pub state: AppState,
    pub should_quit: bool,
}

pub enum AppState {
    TaskList,
    TaskDetails(String),
    ProjectList,
    ProjectDetails(String),
    WorkflowList,
    ServerStatus,
    Settings,
}

impl TuiApp {
    pub fn new() -> Self {
        Self {
            state: AppState::TaskList,
            should_quit: false,
        }
    }
    
    pub async fn run(&mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
        loop {
            terminal.draw(|f| self.ui(f))?;
            
            if let Event::Key(key) = event::read()? {
                self.handle_key_event(key).await?;
            }
            
            if self.should_quit {
                break;
            }
        }
        Ok(())
    }
    
    fn ui(&self, f: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header
                Constraint::Min(0),    // Main content
                Constraint::Length(3), // Footer
            ])
            .split(f.size());
            
        self.render_header(f, chunks[0]);
        self.render_main_content(f, chunks[1]);
        self.render_footer(f, chunks[2]);
    }
}
```

### 3. Configuration Management

```rust
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct CliConfig {
    pub server: ServerConfig,
    pub ui: UiConfig,
    pub output: OutputConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerConfig {
    pub url: String,
    pub api_key: Option<String>,
    pub timeout: u64,
    pub retry_attempts: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UiConfig {
    pub theme: Theme,
    pub refresh_interval: u64,
    pub show_progress: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OutputConfig {
    pub default_format: OutputFormat,
    pub table_style: TableStyle,
    pub colors: bool,
}

impl Default for CliConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                url: "http://localhost:16080".to_string(),
                api_key: None,
                timeout: 30,
                retry_attempts: 3,
            },
            ui: UiConfig {
                theme: Theme::Default,
                refresh_interval: 1,
                show_progress: true,
            },
            output: OutputConfig {
                default_format: OutputFormat::Table,
                table_style: TableStyle::Default,
                colors: true,
            },
        }
    }
}
```

### 4. Output Formatting

```rust
use comfy_table::{Table, presets::UTF8_FULL};
use serde_json;
use serde_yaml;

pub enum OutputFormat {
    Table,
    Json,
    Yaml,
}

pub struct OutputFormatter {
    format: OutputFormat,
    colors: bool,
}

impl OutputFormatter {
    pub fn new(format: OutputFormat, colors: bool) -> Self {
        Self { format, colors }
    }
    
    pub fn format_tasks(&self, tasks: &[Task]) -> String {
        match self.format {
            OutputFormat::Table => self.format_tasks_table(tasks),
            OutputFormat::Json => serde_json::to_string_pretty(tasks).unwrap(),
            OutputFormat::Yaml => serde_yaml::to_string(tasks).unwrap(),
        }
    }
    
    fn format_tasks_table(&self, tasks: &[Task]) -> String {
        let mut table = Table::new();
        table.load_preset(UTF8_FULL);
        
        table.set_header(vec![
            "ID", "Name", "Status", "Priority", "Project", "Created"
        ]);
        
        for task in tasks {
            table.add_row(vec![
                &task.id.to_string()[..8],
                &task.name,
                &format!("{:?}", task.status),
                &format!("{:?}", task.priority),
                task.project.as_deref().unwrap_or("-"),
                &format!("{:?}", task.created_at),
            ]);
        }
        
        table.to_string()
    }
}
```

### 5. Progress Indicators

```rust
use indicatif::{ProgressBar, ProgressStyle, MultiProgress};

pub struct ProgressManager {
    multi: MultiProgress,
}

impl ProgressManager {
    pub fn new() -> Self {
        Self {
            multi: MultiProgress::new(),
        }
    }
    
    pub fn create_task_progress(&self, task_name: &str) -> ProgressBar {
        let pb = self.multi.add(ProgressBar::new_spinner());
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} {msg}")
                .unwrap()
                .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
        );
        pb.set_message(format!("Processing task: {}", task_name));
        pb
    }
    
    pub fn create_download_progress(&self, total: u64) -> ProgressBar {
        let pb = self.multi.add(ProgressBar::new(total));
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
                .unwrap()
                .progress_chars("█▉▊▋▌▍▎▏  "),
        );
        pb
    }
}
```

### 6. Shell Completion Generation

```rust
use clap_complete::{generate, Generator, Shell};
use std::io;

pub fn generate_completions<G: Generator>(generator: G, cmd: &mut Command, name: &str) {
    generate(generator, cmd, name, &mut io::stdout());
}

// Usage in main:
if let Commands::Completions { shell } = &args.command {
    let mut cmd = Cli::command();
    generate_completions(*shell, &mut cmd, "task-queue");
    return Ok(());
}
```

## Error Handling

### Custom Error Types

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("Server connection failed: {0}")]
    ConnectionError(String),
    
    #[error("Authentication failed: {0}")]
    AuthenticationError(String),
    
    #[error("Task not found: {0}")]
    TaskNotFound(String),
    
    #[error("Invalid configuration: {0}")]
    ConfigError(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}
```

## Testing Strategy

### 1. Unit Tests

- Command parsing tests
- Configuration loading/saving tests
- Output formatting tests
- Error handling tests

### 2. Integration Tests

- API communication tests
- End-to-end workflow tests
- Interactive mode tests

### 3. Manual Testing

- Cross-platform compatibility (Windows, macOS, Linux)
- Shell completion testing
- Performance testing with large datasets

## Dependencies

```toml
[dependencies]
clap = { version = "4.4", features = ["derive", "env"] }
clap_complete = "4.4"
ratatui = "0.24"
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
serde_yaml = "0.9"
comfy-table = "7.1"
indicatif = "0.17"
crossterm = "0.27"
reqwest = { version = "0.11", features = ["json"] }
uuid = { version = "1.0", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
anyhow = "1.0"
thiserror = "1.0"
dirs = "5.0"
```

## File Structure

```
src/
├── main.rs                 # Entry point
├── cli/
│   ├── mod.rs
│   ├── args.rs            # Command line arguments
│   ├── commands/
│   │   ├── mod.rs
│   │   ├── tasks.rs       # Task management commands
│   │   ├── projects.rs    # Project management commands
│   │   ├── workflows.rs   # Workflow management commands
│   │   ├── server.rs      # Server operations
│   │   └── config.rs      # Configuration management
│   └── output/
│       ├── mod.rs
│       ├── formatter.rs   # Output formatting
│       └── table.rs       # Table rendering
├── tui/
│   ├── mod.rs
│   ├── app.rs             # Main TUI application
│   ├── components/
│   │   ├── mod.rs
│   │   ├── task_list.rs   # Task list component
│   │   ├── task_details.rs # Task details component
│   │   └── status_bar.rs  # Status bar component
│   └── events.rs          # Event handling
├── config/
│   ├── mod.rs
│   └── manager.rs         # Configuration management
├── client/
│   ├── mod.rs
│   └── api.rs             # API client
└── utils/
    ├── mod.rs
    ├── progress.rs        # Progress indicators
    └── completions.rs     # Shell completion
```

## Implementation Phases

### Phase 1: Core CLI Framework
1. Set up clap command structure
2. Implement basic command parsing
3. Create configuration management
4. Add basic error handling

### Phase 2: API Integration
1. Implement HTTP client for Task Queue API
2. Add authentication support
3. Implement retry logic and error handling
4. Add request/response logging

### Phase 3: Command Implementation
1. Implement task management commands
2. Implement project management commands
3. Implement workflow management commands
4. Implement server operations

### Phase 4: Output Formatting
1. Implement table output formatting
2. Add JSON/YAML output support
3. Add progress indicators
4. Implement colored output

### Phase 5: Interactive TUI
1. Set up ratatui framework
2. Implement main application loop
3. Create task list view
4. Add task details view
5. Implement navigation and key bindings

### Phase 6: Advanced Features
1. Add shell completion generation
2. Implement configuration management
3. Add plugin system hooks
4. Implement batch operations

### Phase 7: Testing and Polish
1. Add comprehensive test suite
2. Cross-platform testing
3. Performance optimization
4. Documentation and examples

## Success Criteria

1. **Functionality**: All core commands work correctly
2. **Usability**: Intuitive command structure and helpful error messages
3. **Performance**: Fast response times and efficient resource usage
4. **Reliability**: Robust error handling and graceful degradation
5. **Compatibility**: Works on Windows, macOS, and Linux
6. **Documentation**: Complete user guide and API documentation
7. **Testing**: 90%+ test coverage with comprehensive integration tests

## Future Enhancements

1. **Plugin System**: Support for custom commands and extensions
2. **Scripting**: Built-in scripting language for automation
3. **Cloud Integration**: Support for cloud-based Task Queue instances
4. **Advanced Analytics**: Built-in reporting and analytics features
5. **Team Collaboration**: Multi-user support and permissions
