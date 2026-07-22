use actix_web::{HttpResponse, Responder, delete, get, patch, post, web};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    api::v1::res::{ApiErrorE, ApiResponse},
    backend::AppState,
    models::{
        auth::{CanManageUsers, CanReadUsers},
        user::User,
    },
    services::user_service::{self, UserServiceError},
};

#[derive(Deserialize)]
pub struct AddUserRequest {
    pass: String,
    user: String,
}

#[post("/users")]
pub async fn create_new_user(
    _: CanManageUsers,
    state: web::Data<AppState>,
    body: web::Json<AddUserRequest>,
) -> impl Responder {
    match user_service::create_user(state.pool(), &body.user, &body.pass).await {
        Ok(u) => HttpResponse::Ok().json(ApiResponse::ok(u)),
        Err(UserServiceError::DuplicateUsername) => {
            HttpResponse::Conflict().json(ApiResponse::error(ApiErrorE::DuplicateUsername))
        }
        Err(_) => {
            HttpResponse::InternalServerError().json(ApiResponse::error(ApiErrorE::Internal))
        }
    }
}

#[get("/users")]
pub async fn users(_: CanReadUsers, state: web::Data<AppState>) -> impl Responder {
    let vec = match sqlx::query_as!(
        User,
        "select id, username, user_perms, project_perms, brand_perms, model_perms, item_perms from users"
    )
    .fetch_all(state.pool())
    .await
    {
        Ok(o) => o,
        Err(e) => {
            println!("Database error: {e}");
            return HttpResponse::InternalServerError()
                .json(ApiResponse::error(ApiErrorE::Internal));
        }
    };
    HttpResponse::Ok().json(ApiResponse::ok(vec))
}

#[derive(Deserialize)]
struct PermPath {
    id: Uuid,
    perm: i32,
}

#[patch("/{id}/{perm}")]
pub async fn add_perm(
    _: CanManageUsers,
    state: web::Data<AppState>,
    path: web::Path<PermPath>,
) -> impl Responder {
    let id = path.id;
    let perm = path.perm;

    match user_service::add_u_perm_to_user(state.pool(), id, perm).await {
        Ok(k) => HttpResponse::Ok().json(ApiResponse::ok(k)),
        Err(_e) => {
            HttpResponse::InternalServerError().json(ApiResponse::error(ApiErrorE::Internal))
        }
    }
}

#[delete("/{id}/{perm}")]
pub async fn delete_perm(
    _: CanManageUsers,
    state: web::Data<AppState>,
    path: web::Path<PermPath>,
) -> impl Responder {
    let id = path.id;
    let perm = path.perm;

    match user_service::remove_u_perm_from_user(state.pool(), id, perm).await {
        Ok(k) => HttpResponse::Ok().json(ApiResponse::ok(k)),
        Err(_e) => {
            HttpResponse::InternalServerError().json(ApiResponse::error(ApiErrorE::Internal))
        }
    }
}
