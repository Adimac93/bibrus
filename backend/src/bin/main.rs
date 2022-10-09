use axum::{
    debug_handler, extract,
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::{Html},
    routing::{get, post},
    Extension, Router,
};
use axum_extra::extract::{
    cookie::{Cookie, Key},
    CookieJar,
};
use backend::{
    auth::{create_session, login_user, try_create_new_user, try_get_session, try_change_pass},
    database::{get_connection_pool, PgPool},
};
use dotenv::dotenv;
use serde::Deserialize;
use std::{net::SocketAddr, str::FromStr};
use time::Duration;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    // load env variables from .env file
    dotenv().ok();

    let key = Key::generate();
    // build our application with a route

    let auth_routes = Router::new()
        .route("/register", post(POST_register_user))
        .route("/login", post(POST_login_user))
        .layer(Extension(key))
        .route("/change-pass", post(POST_change_pass));

    let app = Router::new()
        .route("/", get(handler))
        .layer(middleware::from_fn(auth_middleware))
        .nest("/api/auth", auth_routes)
        .layer(Extension(get_connection_pool()));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

#[derive(Deserialize)]
struct AuthUser {
    login: String,
    email: String,
    password: String,
}

async fn POST_register_user(
    extract::Json(payload): extract::Json<AuthUser>,
    pool: Extension<PgPool>,
) -> Html<&'static str> {
    let mut conn = pool.get().unwrap();

    match try_create_new_user(&mut conn, &payload.login, &payload.email, &payload.password) {
        Ok(_) => Html("<h1>Registered</h1>"),
        Err(_e) => Html("<h1>Failed to register</h1>"),
    }
}

#[derive(Deserialize)]
struct ChangePass {
    login: String,
    pass: String,
    new_pass: String,
}

async fn POST_change_pass(
    extract::Json(payload): extract::Json<ChangePass>,
    pool: Extension<PgPool>,
) -> Html<&'static str> {
    let mut conn = pool.get().unwrap();

    match try_change_pass(&mut conn, &payload.login, &payload.pass, &payload.new_pass) {
        Ok(_) => Html("Password changed"),
        Err(_) => Html("Failed to change password"),
    }
}

async fn POST_login_user(
    extract::Json(payload): extract::Json<AuthUser>,
    pool: Extension<PgPool>,
    jar: CookieJar,
) -> Result<(CookieJar, Html<&'static str>), StatusCode> {
    let mut conn = pool.get().unwrap();
    match login_user(&mut conn, &payload.login, &payload.password) {
        Ok(user_id) => {
            Html("<h1>Logged in</h1>");
            let session_id = create_session(&mut conn, user_id);
            let cookie = Cookie::build("session_id", session_id.to_string())
                .path("/")
                .http_only(true)
                .secure(false)
                .max_age(Duration::minutes(10))
                .finish();
            Ok((jar.add(cookie), Html("<h1>Logged in</h1>")))
        }
        Err(_e) => Err(StatusCode::UNAUTHORIZED),
    }
}

async fn auth_middleware<B>(req: Request<B>, _res: Next<B>) -> Html<&'static str> {
    let pool = req.extensions().get::<PgPool>().unwrap();
    let cookie_header = req.headers().get("cookie");
    let session_cookie = match cookie_header {
        Some(header) => Ok(Cookie::parse(header.to_str().unwrap()).unwrap()),
        None => {
            println!("Invalid session");
            Err(())
        }
    };

    match session_cookie {
        Ok(cookie) => {
            let session_id = Uuid::from_str(cookie.value());
            match session_id {
                Ok(id) => {
                    let mut conn = pool.get().unwrap();
                    match try_get_session(&mut conn, id) {
                        Ok(user) => {
                            println!("{user:#?}");
                            return Html("<h1>Session ok</h1>");
                        }
                        Err(_e) => return Html("<h1>Session expired</h1>"),
                    }
                }
                Err(_e) => return Html("<h1>Unvalid UUID</h1>"),
            }
        }
        Err(_) => return Html("<h1>Invalid cookie</h1>"),
    }
}
