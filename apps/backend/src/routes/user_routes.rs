use actix_web::{get, post, web::{self, Json}, HttpResponse};
use serde::Deserialize;

use crate::{database::{self, DBPool}, FAILED_CONNECTION_POOL};

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
        .expect(FAILED_CONNECTION_POOL);
    
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

#[get("/api/users")]
pub async fn get_users(pool: web::Data<DBPool>) -> HttpResponse {
    println!("Requested for listing users");

    let mut connection = pool.get().expect(FAILED_CONNECTION_POOL);

    let result_users = database::select_users(&mut connection).await;

    match serde_json::to_string(&result_users) {
        Ok(json) => HttpResponse::Ok().body(json),
        Err(e) => {
            eprintln!("Error serializing users: {}", e);
            HttpResponse::InternalServerError().body("Failed to serialize users")
        }
    }
}