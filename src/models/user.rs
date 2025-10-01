//! User data structures and operations

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

    pub fn update_last_login(&mut self) {
        self.last_login = Some(Utc::now());
        self.updated_at = Utc::now();
    }

    pub fn enable_mfa(&mut self, secret: String) {
        self.mfa_enabled = true;
        self.mfa_secret = Some(secret);
        self.updated_at = Utc::now();
    }

    pub fn disable_mfa(&mut self) {
        self.mfa_enabled = false;
        self.mfa_secret = None;
        self.updated_at = Utc::now();
    }

    pub fn verify_email(&mut self) {
        self.is_verified = true;
        self.updated_at = Utc::now();
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
        self.updated_at = Utc::now();
    }

    pub fn activate(&mut self) {
        self.is_active = true;
        self.updated_at = Utc::now();
    }
}
