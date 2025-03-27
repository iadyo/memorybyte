use actix_web::{post, web, HttpResponse};

use crate::database::{self, DBPool};

#[post("/api/users/create")]
pub async fn create_user(pool: web::Data<DBPool>) -> HttpResponse {
    let mut connection = pool.get().await.expect("Failed to get connection from pool");
    let username = "test_user";
    let password = "test_password";
    let email = "jaja@gmail.com";

    if let Err(e) = database::insert_user(&mut connection, username, password, email).await {
        eprintln!("Error inserting user: {}", e);
        return HttpResponse::InternalServerError().body("Failed to create user");
    }
    // If the user was created successfully, return a success response
    HttpResponse::Ok().body("User created successfully")
}