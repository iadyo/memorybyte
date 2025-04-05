use argon2::{
    self, Argon2,
    password_hash::{SaltString, rand_core::OsRng},
};
use base64::{Engine, engine::general_purpose};
use serde::Serialize;
use sqlx::MySqlPool;
use sqlx::Row;

#[derive(Serialize)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub password: String,
    pub created_at: i64,
}

impl User {
    // Rejestracja
    // Returing user data after creating new user
    pub async fn create_user(
        pool: &MySqlPool,
        email: &str,
        username: &str,
        password: &str,
    ) -> Result<Self, sqlx::Error> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let mut output = [0u8; 32];

        argon2
            .hash_password_into(password.as_bytes(), salt.as_ref().as_bytes(), &mut output)
            .expect("Failed to hash password");
        let password_hash = general_purpose::STANDARD.encode(output);

        let query = sqlx::query("INSERT INTO users (email, username, password) VALUES (?, ?, ?)")
            .bind(email)
            .bind(username)
            .bind(password_hash.clone())
            .execute(pool)
            .await?;

        Ok(Self {
            id: query.last_insert_id() as i32,
            email: email.to_string(),
            username: username.to_string(),
            password: password_hash,
            created_at: 0,
        })
    }

    // I think there is no possible way to return as references like str even I would like to use lifetime but SQLx doesn't support the lifetime
    pub async fn get_all_users(
        pool: &MySqlPool,
    ) -> Result<Vec<(i32, String)>, sqlx::Error> {
        let rows = sqlx::query("SELECT id, username, created_at FROM users")
            .fetch_all(pool)
            .await?;

        let mut users = Vec::new();
        for row in rows {
            let id: i32 = row.get("id");
            let username: String = row.get("username");

            users.push((id, username));
        }

        Ok(users)
    }
}
