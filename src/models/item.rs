use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, sqlx::Type)]
#[serde(rename_all = "PascalCase")]
#[sqlx(type_name = "item_status")]
pub enum ItemStatus {
    Active,
    Inactive,
    Broken,
    Service,
    Rental
}

#[derive(Serialize)]
pub struct Item {
    pub id: Uuid,
    pub model_id: Uuid,
    pub project_id: Uuid,
    pub serial: String,
    pub status: ItemStatus
}
