use actix_web::{HttpResponse, Responder, get, post, web};
use serde::Deserialize;

use crate::{
    api::v1::res::{ApiErrorE, ApiResponse},
    backend::AppState,
    models::auth::{CanManageProjects, CanReadProjects},
    services::project_service::{self, ProjectServiceError},
};

#[derive(Deserialize)]
pub struct AddProjectRequest {
    name: String,
    city: String,
}

#[post("/projects")]
pub async fn create_new_project(
    _: CanManageProjects,
    state: web::Data<AppState>,
    body: web::Json<AddProjectRequest>,
) -> impl Responder {
    match project_service::create_project(state.pool(), &body.name, &body.city).await {
        Ok(t) => HttpResponse::Ok().json(ApiResponse::ok(t)),
        Err(ProjectServiceError::CityProjectDuplicate) => {
            HttpResponse::BadRequest().json(ApiResponse::error(ApiErrorE::CityProjectDuplicate))
        }
        Err(_e) => HttpResponse::InternalServerError()
            .json(ApiResponse::error(crate::api::v1::res::ApiErrorE::Internal)),
    }
}

#[get("/projects")]
pub async fn get_projects(_: CanReadProjects, state: web::Data<AppState>) -> impl Responder {
    match project_service::get_project_list(state.pool()).await {
        Ok(t) => HttpResponse::Ok().json(ApiResponse::ok(t)),
        Err(_e) => HttpResponse::InternalServerError()
            .json(ApiResponse::error(crate::api::v1::res::ApiErrorE::Internal)),
    }
}
