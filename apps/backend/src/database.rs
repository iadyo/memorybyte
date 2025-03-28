use sqlx::MySqlPool;
use sqlx::Row;

use crate::models::user::User;

pub async fn create_tables(pool: &MySqlPool) -> Result<(), sqlx::Error> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTO_INCREMENT,
            username TEXT NOT NULL,
            password TEXT NOT NULL,
            created_at BIGINT
        )",
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn create_user(pool: &MySqlPool, username: &str, password: &str) -> Result<User, sqlx::Error> {
    //TODO: Hash the password before storing it

    let time_ms = chrono::Utc::now().timestamp_millis();

    let query = sqlx::query("INSERT INTO users (username, password, created_at) VALUES (?, ?, ?)")
        .bind(username)
        .bind(password)
        .bind(time_ms)
        .execute(pool)
        .await?;

    Ok(User {
        id: query.last_insert_id() as i32,
        username: username.to_string(),
        password: password.to_string(),
        created_at: chrono::Utc::now().timestamp_millis(),
    })
}

pub async fn delete_user_by_id(pool: &MySqlPool, id: i32) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM users WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn select_users(pool: &MySqlPool, filter: Filter) -> Result<Vec<User>, sqlx::Error> {
    let query = match filter {
        Filter::ALL => sqlx::query("SELECT * FROM users"),
        Filter::ID(id) => sqlx::query("SELECT * FROM users WHERE id = ?").bind(id),
        Filter::USERNAME(username) => sqlx::query("SELECT * FROM users WHERE username = ?").bind(username),
    };

    let results = query.fetch_all(pool).await?;
    
    let mut users = Vec::new();
    for row in results {
        let id: i32 = row.get("id");
        let username: String = row.get("username");
        let password: String = row.get("password");
        let created_at: i64 = row.get("created_at");

        users.push(User{ id, username, password, created_at});
    }
    

    Ok(users)
}

pub enum Filter {
    ALL,
    ID(i32),
    USERNAME(String),
}