use actix_web::{HttpResponse, Responder, get, post, web};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    api::v1::res::{ApiErrorE, ApiResponse},
    backend::AppState,
    models::{
        auth::{CanManageItems, CanReadItems},
        item::ItemStatus,
    },
    services::item_service::{self, ItemServiceError},
};

#[derive(Deserialize)]
pub struct AddItemRequest {
    model_id: Uuid,
    project_id: Uuid,
    serial: String,
    status: ItemStatus,
}

#[post("/items")]
pub async fn create_new_item(
    _: CanManageItems,
    state: web::Data<AppState>,
    body: web::Json<AddItemRequest>,
) -> impl Responder {
    match item_service::create_item(
        state.pool(),
        &body.model_id,
        &body.project_id,
        &body.serial,
        &body.status,
    )
    .await
    {
        Ok(t) => HttpResponse::Ok().json(ApiResponse::ok(t)),
        Err(ItemServiceError::ItemDuplicate) => {
            HttpResponse::BadRequest().json(ApiResponse::error(ApiErrorE::ItemDuplicate))
        }
        Err(_e) => HttpResponse::InternalServerError()
            .json(ApiResponse::error(ApiErrorE::Internal)),
    }
}

#[get("/items")]
pub async fn get_items(_: CanReadItems, state: web::Data<AppState>) -> impl Responder {
    match item_service::get_item_list(state.pool()).await {
        Ok(t) => HttpResponse::Ok().json(ApiResponse::ok(t)),
        Err(_e) => HttpResponse::InternalServerError()
            .json(ApiResponse::error(ApiErrorE::Internal)),
    }
}
