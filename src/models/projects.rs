use serde::Serialize;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub city: String,
    pub created_at: DateTime<Utc>,
}
