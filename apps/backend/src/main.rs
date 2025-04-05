use actix_cors::Cors;
use actix_web::{web::{scope, Data}, App, HttpServer};
use dotenvy::dotenv;
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
            .allow_any_origin()
            .allowed_methods(["GET", "POST", "DELETE"])
            .allow_any_header();
            // .allowed_headers([header::AUTHORIZATION, header::CONTENT_TYPE]);

        App::new().app_data(Data::new(pool.clone())).wrap(cors)
        .service(scope("/api").service(routes::create_user).service(routes::get_users)
        )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
