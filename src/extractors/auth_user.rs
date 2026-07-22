use std::ops::Deref;

use actix_web::{FromRequest, http::StatusCode, web};

use crate::{
    api::v1::res::ApiErrorE,
    backend::{AppState, jwt::get_user_from_token},
    extractors::api_error,
    models::user::User,
};

pub struct AuthUser(pub User);

impl Deref for AuthUser {
    type Target = User;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromRequest for AuthUser {
    type Error = actix_web::Error;
    type Future = futures_util::future::LocalBoxFuture<'static, Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let state = req.app_data::<web::Data<AppState>>().cloned();
        let token = req.cookie("access_token").map(|c| c.value().to_owned());

        Box::pin(async move {
            let token =
                token.ok_or_else(|| api_error(StatusCode::UNAUTHORIZED, ApiErrorE::InvalidToken))?;
            let state = state
                .ok_or_else(|| api_error(StatusCode::INTERNAL_SERVER_ERROR, ApiErrorE::Internal))?;
            let user = get_user_from_token(&token, state.pool(), state.jwt_secret())
                .await
                .map_err(|_| api_error(StatusCode::UNAUTHORIZED, ApiErrorE::InvalidToken))?
                .ok_or_else(|| api_error(StatusCode::UNAUTHORIZED, ApiErrorE::InvalidToken))?;

            Ok(AuthUser(user))
        })
    }
}
