# Authentication and Authorization System - Technical Specification

## Overview

This document outlines the comprehensive implementation of an authentication and authorization system for the Task Queue project, providing secure access control, user management, and session handling with modern security practices.

## Architecture Decision Records (ADRs)

### ADR 1: JWT-Based Authentication
**Decision**: Implement JWT tokens as the primary authentication mechanism
**Rationale**: Stateless, scalable, and widely supported across different clients
**Alternatives Considered**: Session-based authentication, OAuth 2.0 only
**Impact**: All API endpoints will require JWT token validation

### ADR 2: Role-Based Access Control (RBAC)
**Decision**: Implement RBAC with predefined roles and permissions
**Rationale**: Flexible permission system that scales with organizational needs
**Alternatives Considered**: Attribute-based access control (ABAC), simple user/admin roles
**Impact**: Complex permission matrix but flexible authorization system

### ADR 3: Multi-Factor Authentication (MFA)
**Decision**: Implement TOTP-based MFA as optional security enhancement
**Rationale**: Industry standard for additional security layer
**Alternatives Considered**: SMS-based MFA, hardware tokens
**Impact**: Additional complexity but significantly improved security

### ADR 4: OAuth 2.0 Integration
**Decision**: Support OAuth 2.0 for third-party authentication
**Rationale**: Enables integration with external identity providers
**Alternatives Considered**: SAML, custom authentication only
**Impact**: Broader compatibility with enterprise systems

### ADR 5: API Key Management
**Decision**: Implement API key system for programmatic access
**Rationale**: Essential for SDKs and automated systems
**Alternatives Considered**: JWT-only authentication
**Impact**: Additional authentication method requiring separate management

## Technical Architecture

### Project Structure

```
src/
├── auth/
│   ├── mod.rs
│   ├── jwt.rs              # JWT token handling
│   ├── oauth.rs            # OAuth 2.0 implementation
│   ├── mfa.rs              # Multi-factor authentication
│   ├── api_keys.rs         # API key management
│   ├── sessions.rs         # Session management
│   ├── permissions.rs      # Permission system
│   ├── roles.rs            # Role management
│   └── middleware.rs       # Authentication middleware
├── models/
│   ├── user.rs             # User data structures
│   ├── session.rs          # Session data structures
│   └── permission.rs        # Permission data structures
└── storage/
    └── auth_storage.rs     # Authentication data persistence
```

### Core Components

#### 1. JWT Token Management

```rust
// src/auth/jwt.rs
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use chrono::{Duration, Utc};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,        // User ID
    pub exp: i64,          // Expiration time
    pub iat: i64,          // Issued at
    pub iss: String,       // Issuer
    pub aud: String,       // Audience
    pub roles: Vec<String>, // User roles
    pub permissions: Vec<String>, // User permissions
    pub session_id: String, // Session identifier
}

impl Claims {
    pub fn new(
        user_id: Uuid,
        roles: Vec<String>,
        permissions: Vec<String>,
        session_id: Uuid,
        expiration_hours: i64,
    ) -> Self {
        let now = Utc::now();
        Self {
            sub: user_id.to_string(),
            exp: (now + Duration::hours(expiration_hours)).timestamp(),
            iat: now.timestamp(),
            iss: "task-queue".to_string(),
            aud: "task-queue-api".to_string(),
            roles,
            permissions,
            session_id: session_id.to_string(),
        }
    }

    pub fn is_expired(&self) -> bool {
        Utc::now().timestamp() > self.exp
    }
}

pub struct JwtManager {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    validation: Validation,
}

impl JwtManager {
    pub fn new(secret: &str) -> Result<Self, jsonwebtoken::errors::Error> {
        let encoding_key = EncodingKey::from_secret(secret.as_ref());
        let decoding_key = DecodingKey::from_secret(secret.as_ref());
        
        let mut validation = Validation::new(Algorithm::HS256);
        validation.set_issuer(&["task-queue"]);
        validation.set_audience(&["task-queue-api"]);

        Ok(Self {
            encoding_key,
            decoding_key,
            validation,
        })
    }

    pub fn generate_token(&self, claims: Claims) -> Result<String, jsonwebtoken::errors::Error> {
        let header = Header::new(Algorithm::HS256);
        encode(&header, &claims, &self.encoding_key)
    }

    pub fn validate_token(&self, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        let token_data = decode::<Claims>(token, &self.decoding_key, &self.validation)?;
        Ok(token_data.claims)
    }

    pub fn refresh_token(&self, old_claims: Claims) -> Result<String, jsonwebtoken::errors::Error> {
        let new_claims = Claims::new(
            Uuid::parse_str(&old_claims.sub).unwrap(),
            old_claims.roles,
            old_claims.permissions,
            Uuid::parse_str(&old_claims.session_id).unwrap(),
            24, // 24 hours
        );
        self.generate_token(new_claims)
    }
}
```

