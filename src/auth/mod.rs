//! Authentication and Authorization module
//!
//! This module provides comprehensive authentication and authorization functionality
//! including JWT tokens, RBAC, MFA, OAuth 2.0, and API key management.

pub mod jwt;
pub mod oauth;
pub mod mfa;
pub mod api_keys;
pub mod sessions;
pub mod permissions;
pub mod roles;
pub mod middleware;
pub mod handlers;

pub use jwt::JwtManager;
pub use oauth::OAuthManager;
pub use mfa::MfaManager;
pub use api_keys::ApiKeyManager;
pub use sessions::SessionManager;
pub use permissions::*;
pub use roles::RoleManager;
pub use middleware::{auth_middleware, api_key_middleware, AuthContext, ApiKeyContext};
pub use handlers::{AuthState, create_auth_router};
