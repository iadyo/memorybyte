use actix_cors::Cors;
use actix_web::{get, http::header, web::Data, App, HttpServer, Responder};
use database::establish_pool;
use routes::user_routes;

mod database;
mod diesel_schema;
mod routes;

const FAILED_CONNECTION_POOL: &str = "Failed to get connection from pool";

#[get("/api/categories")]
async fn get_categories() -> impl Responder {
    "Hello, categories!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = establish_pool().await; // Aby móc mieć connection pool do App Data, ponieważ samemgo connection nie zclonujemy
    println!("Connection pool created successfully");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:5173")
            .allowed_methods(["GET", "POST", "DELETE"])
            .allowed_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
            .max_age(3600);

        App::new()
        .wrap(cors)
        .app_data(Data::new(pool.clone()))
        .service(get_categories)
        .service(user_routes::create_user)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