#### 2. User Management

```rust
// src/models/user.rs
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use bcrypt::{hash, verify, DEFAULT_COST};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub is_active: bool,
    pub is_verified: bool,
    pub mfa_enabled: bool,
    pub mfa_secret: Option<String>,
    pub last_login: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserRequest {
    pub username: Option<String>,
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
    pub mfa_code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub user: UserPublic,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserPublic {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub is_active: bool,
    pub is_verified: bool,
    pub mfa_enabled: bool,
    pub last_login: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

impl User {
    pub fn new(request: CreateUserRequest) -> Result<Self, bcrypt::BcryptError> {
        let password_hash = hash(&request.password, DEFAULT_COST)?;
        let now = Utc::now();
        
        Ok(Self {
            id: Uuid::new_v4(),
            username: request.username,
            email: request.email,
            password_hash,
            first_name: request.first_name,
            last_name: request.last_name,
            is_active: true,
            is_verified: false,
            mfa_enabled: false,
            mfa_secret: None,
            last_login: None,
            created_at: now,
            updated_at: now,
        })
    }

    pub fn verify_password(&self, password: &str) -> Result<bool, bcrypt::BcryptError> {
        verify(password, &self.password_hash)
    }

    pub fn update_password(&mut self, new_password: &str) -> Result<(), bcrypt::BcryptError> {
        self.password_hash = hash(new_password, DEFAULT_COST)?;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn to_public(&self) -> UserPublic {
        UserPublic {
            id: self.id,
            username: self.username.clone(),
            email: self.email.clone(),
            first_name: self.first_name.clone(),
            last_name: self.last_name.clone(),
            is_active: self.is_active,
            is_verified: self.is_verified,
            mfa_enabled: self.mfa_enabled,
            last_login: self.last_login,
            created_at: self.created_at,
        }
    }
}
```

#### 3. Role-Based Access Control

```rust
// src/auth/roles.rs
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
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

        Self { roles }
    }

    pub fn get_role(&self, name: &str) -> Option<&Role> {
        self.roles.get(name)
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
}
```

#### 4. Multi-Factor Authentication

```rust
// src/auth/mfa.rs
use totp_rs::{TOTP, Secret};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct MfaSetup {
    pub secret: String,
    pub qr_code_url: String,
    pub backup_codes: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MfaVerification {
    pub code: String,
    pub backup_code: Option<String>,
}

pub struct MfaManager;

impl MfaManager {
    pub fn generate_secret() -> String {
        Secret::generate_secret().to_string()
    }

    pub fn create_totp(secret: &str, username: &str) -> Result<TOTP, totp_rs::TotpUrlError> {
        let secret = Secret::Raw(secret.as_bytes().to_vec());
        TOTP::new(
            totp_rs::Algorithm::SHA1,
            6,
            1,
            30,
            secret,
            Some("Task Queue".to_string()),
            username.to_string(),
        )
    }

    pub fn generate_qr_code_url(totp: &TOTP) -> String {
        totp.get_url()
    }

    pub fn verify_code(totp: &TOTP, code: &str) -> bool {
        totp.check_current(code).unwrap_or(false)
    }

    pub fn generate_backup_codes() -> Vec<String> {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        (0..10)
            .map(|_| {
                let code: u32 = rng.gen_range(100000..999999);
                format!("{:06}", code)
            })
            .collect()
    }

    pub fn verify_backup_code(backup_codes: &[String], code: &str) -> bool {
        backup_codes.contains(&code.to_string())
    }
}
```

