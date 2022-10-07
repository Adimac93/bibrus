use uuid::Uuid;
use crate::{database::PgPool, schema, models::{NewSchool, School}};
use diesel::{prelude::*, r2d2::{PooledConnection, ConnectionManager}};
use crate::models::{User,NewUser};
use self::schema::users::dsl::*;
use self::schema::schools::dsl::*;
use crate::schema::{users,schools};
use diesel::{insert_into,update};

type PgConn = PooledConnection<ConnectionManager<PgConnection>>;
fn create_school(conn: &mut PgConn, school_name: &str){
    let new_school = NewSchool {name: &school_name};
    insert_into(schools::table)
    .values(&new_school)
    .get_result::<School>(conn)
    .expect("Failed");
}

fn change_school_name(conn: &mut PgConn,user_id: Uuid, new_school_name: &str){
    update(schools.filter(schools::id.eq(user_id)))
    .set(name.eq(new_school_name))
    .get_result::<School>(conn)
    .expect("Failed");
}


fn create_user(conn: &mut PgConn, new_login: &str, new_password: &str){
    let new_user = NewUser {login: new_login, password: new_password, schoolid: None};
    insert_into(users::table)
    .values(&new_user)
    .get_result::<User>(conn)
    .expect("Failed");
}

fn change_user_password(conn: &mut PgConn,user_id: Uuid, new_password: &str){
    update(users.filter(users::id.eq(user_id)))
    .set(password.eq(new_password))
    .get_result::<User>(conn)
    .expect("Failed");
}

fn change_user_login(conn: &mut PgConn,user_id: Uuid, new_login: &str){
    update(users.filter(users::id.eq(user_id)))
    .set(password.eq(new_login))
    .get_result::<User>(conn)
    .expect("Failed");
}

fn add_user_to_school(conn: &mut PgConn, user_id: Uuid, school_id: Uuid){
    update(users.filter(users::id.eq(user_id)))
    .set(schoolid.eq(school_id))
    .get_result::<User>(conn)
    .expect("Failed");
}