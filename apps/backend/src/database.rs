use std::env;
use diesel::r2d2::{self, Pool};
use diesel::{PgConnection, RunQueryDsl};
use dotenvy::dotenv;

use crate::diesel_schema::models::User;
use crate::diesel_schema::schema::users;


pub type DBPool = Pool<diesel::r2d2::ConnectionManager<PgConnection>>;

pub async fn establish_pool() -> DBPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = diesel::r2d2::ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager).expect("Failed to create pool");

    pool
}

pub async fn insert_user(
    connection: &mut PgConnection, 
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
        {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
}

//     JAPIERDOLE POPIERDOLI MNEI TUTAJ KURWA MAĆ JAK TO JEBANE GÓWNO ZROBIĆ JUZ MNIE KREW ZALEJE
pub async fn select_users(
    connection: &mut PgConnection
) -> Vec<User> {
    let results = users::table.load::<User>(connection).expect("Error loading users");
    results
}