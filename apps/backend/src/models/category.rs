use serde::Serialize;
use sqlx::MySqlPool;

#[derive(Serialize)]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub creator_id: i32,
}

impl Category {
    
    pub async fn create_category(
        pool: &MySqlPool,
        name: &str,
        description: &str,
        creator_id: i32,
    ) -> Result<Self, sqlx::Error> {
        let time = chrono::Utc::now().timestamp_millis();
        let query = sqlx::query("INSERT INTO categories (name, description, created_at, updated_at, creator_id) VALUES (?, ?, ?, ?, ?)")
            .bind(name)
            .bind(description)
            .bind(time)
            .bind(time)
            .bind(creator_id)
            .execute(pool)
            .await
            .expect("Failed to create category");

        Ok(Self {
            id: query.last_insert_id() as i32,
            name: name.to_string(),
            description: description.to_string(),
            created_at: time,
            updated_at: time,
            creator_id,
        })
    }
}