use actix_cors::Cors;
use actix_web::{App, HttpServer, Responder, get, http::header};
// use database::establish_connection;
// use diesel::{query_dsl::methods::LimitDsl, MysqlConnection, RunQueryDsl};
// use diesel_mod::{models::{self, NewUser, User}, schema::users};

// mod database;
// mod diesel_mod;

#[get("/api/categories")]
async fn get_categories() -> impl Responder {
    "Hello, categories!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // let connection = &mut establish_connection();

    // create_user(connection, 
    //     "Jaja", "maciek", "szajnowski", 
    //     "superkópadópa123@gmail.com", 
    //     "jaja123");

    // let users = list_users(connection);

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

// fn create_user<'a>(
//     connection: &mut MysqlConnection, 
//     nickname: &'a str, 
//     name: &'a str, 
//     last_name: &'a str,
//     email: &'a str,
//     password: &'a str,
// ) {

//     let new_user = NewUser {
//         nickname,
//         name,
//         last_name,
//         email,
//         password,
//     };

//     diesel::insert_into(users::table)
//     .values(&new_user)
//     .execute(connection)
//     .expect("Error saving new user");

//     println!("User saved successfully");
// }

// async fn list_users(conn: &mut MysqlConnection) -> Vec<User> {
//     use self::diesel_mod::schema::users::dsl::*;

//     let results = users.limit(5).load(conn).expect("Error loading users");

//     return results;
// }