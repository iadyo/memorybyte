use std::env;

use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};
use dotenvy::dotenv;

pub async fn establish_connection() -> AsyncPgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    AsyncPgConnection::establish(&database_url)
    .await.expect("Connection failed")
}

pub async fn insert_user(
    connection: &mut AsyncPgConnection, 
    username: &str, 
    password: &str, 
    email: &str
) {
    use crate::diesel_schema::models::NewUser;
    use crate::diesel_schema::schema::users;

    let new_user = NewUser {
        username,
        password,
        email,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(connection)
        .await.expect("Failed to insert a new user");

    println!("User saved successfully");
}