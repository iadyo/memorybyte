use actix_cors::Cors;
use actix_web::{App, HttpServer, Responder, get, http::header};

#[get("/api/categories")]
async fn get_categories() -> impl Responder {
    "Hello, categories!"
}

// Dodałem tutaj CORS'a, aby troszku zabezpieczyć API
// Teraz tylko frontend może korzystać z tego API
// I można wysyłać jedynie zapytania GET, POST, DELETE
// jak chcesz sobie dopasować, to zmień allowed_methods na swoje potrzeby
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:5173")
            // Zmieniłem tutaj z wektorów na tablicę, ponieważ te metody
            // wymagają implementacji traita IntoIterator, który jest
            // zarówno w wektorze jak i w tablicy (a tablica mniej waży, bo jest na stosie nie na stercie)
            .allowed_methods(["GET", "POST", "DELETE"])
            .allowed_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
            .max_age(3600);

        App::new().wrap(cors).service(get_categories)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
