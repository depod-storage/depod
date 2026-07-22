use actix_web::{HttpResponse, http::StatusCode};

use crate::api::v1::res::{ApiErrorE, ApiResponse};

pub mod auth_user;
pub mod perms;

pub(in crate::extractors) fn api_error(status: StatusCode, err: ApiErrorE) -> actix_web::Error {
    let response = HttpResponse::build(status).json(ApiResponse::error(err));
    actix_web::error::InternalError::from_response("", response).into()
}
