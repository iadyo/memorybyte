
use diesel::prelude::*;
use serde::Serialize;

use crate::diesel_schema::schema::users;

#[derive(Queryable, Selectable, QueryableByName, Serialize)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub created_at: i64,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password: &'a str,
}