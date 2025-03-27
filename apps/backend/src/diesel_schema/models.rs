use std::time::SystemTime;


use diesel::{data_types::PgTimestamp, prelude::*};


use crate::diesel_schema::schema::users;

// This struct represents a row in the `users` table.
// The `Queryable` trait is used to convert a row in the `users` table to a `User` struct.
#[derive(Queryable, Selectable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub password: &'a str,
}