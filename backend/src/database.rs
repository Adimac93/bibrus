use std::env;

use axum::{response::Html, Extension};
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};

type PgPool = Pool<ConnectionManager<PgConnection>>;

pub fn get_connection_pool() -> PgPool {
    let url = env::var("DATABASE_URL").expect("Cannot find DATABASE_URL variable");
    let manager = ConnectionManager::<PgConnection>::new(url);
    Pool::builder()
        .test_on_check_out(true)
        .max_size(10)
        .build(manager)
        .expect("Could not build connection pool")
}
