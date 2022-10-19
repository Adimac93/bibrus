use self::schema::sessions::dsl::*;
use self::schema::users::dsl::*;
use crate::schema::{sessions, users};
use crate::{
    models::{NewUser, Session, User},
    schema,
};
use anyhow::{Context, Result};
use diesel::{delete, insert_into, update};
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, PooledConnection},
};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use thiserror::Error;
use time::Duration;
use uuid::Uuid;

pub type PgConn = PooledConnection<ConnectionManager<PgConnection>>;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("User already exists")]
    UserAlreadyExists,
    #[error("User not found")]
    UserNotFound,
    #[error("Password is too weak")]
    WeakPassword,
    #[error("Wrong password")]
    IncorrectPassword,
    #[error("Session expired")]
    SessionExpired,
    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}

fn hash_pass(pass: &str) -> anyhow::Result<String> {
    let config = argon2::Config::default();
    argon2::hash_encoded(pass.as_bytes(), random_salt().as_bytes(), &config)
        .context("Failed to hash a password.")
}

fn random_salt() -> String {
    let mut rng = thread_rng();
    (0..8).map(|_| rng.sample(Alphanumeric) as char).collect()
}

fn is_strong(user_password: &str, user_inputs: &[&str]) -> bool {
    let score = zxcvbn::zxcvbn(user_password, user_inputs);
    match score {
        Ok(s) => s.score() >= 3,
        Err(_) => false,
    }
}

pub fn try_create_new_user(
    conn: &mut PgConn,
    new_login: &str,
    new_email: &str,
    new_password: &str,
) -> Result<(), AuthError> {
    println!("Trying to create new user");
    if get_by_login(conn, new_login)?.is_some() {
        println!("User with this name already exists");
        return Err(AuthError::UserAlreadyExists);
    }

    if get_by_email(conn, new_email)?.is_some() {
        println!("User with this email already exists");
        return Err(AuthError::UserAlreadyExists);
    }

    if !is_strong(new_password, &[new_login, new_email]) {
        println!("Too weak password");
        return Err(AuthError::WeakPassword);
    }

    let new_user = NewUser {
        login: new_login,
        email: new_email,
        password: &hash_pass(new_password)?,
    };

    let user_id = insert_into(users)
        .values(vec![&new_user])
        .returning(users::id)
        .get_result::<uuid::Uuid>(conn)
        .context("Failed to insert user")?;

    println!("Created user with uuid: {}", user_id);
    Ok(())
}

pub fn try_change_pass(
    conn: &mut PgConn,
    user_login: &str,
    pass: &str,
    new_pass: &str,
) -> Result<(), AuthError> {
    println!("Trying to change user password");
    let res = get_by_login(conn, user_login)?;

    if res == None {
        println!("User does not exist");
        return Err(AuthError::UserNotFound);
    }

    let user = res.unwrap();

    if !argon2::verify_encoded(&user.password, pass.as_bytes())
        .context("Failed to verify password")?
    {
        println!("Wrong password");
        return Err(AuthError::IncorrectPassword);
    }

    if !is_strong(new_pass, &[&user.login, &user.email]) {
        println!("Too weak password");
        return Err(AuthError::WeakPassword);
    }

    update(users.filter(login.eq(user_login)))
        .set(password.eq(hash_pass(new_pass)?))
        .get_result::<User>(conn)
        .context("Failed to update user.")?;

    Ok(())
}

pub fn login_user(
    conn: &mut PgConn,
    user_login: &str,
    user_password: &str,
) -> Result<Uuid, AuthError> {
    let res = users.filter(login.eq(user_login)).first::<User>(conn);

    match res {
        Ok(user) => {
            if argon2::verify_encoded(&user.password, user_password.as_bytes())
                .context("Failed to verify password.")?
            {
                return Ok(user.id);
            }
            println!("Incorrect password!");
            Err(AuthError::IncorrectPassword)
        }

        Err(_) => {
            println!("Login not found!");
            Err(AuthError::UserNotFound)
        }
    }
}

pub fn try_get_session(conn: &mut PgConn, session_id: Uuid) -> Result<User, AuthError> {
    // need to fetch corresponding User
    // finds a corresponding session id
    let session = sessions
        .filter(sessions::id.eq(session_id))
        .first::<Session>(conn)
        .context("Failed to fetch session")?;

    // finds a user with this session id
    let user = users
        .filter(users::id.eq(session.userid))
        .first::<User>(conn)
        .context("Failed to fetch user")?;

    let duration = session
        .iat
        .elapsed()
        .context("Failed to check session time")?;

    // verifies whether the session hasn't expired
    println!("Session time: {:?}", duration);
    if duration < Duration::minutes(10) {
        return Ok(user);
    }
    println!("Session expired!");
    delete(sessions.filter(sessions::id.eq(session.id)))
        .execute(conn)
        .context("Failed to delete session")?;

    Err(AuthError::SessionExpired)
}

pub fn create_session(conn: &mut PgConn, user_id: Uuid) -> anyhow::Result<Uuid> {
    insert_into(sessions::table)
        .values(userid.eq(user_id))
        .returning(sessions::id)
        .get_result::<Uuid>(conn)
        .context("Failed to create session")
}

pub fn get_by_login(conn: &mut PgConn, user_login: &str) -> anyhow::Result<Option<User>> {
    users
        .filter(login.eq(user_login))
        .first::<User>(conn)
        .optional()
        .context("Failed to fetch user by login")
}

pub fn get_by_email(conn: &mut PgConn, user_email: &str) -> anyhow::Result<Option<User>> {
    users
        .filter(email.eq(user_email))
        .first::<User>(conn)
        .optional()
        .context("Failed to fetch user by email")
}
