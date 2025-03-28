use serde::Serialize;

#[derive(Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub created_at: i64,
}
