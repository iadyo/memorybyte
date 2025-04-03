use argon2::{
    self, Argon2,
    password_hash::{SaltString, rand_core::OsRng},
};
use base64::{Engine, engine::general_purpose};
use serde::Serialize;
use sqlx::MySqlPool;

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
    pub async fn create_user(
        pool: &MySqlPool,
        email: &str,
        username: &str,
        password: &str,
    ) -> Result<(), sqlx::Error> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let mut output = [0u8; 32];

        argon2
            .hash_password_into(password.as_bytes(), salt.as_ref().as_bytes(), &mut output)
            .expect("Failed to hash password");
        let password_hash = general_purpose::STANDARD.encode(output);

        sqlx::query("INSERT INTO users (email, username, password) VALUES (?, ?, ?)")
            .bind(email)
            .bind(username)
            .bind(password_hash)
            .execute(pool)
            .await?;

        Ok(())
    }
}
