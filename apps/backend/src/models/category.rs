use serde::Serialize;

#[derive(Serialize)]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub creator_id: i32,
}