#### 5. API Key Management

```rust
// src/auth/api_keys.rs
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use sha2::{Sha256, Digest};
use rand::Rng;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    pub id: Uuid,
    pub name: String,
    pub key_hash: String,
    pub user_id: Uuid,
    pub permissions: Vec<String>,
    pub last_used: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub created_by: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateApiKeyRequest {
    pub name: String,
    pub permissions: Vec<String>,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateApiKeyResponse {
    pub id: Uuid,
    pub name: String,
    pub key: String, // Only returned once during creation
    pub permissions: Vec<String>,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

pub struct ApiKeyManager;

impl ApiKeyManager {
    pub fn generate_key() -> String {
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
        let mut rng = rand::thread_rng();
        
        (0..32)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect()
    }

    pub fn hash_key(key: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(key.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    pub fn create_api_key(
        request: CreateApiKeyRequest,
        user_id: Uuid,
        created_by: Uuid,
    ) -> (ApiKey, String) {
        let key = Self::generate_key();
        let key_hash = Self::hash_key(&key);
        let now = Utc::now();

        let api_key = ApiKey {
            id: Uuid::new_v4(),
            name: request.name,
            key_hash,
            user_id,
            permissions: request.permissions,
            last_used: None,
            expires_at: request.expires_at,
            is_active: true,
            created_at: now,
            created_by,
        };

        (api_key, key)
    }

    pub fn verify_key(api_key: &ApiKey, provided_key: &str) -> bool {
        if !api_key.is_active {
            return false;
        }

        if let Some(expires_at) = api_key.expires_at {
            if Utc::now() > expires_at {
                return false;
            }
        }

        let provided_hash = Self::hash_key(provided_key);
        provided_hash == api_key.key_hash
    }
}
```

#### 6. Authentication Middleware

```rust
// src/auth/middleware.rs
use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use uuid::Uuid;

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

    let token = if auth_header.starts_with("Bearer ") {
        &auth_header[7..]
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    let claims = jwt_manager
        .validate_token(token)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    if claims.is_expired() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Extract user information and add to request
    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let permissions = role_manager.get_user_permissions(&claims.roles);

    // Add user context to request extensions
    request.extensions_mut().insert(AuthContext {
        user_id,
        roles: claims.roles,
        permissions,
        session_id: Uuid::parse_str(&claims.session_id).unwrap(),
    });

    Ok(next.run(request).await)
}

pub async fn api_key_middleware(
    State(api_key_manager): State<ApiKeyManager>,
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
    });

    Ok(next.run(request).await)
}

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
}

pub fn require_permission(permission: Permission) -> impl Fn(AuthContext) -> Result<(), StatusCode> {
    move |auth_context: AuthContext| {
        if role_manager.has_permission(&auth_context.permissions, &permission) {
            Ok(())
        } else {
            Err(StatusCode::FORBIDDEN)
        }
    }
}
```

#### 7. OAuth 2.0 Integration

