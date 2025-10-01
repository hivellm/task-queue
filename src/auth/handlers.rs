//! Authentication endpoints and handlers

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post, put, delete},
    Router,
};
use serde_json::{json, Value};
use uuid::Uuid;
use std::collections::HashMap;

use crate::auth::{JwtManager, RoleManager, AuthContext};
use crate::models::{
    user::{User, CreateUserRequest, UpdateUserRequest, LoginRequest, LoginResponse, UserPublic},
    session::Session,
    permission::Permission,
};

// Authentication state
#[derive(Clone)]
pub struct AuthState {
    pub jwt_manager: JwtManager,
    pub role_manager: RoleManager,
    // In a real implementation, these would be database connections
    pub users: std::sync::Arc<tokio::sync::RwLock<HashMap<Uuid, User>>>,
    pub sessions: std::sync::Arc<tokio::sync::RwLock<HashMap<Uuid, Session>>>,
}

impl AuthState {
    pub fn new(jwt_secret: &str) -> Result<Self, jsonwebtoken::errors::Error> {
        Ok(Self {
            jwt_manager: JwtManager::new(jwt_secret)?,
            role_manager: RoleManager::new(),
            users: std::sync::Arc::new(tokio::sync::RwLock::new(HashMap::new())),
            sessions: std::sync::Arc::new(tokio::sync::RwLock::new(HashMap::new())),
        })
    }
}

pub fn create_auth_router() -> Router<AuthState> {
    Router::new()
        // Authentication endpoints
        .route("/auth/register", post(register_user))
        .route("/auth/login", post(login_user))
        .route("/auth/refresh", post(refresh_token))
        
        // User management endpoints
        // .route("/auth/users/me", get(get_current_user))
        // .route("/auth/users/me", put(update_current_user))
        // .route("/auth/users/me/change-password", post(change_password))
        
        // MFA endpoints
        // .route("/auth/mfa/setup", post(setup_mfa))
        // .route("/auth/mfa/verify", post(verify_mfa))
        // .route("/auth/mfa/disable", post(disable_mfa))
        
        // API key management endpoints
        // .route("/auth/api-keys", post(create_api_key))
        // .route("/auth/api-keys", get(list_api_keys))
        // .route("/auth/api-keys/:id", delete(delete_api_key))
        
        // OAuth endpoints
        // .route("/auth/oauth/:provider", get(initiate_oauth))
        // .route("/auth/oauth/:provider/callback", get(oauth_callback))
}

// POST /auth/register
pub async fn register_user(
    State(state): State<AuthState>,
    Json(request): Json<CreateUserRequest>,
) -> Result<Json<Value>, StatusCode> {
    // Validate input
    if request.username.is_empty() || request.email.is_empty() || request.password.len() < 8 {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Check if user already exists
    let mut users = state.users.write().await;
    for user in users.values() {
        if user.username == request.username || user.email == request.email {
            return Err(StatusCode::CONFLICT);
        }
    }

    // Create new user
    let user = User::new(request).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let user_id = user.id;
    users.insert(user_id, user.clone());

    Ok(Json(json!({
        "user": user.to_public(),
        "message": "User registered successfully. Please verify your email."
    })))
}

// POST /auth/login
pub async fn login_user(
    State(state): State<AuthState>,
    Json(request): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    // Find user by username
    let users = state.users.read().await;
    let user = users.values()
        .find(|u| u.username == request.username)
        .ok_or(StatusCode::UNAUTHORIZED)?
        .clone();
    drop(users);

    // Verify password
    if !user.verify_password(&request.password).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)? {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Check if user is active
    if !user.is_active {
        return Err(StatusCode::FORBIDDEN);
    }

    // TODO: Implement MFA verification if enabled
    if user.mfa_enabled && request.mfa_code.is_none() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Create session
    let session = Session::new(
        user.id,
        None, // device_info
        None, // ip_address
        None, // user_agent
        24,   // 24 hours
    );

    let session_id = session.id;

    let mut sessions = state.sessions.write().await;
    sessions.insert(session_id, session);

    // Get user roles and permissions
    let user_roles = vec!["user".to_string()]; // Default role
    let permissions = state.role_manager.get_user_permissions(&user_roles);

    // Generate tokens
    let access_token = state.jwt_manager.create_access_token(
        user.id,
        user_roles.clone(),
        permissions.clone(),
        session_id,
    ).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let refresh_token = state.jwt_manager.create_refresh_token(
        user.id,
        user_roles,
        permissions,
        session_id,
    ).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Update user last login
    let mut users = state.users.write().await;
    if let Some(user) = users.get_mut(&user.id) {
        user.update_last_login();
    }

    Ok(Json(LoginResponse {
        access_token,
        refresh_token,
        token_type: "Bearer".to_string(),
        expires_in: 86400, // 24 hours
        user: user.to_public(),
    }))
}

// POST /auth/logout
pub async fn logout_user(
    State(state): State<AuthState>,
    auth_context: AuthContext,
) -> Result<Json<Value>, StatusCode> {
    // Invalidate session
    let mut sessions = state.sessions.write().await;
    if let Some(session) = sessions.get_mut(&auth_context.session_id) {
        session.invalidate();
    }

    Ok(Json(json!({
        "message": "Logged out successfully"
    })))
}

// POST /auth/refresh
pub async fn refresh_token(
    State(state): State<AuthState>,
    Json(request): Json<Value>,
) -> Result<Json<Value>, StatusCode> {
    let refresh_token = request["refresh_token"]
        .as_str()
        .ok_or(StatusCode::BAD_REQUEST)?;

    // Validate refresh token
    let claims = state.jwt_manager.validate_token(refresh_token)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    if claims.is_expired() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Check if session is still valid
    let sessions = state.sessions.read().await;
    let session = sessions.get(&claims.get_session_id().unwrap())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    if !session.is_valid() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Generate new access token
    let user_id = claims.get_user_id().unwrap();
    let session_id = claims.get_session_id().unwrap();
    let permissions = claims.permissions.iter()
        .filter_map(|p| Permission::from_string(p))
        .collect();

    let new_access_token = state.jwt_manager.create_access_token(
        user_id,
        claims.roles,
        permissions,
        session_id,
    ).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(json!({
        "access_token": new_access_token,
        "token_type": "Bearer",
        "expires_in": 86400
    })))
}

