use actix_cors::Cors;
use actix_web::{App, HttpServer, http::header, web::Data};
use dotenvy::dotenv;
use routes::{create_user, delete_user, get_users};
use sqlx::MySqlPool;
use std::env;

mod database;
mod models;
mod routes;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let env = dotenv().ok();

    if env.is_none() {
        panic!("Failed to load .env file");
    }

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = MySqlPool::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    if let Err(e) = database::create_tables(&pool).await {
        panic!("Failed to create tables: {:?}", e);
    }

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("localhost:5137")
            .allowed_methods(["GET", "POST", "DELETE"])
            .allowed_headers([header::AUTHORIZATION, header::CONTENT_TYPE])
            .max_age(3600);

        App::new()
        .app_data(Data::new(pool.clone()))
        .wrap(cors)
        .service(create_user)
        .service(delete_user)
        .service(get_users)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
