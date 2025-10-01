//! API Key management for programmatic access

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use sha2::{Sha256, Digest};
use rand::Rng;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub key_hash: String,
    pub created_at: DateTime<Utc>,
    pub last_used: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub is_active: bool,
    pub permissions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateApiKeyRequest {
    pub name: String,
    pub expires_at: Option<DateTime<Utc>>,
    pub permissions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateApiKeyResponse {
    pub id: Uuid,
    pub key: String, // Only returned once during creation
    pub name: String,
    pub expires_at: Option<DateTime<Utc>>,
    pub permissions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyInfo {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub last_used: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub is_active: bool,
    pub permissions: Vec<String>,
}

pub struct ApiKeyManager {
    api_keys: HashMap<Uuid, ApiKey>,
}

impl ApiKeyManager {
    pub fn new() -> Self {
        Self {
            api_keys: HashMap::new(),
        }
    }

    pub fn generate_api_key() -> String {
        let mut rng = rand::thread_rng();
        let key_length = 32;
        let chars: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789".chars().collect();
        
        (0..key_length)
            .map(|_| chars[rng.gen_range(0..chars.len())])
            .collect()
    }

    pub fn hash_api_key(&self, key: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(key.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    pub fn create_api_key(&mut self, user_id: Uuid, request: CreateApiKeyRequest) -> Result<CreateApiKeyResponse, String> {
        let key = Self::generate_api_key();
        let key_hash = self.hash_api_key(&key);
        
        let api_key = ApiKey {
            id: Uuid::new_v4(),
            user_id,
            name: request.name.clone(),
            key_hash,
            created_at: Utc::now(),
            last_used: None,
            expires_at: request.expires_at,
            is_active: true,
            permissions: request.permissions.clone(),
        };

        let key_id = api_key.id;
        self.api_keys.insert(key_id, api_key);
        
        Ok(CreateApiKeyResponse {
            id: key_id,
            key,
            name: request.name,
            expires_at: request.expires_at,
            permissions: request.permissions,
        })
    }

    pub fn verify_api_key(&self, key: &str, stored_hash: &str) -> bool {
        let computed_hash = self.hash_api_key(key);
        computed_hash == stored_hash
    }

    pub fn is_api_key_valid(&self, api_key: &ApiKey) -> bool {
        if !api_key.is_active {
            return false;
        }

        if let Some(expires_at) = api_key.expires_at {
            if Utc::now() > expires_at {
                return false;
            }
        }

        true
    }

    pub fn find_api_key_by_hash(&self, key_hash: &str) -> Option<&ApiKey> {
        self.api_keys.values().find(|key| key.key_hash == key_hash)
    }

    pub fn get_user_api_keys(&self, user_id: Uuid) -> Vec<ApiKeyInfo> {
        self.api_keys
            .values()
            .filter(|key| key.user_id == user_id)
            .map(|key| ApiKeyInfo {
                id: key.id,
                name: key.name.clone(),
                created_at: key.created_at,
                last_used: key.last_used,
                expires_at: key.expires_at,
                is_active: key.is_active,
                permissions: key.permissions.clone(),
            })
            .collect()
    }

    pub fn revoke_api_key(&mut self, key_id: Uuid) -> Result<(), String> {
        if let Some(key) = self.api_keys.get_mut(&key_id) {
            key.is_active = false;
            Ok(())
        } else {
            Err("API key not found".to_string())
        }
    }

    pub fn update_api_key_last_used(&mut self, key_id: Uuid) {
        if let Some(key) = self.api_keys.get_mut(&key_id) {
            key.last_used = Some(Utc::now());
        }
    }

    pub fn cleanup_expired_keys(&mut self) -> usize {
        let now = Utc::now();
        let expired_keys: Vec<Uuid> = self.api_keys
            .iter()
            .filter(|(_, key)| {
                if let Some(expires_at) = key.expires_at {
                    now > expires_at
                } else {
                    false
                }
            })
            .map(|(id, _)| *id)
            .collect();

        for key_id in &expired_keys {
            self.api_keys.remove(key_id);
        }

        expired_keys.len()
    }
}
