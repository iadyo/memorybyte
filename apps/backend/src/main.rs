use std::vec;

use actix_web::{get, App, HttpServer, Responder};
use actix_cors::Cors;

#[get("/api/categories")]
async fn get_categories() -> impl Responder {
    "Hello, categories!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:5173")
            .allowed_methods(vec!["GET", "POST", "DELETE"])
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .service(get_categories)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}

