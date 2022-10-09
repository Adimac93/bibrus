use self::schema::sessions::dsl::*;
use self::schema::users::dsl::*;
use crate::schema::{sessions, users};
use crate::{
    models::{NewSession, NewUser, Session, User},
    schema,
};
use diesel::{delete, insert_into};
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, PooledConnection},
};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use time::Duration;
use uuid::Uuid;

pub type PgConn = PooledConnection<ConnectionManager<PgConnection>>;

#[derive(PartialEq, Debug)]
pub enum Error {
    UserAlreadyExists,
    UserNotFound,
    WeakPassword,
    IncorrectPassword,
    Unexpected,
    SessionExpired,
}

fn hash_pass(pass: &str) -> String {
    let config = argon2::Config::default();
    argon2::hash_encoded(pass.as_bytes(), random_salt().as_bytes(), &config).unwrap()
}

fn random_salt() -> String {
    let mut rng = thread_rng();
    (0..8).map(|_| rng.sample(Alphanumeric) as char).collect()
}

fn is_strong(pass: &str) -> bool {
    let score = zxcvbn::zxcvbn(pass, &[]);
    match score {
        Ok(s) => s.score() >= 3,
        Err(_) => false,
    }
}

pub fn try_create_new_user(
    conn: &mut PgConn,
    new_login: &str,
    new_password: &str,
) -> Result<(), Error> {
    println!("Trying to create new user");
    let is_unique = check_if_user_exsists(conn, new_login);
    if is_unique {
        if is_strong(new_password) {
            let new_user = NewUser {
                login: new_login,
                password: &hash_pass(&new_password),
            };

            let res = insert_into(users)
                .values(vec![&new_user])
                .returning(users::id)
                .get_result::<uuid::Uuid>(conn);

            match res {
                Ok(user_id) => {
                    println!("Created user with uuid: {}", user_id);
                    return Ok(());
                } // register new session with id
                Err(e) => {
                    println!("Cannot register new user");
                    return Err(Error::Unexpected);
                }
            }
        }
        println!("Too weak password");
        return Err(Error::WeakPassword);
    }
    println!("User already exists");
    return Err(Error::UserAlreadyExists);
}

pub fn login_user(
    conn: &mut PgConn,
    user_login: String,
    user_password: String,
) -> Result<Uuid, Error> {
    let res = users.filter(login.eq(user_login)).first::<User>(conn);

    match res {
        Ok(user) => {
            if argon2::verify_encoded(&user.password, user_password.as_bytes()).unwrap() {
                return Ok(user.id);
            }
            println!("Incorrect password!");
            return Err(Error::IncorrectPassword);
        }

        Err(_) => {
            println!("Login not found!");
            Err(Error::UserNotFound)
        }
    }
}

pub fn try_get_session(conn: &mut PgConn, session_id: Uuid) -> Result<User, Error> {
    // need to fetch corrresponding User
    let res = sessions
        .filter(sessions::id.eq(session_id))
        .first::<Session>(conn);

    match res {
        Ok(session) => {
            let res = users
                .filter(users::id.eq(session.userid))
                .first::<User>(conn);
            let user = match res {
                Ok(user) => user,
                Err(e) => return Err(Error::Unexpected),
            };
            match session.iat.elapsed() {
                Ok(duration) => {
                    println!("Session time: {:?}", session.iat.elapsed().unwrap());
                    if duration < Duration::minutes(10) {
                        return Ok(user);
                    }
                    println!("Session expired!");
                    delete(sessions.filter(sessions::id.eq(session.id)))
                        .execute(conn)
                        .unwrap();

                    Err(Error::SessionExpired)
                }
                Err(e) => Err(Error::Unexpected),
            }
        }
        Err(e) => Err(Error::Unexpected),
    }
}

pub fn create_session(conn: &mut PgConn, user_id: Uuid) -> Uuid {
    insert_into(sessions::table)
        .values(userid.eq(user_id))
        .returning(sessions::id)
        .get_result::<Uuid>(conn)
        .expect("Failed to create session")
}

pub fn check_if_user_exsists(conn: &mut PgConn, user_login: &str) -> bool {
    let is_present = users
        .filter(login.eq(user_login))
        .first::<User>(conn)
        .is_err();

    is_present
}
