//! JWT token management and validation

use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use chrono::{Duration, Utc};
use uuid::Uuid;
use crate::models::permission::Permission;

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
        permissions: Vec<Permission>,
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
            permissions: permissions.iter().map(|p| p.as_string()).collect(),
            session_id: session_id.to_string(),
        }
    }

    pub fn is_expired(&self) -> bool {
        Utc::now().timestamp() > self.exp
    }

    pub fn get_user_id(&self) -> Result<Uuid, uuid::Error> {
        Uuid::parse_str(&self.sub)
    }

    pub fn get_session_id(&self) -> Result<Uuid, uuid::Error> {
        Uuid::parse_str(&self.session_id)
    }

    pub fn has_permission(&self, permission: &Permission) -> bool {
        let permission_str = permission.as_string();
        self.permissions.contains(&permission_str) || 
        self.permissions.contains(&Permission::SystemAdmin.as_string())
    }

    pub fn has_role(&self, role: &str) -> bool {
        self.roles.contains(&role.to_string())
    }
}

#[derive(Clone)]
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
        let user_id = old_claims.get_user_id().unwrap();
        let roles = old_claims.roles.clone();
        let permissions = old_claims.permissions.iter()
            .filter_map(|p| Permission::from_string(p))
            .collect();
        let session_id = old_claims.get_session_id().unwrap();
        
        let new_claims = Claims::new(
            user_id,
            roles,
            permissions,
            session_id,
            24, // 24 hours
        );
        self.generate_token(new_claims)
    }

    pub fn extract_token_from_header(auth_header: &str) -> Option<&str> {
        if auth_header.starts_with("Bearer ") {
            Some(&auth_header[7..])
        } else {
            None
        }
    }

    pub fn create_access_token(
        &self,
        user_id: Uuid,
        roles: Vec<String>,
        permissions: Vec<Permission>,
        session_id: Uuid,
    ) -> Result<String, jsonwebtoken::errors::Error> {
        let claims = Claims::new(user_id, roles, permissions, session_id, 24); // 24 hours
        self.generate_token(claims)
    }

    pub fn create_refresh_token(
        &self,
        user_id: Uuid,
        roles: Vec<String>,
        permissions: Vec<Permission>,
        session_id: Uuid,
    ) -> Result<String, jsonwebtoken::errors::Error> {
        let claims = Claims::new(user_id, roles, permissions, session_id, 720); // 30 days
        self.generate_token(claims)
    }
}

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
            vec![Permission::TaskRead],
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
            vec![Permission::TaskRead],
            session_id,
            -1, // Expired
        );

        let token = jwt_manager.generate_token(claims).unwrap();
        let result = jwt_manager.validate_token(&token);
        
        assert!(result.is_err());
    }

    #[test]
    fn test_permission_checking() {
        let user_id = Uuid::new_v4();
        let session_id = Uuid::new_v4();
        
        let claims = Claims::new(
            user_id,
            vec!["user".to_string()],
            vec![Permission::TaskRead, Permission::TaskCreate],
            session_id,
            24,
        );

        assert!(claims.has_permission(&Permission::TaskRead));
        assert!(claims.has_permission(&Permission::TaskCreate));
        assert!(!claims.has_permission(&Permission::TaskDelete));
    }

    #[test]
    fn test_system_admin_permission() {
        let user_id = Uuid::new_v4();
        let session_id = Uuid::new_v4();
        
        let claims = Claims::new(
            user_id,
            vec!["admin".to_string()],
            vec![Permission::SystemAdmin],
            session_id,
            24,
        );

        // System admin should have all permissions
        assert!(claims.has_permission(&Permission::TaskRead));
        assert!(claims.has_permission(&Permission::TaskCreate));
        assert!(claims.has_permission(&Permission::TaskDelete));
        assert!(claims.has_permission(&Permission::UserCreate));
        assert!(claims.has_permission(&Permission::SystemConfig));
    }
}
