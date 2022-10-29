use crate::{
    auth::{
        create_session, login_user, try_change_pass, try_create_new_user, try_get_session,
        AuthError,
    },
    database::PgPool,
};
use axum::{
    extract,
    http::{Request, StatusCode},
    middleware::Next,
    response::Html,
    routing::post,
    Extension, Router,
};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use time::Duration;
use uuid::Uuid;

pub fn router() -> Router {
    Router::new()
        .route("/register", post(post_register_user))
        .route("/login", post(post_login_user))
        .route("/change-pass", post(post_change_pass))
}

#[derive(Serialize, Deserialize, Debug)]
struct AuthUser {
    login: String,
    email: String,
    password: String,
}

async fn post_register_user(
    extract::Json(payload): extract::Json<AuthUser>,
    pool: Extension<PgPool>,
) -> Result<Html<&'static str>, StatusCode> {
    let mut conn = pool.get().map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?;

    match try_create_new_user(&mut conn, &payload.login, &payload.email, &payload.password) {
        Ok(_) => Ok(Html("<h1>Registered</h1>")),
        Err(AuthError::UserAlreadyExists) => Err(StatusCode::BAD_REQUEST),
        Err(AuthError::WeakPassword) => Err(StatusCode::BAD_REQUEST),
        Err(AuthError::Unexpected(_)) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        _ => Err(StatusCode::NOT_IMPLEMENTED),
    }
}

#[derive(Deserialize)]
struct ChangePass {
    login: String,
    pass: String,
    new_pass: String,
}

async fn post_change_pass(
    extract::Json(payload): extract::Json<ChangePass>,
    pool: Extension<PgPool>,
) -> Result<Html<&'static str>, StatusCode> {
    let mut conn = pool.get().map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?;

    match try_change_pass(&mut conn, &payload.login, &payload.pass, &payload.new_pass) {
        Ok(_) => Ok(Html("<h1>Password changed</h1>")),
        Err(AuthError::UserNotFound) => Err(StatusCode::NOT_FOUND),
        Err(AuthError::IncorrectPassword) => Err(StatusCode::FORBIDDEN),
        Err(AuthError::WeakPassword) => Err(StatusCode::BAD_REQUEST),
        Err(AuthError::Unexpected(_)) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        _ => Err(StatusCode::NOT_IMPLEMENTED),
    }
}

async fn post_login_user(
    extract::Json(payload): extract::Json<AuthUser>,
    pool: Extension<PgPool>,
    jar: CookieJar,
) -> Result<(CookieJar, Html<&'static str>), StatusCode> {
    let mut conn = pool.get().map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?;
    match login_user(&mut conn, &payload.login, &payload.password) {
        Ok(user_id) => {
            let session_id = create_session(&mut conn, user_id)
                .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?;

            let cookie = Cookie::build("session_id", session_id.to_string())
                .path("/")
                .http_only(true)
                .secure(false)
                .max_age(Duration::minutes(10))
                .finish();
            Ok((jar.add(cookie), Html("<h1>Logged in</h1>")))
        }
        Err(AuthError::UserNotFound) => Err(StatusCode::UNAUTHORIZED),
        Err(AuthError::IncorrectPassword) => Err(StatusCode::UNAUTHORIZED),
        _ => Err(StatusCode::NOT_IMPLEMENTED),
    }
}

pub async fn middleware<B>(
    req: Request<B>,
    _res: Next<B>,
) -> Result<Html<&'static str>, StatusCode> {
    let option_pool = req.extensions().get::<PgPool>();

    let pool = match option_pool {
        Some(x) => x,
        None => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let cookie_header = req.headers().get("cookie");
    let session_cookie = match cookie_header {
        Some(header) => Cookie::parse(
            header
                .to_str()
                .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?,
        )
        .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?,
        None => {
            println!("Invalid session");
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    let session_id = Uuid::from_str(session_cookie.value());
    match session_id {
        Ok(id) => {
            let mut conn = pool.get().map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?;
            match try_get_session(&mut conn, id) {
                Ok(user) => {
                    println!("{user:#?}");
                    Ok(Html("<h1>Session ok</h1>"))
                }
                // session expired
                Err(_e) => Err(StatusCode::GONE),
            }
        }
        // invalid uuid
        Err(_e) => Err(StatusCode::UNAUTHORIZED),
    }
}
