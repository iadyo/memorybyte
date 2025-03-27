use std::env;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::pooled_connection::bb8::Pool;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use dotenvy::dotenv;


pub type DBPool = Pool<AsyncPgConnection>;

pub async fn establish_pool() -> DBPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(&database_url);
    let pool: Pool<AsyncPgConnection> = Pool::builder().build(config).await.expect("Failed to create pool");
    pool
}

pub async fn insert_user(
    connection: &mut AsyncPgConnection, 
    username: &str, 
    password: &str, 
    email: &str
) -> Result<(), diesel::result::Error> {
    use crate::diesel_schema::models::NewUser;
    use crate::diesel_schema::schema::users;

    let new_user = NewUser {
        username,
        password,
        email,
    };

    // Insert the new user into the database
    match diesel::insert_into(users::table)
        .values(&new_user)
        .execute(connection)
        .await {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
}

//     JAPIERDOLE POPIERDOLI MNEI TUTAJ KURWA MAĆ JAK TO JEBANE GÓWNO ZROBIĆ JUZ MNIE KREW ZALEJE
// pub async fn select_users(
//     connection: &mut AsyncPgConnection
// ) -> Vec<User> {
//     use crate::diesel_schema::schema::users::dsl::*;

//     let results = users
//         .load::<User>(connection)
//         .await
//         .expect("Error loading users");

//     results
// }