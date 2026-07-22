pub mod jwt;
pub mod tls;
pub mod config;

use core::str;
use std::borrow::Cow;

use actix_web::cookie::Cookie;
use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use sqlx::PgPool;

pub fn hash_passwd(pass: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let a2 = Argon2::default();
    let h = a2.hash_password(pass.as_bytes(), &salt)?.to_string();
    Ok(h)
}

#[derive(Clone)]
pub struct AppState {
    pool: PgPool,
    jwt_secret: String,
    jwt_expire: i64,
    is_https: bool,
    is_cookie_secure: bool,
    domain: String
}

impl AppState {
    pub fn new(pool: PgPool, jwt_secret: String, jwt_expire: i64, domain: String) -> Self {
        Self {
            pool,
            jwt_secret,
            jwt_expire,
            is_https: false,
            is_cookie_secure: false,
            domain: domain.to_owned()
        }
    }

    pub fn is_https(&self) -> &bool {
        &self.is_https
    }

    pub fn is_cookie_secure(&self) -> bool {
        self.is_cookie_secure
    }

    pub fn jwt_secret(&self) -> &str {
        &self.jwt_secret
    }

    pub fn jwt_expire(&self) -> &i64 {
        &self.jwt_expire
    }

    pub fn set_https(&mut self, tls: bool) {
        self.is_https = tls;
    }

    pub fn set_cookie_secure(&mut self, mode: bool) {
        self.is_cookie_secure = mode;
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    pub fn domain(&self) -> &str {
        &self.domain
    }
}

pub fn bake_cookie(
    state: &AppState,
    name: &'static str,
    value: impl Into<Cow<'static, str>>,
) -> Cookie<'static> {
    Cookie::build(name, value)
        .http_only(true)
        .secure(state.is_cookie_secure())
        .same_site(actix_web::cookie::SameSite::Lax)
        .domain(state.domain().to_owned())
        .path("/")
        .finish()
}
