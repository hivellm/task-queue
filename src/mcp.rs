//! MCP (Model Context Protocol) integration using rmcp crate
//!
//! This module provides MCP functionality using the rmcp crate with SSE transport,
//! compatible with Cursor's MCP client.
//!

use std::sync::Arc;
use std::borrow::Cow;

use axum::Router as AxumRouter;
use rmcp::{
    handler::server::ServerHandler,
    model::{CallToolResult, Content, ErrorData, ListToolsResult, ListResourcesResult, ProtocolVersion, ServerCapabilities, Tool, ServerInfo, Implementation},
    transport::sse_server::{SseServer, SseServerConfig},
    service::{RequestContext, RoleServer},
};
use std::net::SocketAddr;
use serde_json::json;

use crate::server::TaskQueueServer;

#[derive(Clone)]
pub struct TaskQueueMcpServer {
    task_queue: Arc<TaskQueueServer>,
}

impl TaskQueueMcpServer {
    pub fn new(task_queue: Arc<TaskQueueServer>) -> Self {
        Self { task_queue }
    }

    /// Generate workflow instructions based on current task status
    fn generate_workflow_instructions(&self, task: &crate::core::Task) -> String {
        // Use current_phase for dynamic instructions
        let current_phase = &task.current_phase;
        
        match current_phase {
            crate::core::TaskStatus::Planning => {
                format!(r#"
📋 **WORKFLOW INSTRUCTIONS - PHASE: PLANNING**

🚨 **CRITICAL**: You MUST follow this exact workflow process. Do NOT skip phases!

**PHASE 1 - PLANNING** (Current Phase)
✅ **REQUIRED ACTIONS**:
1. Generate comprehensive technical documentation in `/docs` directory
2. Document all implementation details, architecture decisions, and technical specifications
3. Create detailed implementation plan with code examples
4. Document API contracts, data structures, and integration points
5. **AFTER completing documentation, change task status to "Implementation"**

**NEXT PHASES** (Do NOT proceed until current phase is completed):
- **Implementation**: Create actual code based on documentation
- **TestCreation**: Create comprehensive test suite
- **Testing**: Execute tests and fix issues
- **AIReview**: Get reviews from 3 AI models before completion

⚠️ **FAILURE TO FOLLOW WORKFLOW WILL RESULT IN TASK REJECTION**
                "#)
            },
            crate::core::TaskStatus::Implementation => {
                format!(r#"
📋 **WORKFLOW INSTRUCTIONS - PHASE: IMPLEMENTATION** (IN PROGRESS)

🔄 **CONTINUE IMPLEMENTATION PHASE**:
- Implement code according to technical documentation created in Planning phase
- Follow all architectural decisions and technical specifications documented
- Ensure code quality and follow established patterns
- **When implementation is complete, change status to "TestCreation"**

📝 **Remember**: Test creation comes AFTER implementation is finished
                "#)
            },
            crate::core::TaskStatus::TestCreation => {
                format!(r#"
📋 **WORKFLOW INSTRUCTIONS - PHASE: TEST CREATION** (IN PROGRESS)

🔄 **CONTINUE TEST CREATION PHASE**:
- Create comprehensive test suite (unit, integration, e2e)
- Aim for 90%+ code coverage
- Test all edge cases and error scenarios
- Include performance and security tests
- **When all tests are created, change status to "Testing"**

🧪 **Focus**: Test creation, not test execution yet
                "#)
            },
            crate::core::TaskStatus::Testing => {
                format!(r#"
📋 **WORKFLOW INSTRUCTIONS - PHASE: TESTING** (IN PROGRESS)

🚨 **CRITICAL TESTING REQUIREMENTS**:
- **MUST ACTUALLY EXECUTE ALL TESTS** using `cargo test` command
- **ALL tests must PASS** before advancing to AIReview phase
- **Test Coverage**: Achieve minimum 85% code coverage with actual execution
- **Test Execution**: Use `run_terminal_cmd` to execute tests and verify results
- **Test Fixes**: Fix any failing tests before proceeding to next phase
- **Test Documentation**: Document test results and coverage metrics

⚠️ **NO ADVANCEMENT WITHOUT PASSING TESTS** - All tests must be executed and pass successfully!

**When ALL tests pass, change status to "AIReview"**
                "#)
            },
            crate::core::TaskStatus::AIReview => {
                format!(r#"
📋 **WORKFLOW INSTRUCTIONS - PHASE: AI REVIEW** (IN PROGRESS)

🔄 **CONTINUE AI REVIEW PHASE**:
- Select 3 different AI models for code review
- Each model must generate a detailed review report
- Address all critical issues found
- Document review results and improvements made
- **When all 3 AI models approve the code, change status to "Completed"**

🤖 **REQUIREMENT**: All 3 AI models must approve before completion
                "#)
            },
            crate::core::TaskStatus::Completed => {
                "✅ **WORKFLOW COMPLETED**: Task has successfully passed all phases.".to_string()
            },
            crate::core::TaskStatus::Failed => {
                "❌ **WORKFLOW FAILED**: Task did not meet quality standards.".to_string()
            },
            _ => {
                format!(r#"
📋 **WORKFLOW INSTRUCTIONS - PHASE: {:?}**

🚀 **READY TO BEGIN**:
- Use 'advance_workflow_phase' tool to start the Planning phase
- First step: Generate comprehensive technical documentation

**Next**: Planning Phase
                "#, current_phase)
            }
        }
    }

    /// Generate workflow instructions based on workflow status (not task status)
    fn generate_workflow_instructions_from_status(&self, status: &crate::core::DevelopmentWorkflowStatus) -> String {
        match status {
            crate::core::DevelopmentWorkflowStatus::Planning => {
                r#"
📋 **WORKFLOW STATUS: PLANNING** (IN PROGRESS)

🔄 **CONTINUE PLANNING PHASE**:
- Complete technical documentation in `/docs`
- Ensure all implementation details are documented
- Document all edge cases and error scenarios
- Use 'set_technical_documentation' tool when documentation is complete

**Next**: Implementation Phase
                "#.to_string()
            },
            crate::core::DevelopmentWorkflowStatus::InImplementation => {
                r#"
📋 **WORKFLOW STATUS: IMPLEMENTATION** (IN PROGRESS)

🔄 **CONTINUE IMPLEMENTATION PHASE**:
- Implement code according to technical documentation created in Planning phase
- Follow all architectural decisions and technical specifications documented
- Ensure code quality and follow established patterns
- Use 'advance_workflow_phase' tool when implementation is complete

**Next**: Test Creation Phase
                "#.to_string()
            },
            crate::core::DevelopmentWorkflowStatus::TestCreation => {
                r#"
📋 **WORKFLOW STATUS: TEST CREATION** (IN PROGRESS)

🔄 **CONTINUE TEST CREATION PHASE**:
- Create comprehensive test suite (unit, integration, e2e)
- Aim for 90%+ code coverage
- Test all edge cases and error scenarios
- Include performance and security tests
- Use 'set_test_coverage' and 'advance_workflow_phase' tools when tests are created

**Next**: Testing Phase
                "#.to_string()
            },
            crate::core::DevelopmentWorkflowStatus::Testing => {
                r#"
📋 **WORKFLOW STATUS: TESTING** (IN PROGRESS)

🚨 **CRITICAL TESTING REQUIREMENTS**:
- **MUST ACTUALLY EXECUTE ALL TESTS** using `cargo test` command
- **ALL tests must PASS** before advancing to AIReview phase
- **Test Coverage**: Achieve minimum 85% code coverage with actual execution
- **Test Execution**: Use `run_terminal_cmd` to execute tests and verify results
- **Test Fixes**: Fix any failing tests before proceeding to next phase
- **Test Documentation**: Document test results and coverage metrics

⚠️ **NO ADVANCEMENT WITHOUT PASSING TESTS** - All tests must be executed and pass successfully!

**Next**: AI Review Phase
                "#.to_string()
            },
            crate::core::DevelopmentWorkflowStatus::AIReview => {
                r#"
📋 **WORKFLOW STATUS: AI REVIEW** (IN PROGRESS)

🔄 **CONTINUE AI REVIEW PHASE**:
- Select 3 different AI models for code review
- Each model must generate a detailed review report using 'add_ai_review_report' tool
- Address all critical issues found
- Document review results and improvements made
- Use 'advance_workflow_phase' tool when all 3 AI models approve

**Next**: Completed
                "#.to_string()
            },
            crate::core::DevelopmentWorkflowStatus::Completed => {
                "✅ **WORKFLOW COMPLETED**: Task has successfully passed all phases!".to_string()
            },
            crate::core::DevelopmentWorkflowStatus::Failed => {
                "❌ **WORKFLOW FAILED**: Task did not meet quality standards.".to_string()
            }
            crate::core::DevelopmentWorkflowStatus::NotStarted => {
                r#"
📋 **WORKFLOW STATUS: NOT STARTED**

🚀 **READY TO BEGIN**:
- Use 'advance_workflow_phase' tool to start the Planning phase
- First step: Generate comprehensive technical documentation

**Next**: Planning Phase
                "#.to_string()
            }
        }
    }

    async fn submit_task(
        &self,
        name: String,
        command: String,
        project_id: String,
        priority: Option<String>,
    ) -> Result<CallToolResult, String> {
        let project_id_uuid = match uuid::Uuid::parse_str(&project_id) {
            Ok(id) => id,
            Err(_) => return Err("Invalid project ID format".to_string()),
        };

        let priority = match priority.as_deref() {
            Some("Low") => crate::core::TaskPriority::Low,
            Some("High") => crate::core::TaskPriority::High,
            Some("Critical") => crate::core::TaskPriority::Critical,
            _ => crate::core::TaskPriority::Normal,
        };

        let task = crate::core::Task {
            id: uuid::Uuid::new_v4(),
            name: name.clone(),
            command: command.clone(),
            description: format!("Task: {}", name),
            technical_specs: None,
            acceptance_criteria: vec![],
            project: None,
            task_type: crate::core::TaskType::Simple,
            priority,
            project_id: Some(project_id_uuid),
            dependencies: vec![],
            timeout: None,
            retry_attempts: 3,
            retry_delay: std::time::Duration::from_secs(30),
            environment: std::collections::HashMap::new(),
            working_directory: None,
            created_at: std::time::SystemTime::now(),
            updated_at: std::time::SystemTime::now(),
            status: crate::core::TaskStatus::Planning,
            result: None,
            phases: vec![crate::core::TaskPhase {
                phase: crate::core::TaskStatus::Planning,
                started_at: Some(chrono::Utc::now()),
                completed_at: None,
                documentation: None,
                artifacts: vec![],
                ai_reviews: vec![],
            }],
            current_phase: crate::core::TaskStatus::Planning,
            ai_reviews_required: 3,
            ai_reviews_completed: 0,
            development_workflow: Some(crate::core::DevelopmentWorkflow {
                technical_documentation_path: None,
                test_coverage_percentage: None,
                ai_review_reports: vec![],
                workflow_status: crate::core::DevelopmentWorkflowStatus::NotStarted,
                started_at: Some(chrono::Utc::now()),
                completed_at: None,
            }),
            metadata: std::collections::HashMap::new(),
        };

        match self.task_queue.submit_task(task.clone()).await {
            Ok(task_id) => {
                let workflow_instructions = self.generate_workflow_instructions(&task);
                let response = format!(
                    "✅ Task submitted successfully!\n\nTask ID: {}\n\n{}",
                    task_id, workflow_instructions
                );
                Ok(CallToolResult::success(vec![
                    Content::text(response),
                ]))
            },
            Err(e) => Err(format!("Failed to submit task: {}", e)),
        }
    }

    async fn get_task(&self, task_id: String) -> Result<CallToolResult, String> {
        match uuid::Uuid::parse_str(&task_id) {
            Ok(id) => match self.task_queue.get_task(id).await {
                Ok(task) => {
                    let workflow_instructions = self.generate_workflow_instructions(&task);
                    let task_info = format!(
                        "Task: {}\nStatus: {:?}\nPriority: {:?}\nType: {:?}\n\n{}",
                        task.name, task.status, task.priority, task.task_type, workflow_instructions
                    );
                    Ok(CallToolResult::success(vec![
                        Content::text(task_info),
                    ]))
                },
                Err(e) => Err(format!("Failed to get task: {}", e)),
            },
            Err(_) => Err("Invalid task ID format".to_string()),
        }
    }

    async fn list_tasks(&self, limit: Option<u32>) -> Result<CallToolResult, String> {
        let _limit = limit.unwrap_or(50) as usize;
        match self.task_queue.list_tasks(None, None).await {
            Ok(tasks) => {
                let content = if tasks.is_empty() {
                    "No tasks found".to_string()
                } else {
                    let mut result = format!("Found {} tasks:\n", tasks.len());
                    for task in tasks.iter().take(10) {
                        let workflow_status = task.development_workflow
                            .as_ref()
                            .map(|w| format!("{:?}", w.workflow_status))
                            .unwrap_or_else(|| "NotStarted".to_string());
                        result.push_str(&format!("- {} ({}): Status={:?}, Workflow={}\n",
                            task.name, task.id, task.status, workflow_status));
                    }
                    if tasks.len() > 10 {
                        result.push_str(&format!("... and {} more", tasks.len() - 10));
                    }

                    // Add workflow reminder for tasks that need attention
                    let incomplete_tasks = tasks.iter()
                        .filter(|t| {
                            t.development_workflow.as_ref()
                                .map(|w| w.workflow_status != crate::core::DevelopmentWorkflowStatus::Completed)
                                .unwrap_or(true)
                        })
                        .count();

                    if incomplete_tasks > 0 {
                        result.push_str(&format!("\n⚠️  {} tasks require workflow completion. Use 'get_task' for detailed instructions.\n",
                            incomplete_tasks));
                    }

                    result
                };
                Ok(CallToolResult::success(vec![
                    Content::text(content),
                ]))
            },
            Err(e) => Err(format!("Failed to list tasks: {}", e)),
        }
    }

                async fn cancel_task(&self, task_id: String) -> Result<bool, String> {
                    match uuid::Uuid::parse_str(&task_id) {
                        Ok(id) => match self.task_queue.cancel_task(id, "Cancelled via MCP".to_string()).await {
                            Ok(()) => Ok(true),
                            Err(e) => Err(format!("Failed to cancel task: {}", e)),
                        },
                        Err(_) => Err("Invalid task ID format".to_string()),
                    }
                }

                async fn delete_task(&self, task_id: String) -> Result<bool, String> {
                    match uuid::Uuid::parse_str(&task_id) {
                        Ok(id) => match self.task_queue.delete_task(id).await {
                            Ok(()) => Ok(true),
                            Err(e) => Err(format!("Failed to delete task: {}", e)),
                        },
                        Err(_) => Err("Invalid task ID format".to_string()),
                    }
                }

                async fn update_task(
                    &self,
                    task_id: String,
                    name: Option<String>,
                    command: Option<String>,
                    description: Option<String>,
                    priority: Option<String>,
                    status: Option<String>,
                    project_id: Option<String>,
                ) -> Result<serde_json::Value, String> {
                    let task_id_uuid = uuid::Uuid::parse_str(&task_id).map_err(|e| e.to_string())?;
                    
                    let priority_enum = if let Some(p) = priority {
                        match p.as_str() {
                            "Low" => Some(crate::core::TaskPriority::Low),
                            "Normal" => Some(crate::core::TaskPriority::Normal),
                            "High" => Some(crate::core::TaskPriority::High),
                            "Critical" => Some(crate::core::TaskPriority::Critical),
                            _ => None,
                        }
                    } else {
                        None
                    };

                    let status_enum = if let Some(s) = status {
                        match s.as_str() {
                            "Planning" => Some(crate::core::TaskStatus::Planning),
                            "Implementation" => Some(crate::core::TaskStatus::Implementation),
                            "TestCreation" => Some(crate::core::TaskStatus::TestCreation),
                            "Testing" => Some(crate::core::TaskStatus::Testing),
                            "AIReview" => Some(crate::core::TaskStatus::AIReview),
                            "Finalized" => Some(crate::core::TaskStatus::Finalized),
                            "Pending" => Some(crate::core::TaskStatus::Pending),
                            "Running" => Some(crate::core::TaskStatus::Running),
                            "Completed" => Some(crate::core::TaskStatus::Completed),
                            "Failed" => Some(crate::core::TaskStatus::Failed),
                            "Cancelled" => Some(crate::core::TaskStatus::Cancelled),
                            _ => None,
                        }
                    } else {
                        None
                    };

                    let project_id_uuid = if let Some(p) = project_id {
                        if p.is_empty() {
                            Some(None)
                        } else {
                            match uuid::Uuid::parse_str(&p) {
                                Ok(id) => Some(Some(id)),
                                Err(_) => return Err("Invalid project ID format".to_string()),
                            }
                        }
                    } else {
                        None
                    };

                    match self.task_queue.update_task(
                        task_id_uuid,
                        name,
                        command,
                        description,
                        priority_enum,
                        status_enum,
                        project_id_uuid,
                    ).await {
                        Ok(task) => Ok(json!({
                            "id": task.id,
                            "name": task.name,
                            "command": task.command,
                            "description": task.description,
                            "status": format!("{:?}", task.status),
                            "current_phase": format!("{:?}", task.current_phase),
                            "priority": format!("{:?}", task.priority),
                            "updated_at": task.updated_at,
                        })),
                        Err(e) => Err(format!("Failed to update task: {}", e)),
                    }
                }

                async fn upsert_task(
                    &self,
                    name: String,
                    command: String,
                    description: String,
                    project_id: String,
                    priority: Option<String>,
                    technical_specs: Option<String>,
                    acceptance_criteria: Option<Vec<String>>,
                ) -> Result<serde_json::Value, String> {
                    let project_id_uuid = match uuid::Uuid::parse_str(&project_id) {
                        Ok(id) => id,
                        Err(_) => return Err("Invalid project ID format".to_string()),
                    };

                    let priority_enum = match priority.as_deref() {
                        Some("Low") => crate::core::TaskPriority::Low,
                        Some("Normal") => crate::core::TaskPriority::Normal,
                        Some("High") => crate::core::TaskPriority::High,
                        Some("Critical") => crate::core::TaskPriority::Critical,
                        _ => crate::core::TaskPriority::Normal,
                    };

                    match self.task_queue.upsert_task(
                        name,
                        command,
                        description,
                        project_id_uuid,
                        priority_enum,
                        technical_specs,
                        acceptance_criteria,
                    ).await {
                        Ok(task) => Ok(json!({
                            "id": task.id,
                            "name": task.name,
                            "command": task.command,
                            "description": task.description,
                            "status": format!("{:?}", task.status),
                            "current_phase": format!("{:?}", task.current_phase),
                            "priority": format!("{:?}", task.priority),
                            "created_at": task.created_at,
                            "updated_at": task.updated_at,
                        })),
                        Err(e) => Err(format!("Failed to upsert task: {}", e)),
                    }
                }

                async fn create_project(
                    &self,
                    name: String,
                    description: Option<String>,
                ) -> Result<uuid::Uuid, String> {
                    self.task_queue
                        .create_project(name, description)
                        .await
                        .map_err(|e| e.to_string())
                }

                async fn get_project(&self, project_id: String) -> Result<serde_json::Value, String> {
                    let project_id_uuid = uuid::Uuid::parse_str(&project_id).map_err(|e| e.to_string())?;
                    let project = self
                        .task_queue
                        .get_project(&project_id_uuid)
                        .await
                        .map_err(|e| e.to_string())?;
                    match project {
                        Some(p) => Ok(json!({
                            "id": p.id,
                            "name": p.name,
                            "description": p.description,
                            "status": format!("{:?}", p.status),
                            "created_at": p.created_at.to_rfc3339(),
                            "updated_at": p.updated_at.to_rfc3339(),
                            "due_date": p.due_date.map(|d| d.to_rfc3339()),
                            "tags": p.tags,
                            "metadata": p.metadata,
                        })),
                        None => Err(format!("Project with ID {} not found", project_id)),
                    }
                }

                async fn list_projects(&self) -> Result<serde_json::Value, String> {
                    let projects = self
                        .task_queue
                        .list_projects()
                        .await
                        .map_err(|e| e.to_string())?;

                    let project_list_json: Vec<serde_json::Value> = projects
                        .iter()
                        .map(|p| {
                            json!({
                                "id": p.id,
                                "name": p.name,
                                "status": format!("{:?}", p.status),
                                "created_at": p.created_at.to_rfc3339(),
                            })
                        })
                        .collect();

                    Ok(json!(project_list_json))
                }

                async fn get_project_tasks(&self, project_id: String) -> Result<serde_json::Value, String> {
                    let project_id_uuid = uuid::Uuid::parse_str(&project_id).map_err(|e| e.to_string())?;
                    let tasks = self
                        .task_queue
                        .get_tasks_by_project(&project_id_uuid)
                        .await
                        .map_err(|e| e.to_string())?;

                    let task_list_json: Vec<serde_json::Value> = tasks
                        .iter()
                        .map(|t| {
                            json!({
                                "id": t.id,
                                "name": t.name,
                                "status": format!("{:?}", t.status),
                                "current_phase": format!("{:?}", t.current_phase),
                                "priority": format!("{:?}", t.priority),
                            })
                        })
                        .collect();

                    Ok(json!(task_list_json))
                }
}

impl ServerHandler for TaskQueueMcpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::default(),
            capabilities: ServerCapabilities::builder()
                .enable_tools()
                .build(),
            server_info: Implementation {
                name: "task-queue-mcp-server".to_string(),
                title: Some("Task Queue MCP Server".to_string()),
                version: env!("CARGO_PKG_VERSION").to_string(),
                website_url: None,
                icons: None,
            },
            instructions: Some("This server provides task queue management capabilities. You can submit tasks, get task status, list tasks, and cancel tasks.".to_string()),
        }
    }

    fn list_tools(
        &self,
        _request: Option<rmcp::model::PaginatedRequestParam>,
        _context: RequestContext<RoleServer>,
    ) -> impl std::future::Future<Output = Result<ListToolsResult, ErrorData>> + Send + '_ {
        async move {
            let tools = vec![
                Tool {
                    name: Cow::Borrowed("submit_task"),
                    title: None,
                    description: Some(Cow::Borrowed("Submit a new task to the queue")),
                    input_schema: json!({
                        "type": "object",
                        "properties": {
                            "name": {"type": "string", "description": "Task name"},
                            "command": {"type": "string", "description": "Command to execute"},
                            "project_id": {"type": "string", "description": "Project ID to associate the task with"},
                            "priority": {"type": "string", "enum": ["Low", "Normal", "High", "Critical"], "description": "Task priority"}
                        },
                        "required": ["name", "command", "project_id"]
                    }).as_object().unwrap().clone().into(),
                    annotations: None,
                    icons: None,
                    output_schema: None,
                },
                Tool {
                    name: Cow::Borrowed("get_task"),
                    title: None,
                    description: Some(Cow::Borrowed("Get task details by ID")),
                    input_schema: json!({
                        "type": "object",
                        "properties": {
                            "task_id": {"type": "string", "description": "Task ID"}
                        },
                        "required": ["task_id"]
                    }).as_object().unwrap().clone().into(),
                    annotations: None,
                    icons: None,
                    output_schema: None,
                },
                Tool {
                    name: Cow::Borrowed("list_tasks"),
                    title: None,
                    description: Some(Cow::Borrowed("List tasks in the queue")),
                    input_schema: json!({
                        "type": "object",
                        "properties": {
                            "limit": {"type": "number", "description": "Maximum number of tasks to return"}
                        }
                    }).as_object().unwrap().clone().into(),
                    annotations: None,
                    icons: None,
                    output_schema: None,
                },
                            Tool {
                                name: Cow::Borrowed("cancel_task"),
                                title: None,
                                description: Some(Cow::Borrowed("Cancel a task by ID")),
                                input_schema: json!({
                                    "type": "object",
                                    "properties": {
                                        "task_id": {"type": "string", "description": "Task ID to cancel"}
                                    },
                                    "required": ["task_id"]
                                }).as_object().unwrap().clone().into(),
                                annotations: None,
                                icons: None,
                                output_schema: None,
                            },
                            Tool {
                                name: Cow::Borrowed("delete_task"),
                                title: None,
                                description: Some(Cow::Borrowed("Delete a task by ID")),
                                input_schema: json!({
                                    "type": "object",
                                    "properties": {
                                        "task_id": {"type": "string", "description": "Task ID to delete"}
                                    },
                                    "required": ["task_id"]
                                }).as_object().unwrap().clone().into(),
                                annotations: None,
                                icons: None,
                                output_schema: None,
                            },
                            Tool {
                                name: Cow::Borrowed("update_task"),
                                title: None,
                                description: Some(Cow::Borrowed("Update an existing task")),
                                input_schema: json!({
                                    "type": "object",
                                    "properties": {
                                        "task_id": {"type": "string", "description": "Task ID to update"},
                                        "name": {"type": "string", "description": "New task name"},
                                        "command": {"type": "string", "description": "New command"},
                                        "description": {"type": "string", "description": "New description"},
                                        "priority": {"type": "string", "enum": ["Low", "Normal", "High", "Critical"], "description": "New priority"},
                                        "status": {"type": "string", "enum": ["Planning", "Implementation", "TestCreation", "Testing", "AIReview", "Finalized", "Pending", "Running", "Completed", "Failed", "Cancelled"], "description": "New status"},
                                        "project_id": {"type": "string", "description": "Project ID to associate with task (empty string to remove association)"}
                                    },
                                    "required": ["task_id"]
                                }).as_object().unwrap().clone().into(),
                                annotations: None,
                                icons: None,
                                output_schema: None,
                            },
                            Tool {
                                name: Cow::Borrowed("upsert_task"),
                                title: None,
                                description: Some(Cow::Borrowed("Create or update a task by name")),
                                input_schema: json!({
                                    "type": "object",
                                    "properties": {
                                        "name": {"type": "string", "description": "Task name (used for lookup)"},
                                        "command": {"type": "string", "description": "Command to execute"},
                                        "description": {"type": "string", "description": "Task description"},
                                        "project_id": {"type": "string", "description": "Project ID to associate the task with"},
                                        "priority": {"type": "string", "enum": ["Low", "Normal", "High", "Critical"], "description": "Task priority"},
                                        "technical_specs": {"type": "string", "description": "Technical specifications"},
                                        "acceptance_criteria": {"type": "array", "items": {"type": "string"}, "description": "Acceptance criteria"}
                                    },
                                    "required": ["name", "command", "description", "project_id"]
                                }).as_object().unwrap().clone().into(),
                                annotations: None,
                                icons: None,
                                output_schema: None,
                            },
                            Tool {
                                name: Cow::Borrowed("create_project"),
                                title: None,
                                description: Some(Cow::Borrowed("Create a new project")),
                                input_schema: json!({
                                    "type": "object",
                                    "properties": {
                                        "name": {"type": "string", "description": "Project name"},
                                        "description": {"type": "string", "description": "Project description"}
                                    },
                                    "required": ["name"]
                                }).as_object().unwrap().clone().into(),
                                annotations: None,
                                icons: None,
                                output_schema: None,
                            },
                            Tool {
                                name: Cow::Borrowed("get_project"),
                                title: None,
                                description: Some(Cow::Borrowed("Get project details by ID")),
                                input_schema: json!({
                                    "type": "object",
                                    "properties": {
                                        "project_id": {"type": "string", "description": "Project ID"}
                                    },
                                    "required": ["project_id"]
                                }).as_object().unwrap().clone().into(),
                                annotations: None,
                                icons: None,
                                output_schema: None,
                            },
                            Tool {
                                name: Cow::Borrowed("list_projects"),
                                title: None,
                                description: Some(Cow::Borrowed("List all projects")),
                                input_schema: json!({
                                    "type": "object",
                                    "properties": {}
                                }).as_object().unwrap().clone().into(),
                                annotations: None,
                                icons: None,
                                output_schema: None,
                            },
                            Tool {
                                name: Cow::Borrowed("get_project_tasks"),
                                title: None,
                                description: Some(Cow::Borrowed("Get tasks by project ID")),
                                input_schema: json!({
                                    "type": "object",
                                    "properties": {
                                        "project_id": {"type": "string", "description": "Project ID"}
                                    },
                                    "required": ["project_id"]
                                }).as_object().unwrap().clone().into(),
                                annotations: None,
                                icons: None,
                                output_schema: None,
                            },
                            Tool {
                                name: Cow::Borrowed("advance_workflow_phase"),
                                title: None,
                                description: Some(Cow::Borrowed("Advance task to next development workflow phase")),
                                input_schema: json!({
                                    "type": "object",
                                    "properties": {
                                        "task_id": {"type": "string", "description": "Task ID to advance"}
                                    },
                                    "required": ["task_id"]
                                }).as_object().unwrap().clone().into(),
                                annotations: None,
                                icons: None,
                                output_schema: None,
                            },
                            Tool {
                                name: Cow::Borrowed("set_technical_documentation"),
                                title: None,
                                description: Some(Cow::Borrowed("Set technical documentation path for planning phase")),
                                input_schema: json!({
                                    "type": "object",
                                    "properties": {
                                        "task_id": {"type": "string", "description": "Task ID"},
                                        "doc_path": {"type": "string", "description": "Path to technical documentation"}
                                    },
                                    "required": ["task_id", "doc_path"]
                                }).as_object().unwrap().clone().into(),
                                annotations: None,
                                icons: None,
                                output_schema: None,
                            },
                            Tool {
                                name: Cow::Borrowed("set_test_coverage"),
                                title: None,
                                description: Some(Cow::Borrowed("Set test coverage percentage for testing phase")),
                                input_schema: json!({
                                    "type": "object",
                                    "properties": {
                                        "task_id": {"type": "string", "description": "Task ID"},
                                        "coverage": {"type": "number", "description": "Test coverage percentage (0.0-1.0)"}
                                    },
                                    "required": ["task_id", "coverage"]
                                }).as_object().unwrap().clone().into(),
                                annotations: None,
                                icons: None,
                                output_schema: None,
                            },
                            Tool {
                                name: Cow::Borrowed("add_ai_review_report"),
                                title: None,
                                description: Some(Cow::Borrowed("Add AI review report for development phase")),
                                input_schema: json!({
                                    "type": "object",
                                    "properties": {
                                        "task_id": {"type": "string", "description": "Task ID"},
                                        "model_name": {"type": "string", "description": "AI model name"},
                                        "review_type": {"type": "string", "enum": ["CodeQuality", "Security", "Performance", "Documentation", "Testing", "Architecture"], "description": "Type of review"},
                                        "content": {"type": "string", "description": "Review content"},
                                        "score": {"type": "number", "description": "Review score (0.0-1.0)"},
                                        "approved": {"type": "boolean", "description": "Whether the code is approved"},
                                        "suggestions": {"type": "array", "items": {"type": "string"}, "description": "List of suggestions"}
                                    },
                                    "required": ["task_id", "model_name", "review_type", "content", "score", "approved"]
                                }).as_object().unwrap().clone().into(),
                                annotations: None,
                                icons: None,
                                output_schema: None,
                            },
            ];

            Ok(ListToolsResult { 
                tools,
                next_cursor: None,
            })
        }
    }

    fn call_tool(
        &self,
        request: rmcp::model::CallToolRequestParam,
        _context: RequestContext<RoleServer>,
    ) -> impl std::future::Future<Output = Result<CallToolResult, ErrorData>> + Send + '_ {
        async move {
            match request.name.as_ref() {
                "submit_task" => {
                    let args = request
                        .arguments
                        .as_ref()
                        .ok_or_else(|| ErrorData::invalid_params("Missing arguments", None))?;

                    let name = args
                        .get("name")
                        .and_then(|n| n.as_str())
                        .ok_or_else(|| ErrorData::invalid_params("Missing name parameter", None))?;

                    let command = args
                        .get("command")
                        .and_then(|c| c.as_str())
                        .ok_or_else(|| ErrorData::invalid_params("Missing command parameter", None))?;

                    let project_id = args
                        .get("project_id")
                        .and_then(|p| p.as_str())
                        .ok_or_else(|| ErrorData::invalid_params("Missing project_id parameter", None))?;

                    let priority = args.get("priority").and_then(|p| p.as_str()).map(|s| s.to_string());

                    match self.submit_task(name.to_string(), command.to_string(), project_id.to_string(), priority).await {
                        Ok(result) => {
                            let result_text = json!({
                                "task_id": result,
                                "status": "submitted",
                                "message": "Task submitted successfully"
                            }).to_string();

                            Ok(CallToolResult {
                                content: vec![Content::text(result_text)],
                                structured_content: None,
                                is_error: Some(false),
                                meta: None,
                            })
                        }
                        Err(e) => Err(ErrorData::internal_error(format!("Failed to submit task: {}", e), None))
                    }
                },
                "get_task" => {
                    let args = request
                        .arguments
                        .as_ref()
                        .ok_or_else(|| ErrorData::invalid_params("Missing arguments", None))?;

                    let task_id = args
                        .get("task_id")
                        .and_then(|t| t.as_str())
                        .ok_or_else(|| ErrorData::invalid_params("Missing task_id parameter", None))?;

                    match self.get_task(task_id.to_string()).await {
                        Ok(result) => {
                            let result_text = json!({
                                "task": result,
                                "status": "found"
                            }).to_string();

                            Ok(CallToolResult {
                                content: vec![Content::text(result_text)],
                                structured_content: None,
                                is_error: Some(false),
                                meta: None,
                            })
                        }
                        Err(e) => Err(ErrorData::internal_error(format!("Failed to get task: {}", e), None))
                    }
                },
                "list_tasks" => {
                    let args = request.arguments.as_ref();
                    let limit = args
                        .and_then(|a| a.get("limit"))
                        .and_then(|l| l.as_u64())
                        .map(|l| l as u32);

                    match self.list_tasks(limit).await {
                        Ok(result) => {
                            let result_text = json!({
                                "tasks": result,
                                "status": "success"
                            }).to_string();

                            Ok(CallToolResult {
                                content: vec![Content::text(result_text)],
                                structured_content: None,
                                is_error: Some(false),
                                meta: None,
                            })
                        }
                        Err(e) => Err(ErrorData::internal_error(format!("Failed to list tasks: {}", e), None))
                    }
                },
                "cancel_task" => {
                    let args = request
                        .arguments
                        .as_ref()
                        .ok_or_else(|| ErrorData::invalid_params("Missing arguments", None))?;

                    let task_id = args
                        .get("task_id")
                        .and_then(|t| t.as_str())
                        .ok_or_else(|| ErrorData::invalid_params("Missing task_id parameter", None))?;

                    match self.cancel_task(task_id.to_string()).await {
                        Ok(cancelled) => {
                            let result_text = json!({
                                "task_id": task_id,
                                "cancelled": cancelled,
                                "status": if cancelled { "cancelled" } else { "not_found" }
                            }).to_string();

                            Ok(CallToolResult {
                                content: vec![Content::text(result_text)],
                                structured_content: None,
                                is_error: Some(false),
                                meta: None,
                            })
                        }
                        Err(e) => Err(ErrorData::internal_error(format!("Failed to cancel task: {}", e), None))
                    }
                },
                "delete_task" => {
                    let args = request
                        .arguments
                        .as_ref()
                        .ok_or_else(|| ErrorData::invalid_params("Missing arguments", None))?;

                    let task_id = args
                        .get("task_id")
                        .and_then(|t| t.as_str())
                        .ok_or_else(|| ErrorData::invalid_params("Missing task_id parameter", None))?;

                    match self.delete_task(task_id.to_string()).await {
                        Ok(result) => {
                            let result_text = json!({
                                "deleted": result,
                                "status": "success",
                                "message": "Task deleted successfully"
                            }).to_string();

                            Ok(CallToolResult {
                                content: vec![Content::text(result_text)],
                                structured_content: None,
                                is_error: Some(false),
                                meta: None,
                            })
                        }
                        Err(e) => Err(ErrorData::internal_error(format!("Failed to delete task: {}", e), None))
                    }
                },
                "update_task" => {
                    let args = request
                        .arguments
                        .as_ref()
                        .ok_or_else(|| ErrorData::invalid_params("Missing arguments", None))?;

                    let task_id = args
                        .get("task_id")
                        .and_then(|t| t.as_str())
                        .ok_or_else(|| ErrorData::invalid_params("Missing task_id parameter", None))?;

                    let name = args.get("name").and_then(|n| n.as_str()).map(|s| s.to_string());
                    let command = args.get("command").and_then(|c| c.as_str()).map(|s| s.to_string());
                    let description = args.get("description").and_then(|d| d.as_str()).map(|s| s.to_string());
                    let priority = args.get("priority").and_then(|p| p.as_str()).map(|s| s.to_string());
                    let status = args.get("status").and_then(|s| s.as_str()).map(|s| s.to_string());
                    let project_id = args.get("project_id").and_then(|p| p.as_str()).map(|s| s.to_string());

                    match self.update_task(task_id.to_string(), name, command, description, priority, status, project_id).await {
                        Ok(result) => {
                            let result_text = json!({
                                "task": result,
                                "status": "updated"
                            }).to_string();

                            Ok(CallToolResult {
                                content: vec![Content::text(result_text)],
                                structured_content: None,
                                is_error: Some(false),
                                meta: None,
                            })
                        }
                        Err(e) => Err(ErrorData::internal_error(format!("Failed to update task: {}", e), None))
                    }
                },
                "upsert_task" => {
                    let args = request
                        .arguments
                        .as_ref()
                        .ok_or_else(|| ErrorData::invalid_params("Missing arguments", None))?;

                    let name = args
                        .get("name")
                        .and_then(|n| n.as_str())
                        .ok_or_else(|| ErrorData::invalid_params("Missing name parameter", None))?;

                    let command = args
                        .get("command")
                        .and_then(|c| c.as_str())
                        .ok_or_else(|| ErrorData::invalid_params("Missing command parameter", None))?;

                    let description = args
                        .get("description")
                        .and_then(|d| d.as_str())
                        .ok_or_else(|| ErrorData::invalid_params("Missing description parameter", None))?;

                    let project_id = args
                        .get("project_id")
                        .and_then(|p| p.as_str())
                        .ok_or_else(|| ErrorData::invalid_params("Missing project_id parameter", None))?;

                    let priority = args.get("priority").and_then(|p| p.as_str()).map(|s| s.to_string());
                    let technical_specs = args.get("technical_specs").and_then(|t| t.as_str()).map(|s| s.to_string());
                    let acceptance_criteria = args.get("acceptance_criteria").and_then(|a| a.as_array()).map(|arr| {
                        arr.iter().filter_map(|v| v.as_str()).map(|s| s.to_string()).collect()
                    });

                    match self.upsert_task(
                        name.to_string(),
                        command.to_string(),
                        description.to_string(),
                        project_id.to_string(),
                        priority,
                        technical_specs,
                        acceptance_criteria,
                    ).await {
                        Ok(result) => {
                            let result_text = json!({
                                "task": result,
                                "status": "upserted"
                            }).to_string();

                            Ok(CallToolResult {
                                content: vec![Content::text(result_text)],
                                structured_content: None,
                                is_error: Some(false),
                                meta: None,
                            })
                        }
                        Err(e) => Err(ErrorData::internal_error(format!("Failed to upsert task: {}", e), None))
                    }
                },
                            "create_project" => {
                                let args = request
                                    .arguments
                                    .as_ref()
                                    .ok_or_else(|| ErrorData::invalid_params("Missing arguments", None))?;

                                let name = args
                                    .get("name")
                                    .and_then(|n| n.as_str())
                                    .ok_or_else(|| ErrorData::invalid_params("Missing name parameter", None))?;

                                let description = args.get("description").and_then(|d| d.as_str()).map(|s| s.to_string());

                                match self.create_project(name.to_string(), description).await {
                                    Ok(result) => {
                                        let result_text = json!({
                                            "project_id": result,
                                            "status": "created",
                                            "message": "Project created successfully"
                                        }).to_string();

                                        Ok(CallToolResult {
                                            content: vec![Content::text(result_text)],
                                            structured_content: None,
                                            is_error: Some(false),
                                            meta: None,
                                        })
                                    }
                                    Err(e) => Err(ErrorData::internal_error(format!("Failed to create project: {}", e), None))
                                }
                            },
                            "get_project" => {
                                let args = request
                                    .arguments
                                    .as_ref()
                                    .ok_or_else(|| ErrorData::invalid_params("Missing arguments", None))?;

                                let project_id = args
                                    .get("project_id")
                                    .and_then(|p| p.as_str())
                                    .ok_or_else(|| ErrorData::invalid_params("Missing project_id parameter", None))?;

                                match self.get_project(project_id.to_string()).await {
                                    Ok(result) => {
                                        let result_text = json!({
                                            "project": result,
                                            "status": "found"
                                        }).to_string();

                                        Ok(CallToolResult {
                                            content: vec![Content::text(result_text)],
                                            structured_content: None,
                                            is_error: Some(false),
                                            meta: None,
                                        })
                                    }
                                    Err(e) => Err(ErrorData::internal_error(format!("Failed to get project: {}", e), None))
                                }
                            },
                            "list_projects" => {
                                match self.list_projects().await {
                                    Ok(result) => {
                                        let result_text = json!({
                                            "projects": result,
                                            "status": "success"
                                        }).to_string();

                                        Ok(CallToolResult {
                                            content: vec![Content::text(result_text)],
                                            structured_content: None,
                                            is_error: Some(false),
                                            meta: None,
                                        })
                                    }
                                    Err(e) => Err(ErrorData::internal_error(format!("Failed to list projects: {}", e), None))
                                }
                            },
                            "get_project_tasks" => {
                                let args = request
                                    .arguments
                                    .as_ref()
                                    .ok_or_else(|| ErrorData::invalid_params("Missing arguments", None))?;

                                let project_id = args
                                    .get("project_id")
                                    .and_then(|p| p.as_str())
                                    .ok_or_else(|| ErrorData::invalid_params("Missing project_id parameter", None))?;

                                match self.get_project_tasks(project_id.to_string()).await {
                                    Ok(result) => {
                                        let result_text = json!({
                                            "tasks": result,
                                            "status": "success"
                                        }).to_string();

                                        Ok(CallToolResult {
                                            content: vec![Content::text(result_text)],
                                            structured_content: None,
                                            is_error: Some(false),
                                            meta: None,
                                        })
                                    }
                                    Err(e) => Err(ErrorData::internal_error(format!("Failed to get project tasks: {}", e), None))
                                }
                            },
                            "advance_workflow_phase" => {
                                let args = request
                                    .arguments
                                    .as_ref()
                                    .ok_or_else(|| ErrorData::invalid_params("Missing arguments", None))?;

                                let task_id_str = args
                                    .get("task_id")
                                    .and_then(|t| t.as_str())
                                    .ok_or_else(|| ErrorData::invalid_params("Missing task_id parameter", None))?;

                                let task_id = uuid::Uuid::parse_str(task_id_str)
                                    .map_err(|_| ErrorData::invalid_params("Invalid task ID format", None))?;

                                match self.task_queue.advance_development_workflow(task_id).await {
                                    Ok(new_status) => {
                                        let result_text = format!(
                                            "✅ Task workflow advanced successfully!\n\nNew Status: {:?}\n\n{}",
                                            new_status,
                                            self.generate_workflow_instructions_from_status(&new_status)
                                        );

                                        Ok(CallToolResult {
                                            content: vec![Content::text(result_text)],
                                            structured_content: None,
                                            is_error: Some(false),
                                            meta: None,
                                        })
                                    }
                                    Err(e) => Err(ErrorData::internal_error(format!("Failed to advance workflow: {}", e), None))
                                }
                            },
                            "set_technical_documentation" => {
                                let args = request
                                    .arguments
                                    .as_ref()
                                    .ok_or_else(|| ErrorData::invalid_params("Missing arguments", None))?;

                                let task_id_str = args
                                    .get("task_id")
                                    .and_then(|t| t.as_str())
                                    .ok_or_else(|| ErrorData::invalid_params("Missing task_id parameter", None))?;

                                let doc_path = args
                                    .get("doc_path")
                                    .and_then(|d| d.as_str())
                                    .ok_or_else(|| ErrorData::invalid_params("Missing doc_path parameter", None))?;

                                let task_id = uuid::Uuid::parse_str(task_id_str)
                                    .map_err(|_| ErrorData::invalid_params("Invalid task ID format", None))?;

                                match self.task_queue.set_technical_documentation(task_id, doc_path.to_string()).await {
                                    Ok(()) => {
                                        let result_text = format!(
                                            "✅ Technical documentation path set successfully!\n\nPath: {}\n\n📋 **Next Step**: When documentation is complete, advance to Implementation phase.",
                                            doc_path
                                        );

                                        Ok(CallToolResult {
                                            content: vec![Content::text(result_text)],
                                            structured_content: None,
                                            is_error: Some(false),
                                            meta: None,
                                        })
                                    }
                                    Err(e) => Err(ErrorData::internal_error(format!("Failed to set documentation path: {}", e), None))
                                }
                            },
                            "set_test_coverage" => {
                                let args = request
                                    .arguments
                                    .as_ref()
                                    .ok_or_else(|| ErrorData::invalid_params("Missing arguments", None))?;

                                let task_id_str = args
                                    .get("task_id")
                                    .and_then(|t| t.as_str())
                                    .ok_or_else(|| ErrorData::invalid_params("Missing task_id parameter", None))?;

                                let coverage = args
                                    .get("coverage")
                                    .and_then(|c| c.as_f64())
                                    .ok_or_else(|| ErrorData::invalid_params("Missing or invalid coverage parameter", None))?;

                                let task_id = uuid::Uuid::parse_str(task_id_str)
                                    .map_err(|_| ErrorData::invalid_params("Invalid task ID format", None))?;

                                match self.task_queue.set_test_coverage(task_id, coverage).await {
                                    Ok(()) => {
                                        let result_text = format!(
                                            "✅ Test coverage set successfully!\n\nCoverage: {:.1}%\n\n🧪 **Next Step**: When all tests pass consistently, advance to AIReview phase.",
                                            coverage * 100.0
                                        );

                                        Ok(CallToolResult {
                                            content: vec![Content::text(result_text)],
                                            structured_content: None,
                                            is_error: Some(false),
                                            meta: None,
                                        })
                                    }
                                    Err(e) => Err(ErrorData::internal_error(format!("Failed to set test coverage: {}", e), None))
                                }
                            },
                            "add_ai_review_report" => {
                                let args = request
                                    .arguments
                                    .as_ref()
                                    .ok_or_else(|| ErrorData::invalid_params("Missing arguments", None))?;

                                let task_id_str = args
                                    .get("task_id")
                                    .and_then(|t| t.as_str())
                                    .ok_or_else(|| ErrorData::invalid_params("Missing task_id parameter", None))?;

                                let model_name = args
                                    .get("model_name")
                                    .and_then(|m| m.as_str())
                                    .ok_or_else(|| ErrorData::invalid_params("Missing model_name parameter", None))?;

                                let review_type_str = args
                                    .get("review_type")
                                    .and_then(|r| r.as_str())
                                    .ok_or_else(|| ErrorData::invalid_params("Missing review_type parameter", None))?;

                                let content = args
                                    .get("content")
                                    .and_then(|c| c.as_str())
                                    .ok_or_else(|| ErrorData::invalid_params("Missing content parameter", None))?;

                                let score = args
                                    .get("score")
                                    .and_then(|s| s.as_f64())
                                    .ok_or_else(|| ErrorData::invalid_params("Missing or invalid score parameter", None))?;

                                let approved = args
                                    .get("approved")
                                    .and_then(|a| a.as_bool())
                                    .ok_or_else(|| ErrorData::invalid_params("Missing or invalid approved parameter", None))?;

                                let suggestions = args
                                    .get("suggestions")
                                    .and_then(|s| s.as_array())
                                    .map(|arr| arr.iter().filter_map(|v| v.as_str()).map(|s| s.to_string()).collect())
                                    .unwrap_or_default();

                                let review_type = match review_type_str {
                                    "CodeQuality" => crate::core::AIReviewType::CodeQuality,
                                    "Security" => crate::core::AIReviewType::Security,
                                    "Performance" => crate::core::AIReviewType::Performance,
                                    "Documentation" => crate::core::AIReviewType::Documentation,
                                    "Testing" => crate::core::AIReviewType::Testing,
                                    "Architecture" => crate::core::AIReviewType::Architecture,
                                    _ => return Err(ErrorData::invalid_params("Invalid review_type", None))
                                };

                                let task_id = uuid::Uuid::parse_str(task_id_str)
                                    .map_err(|_| ErrorData::invalid_params("Invalid task ID format", None))?;

                                let review = crate::core::AIDevelopmentReview {
                                    model_name: model_name.to_string(),
                                    review_type: review_type.clone(),
                                    content: content.to_string(),
                                    score,
                                    approved,
                                    suggestions,
                                    reviewed_at: chrono::Utc::now(),
                                };

                                match self.task_queue.add_ai_review_report(task_id, review).await {
                                    Ok(()) => {
                                        let result_text = format!(
                                            "✅ AI review report added successfully!\n\nModel: {}\nType: {:?}\nScore: {:.2}\nApproved: {}\n\n🤖 **Progress**: Review completed. When all {} required AI models approve, advance to Completed phase.",
                                            model_name, &review_type, score, approved,
                                            3 // Default required reviews
                                        );

                                        Ok(CallToolResult {
                                            content: vec![Content::text(result_text)],
                                            structured_content: None,
                                            is_error: Some(false),
                                            meta: None,
                                        })
                                    }
                                    Err(e) => Err(ErrorData::internal_error(format!("Failed to add AI review: {}", e), None))
                                }
                            },
                            _ => Err(ErrorData::invalid_params("Unknown tool", None)),
                        }
                    }
                }

    fn list_resources(
        &self,
        _request: Option<rmcp::model::PaginatedRequestParam>,
        _context: RequestContext<RoleServer>,
    ) -> impl std::future::Future<Output = Result<ListResourcesResult, ErrorData>> + Send + '_ {
        async move {
            Ok(ListResourcesResult { 
                resources: vec![],
                next_cursor: None,
            })
        }
    }
}

/// Create MCP router using rmcp crate
pub async fn create_mcp_router(task_queue: Arc<TaskQueueServer>) -> AxumRouter {
    let config = SseServerConfig {
        bind: "0.0.0.0:0".parse::<SocketAddr>().expect("Invalid bind address"), // Port 0 means don't bind, just create router
        sse_path: "/mcp/sse".into(),
        post_path: "/mcp/message".into(),
        ct: Default::default(),
        sse_keep_alive: Some(std::time::Duration::from_secs(30)),
    };

    let (sse, axum_router) = SseServer::new(config);

    // Create the MCP server and register it with the SSE server
    let _cancel = sse.with_service(move || {
        let mcp_server = TaskQueueMcpServer::new(task_queue.clone());
        mcp_server
    });

    axum_router
}