```rust
// src/auth/oauth.rs
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct OAuthProvider {
    pub id: Uuid,
    pub name: String,
    pub client_id: String,
    pub client_secret: String,
    pub auth_url: String,
    pub token_url: String,
    pub user_info_url: String,
    pub scopes: Vec<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OAuthToken {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: Option<i64>,
    pub refresh_token: Option<String>,
    pub scope: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OAuthUserInfo {
    pub id: String,
    pub username: String,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub avatar_url: Option<String>,
}

pub struct OAuthManager {
    providers: std::collections::HashMap<String, OAuthProvider>,
}

impl OAuthManager {
    pub fn new() -> Self {
        let mut providers = std::collections::HashMap::new();
        
        // Google OAuth provider
        providers.insert("google".to_string(), OAuthProvider {
            id: Uuid::new_v4(),
            name: "Google".to_string(),
            client_id: "".to_string(), // Will be loaded from config
            client_secret: "".to_string(),
            auth_url: "https://accounts.google.com/o/oauth2/v2/auth".to_string(),
            token_url: "https://oauth2.googleapis.com/token".to_string(),
            user_info_url: "https://www.googleapis.com/oauth2/v2/userinfo".to_string(),
            scopes: vec!["openid".to_string(), "email".to_string(), "profile".to_string()],
            is_active: true,
            created_at: Utc::now(),
        });

        // GitHub OAuth provider
        providers.insert("github".to_string(), OAuthProvider {
            id: Uuid::new_v4(),
            name: "GitHub".to_string(),
            client_id: "".to_string(),
            client_secret: "".to_string(),
            auth_url: "https://github.com/login/oauth/authorize".to_string(),
            token_url: "https://github.com/login/oauth/access_token".to_string(),
            user_info_url: "https://api.github.com/user".to_string(),
            scopes: vec!["user:email".to_string()],
            is_active: true,
            created_at: Utc::now(),
        });

        Self { providers }
    }

    pub fn get_auth_url(&self, provider: &str, state: &str) -> Option<String> {
        let provider = self.providers.get(provider)?;
        
        let mut url = format!(
            "{}?client_id={}&redirect_uri={}&response_type=code&scope={}&state={}",
            provider.auth_url,
            provider.client_id,
            urlencoding::encode("http://localhost:16080/auth/callback"),
            urlencoding::encode(&provider.scopes.join(" ")),
            urlencoding::encode(state)
        );

        Some(url)
    }

    pub async fn exchange_code_for_token(
        &self,
        provider: &str,
        code: &str,
    ) -> Result<OAuthToken, Box<dyn std::error::Error>> {
        let provider = self.providers.get(provider)
            .ok_or("Provider not found")?;

        let client = reqwest::Client::new();
        let params = [
            ("client_id", provider.client_id.as_str()),
            ("client_secret", provider.client_secret.as_str()),
            ("code", code),
            ("redirect_uri", "http://localhost:16080/auth/callback"),
            ("grant_type", "authorization_code"),
        ];

        let response = client
            .post(&provider.token_url)
            .form(&params)
            .send()
            .await?;

        let token: OAuthToken = response.json().await?;
        Ok(token)
    }

    pub async fn get_user_info(
        &self,
        provider: &str,
        access_token: &str,
    ) -> Result<OAuthUserInfo, Box<dyn std::error::Error>> {
        let provider = self.providers.get(provider)
            .ok_or("Provider not found")?;

        let client = reqwest::Client::new();
        let response = client
            .get(&provider.user_info_url)
            .bearer_auth(access_token)
            .send()
            .await?;

        let user_info: OAuthUserInfo = response.json().await?;
        Ok(user_info)
    }
}
```

## API Endpoints

### Authentication Endpoints

#### POST /auth/register
Register a new user account.

**Request Body:**
```json
{
  "username": "johndoe",
  "email": "john@example.com",
  "password": "securepassword123",
  "first_name": "John",
  "last_name": "Doe"
}
```

**Response:**
```json
{
  "user": {
    "id": "123e4567-e89b-12d3-a456-426614174000",
    "username": "johndoe",
    "email": "john@example.com",
    "first_name": "John",
    "last_name": "Doe",
    "is_active": true,
    "is_verified": false,
    "mfa_enabled": false,
    "created_at": "2024-01-01T00:00:00Z"
  },
  "message": "User registered successfully. Please verify your email."
}
```

#### POST /auth/login
Authenticate user and return JWT tokens.

**Request Body:**
```json
{
  "username": "johndoe",
  "password": "securepassword123",
  "mfa_code": "123456"
}
```

