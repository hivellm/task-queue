//! Session data structures and operations

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    pub device_info: Option<String>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub is_active: bool,
    pub last_activity: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionData {
    pub session_id: Uuid,
    pub user_id: Uuid,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
    pub expires_at: DateTime<Utc>,
}

impl Session {
    pub fn new(
        user_id: Uuid,
        device_info: Option<String>,
        ip_address: Option<String>,
        user_agent: Option<String>,
        session_duration_hours: i64,
    ) -> Self {
        let now = Utc::now();
        let expires_at = now + chrono::Duration::hours(session_duration_hours);

        Self {
            id: Uuid::new_v4(),
            user_id,
            device_info,
            ip_address,
            user_agent,
            is_active: true,
            last_activity: now,
            expires_at,
            created_at: now,
        }
    }

    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }

    pub fn update_activity(&mut self) {
        self.last_activity = Utc::now();
    }

    pub fn extend_session(&mut self, additional_hours: i64) {
        self.expires_at = self.expires_at + chrono::Duration::hours(additional_hours);
    }

    pub fn invalidate(&mut self) {
        self.is_active = false;
    }

    pub fn is_valid(&self) -> bool {
        self.is_active && !self.is_expired()
    }
}
