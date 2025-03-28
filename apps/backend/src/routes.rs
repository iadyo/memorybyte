use actix_web::{post, web::{self, Json}, HttpResponse};
use serde::Deserialize;
use sqlx::MySqlPool;

use crate::database;

#[derive(Deserialize)]
struct UserInput {
    username: String,
    password: String,
}

#[post("/api/users/create")]
pub async fn create_user(pool: web::Data<MySqlPool>, json: Json<UserInput>) -> HttpResponse {
    let user = database::create_user(&pool, &json.username, &json.password).await;
    match user {
        Ok(user) => HttpResponse::Created().json(user),
        Err(e) => {
            eprintln!("Error creating user: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }

}