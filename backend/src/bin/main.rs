use axum::{
    debug_handler, extract,
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::Html,
    routing::{get, post},
    Extension, Router,
};
use axum_extra::extract::{
    cookie::{Cookie, Key},
    CookieJar,
};
use backend::{
    auth::{
        create_session, login_user, try_change_pass, try_create_new_user, try_get_session,
        AuthError,
    },
    database::{get_connection_pool, PgPool},
};
use dotenv::dotenv;
use serde::Deserialize;
use std::{net::SocketAddr, str::FromStr};
use time::Duration;
use uuid::Uuid;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "backend=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app().into_make_service())
        .await
        .expect("Failed to run axum server");
}

    let key = Key::generate();
    // build our application with a route
pub fn app() -> Router {
    dotenv().ok();

    let auth_routes = Router::new()
        .route("/register", post(post_register_user))
        .route("/login", post(post_login_user))
        .route("/change-pass", post(post_change_pass));

    Router::new()
        .route("/", get(handler))
        .layer(middleware::from_fn(auth_middleware))
        .nest("/api/auth", auth_routes)
        .layer(Extension(get_connection_pool()))
        .layer(TraceLayer::new_for_http())
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

#[debug_handler]
async fn post_register_user(
    extract::Json(payload): extract::Json<AuthUser>,
    pool: Extension<PgPool>,
) -> Result<Html<&'static str>, StatusCode> {
    //unresolved error
    let mut conn = pool.get().unwrap();

    match try_create_new_user(&mut conn, &payload.login, &payload.email, &payload.password) {
        Ok(_) => Ok(Html("<h1>Registered</h1>")),
        Err(AuthError::UserAlreadyExists) => Err(StatusCode::BAD_REQUEST), //which status code?
        Err(AuthError::WeakPassword) => Err(StatusCode::BAD_REQUEST),      //which status code?
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
    //unresolved error
    let mut conn = pool.get().unwrap();

    match try_change_pass(&mut conn, &payload.login, &payload.pass, &payload.new_pass) {
        Ok(_) => Ok(Html("<h1>Password changed</h1>")),
        Err(AuthError::UserNotFound) => Err(StatusCode::NOT_FOUND),
        Err(AuthError::IncorrectPassword) => Err(StatusCode::FORBIDDEN), //which status code?
        Err(AuthError::WeakPassword) => Err(StatusCode::BAD_REQUEST),    //which status code?
        Err(AuthError::Unexpected(_)) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        _ => Err(StatusCode::NOT_IMPLEMENTED),
    }
}

async fn post_login_user(
    extract::Json(payload): extract::Json<AuthUser>,
    pool: Extension<PgPool>,
    jar: CookieJar,
) -> Result<(CookieJar, Html<&'static str>), StatusCode> {
    //unresolved error
    let mut conn = pool.get().unwrap();
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

async fn auth_middleware<B>(
    req: Request<B>,
    _res: Next<B>,
) -> Result<Html<&'static str>, StatusCode> {
    //unresolved error
    let pool = req.extensions().get::<PgPool>().unwrap();
    let cookie_header = req.headers().get("cookie");
    let session_cookie = match cookie_header {
        //2 unresolved errors
        Some(header) => Cookie::parse(header.to_str().unwrap()).unwrap(),
        None => {
            println!("Invalid session");
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    let session_id = Uuid::from_str(session_cookie.value());
    match session_id {
        Ok(id) => {
            //unresolved error
            let mut conn = pool.get().unwrap();
            match try_get_session(&mut conn, id) {
                Ok(user) => {
                    println!("{user:#?}");
                    Ok(Html("<h1>Session ok</h1>"))
                }
                Err(_e) => Err(StatusCode::GONE), //session expired
            }
        }
        Err(_e) => Err(StatusCode::UNAUTHORIZED), //invalid uuid, which status code?
    }
}
