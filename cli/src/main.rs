//! Task Queue CLI - Command-line interface for Task Queue system
//!
//! This CLI provides a comprehensive interface for managing tasks, projects,
//! workflows, and system operations through both command-line and interactive modes.

use clap::{Parser, CommandFactory, ValueEnum};
use clap_complete::{generate, Generator};
use std::io;
use anyhow::Result;

mod cli;
mod tui;
mod config;
mod client;
mod output;
mod utils;

use cli::args::{Cli, Commands};
use config::ConfigManager;
use client::ApiClient;

#[derive(Clone, Debug, ValueEnum, serde::Serialize, serde::Deserialize)]
pub enum OutputFormat {
    Table,
    Json,
    Yaml,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Parse command line arguments
    let args = Cli::parse();

    // Handle shell completion generation
    if let Commands::Completions { shell } = &args.command {
        let mut cmd = Cli::command();
        generate_completions(*shell, &mut cmd, "task-queue");
        return Ok(());
    }

    // Load configuration
    let config_manager = ConfigManager::new(args.global.config.clone())?;
    let config = config_manager.load_config()?;

    // Create API client
    let api_client = ApiClient::new(
        args.global.server_url.clone(),
        args.global.api_key.clone(),
        config.server.timeout,
        config.server.retry_attempts,
    );

    // Handle interactive mode
    if matches!(args.command, Commands::Interactive) {
        return tui::run_interactive_mode(api_client, config).await;
    }

    // Execute command
    execute_command(args.command, api_client, config, args.global.format).await?;

    Ok(())
}

async fn execute_command(
    command: Commands,
    api_client: ApiClient,
    config: config::CliConfig,
    format: OutputFormat,
) -> Result<()> {
    match command {
        Commands::Tasks(cmd) => cli::commands::tasks::handle_tasks_command(cmd, api_client, format).await,
        Commands::Projects(cmd) => cli::commands::projects::handle_projects_command(cmd, api_client, format).await,
        Commands::Workflows(cmd) => cli::commands::workflows::handle_workflows_command(cmd, api_client, format).await,
        Commands::Server(cmd) => cli::commands::server::handle_server_command(cmd, api_client, format).await,
        Commands::Config(cmd) => cli::commands::config::handle_config_command(cmd, config).await,
        Commands::Interactive => unreachable!(), // Handled in main()
        Commands::Completions { .. } => unreachable!(), // Handled in main()
    }
}

