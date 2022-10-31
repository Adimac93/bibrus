use crate::{
    auth::{
        create_session, login_user, try_change_pass, try_create_new_user, try_get_session,
        AuthError,
    },
    database::PgPool,
    models::User,
};
use axum::{
    extract,
    http::{self, Request, StatusCode},
    middleware::Next,
    response::{Html, Response},
    routing::{get, post},
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
        .merge(
            Router::new()
                .route("/greet", get(greet))
                .route_layer(axum::middleware::from_fn(middleware)),
        )
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

pub async fn middleware<B>(mut req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    let pool = req
        .extensions()
        .get::<PgPool>()
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

    let cookie_header = req
        .headers()
        .get(http::header::COOKIE)
        .ok_or(StatusCode::UNAUTHORIZED)?;
    let cookie = Cookie::parse(
        cookie_header
            .to_str()
            .map_err(|_| StatusCode::UNAUTHORIZED)?,
    )
    .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let session_id = Uuid::from_str(cookie.value()).map_err(|_| StatusCode::UNAUTHORIZED)?;

    let mut conn = pool.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let user = try_get_session(&mut conn, session_id).map_err(|_| StatusCode::UNAUTHORIZED)?;

    req.extensions_mut().insert(user);
    Ok(next.run(req).await)
}

async fn greet(Extension(current_user): Extension<User>) -> Html<String> {
    Html(format!("Hello {}", current_user.login))
}
