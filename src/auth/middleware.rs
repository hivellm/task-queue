//! Authentication middleware for Axum

use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use uuid::Uuid;
use crate::auth::jwt::{JwtManager, Claims};
use crate::auth::roles::RoleManager;
use crate::models::permission::Permission;

#[derive(Debug, Clone)]
pub struct AuthContext {
    pub user_id: Uuid,
    pub roles: Vec<String>,
    pub permissions: Vec<Permission>,
    pub session_id: Uuid,
}

#[derive(Debug, Clone)]
pub struct ApiKeyContext {
    pub api_key: String,
    pub user_id: Option<Uuid>,
    pub permissions: Vec<Permission>,
}

pub async fn auth_middleware(
    State(jwt_manager): State<JwtManager>,
    State(role_manager): State<RoleManager>,
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = headers
        .get("authorization")
        .and_then(|header| header.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let token = JwtManager::extract_token_from_header(auth_header)
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let claims = jwt_manager
        .validate_token(token)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    if claims.is_expired() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Extract user information and add to request
    let user_id = claims.get_user_id()
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let permissions = role_manager.get_user_permissions(&claims.roles);

    // Add user context to request extensions
    request.extensions_mut().insert(AuthContext {
        user_id,
        roles: claims.roles.clone(),
        permissions,
        session_id: claims.get_session_id().unwrap(),
    });

    Ok(next.run(request).await)
}

pub async fn api_key_middleware(
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let api_key_header = headers
        .get("x-api-key")
        .and_then(|header| header.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // In a real implementation, you would look up the API key in the database
    // For now, we'll assume the lookup is done elsewhere
    request.extensions_mut().insert(ApiKeyContext {
        api_key: api_key_header.to_string(),
        user_id: None, // Would be populated from database lookup
        permissions: vec![], // Would be populated from database lookup
    });

    Ok(next.run(request).await)
}

pub fn require_permission(permission: Permission) -> impl Fn(AuthContext) -> Result<(), StatusCode> {
    move |auth_context: AuthContext| {
        if auth_context.permissions.contains(&permission) || 
           auth_context.permissions.contains(&Permission::SystemAdmin) {
            Ok(())
        } else {
            Err(StatusCode::FORBIDDEN)
        }
    }
}

pub fn require_role(role: &str) -> impl Fn(AuthContext) -> Result<(), StatusCode> {
    let role = role.to_string();
    move |auth_context: AuthContext| {
        if auth_context.roles.contains(&role) || 
           auth_context.roles.contains(&"admin".to_string()) {
            Ok(())
        } else {
            Err(StatusCode::FORBIDDEN)
        }
    }
}

pub fn require_admin() -> impl Fn(AuthContext) -> Result<(), StatusCode> {
    move |auth_context: AuthContext| {
        if auth_context.roles.contains(&"admin".to_string()) {
            Ok(())
        } else {
            Err(StatusCode::FORBIDDEN)
        }
    }
}

pub async fn optional_auth_middleware(
    State(jwt_manager): State<JwtManager>,
    State(role_manager): State<RoleManager>,
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Try to extract and validate token, but don't fail if not present
    if let Some(auth_header) = headers.get("authorization")
        .and_then(|header| header.to_str().ok()) {
        
        if let Some(token) = JwtManager::extract_token_from_header(auth_header) {
            if let Ok(claims) = jwt_manager.validate_token(token) {
                if !claims.is_expired() {
                    if let Ok(user_id) = claims.get_user_id() {
                        let permissions = role_manager.get_user_permissions(&claims.roles);
                        
                        request.extensions_mut().insert(AuthContext {
                            user_id,
                            roles: claims.roles.clone(),
                            permissions,
                            session_id: claims.get_session_id().unwrap(),
                        });
                    }
                }
            }
        }
    }

    Ok(next.run(request).await)
}

pub fn extract_auth_context(request: &Request) -> Option<AuthContext> {
    request.extensions().get::<AuthContext>().cloned()
}

pub fn extract_api_key_context(request: &Request) -> Option<ApiKeyContext> {
    request.extensions().get::<ApiKeyContext>().cloned()
}

pub fn has_permission(auth_context: &AuthContext, permission: &Permission) -> bool {
    auth_context.permissions.contains(permission) || 
    auth_context.permissions.contains(&Permission::SystemAdmin)
}

pub fn has_role(auth_context: &AuthContext, role: &str) -> bool {
    auth_context.roles.contains(&role.to_string()) || 
    auth_context.roles.contains(&"admin".to_string())
}

pub fn is_admin(auth_context: &AuthContext) -> bool {
    auth_context.roles.contains(&"admin".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
        Router,
    };
    use tower::ServiceExt;

    #[test]
    fn test_permission_checking() {
        let auth_context = AuthContext {
            user_id: Uuid::new_v4(),
            roles: vec!["user".to_string()],
            permissions: vec![Permission::TaskCreate, Permission::TaskRead],
            session_id: Uuid::new_v4(),
        };

        assert!(has_permission(&auth_context, &Permission::TaskCreate));
        assert!(has_permission(&auth_context, &Permission::TaskRead));
        assert!(!has_permission(&auth_context, &Permission::TaskDelete));
    }

    #[test]
    fn test_admin_permission_checking() {
        let auth_context = AuthContext {
            user_id: Uuid::new_v4(),
            roles: vec!["admin".to_string()],
            permissions: vec![Permission::SystemAdmin],
            session_id: Uuid::new_v4(),
        };

        // Admin should have all permissions
        assert!(has_permission(&auth_context, &Permission::TaskCreate));
        assert!(has_permission(&auth_context, &Permission::TaskDelete));
        assert!(has_permission(&auth_context, &Permission::SystemAdmin));
        assert!(has_permission(&auth_context, &Permission::UserCreate));
    }

    #[test]
    fn test_role_checking() {
        let auth_context = AuthContext {
            user_id: Uuid::new_v4(),
            roles: vec!["user".to_string()],
            permissions: vec![Permission::TaskCreate],
            session_id: Uuid::new_v4(),
        };

        assert!(has_role(&auth_context, "user"));
        assert!(!has_role(&auth_context, "admin"));
        assert!(is_admin(&auth_context) == false);
    }

    #[test]
    fn test_admin_role_checking() {
        let auth_context = AuthContext {
            user_id: Uuid::new_v4(),
            roles: vec!["admin".to_string()],
            permissions: vec![Permission::SystemAdmin],
            session_id: Uuid::new_v4(),
        };

        assert!(has_role(&auth_context, "admin"));
        assert!(has_role(&auth_context, "user")); // Admin should have all roles
        assert!(is_admin(&auth_context));
    }
}
