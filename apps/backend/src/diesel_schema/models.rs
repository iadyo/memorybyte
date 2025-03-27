use diesel::prelude::*;
use serde::Serialize;
use crate::diesel_schema::schema::users;

#[derive(Queryable, Selectable, QueryableByName, Serialize)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    //TODO: ogarnąć date do tych bo są nie kompatybilne 
    // pub created_at: SystemTime, 
    // pub updated_at: SystemTime,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub password: &'a str,
}