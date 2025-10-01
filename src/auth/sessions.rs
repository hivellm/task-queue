//! Session management for user authentication

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionData {
    pub user_id: Uuid,
    pub session_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub last_activity: DateTime<Utc>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}

pub struct SessionManager {
    // Session configuration
}

impl SessionManager {
    pub fn new() -> Self {
        SessionManager {}
    }

    pub fn create_session(&self, user_id: Uuid, ip_address: Option<String>, user_agent: Option<String>) -> SessionData {
        let now = Utc::now();
        SessionData {
            user_id,
            session_id: Uuid::new_v4(),
            created_at: now,
            expires_at: now + chrono::Duration::hours(24), // 24 hour session
            last_activity: now,
            ip_address,
            user_agent,
        }
    }

    pub fn is_session_valid(&self, session: &SessionData) -> bool {
        Utc::now() < session.expires_at
    }

    pub fn refresh_session(&self, session: &mut SessionData) {
        session.last_activity = Utc::now();
        session.expires_at = Utc::now() + chrono::Duration::hours(24);
    }

    pub fn invalidate_session(&self, session: &mut SessionData) {
        session.expires_at = Utc::now();
    }
}
