//! Permission management utilities

use crate::models::permission::Permission;
use std::collections::HashSet;

pub fn has_permission(user_permissions: &[Permission], required_permission: Permission) -> bool {
    user_permissions.contains(&required_permission)
}

pub fn has_any_permission(user_permissions: &[Permission], required_permissions: &[Permission]) -> bool {
    required_permissions.iter().any(|perm| user_permissions.contains(perm))
}

pub fn has_all_permissions(user_permissions: &[Permission], required_permissions: &[Permission]) -> bool {
    required_permissions.iter().all(|perm| user_permissions.contains(perm))
}

pub fn get_permission_set(permissions: &[Permission]) -> HashSet<Permission> {
    permissions.iter().cloned().collect()
}

pub fn validate_permission_hierarchy(user_permissions: &[Permission], required_permission: Permission) -> bool {
    // Check if user has the required permission or a higher-level permission
    match required_permission {
        Permission::TaskRead => {
            user_permissions.contains(&Permission::TaskRead) ||
            user_permissions.contains(&Permission::SystemAdmin)
        }
        Permission::TaskCreate => {
            user_permissions.contains(&Permission::TaskCreate) ||
            user_permissions.contains(&Permission::SystemAdmin)
        }
        Permission::TaskUpdate => {
            user_permissions.contains(&Permission::TaskUpdate) ||
            user_permissions.contains(&Permission::SystemAdmin)
        }
        Permission::TaskDelete => {
            user_permissions.contains(&Permission::TaskDelete) ||
            user_permissions.contains(&Permission::SystemAdmin)
        }
        Permission::ProjectRead => {
            user_permissions.contains(&Permission::ProjectRead) ||
            user_permissions.contains(&Permission::SystemAdmin)
        }
        Permission::ProjectCreate => {
            user_permissions.contains(&Permission::ProjectCreate) ||
            user_permissions.contains(&Permission::SystemAdmin)
        }
        Permission::ProjectUpdate => {
            user_permissions.contains(&Permission::ProjectUpdate) ||
            user_permissions.contains(&Permission::SystemAdmin)
        }
        Permission::ProjectDelete => {
            user_permissions.contains(&Permission::ProjectDelete) ||
            user_permissions.contains(&Permission::SystemAdmin)
        }
        Permission::UserRead => {
            user_permissions.contains(&Permission::UserRead) ||
            user_permissions.contains(&Permission::SystemAdmin)
        }
        Permission::UserCreate => {
            user_permissions.contains(&Permission::UserCreate) ||
            user_permissions.contains(&Permission::SystemAdmin)
        }
        Permission::UserUpdate => {
            user_permissions.contains(&Permission::UserUpdate) ||
            user_permissions.contains(&Permission::SystemAdmin)
        }
        Permission::UserDelete => {
            user_permissions.contains(&Permission::UserDelete) ||
            user_permissions.contains(&Permission::SystemAdmin)
        }
        Permission::SystemAdmin => {
            user_permissions.contains(&Permission::SystemAdmin)
        }
        _ => user_permissions.contains(&required_permission)
    }
}
