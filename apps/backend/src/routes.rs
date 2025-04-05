use actix_web::{
    get, post, web::{self, Json}, HttpResponse
};
use serde::Deserialize;
use sqlx::MySqlPool;

use crate::models::user::User;

#[derive(Deserialize)]
pub struct UserInput {
    pub email: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
}

#[post("users/create")]
pub async fn create_user(pool: web::Data<MySqlPool>, json: Json<UserInput>) -> HttpResponse {
    if json.username.is_none() || json.password.is_none() {
        return HttpResponse::BadRequest().finish();
    }
    let email = json.email.as_ref().unwrap();
    let username = json.username.as_ref().unwrap();
    let password = json.password.as_ref().unwrap();

    let user = User::create_user(&pool, email, username, password).await;
    match user {
        Ok(user) => HttpResponse::Created().json(user),
        Err(e) => {
            eprintln!("Error creating user: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("users")]
pub async fn get_users(pool: web::Data<MySqlPool>) -> HttpResponse {
    let users = User::get_all_users(&pool).await;
    match users {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => {
            eprintln!("Error fetching users: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
