use std::time::SystemTime;


use diesel::{prelude::{Insertable, Queryable}, Selectable};


use crate::diesel_schema::schema::users;

// This struct represents a row in the `users` table.
// The `Queryable` trait is used to convert a row in the `users` table to a `User` struct.
#[derive(Queryable, Selectable, PartialEq, Debug)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub password: &'a str,
}