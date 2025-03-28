use sqlx::MySqlPool;

use crate::models::user::User;

pub async fn create_tables(pool: &MySqlPool) -> Result<(), sqlx::Error> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTO_INCREMENT,
            username TEXT NOT NULL,
            password TEXT NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn create_user(pool: &MySqlPool, username: &str, password: &str) -> Result<User, sqlx::Error> {
    //TODO: Hash the password before storing it

    sqlx::query("INSERT INTO users (username, password, created_at) VALUES (?, ?, ?)")
        .bind(username)
        .bind(password)
        .bind(chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string())
        .execute(pool)
        .await?;

    Ok(User {
        username: username.to_string(),
        password: password.to_string(),
        created_at: chrono::Utc::now().timestamp_millis(),
    })
    
}