// GET /auth/users/me
pub async fn get_current_user(
    State(state): State<AuthState>,
    auth_context: AuthContext,
) -> Result<Json<UserPublic>, StatusCode> {
    let users = state.users.read().await;
    let user = users.get(&auth_context.user_id)
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(user.to_public()))
}

// PUT /auth/users/me
pub async fn update_current_user(
    State(state): State<AuthState>,
    auth_context: AuthContext,
    Json(request): Json<UpdateUserRequest>,
) -> Result<Json<UserPublic>, StatusCode> {
    let mut users = state.users.write().await;
    let user = users.get_mut(&auth_context.user_id)
        .ok_or(StatusCode::NOT_FOUND)?;

    // Update user fields
    if let Some(username) = request.username {
        user.username = username;
    }
    if let Some(email) = request.email {
        user.email = email;
    }
    if let Some(first_name) = request.first_name {
        user.first_name = Some(first_name);
    }
    if let Some(last_name) = request.last_name {
        user.last_name = Some(last_name);
    }
    if let Some(is_active) = request.is_active {
        user.is_active = is_active;
    }

    user.updated_at = chrono::Utc::now();

    Ok(Json(user.to_public()))
}

// POST /auth/users/me/change-password
pub async fn change_password(
    State(state): State<AuthState>,
    auth_context: AuthContext,
    Json(request): Json<Value>,
) -> Result<Json<Value>, StatusCode> {
    let current_password = request["current_password"]
        .as_str()
        .ok_or(StatusCode::BAD_REQUEST)?;
    let new_password = request["new_password"]
        .as_str()
        .ok_or(StatusCode::BAD_REQUEST)?;

    if new_password.len() < 8 {
        return Err(StatusCode::BAD_REQUEST);
    }

    let mut users = state.users.write().await;
    let user = users.get_mut(&auth_context.user_id)
        .ok_or(StatusCode::NOT_FOUND)?;

    // Verify current password
    if !user.verify_password(current_password).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)? {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Update password
    user.update_password(new_password).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(json!({
        "message": "Password changed successfully"
    })))
}

// Placeholder implementations for MFA and API key endpoints
pub async fn setup_mfa(
    State(_state): State<AuthState>,
    auth_context: AuthContext,
) -> Result<Json<Value>, StatusCode> {
    // TODO: Implement MFA setup
    Ok(Json(json!({
        "message": "MFA setup not yet implemented",
        "user_id": auth_context.user_id
    })))
}

pub async fn verify_mfa(
    State(_state): State<AuthState>,
    auth_context: AuthContext,
    Json(_request): Json<Value>,
) -> Result<Json<Value>, StatusCode> {
    // TODO: Implement MFA verification
    Ok(Json(json!({
        "message": "MFA verification not yet implemented",
        "user_id": auth_context.user_id
    })))
}

pub async fn disable_mfa(
    State(_state): State<AuthState>,
    auth_context: AuthContext,
    Json(_request): Json<Value>,
) -> Result<Json<Value>, StatusCode> {
    // TODO: Implement MFA disable
    Ok(Json(json!({
        "message": "MFA disable not yet implemented",
        "user_id": auth_context.user_id
    })))
}

pub async fn create_api_key(
    State(_state): State<AuthState>,
    auth_context: AuthContext,
    Json(_request): Json<Value>,
) -> Result<Json<Value>, StatusCode> {
    // TODO: Implement API key creation
    Ok(Json(json!({
        "message": "API key creation not yet implemented",
        "user_id": auth_context.user_id
    })))
}

pub async fn list_api_keys(
    State(_state): State<AuthState>,
    auth_context: AuthContext,
) -> Result<Json<Value>, StatusCode> {
    // TODO: Implement API key listing
    Ok(Json(json!({
        "message": "API key listing not yet implemented",
        "user_id": auth_context.user_id,
        "api_keys": []
    })))
}

pub async fn delete_api_key(
    State(_state): State<AuthState>,
    auth_context: AuthContext,
    Path(_id): Path<Uuid>,
) -> Result<Json<Value>, StatusCode> {
    // TODO: Implement API key deletion
    Ok(Json(json!({
        "message": "API key deletion not yet implemented",
        "user_id": auth_context.user_id
    })))
}

pub async fn initiate_oauth(
    State(_state): State<AuthState>,
    Path(_provider): Path<String>,
) -> Result<Json<Value>, StatusCode> {
    // TODO: Implement OAuth initiation
    Ok(Json(json!({
        "message": "OAuth initiation not yet implemented"
    })))
}

pub async fn oauth_callback(
    State(_state): State<AuthState>,
    Path(_provider): Path<String>,
    Query(_params): Query<HashMap<String, String>>,
) -> Result<Json<Value>, StatusCode> {
    // TODO: Implement OAuth callback
    Ok(Json(json!({
        "message": "OAuth callback not yet implemented"
    })))
}
