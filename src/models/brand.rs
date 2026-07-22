use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct Brand {
    pub id: Uuid,
    pub name: String
}
