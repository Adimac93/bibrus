use std::env;

use axum::{response::Html, Extension};
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};

type PgPool = Pool<ConnectionManager<PgConnection>>;

pub async fn db_test(pool: Extension<PgPool>) -> Html<&'static str> {
    let mut conn = pool.get().expect("Failed to extablish db connection");
    use crate::models::*;
    use crate::schema::users::dsl::*;

    let new_user = NewUser {
        age: &10,
        name: "Bogusław",
    };

    let inserted_user = diesel::insert_into(users)
        .values(&new_user)
        .get_result::<User>(&mut conn)
        .expect("Failed to insert");

    let pull_users = users
        .filter(name.eq("Bogusław"))
        .load::<User>(&mut conn)
        .expect("Failed to fetch users");

    println!("{new_user:?},{pull_users:#?}");
    Html("<h1>Db connection!</h1>")
}

pub fn get_connection_pool() -> PgPool {
    let url = env::var("DATABASE_URL").expect("Cannot find DATABASE_URL variable");
    let manager = ConnectionManager::<PgConnection>::new(url);
    Pool::builder()
        .test_on_check_out(true)
        .max_size(10)
        .build(manager)
        .expect("Could not build connection pool")
}
