use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Debug, Deserialize, Serialize, sqlx::Type)]
#[serde(rename_all = "PascalCase")]
#[sqlx(type_name = "model_type")]
pub enum ModelType {
    GPS,
    TotalStation,
    HandUnit,
    Nivo,
}

#[derive(Serialize)]
pub struct Model {
    pub id: Uuid,
    pub brand_id: Uuid,
    pub name: String,
    pub ty: ModelType,
}
