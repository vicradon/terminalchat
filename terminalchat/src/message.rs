use chrono::{DateTime, Utc};

pub struct Message {
    id: String,
    user_id: String,
    room_id: String,
    created_at: DateTime<Utc>,
}