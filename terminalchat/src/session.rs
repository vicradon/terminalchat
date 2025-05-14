use chrono::{DateTime, Utc};

pub struct Session {
    id: String,
    user_id: String,
    expires_at: DateTime<Utc>,
}
