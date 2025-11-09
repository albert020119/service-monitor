use chrono::{DateTime, Utc};

pub struct Incident {
    pub service_name: String,
    pub started_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
    pub message: String,
}
