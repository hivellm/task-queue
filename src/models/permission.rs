//! Permission and role data structures

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Permission {
    // Task permissions
    TaskCreate,
    TaskRead,
    TaskUpdate,
    TaskDelete,
    TaskCancel,
    TaskExecute,
    
    // Project permissions
    ProjectCreate,
    ProjectRead,
    ProjectUpdate,
    ProjectDelete,
    
    // Workflow permissions
    WorkflowCreate,
    WorkflowRead,
    WorkflowUpdate,
    WorkflowDelete,
    WorkflowExecute,
    
    // User management permissions
    UserCreate,
    UserRead,
    UserUpdate,
    UserDelete,
    UserManageRoles,
    
    // System permissions
    SystemAdmin,
    SystemMonitor,
    SystemConfig,
    
    // API permissions
    ApiKeyCreate,
    ApiKeyRead,
    ApiKeyUpdate,
    ApiKeyDelete,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub permissions: Vec<Permission>,
    pub is_system_role: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRole {
    pub user_id: Uuid,
    pub role_id: Uuid,
    pub assigned_at: chrono::DateTime<chrono::Utc>,
    pub assigned_by: Uuid,
}

impl Permission {
    pub fn as_string(&self) -> String {
        match self {
            Permission::TaskCreate => "TaskCreate".to_string(),
            Permission::TaskRead => "TaskRead".to_string(),
            Permission::TaskUpdate => "TaskUpdate".to_string(),
            Permission::TaskDelete => "TaskDelete".to_string(),
            Permission::TaskCancel => "TaskCancel".to_string(),
            Permission::TaskExecute => "TaskExecute".to_string(),
            Permission::ProjectCreate => "ProjectCreate".to_string(),
            Permission::ProjectRead => "ProjectRead".to_string(),
            Permission::ProjectUpdate => "ProjectUpdate".to_string(),
            Permission::ProjectDelete => "ProjectDelete".to_string(),
            Permission::WorkflowCreate => "WorkflowCreate".to_string(),
            Permission::WorkflowRead => "WorkflowRead".to_string(),
            Permission::WorkflowUpdate => "WorkflowUpdate".to_string(),
            Permission::WorkflowDelete => "WorkflowDelete".to_string(),
            Permission::WorkflowExecute => "WorkflowExecute".to_string(),
            Permission::UserCreate => "UserCreate".to_string(),
            Permission::UserRead => "UserRead".to_string(),
            Permission::UserUpdate => "UserUpdate".to_string(),
            Permission::UserDelete => "UserDelete".to_string(),
            Permission::UserManageRoles => "UserManageRoles".to_string(),
            Permission::SystemAdmin => "SystemAdmin".to_string(),
            Permission::SystemMonitor => "SystemMonitor".to_string(),
            Permission::SystemConfig => "SystemConfig".to_string(),
            Permission::ApiKeyCreate => "ApiKeyCreate".to_string(),
            Permission::ApiKeyRead => "ApiKeyRead".to_string(),
            Permission::ApiKeyUpdate => "ApiKeyUpdate".to_string(),
            Permission::ApiKeyDelete => "ApiKeyDelete".to_string(),
        }
    }

    pub fn from_string(s: &str) -> Option<Self> {
        match s {
            "TaskCreate" => Some(Permission::TaskCreate),
            "TaskRead" => Some(Permission::TaskRead),
            "TaskUpdate" => Some(Permission::TaskUpdate),
            "TaskDelete" => Some(Permission::TaskDelete),
            "TaskCancel" => Some(Permission::TaskCancel),
            "TaskExecute" => Some(Permission::TaskExecute),
            "ProjectCreate" => Some(Permission::ProjectCreate),
            "ProjectRead" => Some(Permission::ProjectRead),
            "ProjectUpdate" => Some(Permission::ProjectUpdate),
            "ProjectDelete" => Some(Permission::ProjectDelete),
            "WorkflowCreate" => Some(Permission::WorkflowCreate),
            "WorkflowRead" => Some(Permission::WorkflowRead),
            "WorkflowUpdate" => Some(Permission::WorkflowUpdate),
            "WorkflowDelete" => Some(Permission::WorkflowDelete),
            "WorkflowExecute" => Some(Permission::WorkflowExecute),
            "UserCreate" => Some(Permission::UserCreate),
            "UserRead" => Some(Permission::UserRead),
            "UserUpdate" => Some(Permission::UserUpdate),
            "UserDelete" => Some(Permission::UserDelete),
            "UserManageRoles" => Some(Permission::UserManageRoles),
            "SystemAdmin" => Some(Permission::SystemAdmin),
            "SystemMonitor" => Some(Permission::SystemMonitor),
            "SystemConfig" => Some(Permission::SystemConfig),
            "ApiKeyCreate" => Some(Permission::ApiKeyCreate),
            "ApiKeyRead" => Some(Permission::ApiKeyRead),
            "ApiKeyUpdate" => Some(Permission::ApiKeyUpdate),
            "ApiKeyDelete" => Some(Permission::ApiKeyDelete),
            _ => None,
        }
    }
}

impl Role {
    pub fn new(name: String, description: String, permissions: Vec<Permission>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            description,
            permissions,
            is_system_role: false,
            created_at: chrono::Utc::now(),
        }
    }

    pub fn has_permission(&self, permission: &Permission) -> bool {
        self.permissions.contains(permission) || self.permissions.contains(&Permission::SystemAdmin)
    }
}
