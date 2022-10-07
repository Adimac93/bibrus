use diesel::{prelude::*};
use crate::schema::{users,schools};

#[derive(Queryable, Debug)]
pub struct User {
    pub id: uuid::Uuid,
    pub login: String,
    pub password: String,
    pub schoolid: Option<uuid::Uuid>
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a>{
    pub login: &'a str,
    pub password: &'a str,
    pub schoolid: Option<uuid::Uuid>
}

#[derive(Queryable,Debug)]
pub struct School{
    pub id: uuid::Uuid,
    pub name: String
}

#[derive(Insertable)]
#[diesel(table_name = schools)]
pub struct NewSchool<'a>{
   pub  name: &'a str
}