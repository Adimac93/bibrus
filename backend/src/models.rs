use diesel::prelude::*;

use crate::schema::users;

#[derive(Queryable, PartialEq, Debug)]
pub struct User {
    id: i32,
    name: String,
    age: i32,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub age: &'a i32,
}
