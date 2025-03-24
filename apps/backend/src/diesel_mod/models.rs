use std::time::SystemTime;

use diesel::prelude::{Insertable, Queryable};

use crate::diesel_mod::schema::users;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub nickname: String,
    pub name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub nickname: &'a str,
    pub name: &'a str,
    pub last_name: &'a str,
    pub email: &'a str,
    pub password: &'a str,
}