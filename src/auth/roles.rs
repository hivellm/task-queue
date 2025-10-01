//! Role-based access control (RBAC) implementation

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;
use crate::models::permission::{Permission, Role, UserRole};

#[derive(Clone)]
pub struct RoleManager {
    roles: HashMap<String, Role>,
}

impl RoleManager {
    pub fn new() -> Self {
        let mut roles = HashMap::new();
        
        // System roles
        roles.insert("admin".to_string(), Role {
            id: Uuid::new_v4(),
            name: "admin".to_string(),
            description: "System administrator with full access".to_string(),
            permissions: vec![
                Permission::SystemAdmin,
                Permission::UserCreate,
                Permission::UserRead,
                Permission::UserUpdate,
                Permission::UserDelete,
                Permission::UserManageRoles,
                Permission::TaskCreate,
                Permission::TaskRead,
                Permission::TaskUpdate,
                Permission::TaskDelete,
                Permission::TaskCancel,
                Permission::TaskExecute,
                Permission::ProjectCreate,
                Permission::ProjectRead,
                Permission::ProjectUpdate,
                Permission::ProjectDelete,
                Permission::WorkflowCreate,
                Permission::WorkflowRead,
                Permission::WorkflowUpdate,
                Permission::WorkflowDelete,
                Permission::WorkflowExecute,
                Permission::ApiKeyCreate,
                Permission::ApiKeyRead,
                Permission::ApiKeyUpdate,
                Permission::ApiKeyDelete,
            ],
            is_system_role: true,
            created_at: chrono::Utc::now(),
        });

        roles.insert("user".to_string(), Role {
            id: Uuid::new_v4(),
            name: "user".to_string(),
            description: "Standard user with basic access".to_string(),
            permissions: vec![
                Permission::TaskCreate,
                Permission::TaskRead,
                Permission::TaskUpdate,
                Permission::TaskCancel,
                Permission::ProjectCreate,
                Permission::ProjectRead,
                Permission::ProjectUpdate,
                Permission::WorkflowCreate,
                Permission::WorkflowRead,
                Permission::WorkflowUpdate,
            ],
            is_system_role: true,
            created_at: chrono::Utc::now(),
        });

        roles.insert("viewer".to_string(), Role {
            id: Uuid::new_v4(),
            name: "viewer".to_string(),
            description: "Read-only access to tasks and projects".to_string(),
            permissions: vec![
                Permission::TaskRead,
                Permission::ProjectRead,
                Permission::WorkflowRead,
            ],
            is_system_role: true,
            created_at: chrono::Utc::now(),
        });

        roles.insert("developer".to_string(), Role {
            id: Uuid::new_v4(),
            name: "developer".to_string(),
            description: "Developer with task execution permissions".to_string(),
            permissions: vec![
                Permission::TaskCreate,
                Permission::TaskRead,
                Permission::TaskUpdate,
                Permission::TaskExecute,
                Permission::ProjectCreate,
                Permission::ProjectRead,
                Permission::ProjectUpdate,
                Permission::WorkflowCreate,
                Permission::WorkflowRead,
                Permission::WorkflowUpdate,
                Permission::WorkflowExecute,
            ],
            is_system_role: true,
            created_at: chrono::Utc::now(),
        });

        Self { roles }
    }

    pub fn get_role(&self, name: &str) -> Option<&Role> {
        self.roles.get(name)
    }

    pub fn get_all_roles(&self) -> Vec<&Role> {
        self.roles.values().collect()
    }

    pub fn get_user_permissions(&self, user_roles: &[String]) -> Vec<Permission> {
        let mut permissions = Vec::new();
        
        for role_name in user_roles {
            if let Some(role) = self.roles.get(role_name) {
                permissions.extend(role.permissions.clone());
            }
        }
        
        permissions.sort();
        permissions.dedup();
        permissions
    }

    pub fn has_permission(&self, user_permissions: &[Permission], required_permission: &Permission) -> bool {
        user_permissions.contains(required_permission) || 
        user_permissions.contains(&Permission::SystemAdmin)
    }

    pub fn create_custom_role(
        &mut self,
        name: String,
        description: String,
        permissions: Vec<Permission>,
    ) -> Result<Role, String> {
        if self.roles.contains_key(&name) {
            return Err(format!("Role '{}' already exists", name));
        }

        let role = Role::new(name.clone(), description, permissions);
        self.roles.insert(name, role.clone());
        Ok(role)
    }

    pub fn update_role_permissions(
        &mut self,
        role_name: &str,
        permissions: Vec<Permission>,
    ) -> Result<(), String> {
        if let Some(role) = self.roles.get_mut(role_name) {
            if role.is_system_role {
                return Err(format!("Cannot modify system role '{}'", role_name));
            }
            role.permissions = permissions;
            Ok(())
        } else {
            Err(format!("Role '{}' not found", role_name))
        }
    }

    pub fn delete_role(&mut self, role_name: &str) -> Result<(), String> {
        if let Some(role) = self.roles.get(role_name) {
            if role.is_system_role {
                return Err(format!("Cannot delete system role '{}'", role_name));
            }
            self.roles.remove(role_name);
            Ok(())
        } else {
            Err(format!("Role '{}' not found", role_name))
        }
    }

