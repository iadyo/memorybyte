use actix_web::{
    HttpResponse, get, post,
    web::{self, Json},
};
use serde::Deserialize;
use sqlx::MySqlPool;

use crate::database;

#[derive(Deserialize)]
struct UserInput {
    id: Option<i32>,
    username: Option<String>,
    password: Option<String>,
}

#[get("/api/users")]
pub async fn get_users(pool: web::Data<MySqlPool>) -> HttpResponse {
    let users = database::select_users(&pool, database::Filter::ALL).await;

    match users {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => {
            eprintln!("Error {e}");
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("/api/users/create")]
pub async fn create_user(pool: web::Data<MySqlPool>, json: Json<UserInput>) -> HttpResponse {
    if json.username.is_none() || json.password.is_none() {
        return HttpResponse::BadRequest().finish();
    }
    let username = json.username.as_ref().unwrap();
    let password = json.password.as_ref().unwrap();

    let user = database::create_user(&pool, username, password).await;
    match user {
        Ok(user) => HttpResponse::Created().json(user),
        Err(e) => {
            eprintln!("Error creating user: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("/api/users/delete")]
pub async fn delete_user(pool: web::Data<MySqlPool>, json: Json<UserInput>) -> HttpResponse {
    if json.id.is_none() {
        return HttpResponse::BadRequest().finish();
    }
    let id = json.id.unwrap();

    let user = database::delete_user_by_id(&pool, id).await;
    match user {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            eprintln!("Error deleting user: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
