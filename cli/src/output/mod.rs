//! Output formatting utilities

use crate::OutputFormat;
use crate::client::{Task, Project, Workflow, ServerStats};
use comfy_table::{Table, presets::UTF8_FULL};
use serde_json;
use serde_yaml;

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
    
    pub fn format_task_details(&self, task: &Task) -> String {
        match self.format {
            OutputFormat::Table => self.format_task_details_table(task),
            OutputFormat::Json => serde_json::to_string_pretty(task).unwrap(),
            OutputFormat::Yaml => serde_yaml::to_string(task).unwrap(),
        }
    }
    
    pub fn format_projects(&self, projects: &[Project]) -> String {
        match self.format {
            OutputFormat::Table => self.format_projects_table(projects),
            OutputFormat::Json => serde_json::to_string_pretty(projects).unwrap(),
            OutputFormat::Yaml => serde_yaml::to_string(projects).unwrap(),
        }
    }
    
    pub fn format_workflows(&self, workflows: &[Workflow]) -> String {
        match self.format {
            OutputFormat::Table => self.format_workflows_table(workflows),
            OutputFormat::Json => serde_json::to_string_pretty(workflows).unwrap(),
            OutputFormat::Yaml => serde_yaml::to_string(workflows).unwrap(),
        }
    }
    
    pub fn format_server_stats(&self, stats: &ServerStats) -> String {
        match self.format {
            OutputFormat::Table => self.format_server_stats_table(stats),
            OutputFormat::Json => serde_json::to_string_pretty(stats).unwrap(),
            OutputFormat::Yaml => serde_yaml::to_string(stats).unwrap(),
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
                &task.priority,
                &task.project_id.map(|id| id.to_string()[..8].to_string()).unwrap_or_else(|| "-".to_string()),
                &task.created_at,
            ]);
        }
        
        table.to_string()
    }
    
    fn format_task_details_table(&self, task: &Task) -> String {
        let mut table = Table::new();
        table.load_preset(UTF8_FULL);
        
        table.add_row(vec!["Field", "Value"]);
        table.add_row(vec!["ID", &task.id.to_string()]);
        table.add_row(vec!["Name", &task.name]);
        table.add_row(vec!["Command", &task.command]);
        table.add_row(vec!["Description", &task.description]);
        table.add_row(vec!["Status", &format!("{:?}", task.status)]);
        table.add_row(vec!["Priority", &task.priority]);
        table.add_row(vec!["Project ID", &task.project_id.map(|id| id.to_string()).unwrap_or_else(|| "None".to_string())]);
        table.add_row(vec!["Created", &task.created_at]);
        table.add_row(vec!["Updated", &task.updated_at]);
        
        table.to_string()
    }
    
    fn format_projects_table(&self, projects: &[Project]) -> String {
        let mut table = Table::new();
        table.load_preset(UTF8_FULL);
        
        table.set_header(vec![
            "ID", "Name", "Description", "Created"
        ]);
        
        for project in projects {
            table.add_row(vec![
                &project.id.to_string()[..8],
                &project.name,
                project.description.as_deref().unwrap_or("-"),
                &project.created_at,
            ]);
        }
        
        table.to_string()
    }
    
    fn format_workflows_table(&self, workflows: &[Workflow]) -> String {
        let mut table = Table::new();
        table.load_preset(UTF8_FULL);
        
        table.set_header(vec![
            "ID", "Name", "Status", "Description", "Created"
        ]);
        
        for workflow in workflows {
            table.add_row(vec![
                &workflow.id.to_string()[..8],
                &workflow.name,
                &workflow.status,
                workflow.description.as_deref().unwrap_or("-"),
                &workflow.created_at,
            ]);
        }
        
        table.to_string()
    }
    
    fn format_server_stats_table(&self, stats: &ServerStats) -> String {
        let mut table = Table::new();
        table.load_preset(UTF8_FULL);
        
        table.add_row(vec!["Metric", "Value"]);
        table.add_row(vec!["Total Tasks", &stats.total_tasks.to_string()]);
        table.add_row(vec!["Active Tasks", &stats.active_tasks.to_string()]);
        table.add_row(vec!["Pending Tasks", &stats.pending_tasks.to_string()]);
        table.add_row(vec!["Completed Tasks", &stats.completed_tasks.to_string()]);
        table.add_row(vec!["Failed Tasks", &stats.failed_tasks.to_string()]);
        table.add_row(vec!["Total Workflows", &stats.total_workflows.to_string()]);
        
        table.to_string()
    }
}