    pub fn assign_role_to_user(
        &self,
        user_id: Uuid,
        role_name: &str,
        assigned_by: Uuid,
    ) -> Result<UserRole, String> {
        if !self.roles.contains_key(role_name) {
            return Err(format!("Role '{}' not found", role_name));
        }

        Ok(UserRole {
            user_id,
            role_id: self.roles[role_name].id,
            assigned_at: chrono::Utc::now(),
            assigned_by,
        })
    }

    pub fn validate_user_access(
        &self,
        user_roles: &[String],
        required_permission: &Permission,
    ) -> bool {
        let user_permissions = self.get_user_permissions(user_roles);
        self.has_permission(&user_permissions, required_permission)
    }

    pub fn get_role_hierarchy(&self) -> HashMap<String, Vec<String>> {
        let mut hierarchy = HashMap::new();
        
        // Define role hierarchy (higher roles inherit permissions from lower roles)
        hierarchy.insert("admin".to_string(), vec![
            "developer".to_string(),
            "user".to_string(),
            "viewer".to_string(),
        ]);
        
        hierarchy.insert("developer".to_string(), vec![
            "user".to_string(),
            "viewer".to_string(),
        ]);
        
        hierarchy.insert("user".to_string(), vec![
            "viewer".to_string(),
        ]);
        
        hierarchy
    }

    pub fn get_effective_permissions(&self, user_roles: &[String]) -> Vec<Permission> {
        let mut permissions = Vec::new();
        let hierarchy = self.get_role_hierarchy();
        
        for role_name in user_roles {
            // Add direct permissions
            if let Some(role) = self.roles.get(role_name) {
                permissions.extend(role.permissions.clone());
            }
            
            // Add inherited permissions
            if let Some(inherited_roles) = hierarchy.get(role_name) {
                for inherited_role in inherited_roles {
                    if let Some(role) = self.roles.get(inherited_role) {
                        permissions.extend(role.permissions.clone());
                    }
                }
            }
        }
        
        permissions.sort();
        permissions.dedup();
        permissions
    }
}

impl Default for RoleManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_role_creation_and_permissions() {
        let role_manager = RoleManager::new();
        
        // Test admin role
        let admin_role = role_manager.get_role("admin").unwrap();
        assert!(admin_role.has_permission(&Permission::SystemAdmin));
        assert!(admin_role.has_permission(&Permission::TaskCreate));
        assert!(admin_role.has_permission(&Permission::UserCreate));
        
        // Test user role
        let user_role = role_manager.get_role("user").unwrap();
        assert!(user_role.has_permission(&Permission::TaskCreate));
        assert!(!user_role.has_permission(&Permission::SystemAdmin));
        assert!(!user_role.has_permission(&Permission::UserCreate));
        
        // Test viewer role
        let viewer_role = role_manager.get_role("viewer").unwrap();
        assert!(viewer_role.has_permission(&Permission::TaskRead));
        assert!(!viewer_role.has_permission(&Permission::TaskCreate));
    }

    #[test]
    fn test_user_permission_aggregation() {
        let role_manager = RoleManager::new();
        
        let user_roles = vec!["user".to_string(), "viewer".to_string()];
        let permissions = role_manager.get_user_permissions(&user_roles);
        
        assert!(permissions.contains(&Permission::TaskCreate));
        assert!(permissions.contains(&Permission::TaskRead));
        assert!(permissions.contains(&Permission::ProjectRead));
        assert!(!permissions.contains(&Permission::SystemAdmin));
    }

    #[test]
    fn test_permission_validation() {
        let role_manager = RoleManager::new();
        
        let user_roles = vec!["user".to_string()];
        assert!(role_manager.validate_user_access(&user_roles, &Permission::TaskCreate));
        assert!(!role_manager.validate_user_access(&user_roles, &Permission::SystemAdmin));
        
        let admin_roles = vec!["admin".to_string()];
        assert!(role_manager.validate_user_access(&admin_roles, &Permission::TaskCreate));
        assert!(role_manager.validate_user_access(&admin_roles, &Permission::SystemAdmin));
    }

    #[test]
    fn test_custom_role_creation() {
        let mut role_manager = RoleManager::new();
        
        let custom_role = role_manager.create_custom_role(
            "custom".to_string(),
            "Custom role".to_string(),
            vec![Permission::TaskRead, Permission::TaskCreate],
        ).unwrap();
        
        assert_eq!(custom_role.name, "custom");
        assert_eq!(custom_role.permissions.len(), 2);
        
        // Test that custom role can be retrieved
        let retrieved_role = role_manager.get_role("custom").unwrap();
        assert_eq!(retrieved_role.name, "custom");
    }

    #[test]
    fn test_role_hierarchy() {
        let role_manager = RoleManager::new();
        
        let admin_roles = vec!["admin".to_string()];
        let effective_permissions = role_manager.get_effective_permissions(&admin_roles);
        
        // Admin should have all permissions including inherited ones
        assert!(effective_permissions.contains(&Permission::SystemAdmin));
        assert!(effective_permissions.contains(&Permission::TaskCreate));
        assert!(effective_permissions.contains(&Permission::TaskRead));
        assert!(effective_permissions.contains(&Permission::ProjectRead));
    }
}
