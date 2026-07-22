use actix_web::{HttpResponse, Responder, get, post, web};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    api::v1::res::{ApiErrorE, ApiResponse},
    backend::AppState,
    models::auth::{CanManageBrands, CanReadBrands, CanReadModels},
    services::{brand_service::{self, BrandServiceError}, model_service}, };

#[derive(Deserialize)]
pub struct AddBrandRequest {
    name: String,
}

#[post("/brands")]
pub async fn create_new_brand(
    _: CanManageBrands,
    state: web::Data<AppState>,
    body: web::Json<AddBrandRequest>,
) -> impl Responder {
    match brand_service::create_brand(state.pool(), &body.name).await {
        Ok(t) => HttpResponse::Ok().json(ApiResponse::ok(t)),
        Err(BrandServiceError::BrandDuplicate) => {
            HttpResponse::BadRequest().json(ApiResponse::error(ApiErrorE::BrandDuplicate))
        }
        Err(_e) => HttpResponse::InternalServerError()
            .json(ApiResponse::error(crate::api::v1::res::ApiErrorE::Internal)),
    }
}

#[get("/brands")]
pub async fn get_brands(_: CanReadBrands, state: web::Data<AppState>) -> impl Responder {
    match brand_service::get_brand_list(state.pool()).await {
        Ok(t) => HttpResponse::Ok().json(ApiResponse::ok(t)),
        Err(_e) => HttpResponse::InternalServerError()
            .json(ApiResponse::error(crate::api::v1::res::ApiErrorE::Internal)),
    }
}

#[derive(Deserialize)]
struct GetModelsId {
    brand_id: Uuid
}

#[get("/{brand_id}/models")]
pub async fn get_models_of_brand(_: CanReadBrands, _:CanReadModels, state: web::Data<AppState>, path: web::Path<GetModelsId>) -> impl Responder {
    let brand_id = path.brand_id;
    match model_service::get_model_list_of_brand(state.pool(), brand_id).await {
        Ok(t) => HttpResponse::Ok().json(ApiResponse::ok(t)),
        Err(_e) => HttpResponse::InternalServerError()
            .json(ApiResponse::error(crate::api::v1::res::ApiErrorE::Internal)),
    }
}

#[derive(Deserialize)]
pub struct GetBrandPath {
    id: Uuid
}

#[get("/brands/{id}")]
pub async fn get_brand(_: CanReadModels, state: web::Data<AppState>, path: web::Path<GetBrandPath>) -> impl Responder {
    match brand_service::get_brand(state.pool(), path.id).await {
        Ok(t) => HttpResponse::Ok().json(ApiResponse::ok(t)),
        Err(_e) => HttpResponse::InternalServerError()
            .json(ApiResponse::error(crate::api::v1::res::ApiErrorE::Internal)),
    }
}

