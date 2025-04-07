use serde::Serialize;
use sqlx::MySqlPool;

#[derive(Serialize)]
pub struct FlipCard {
    pub id: i32,
    pub first_side: String,
    pub second_side: String,
    pub category_id: i32,
}

impl FlipCard {
    
    pub async fn create_flipcard(
        pool: &MySqlPool, 
        first_side: &str, 
        second_side: &str, 
        category_id: i32
    ) -> Result<Self, sqlx::Error> {
        let query = sqlx::query("INSERT INTO flipcards (first_side, second_side, category_id) VALUES (?, ?, ?)")
            .bind(first_side)
            .bind(second_side)
            .bind(category_id)
            .execute(pool)
            .await?;

        Ok(Self {
            id: query.last_insert_id() as i32,
            first_side: first_side.to_string(),
            second_side: second_side.to_string(),
            category_id,
        })
    }
}
