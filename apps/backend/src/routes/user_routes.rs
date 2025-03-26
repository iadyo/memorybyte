use actix_web::{post, HttpResponse};

#[post("/api/users/create")]
pub async fn create_user() -> HttpResponse {
    HttpResponse::Ok().body("User created successfully")
}