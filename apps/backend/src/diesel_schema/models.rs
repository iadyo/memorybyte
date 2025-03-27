
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::diesel_schema::schema::users;

#[derive(Queryable, Selectable, QueryableByName, Serialize, Deserialize, Debug)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub created_at: i32,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password: &'a str,
}