**Response:**
```json
{
  "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "token_type": "Bearer",
  "expires_in": 86400,
  "user": {
    "id": "123e4567-e89b-12d3-a456-426614174000",
    "username": "johndoe",
    "email": "john@example.com",
    "first_name": "John",
    "last_name": "Doe",
    "is_active": true,
    "is_verified": true,
    "mfa_enabled": true,
    "last_login": "2024-01-01T00:00:00Z",
    "created_at": "2024-01-01T00:00:00Z"
  }
}
```

#### POST /auth/refresh
Refresh access token using refresh token.

**Request Body:**
```json
{
  "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
}
```

**Response:**
```json
{
  "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "token_type": "Bearer",
  "expires_in": 86400
}
```

#### POST /auth/logout
Invalidate user session and tokens.

**Headers:**
```
Authorization: Bearer <access_token>
```

**Response:**
```json
{
  "message": "Logged out successfully"
}
```

### MFA Endpoints

#### POST /auth/mfa/setup
Generate MFA setup information for user.

**Headers:**
```
Authorization: Bearer <access_token>
```

**Response:**
```json
{
  "secret": "JBSWY3DPEHPK3PXP",
  "qr_code_url": "otpauth://totp/Task%20Queue:johndoe?secret=JBSWY3DPEHPK3PXP&issuer=Task%20Queue",
  "backup_codes": [
    "123456",
    "234567",
    "345678"
  ]
}
```

#### POST /auth/mfa/verify
Verify MFA setup with TOTP code.

**Request Body:**
```json
{
  "code": "123456"
}
```

**Response:**
```json
{
  "message": "MFA enabled successfully"
}
```

#### POST /auth/mfa/disable
Disable MFA for user account.

**Headers:**
```
Authorization: Bearer <access_token>
```

**Request Body:**
```json
{
  "password": "securepassword123",
  "mfa_code": "123456"
}
```

**Response:**
```json
{
  "message": "MFA disabled successfully"
}
```

### OAuth Endpoints

#### GET /auth/oauth/{provider}
Initiate OAuth flow with external provider.

**Query Parameters:**
- `redirect_uri`: Optional redirect URI after authentication

**Response:** Redirect to OAuth provider

#### GET /auth/oauth/{provider}/callback
Handle OAuth callback from external provider.

**Query Parameters:**
- `code`: Authorization code from provider
- `state`: State parameter for CSRF protection

**Response:** Redirect to frontend with tokens

### API Key Management

#### POST /auth/api-keys
Create a new API key.

**Headers:**
```
Authorization: Bearer <access_token>
```

**Request Body:**
```json
{
  "name": "My API Key",
  "permissions": ["TaskCreate", "TaskRead", "TaskUpdate"],
  "expires_at": "2024-12-31T23:59:59Z"
}
```

**Response:**
```json
{
  "id": "123e4567-e89b-12d3-a456-426614174000",
  "name": "My API Key",
  "key": "ak_1234567890abcdef1234567890abcdef",
  "permissions": ["TaskCreate", "TaskRead", "TaskUpdate"],
  "expires_at": "2024-12-31T23:59:59Z",
  "created_at": "2024-01-01T00:00:00Z"
}
```

#### GET /auth/api-keys
List user's API keys.

**Headers:**
```
Authorization: Bearer <access_token>
```

**Response:**
```json
[
  {
    "id": "123e4567-e89b-12d3-a456-426614174000",
    "name": "My API Key",
    "permissions": ["TaskCreate", "TaskRead", "TaskUpdate"],
    "last_used": "2024-01-01T12:00:00Z",
    "expires_at": "2024-12-31T23:59:59Z",
    "is_active": true,
    "created_at": "2024-01-01T00:00:00Z"
  }
]
```

#### DELETE /auth/api-keys/{id}
Delete an API key.

**Headers:**
```
Authorization: Bearer <access_token>
```

**Response:**
```json
{
  "message": "API key deleted successfully"
}
```

### User Management

#### GET /auth/users/me
Get current user information.

**Headers:**
```
Authorization: Bearer <access_token>
```

