use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};
use std::env;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

pub fn get_connection_pool() -> PgPool {
    let url = env::var("DATABASE_URL").expect("Cannot find DATABASE_URL variable");
    let manager = ConnectionManager::<PgConnection>::new(url);
    Pool::builder()
        .test_on_check_out(true)
        .max_size(10)
        .build(manager)
        .expect("Could not build connection pool")
}
