use actix_cors::Cors;
use actix_web::{App, HttpServer, Responder, get, http::header};
use database::establish_connection;

mod database;

#[get("/api/categories")]
async fn get_categories() -> impl Responder {
    "Hello, categories!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let connection = &mut establish_connection();


    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:5173")
            .allowed_methods(["GET", "POST", "DELETE"])
            .allowed_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
            .max_age(3600);

        App::new().wrap(cors).service(get_categories)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
