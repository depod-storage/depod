use std::ops::Deref;
use actix_web::{FromRequest, http::StatusCode};
use crate::{api::v1::res::ApiErrorE, extractors::{api_error, auth_user::AuthUser}, models::{auth::{BrandPermissions, ItemPermissions, ModelPermissions, ProjectPermisions, UserPermissions}, user::User}};

macro_rules! require_perm_extractor {
    ($name:ident, $perm_ty:ty, $field:ident) => {
        pub struct $name<const PERMS: i32>(pub User);

        impl<const PERMS: i32> Deref for $name<PERMS> {
            type Target = User;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<const PERMS: i32> FromRequest for $name<PERMS> {
            type Error = actix_web::Error;
            type Future = futures_util::future::LocalBoxFuture<'static, Result<Self, Self::Error>>;

            fn from_request(req: &actix_web::HttpRequest, payload: &mut actix_web::dev::Payload) -> Self::Future {
                let fut = AuthUser::from_request(req, payload);

                Box::pin(async move {
                    let user = fut.await?;
                    let perms = <$perm_ty>::from_bits_truncate(user.$field);
                    if !perms.contains(<$perm_ty>::from_bits_retain(PERMS)) {
                        return Err(api_error(StatusCode::UNAUTHORIZED, ApiErrorE::Unathorized));
                    }
                    Ok(Self(user.0))
                })
            }
        }
    };
}

require_perm_extractor!(RequireUserPerm, UserPermissions, user_perms);
require_perm_extractor!(RequireProjectPerm, ProjectPermisions, project_perms);
require_perm_extractor!(RequireBrandPerm, BrandPermissions, brand_perms);
require_perm_extractor!(RequireModelPerm, ModelPermissions, model_perms);
require_perm_extractor!(RequireItemPerm, ItemPermissions, item_perms);