**Response:**
```json
{
  "id": "123e4567-e89b-12d3-a456-426614174000",
  "username": "johndoe",
  "email": "john@example.com",
  "first_name": "John",
  "last_name": "Doe",
  "is_active": true,
  "is_verified": true,
  "mfa_enabled": true,
  "last_login": "2024-01-01T00:00:00Z",
  "created_at": "2024-01-01T00:00:00Z"
}
```

#### PUT /auth/users/me
Update current user information.

**Headers:**
```
Authorization: Bearer <access_token>
```

**Request Body:**
```json
{
  "first_name": "John",
  "last_name": "Smith",
  "email": "john.smith@example.com"
}
```

**Response:**
```json
{
  "id": "123e4567-e89b-12d3-a456-426614174000",
  "username": "johndoe",
  "email": "john.smith@example.com",
  "first_name": "John",
  "last_name": "Smith",
  "is_active": true,
  "is_verified": true,
  "mfa_enabled": true,
  "last_login": "2024-01-01T00:00:00Z",
  "created_at": "2024-01-01T00:00:00Z"
}
```

#### POST /auth/users/me/change-password
Change user password.

**Headers:**
```
Authorization: Bearer <access_token>
```

**Request Body:**
```json
{
  "current_password": "oldpassword123",
  "new_password": "newpassword123"
}
```

**Response:**
```json
{
  "message": "Password changed successfully"
}
```

## Security Considerations

### Password Security
- Passwords are hashed using bcrypt with cost factor 12
- Minimum password length of 8 characters
- Password complexity requirements (uppercase, lowercase, numbers, symbols)
- Password history to prevent reuse of recent passwords

### JWT Security
- Tokens signed with HS256 algorithm
- Short expiration times (24 hours for access tokens)
- Refresh tokens with longer expiration (30 days)
- Token blacklisting for logout functionality
- Secure token storage recommendations

### MFA Security
- TOTP-based MFA using RFC 6238 standard
- Backup codes for account recovery
- Rate limiting on MFA verification attempts
- Secure secret generation and storage

### API Key Security
- API keys are hashed before storage
- Keys are only shown once during creation
- Expiration dates for automatic key rotation
- Permission-based access control
- Usage tracking and monitoring

### Session Management
- Secure session storage
- Session timeout and renewal
- Concurrent session limits
- Session invalidation on password change

## Configuration

### Environment Variables

```bash
# JWT Configuration
JWT_SECRET=your-super-secret-jwt-key-here
JWT_ACCESS_TOKEN_EXPIRY=86400
JWT_REFRESH_TOKEN_EXPIRY=2592000

# Database Configuration
DATABASE_URL=sqlite:///task-queue.db

# OAuth Configuration
GOOGLE_CLIENT_ID=your-google-client-id
GOOGLE_CLIENT_SECRET=your-google-client-secret
GITHUB_CLIENT_ID=your-github-client-id
GITHUB_CLIENT_SECRET=your-github-client-secret

# Email Configuration (for verification)
SMTP_HOST=smtp.gmail.com
SMTP_PORT=587
SMTP_USERNAME=your-email@gmail.com
SMTP_PASSWORD=your-app-password

# Security Configuration
BCRYPT_COST=12
MFA_ISSUER=Task Queue
SESSION_TIMEOUT=3600
MAX_LOGIN_ATTEMPTS=5
LOCKOUT_DURATION=900
```

### Configuration File

```yaml
# config/auth.yml
jwt:
  secret: "${JWT_SECRET}"
  access_token_expiry: 86400
  refresh_token_expiry: 2592000
  issuer: "task-queue"
  audience: "task-queue-api"

oauth:
  providers:
    google:
      client_id: "${GOOGLE_CLIENT_ID}"
      client_secret: "${GOOGLE_CLIENT_SECRET}"
      enabled: true
    github:
      client_id: "${GITHUB_CLIENT_ID}"
      client_secret: "${GITHUB_CLIENT_SECRET}"
      enabled: true

security:
  bcrypt_cost: 12
  mfa_issuer: "Task Queue"
  session_timeout: 3600
  max_login_attempts: 5
  lockout_duration: 900
  password_min_length: 8
  password_require_uppercase: true
  password_require_lowercase: true
  password_require_numbers: true
  password_require_symbols: true

api_keys:
  default_expiry_days: 365
  max_keys_per_user: 10
  key_length: 32
```

