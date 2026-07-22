use serde::Serialize;

#[derive(Serialize)]
pub struct User {
    pub id: uuid::Uuid,
    pub username: String,
    pub user_perms: i32,
    pub project_perms: i32,
    pub brand_perms: i32,
    pub model_perms: i32,
    pub item_perms: i32,
}

#[derive(Serialize)]
pub struct UserWithPass {
    pub id: uuid::Uuid,
    pub username: String,
    pub user_perms: i32,
    pub project_perms: i32,
    pub brand_perms: i32,
    pub model_perms: i32,
    pub item_perms: i32,
    pub password_hash: String,
}


