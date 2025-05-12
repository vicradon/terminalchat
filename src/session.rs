use chrono::{DateTime, Utc};

struct Session {
    id: String,
    user_id: String,
    expires_at: DateTime<Utc>,
}
