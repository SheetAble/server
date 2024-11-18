use diesel::prelude::*;

use crate::database::schema::users;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::database::schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub library: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub email: &'a str, // why pointer?
    pub password: &'a str,
    pub library: &'a str,
}