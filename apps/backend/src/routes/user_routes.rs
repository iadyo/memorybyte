use actix_web::{post, web::{self, Json}, HttpResponse};
use serde::Deserialize;

use crate::database::{self, DBPool};

#[derive(Deserialize)]
struct NewUserJson {
    username: String,
    password: String,
    email: String,
}

#[post("/api/users/create")]
pub async fn create_user(pool: web::Data<DBPool>, json: Json<NewUserJson>) -> HttpResponse {
    let mut connection = pool
        .get()
        .await
        .expect("Failed to get connection from pool");
    
    let username = &json.username;
    let password = &json.password;
    let email = &json.email;

    if let Err(e) = database::insert_user(&mut connection, username, password, email).await {
        eprintln!("Error inserting user: {}", e);
        return HttpResponse::InternalServerError().body("Failed to create user");
    }
    // If the user was created successfully, return a success response
    HttpResponse::Ok().body("User created successfully")
}