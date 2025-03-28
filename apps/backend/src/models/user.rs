use serde::Serialize;

#[derive(Serialize)]
pub struct User {
    pub username: String,
    pub password: String,
    pub created_at: i64,
}
