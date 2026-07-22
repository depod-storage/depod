use actix_web::{HttpResponse, Responder, get, post, web};
use serde::Deserialize;

use crate::{
    api::v1::res::{ApiErrorE, ApiResponse},
    backend::{AppState, bake_cookie},
    extractors::auth_user::AuthUser,
    services::{
        auth_service::{self, AuthError},
        user_service::{self, UserServiceError},
    },
};

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[post("/login")]
pub async fn login(state: web::Data<AppState>, body: web::Json<LoginRequest>) -> impl Responder {
    let token = auth_service::login_user(
        state.pool(),
        &body.username,
        &body.password,
        state.jwt_secret(),
    )
    .await;
    match token {
        Ok(t) => {
            let cookie = bake_cookie(&state, "access_token", t);
            HttpResponse::Ok().cookie(cookie).json(ApiResponse::suc())
        }
        Err(AuthError::LoginFail) => {
            HttpResponse::Unauthorized().json(ApiResponse::error(ApiErrorE::WrongLogin))
        }
        Err(_) => HttpResponse::BadRequest().json(ApiResponse::error(ApiErrorE::Internal)),
    }
}

#[get("/me")]
pub async fn me(state: web::Data<AppState>, u: AuthUser) -> impl Responder {
    match user_service::get_user(state.pool(), u.id).await {
        Ok(u) => HttpResponse::Ok().json(ApiResponse::ok(u)),
        Err(UserServiceError::NoUser) => {
            HttpResponse::Unauthorized().json(ApiResponse::error(ApiErrorE::Unathorized))
        }
        Err(_) => HttpResponse::BadRequest().json(ApiResponse::error(ApiErrorE::Internal)),
    }
}