## Testing Strategy

### Unit Tests

```rust
// tests/auth/jwt_test.rs
#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_jwt_token_generation() {
        let jwt_manager = JwtManager::new("test-secret").unwrap();
        let user_id = Uuid::new_v4();
        let session_id = Uuid::new_v4();
        
        let claims = Claims::new(
            user_id,
            vec!["user".to_string()],
            vec!["TaskRead".to_string()],
            session_id,
            24,
        );

        let token = jwt_manager.generate_token(claims.clone()).unwrap();
        let decoded_claims = jwt_manager.validate_token(&token).unwrap();

        assert_eq!(decoded_claims.sub, claims.sub);
        assert_eq!(decoded_claims.roles, claims.roles);
    }

    #[test]
    fn test_jwt_token_expiration() {
        let jwt_manager = JwtManager::new("test-secret").unwrap();
        let user_id = Uuid::new_v4();
        let session_id = Uuid::new_v4();
        
        let claims = Claims::new(
            user_id,
            vec!["user".to_string()],
            vec!["TaskRead".to_string()],
            session_id,
            -1, // Expired
        );

        let token = jwt_manager.generate_token(claims).unwrap();
        let result = jwt_manager.validate_token(&token);
        
        assert!(result.is_err());
    }
}
```

### Integration Tests

```rust
// tests/auth/integration_test.rs
#[cfg(test)]
mod integration_tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
        Router,
    };
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_user_registration_and_login() {
        let app = create_test_app().await;
        
        // Test user registration
        let register_request = Request::builder()
            .method("POST")
            .uri("/auth/register")
            .header("content-type", "application/json")
            .body(Body::from(r#"{
                "username": "testuser",
                "email": "test@example.com",
                "password": "password123",
                "first_name": "Test",
                "last_name": "User"
            }"#))
            .unwrap();

        let response = app.clone().oneshot(register_request).await.unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);

        // Test user login
        let login_request = Request::builder()
            .method("POST")
            .uri("/auth/login")
            .header("content-type", "application/json")
            .body(Body::from(r#"{
                "username": "testuser",
                "password": "password123"
            }"#))
            .unwrap();

        let response = app.oneshot(login_request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_protected_endpoint_access() {
        let app = create_test_app().await;
        
        // Test access without token
        let request = Request::builder()
            .method("GET")
            .uri("/tasks")
            .body(Body::empty())
            .unwrap();

        let response = app.clone().oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

        // Test access with valid token
        let token = get_test_token().await;
        let request = Request::builder()
            .method("GET")
            .uri("/tasks")
            .header("authorization", format!("Bearer {}", token))
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }
}
```

## Performance Considerations

### Database Optimization
- Indexed user lookups by username and email
- Efficient session storage with TTL
- Connection pooling for database operations
- Prepared statements for security and performance

### Caching Strategy
- Redis caching for user sessions
- JWT token validation caching
- Permission cache for role-based access
- API key validation caching

### Security Performance
- Bcrypt cost factor optimization
- JWT token size minimization
- Efficient permission checking
- Rate limiting with minimal overhead

## Deployment Considerations

### Production Security
- Secure JWT secret generation
- HTTPS enforcement
- Secure cookie settings
- CORS configuration
- Security headers implementation

### Monitoring and Logging
- Authentication attempt logging
- Failed login attempt monitoring
- API key usage tracking
- Security event alerting
- Performance metrics collection

### Backup and Recovery
- User data backup procedures
- Session data persistence
- API key backup strategies
- Disaster recovery planning

This technical specification provides a comprehensive foundation for implementing a robust authentication and authorization system with modern security practices, scalability considerations, and thorough testing strategies.
