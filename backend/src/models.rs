use crate::schema::{sessions, users};
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Debug, PartialEq, Eq)]
pub struct User {
    pub id: Uuid,
    pub login: String,
    pub email: String,
    pub password: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub login: &'a str,
    pub email: &'a str,
    pub password: &'a str,
}

#[derive(Queryable)]
pub struct Session {
    pub id: Uuid,
    pub iat: std::time::SystemTime,
    pub userid: Uuid,
}

#[derive(Insertable)]
#[diesel(table_name = sessions)]
pub struct NewSession {
    pub userid: Uuid,
}