fn generate_completions<G: Generator>(generator: G, cmd: &mut clap::Command, name: &str) {
    generate(generator, cmd, name, &mut io::stdout());
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn test_cli_parsing_basic() {
        let args = Cli::try_parse_from(&["task-queue", "tasks", "list"]).unwrap();
        assert!(matches!(args.command, Commands::Tasks(_)));
    }

    #[test]
    fn test_cli_parsing_with_options() {
        let args = Cli::try_parse_from(&[
            "task-queue",
            "--server-url", "http://localhost:3000",
            "--verbose",
            "tasks", "create",
            "--name", "test-task",
            "--command", "echo hello",
            "--project", "123e4567-e89b-12d3-a456-426614174000"
        ]).unwrap();
        
        assert_eq!(args.global.server_url, "http://localhost:3000");
        assert!(args.global.verbose);
        assert!(matches!(args.command, Commands::Tasks(_)));
    }

    #[test]
    fn test_cli_parsing_interactive() {
        let args = Cli::try_parse_from(&["task-queue", "interactive"]).unwrap();
        assert!(matches!(args.command, Commands::Interactive));
    }

    #[test]
    fn test_cli_parsing_completions() {
        let args = Cli::try_parse_from(&["task-queue", "completions", "bash"]).unwrap();
        assert!(matches!(args.command, Commands::Completions { .. }));
    }

    #[test]
    fn test_cli_parsing_projects() {
        let args = Cli::try_parse_from(&[
            "task-queue",
            "projects", "create",
            "--name", "test-project",
            "--description", "A test project"
        ]).unwrap();
        
        assert!(matches!(args.command, Commands::Projects(_)));
    }

    #[test]
    fn test_cli_parsing_server() {
        let args = Cli::try_parse_from(&["task-queue", "server", "status"]).unwrap();
        assert!(matches!(args.command, Commands::Server(_)));
    }

    #[test]
    fn test_cli_parsing_config() {
        let args = Cli::try_parse_from(&["task-queue", "config", "show"]).unwrap();
        assert!(matches!(args.command, Commands::Config(_)));
    }

    #[test]
    fn test_cli_parsing_workflows() {
        let args = Cli::try_parse_from(&[
            "task-queue",
            "workflows", "create",
            "--name", "test-workflow",
            "--tasks", "task1,task2"
        ]).unwrap();
        
        assert!(matches!(args.command, Commands::Workflows(_)));
    }

    #[test]
    fn test_cli_parsing_invalid_command() {
        let result = Cli::try_parse_from(&["task-queue", "invalid-command"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_cli_parsing_missing_required_args() {
        let result = Cli::try_parse_from(&[
            "task-queue",
            "tasks", "create",
            "--name", "test-task"
            // Missing required --command and --project
        ]);
        assert!(result.is_err());
    }

    #[test]
    fn test_output_format_enum() {
        assert_eq!(format!("{:?}", OutputFormat::Table), "Table");
        assert_eq!(format!("{:?}", OutputFormat::Json), "Json");
        assert_eq!(format!("{:?}", OutputFormat::Yaml), "Yaml");
    }

    #[test]
    fn test_task_priority_enum() {
        use crate::cli::args::TaskPriority;
        assert_eq!(format!("{:?}", TaskPriority::Low), "Low");
        assert_eq!(format!("{:?}", TaskPriority::Normal), "Normal");
        assert_eq!(format!("{:?}", TaskPriority::High), "High");
        assert_eq!(format!("{:?}", TaskPriority::Critical), "Critical");
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use crate::cli::args::{Cli, Commands, TasksAction, ProjectsAction, ServerAction};

    #[test]
    fn test_tasks_command_integration() {
        // Test tasks list command
        let args = Cli::try_parse_from(&["task-queue", "tasks", "list"]).unwrap();
        if let Commands::Tasks(cmd) = args.command {
            assert!(matches!(cmd.action, TasksAction::List { .. }));
        } else {
            panic!("Expected Tasks command");
        }

        // Test tasks create command
        let args = Cli::try_parse_from(&[
            "task-queue", "tasks", "create",
            "--name", "test-task",
            "--command", "echo hello",
            "--project", "123e4567-e89b-12d3-a456-426614174000"
        ]).unwrap();
        
        if let Commands::Tasks(cmd) = args.command {
            assert!(matches!(cmd.action, TasksAction::Create { .. }));
        } else {
            panic!("Expected Tasks command");
        }

        // Test tasks get command
        let args = Cli::try_parse_from(&[
            "task-queue", "tasks", "get",
            "123e4567-e89b-12d3-a456-426614174000"
        ]).unwrap();
        
        if let Commands::Tasks(cmd) = args.command {
            assert!(matches!(cmd.action, TasksAction::Get { .. }));
        } else {
            panic!("Expected Tasks command");
        }
    }

    #[test]
    fn test_projects_command_integration() {
        // Test projects list command
        let args = Cli::try_parse_from(&["task-queue", "projects", "list"]).unwrap();
        if let Commands::Projects(cmd) = args.command {
            assert!(matches!(cmd.action, ProjectsAction::List));
        } else {
            panic!("Expected Projects command");
        }

        // Test projects create command
        let args = Cli::try_parse_from(&[
            "task-queue", "projects", "create",
            "--name", "test-project",
            "--description", "A test project"
        ]).unwrap();
        
        if let Commands::Projects(cmd) = args.command {
            assert!(matches!(cmd.action, ProjectsAction::Create { .. }));
        } else {
            panic!("Expected Projects command");
        }
    }

    #[test]
    fn test_server_command_integration() {
        // Test server status command
        let args = Cli::try_parse_from(&["task-queue", "server", "status"]).unwrap();
        if let Commands::Server(cmd) = args.command {
            assert!(matches!(cmd.action, ServerAction::Status));
        } else {
            panic!("Expected Server command");
        }

        // Test server health command
        let args = Cli::try_parse_from(&["task-queue", "server", "health"]).unwrap();
        if let Commands::Server(cmd) = args.command {
            assert!(matches!(cmd.action, ServerAction::Health));
        } else {
            panic!("Expected Server command");
        }
    }

    #[test]
    fn test_global_options_integration() {
        // Test with verbose flag
        let args = Cli::try_parse_from(&[
            "task-queue", "--verbose", "tasks", "list"
        ]).unwrap();
        assert!(args.global.verbose);

        // Test with quiet flag
        let args = Cli::try_parse_from(&[
            "task-queue", "--quiet", "tasks", "list"
        ]).unwrap();
        assert!(args.global.quiet);

        // Test with custom server URL
        let args = Cli::try_parse_from(&[
            "task-queue", "--server-url", "http://custom:8080", "tasks", "list"
        ]).unwrap();
        assert_eq!(args.global.server_url, "http://custom:8080");

        // Test with API key
        let args = Cli::try_parse_from(&[
            "task-queue", "--api-key", "secret-key", "tasks", "list"
        ]).unwrap();
        assert_eq!(args.global.api_key, Some("secret-key".to_string()));

        // Test with JSON format
        let args = Cli::try_parse_from(&[
            "task-queue", "--format", "json", "tasks", "list"
        ]).unwrap();
        assert!(matches!(args.global.format, OutputFormat::Json));

        // Test with YAML format
        let args = Cli::try_parse_from(&[
            "task-queue", "--format", "yaml", "tasks", "list"
        ]).unwrap();
        assert!(matches!(args.global.format, OutputFormat::Yaml));
    }

    #[test]
    fn test_command_combinations() {
        // Test multiple global options
        let args = Cli::try_parse_from(&[
            "task-queue",
            "--verbose",
            "--server-url", "http://test:8080",
            "--format", "json",
            "tasks", "list", "--status", "pending"
        ]).unwrap();
        
        assert!(args.global.verbose);
        assert_eq!(args.global.server_url, "http://test:8080");
        assert!(matches!(args.global.format, OutputFormat::Json));
        
        if let Commands::Tasks(cmd) = args.command {
            if let TasksAction::List { status, .. } = cmd.action {
                assert_eq!(status, Some("pending".to_string()));
            } else {
                panic!("Expected List action");
            }
        } else {
            panic!("Expected Tasks command");
        }
    }

    #[test]
    fn test_error_cases() {
        // Test invalid UUID format
        let result = Cli::try_parse_from(&[
            "task-queue", "tasks", "create",
            "--name", "test",
            "--command", "echo",
            "--project", "invalid-uuid"
        ]);
        // This should parse successfully (UUID validation happens later)
        assert!(result.is_ok());

        // Test missing required arguments
        let result = Cli::try_parse_from(&[
            "task-queue", "tasks", "create",
            "--name", "test"
            // Missing --command and --project
        ]);
        assert!(result.is_err());

        // Test invalid command
        let result = Cli::try_parse_from(&[
            "task-queue", "invalid-command"
        ]);
        assert!(result.is_err());

        // Test invalid subcommand
        let result = Cli::try_parse_from(&[
            "task-queue", "tasks", "invalid-action"
        ]);
        assert!(result.is_err());
    }

    #[test]
    fn test_help_and_version() {
        // Test help generation (should not panic)
        let mut cmd = Cli::command();
        let help = cmd.render_help().to_string();
        assert!(help.contains("Task Queue CLI"));
        assert!(help.contains("Manage tasks, projects, and workflows"));

        // Test version
        let version = cmd.render_version();
        assert!(version.contains("task-queue"));
    }
}