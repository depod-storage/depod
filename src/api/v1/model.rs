use actix_web::{HttpResponse, Responder, get, post, web};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    api::v1::res::{ApiErrorE, ApiResponse},
    backend::AppState,
    models::{
        auth::{CanManageModels, CanReadModels},
        model::ModelType,
    },
    services::model_service::{self, ModelServiceError},
};

#[derive(Deserialize)]
pub struct AddModelRequest {
    brand_id: Uuid,
    name: String,
    ty: ModelType,
}

#[post("/models")]
pub async fn create_new_model(
    _: CanManageModels,
    state: web::Data<AppState>,
    body: web::Json<AddModelRequest>,
) -> impl Responder {
    match model_service::create_model(state.pool(), &body.brand_id, &body.name, &body.ty).await {
        Ok(t) => HttpResponse::Ok().json(ApiResponse::ok(t)),
        Err(ModelServiceError::ModelDuplicate) => {
            HttpResponse::BadRequest().json(ApiResponse::error(ApiErrorE::ModelDuplicate))
        }
        Err(_e) => HttpResponse::InternalServerError()
            .json(ApiResponse::error(crate::api::v1::res::ApiErrorE::Internal)),
    }
}

#[get("/models")]
pub async fn get_models(_: CanReadModels, state: web::Data<AppState>) -> impl Responder {
    match model_service::get_model_list(state.pool()).await {
        Ok(t) => HttpResponse::Ok().json(ApiResponse::ok(t)),
        Err(_e) => HttpResponse::InternalServerError()
            .json(ApiResponse::error(crate::api::v1::res::ApiErrorE::Internal)),
    }
}

#[derive(Deserialize)]
pub struct GetModelPath {
    id: Uuid
}

#[get("/models/{id}")]
pub async fn get_model(_: CanReadModels, state: web::Data<AppState>, path: web::Path<GetModelPath>) -> impl Responder {
    match model_service::get_model(state.pool(), path.id).await {
        Ok(t) => HttpResponse::Ok().json(ApiResponse::ok(t)),
        Err(_e) => HttpResponse::InternalServerError()
            .json(ApiResponse::error(crate::api::v1::res::ApiErrorE::Internal)),
    }
}

