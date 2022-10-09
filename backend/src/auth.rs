use self::schema::sessions::dsl::*;
use self::schema::users::dsl::*;
use crate::schema::{sessions, users};
use crate::{
    models::{NewSession, NewUser, Session, User},
    schema,
};
use axum::extract::Query;
use diesel::{delete, insert_into, update};
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
    if let Some(_) = get_user(conn, new_login) {
        println!("User already exists");
        return Err(Error::UserAlreadyExists);
    }

    if !is_strong(new_password) {
        println!("Too weak password");
        return Err(Error::WeakPassword);
    }

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
        Err(_e) => {
            println!("Cannot register new user");
            return Err(Error::Unexpected);
        }
    }
}

pub fn try_change_pass(
    conn: &mut PgConn,
    user_login: &str,
    pass: &str,
    new_pass: &str,
) -> Result<(), Error> {
    println!("Trying to change user password");
    let res = get_user(conn, user_login);
    
    if res == None {
        println!("User does not exist");
        return Err(Error::UserNotFound);
    }

    let user = res.unwrap();
    if !argon2::verify_encoded(&user.password, pass.as_bytes()).unwrap() {
        println!("Wrong password");
        return Err(Error::IncorrectPassword)
    }

    let query = update(users.filter(login.eq(user_login)))
    .set(password.eq(hash_pass(new_pass)))
    .get_result::<User>(conn);

    if query.is_err() {
        println!("Query failed");
        Err(Error::Unexpected)
    } else {
        Ok(())
    }
}

pub fn login_user(
    conn: &mut PgConn,
    user_login: &str,
    user_password: &str,
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
    // need to fetch corresponding User
    // finds a corresponding session id
    let res = sessions
        .filter(sessions::id.eq(session_id))
        .first::<Session>(conn);

    match res {
        Ok(session) => {
            // finds a user with this session id
            let res = users
                .filter(users::id.eq(session.userid))
                .first::<User>(conn);
            let user = match res {
                Ok(user) => user,
                Err(_e) => return Err(Error::Unexpected),
            };
            match session.iat.elapsed() {
                Ok(duration) => {
                    // verifies whether the session hasn't expired
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
                Err(_e) => Err(Error::Unexpected),
            }
        }
        Err(_e) => Err(Error::Unexpected),
    }
}

pub fn create_session(conn: &mut PgConn, user_id: Uuid) -> Uuid {
    insert_into(sessions::table)
        .values(userid.eq(user_id))
        .returning(sessions::id)
        .get_result::<Uuid>(conn)
        .expect("Failed to create session")
}

pub fn get_user(conn: &mut PgConn, user_login: &str) -> Option<User> {
    let is_present = users
        .filter(login.eq(user_login))
        .first::<User>(conn)
        .optional()
        .unwrap();

    is_present
}
