use backend::database::{get_connection_pool};
use axum::{
    debug_handler, extract,
    http::StatusCode,
    response::{Html, Json},
    routing::{get, post},
    Extension, Router,
};
use axum_extra::extract::cookie::{Cookie, Key, SignedCookieJar};
use backend::auth;
use std::net::SocketAddr;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

type UsersState = Arc<RwLock<auth::Users>>;
type SessionState = Arc<RwLock<auth::SessionStorage>>;

#[tokio::main]
async fn main() {
    let users_state = Arc::new(RwLock::new(auth::Users::new()));
    let sessions: SessionState = Arc::new(RwLock::new(auth::SessionStorage::default()));
    let key = Key::generate();
    // build our application with a route

    let auth_routes = Router::new()
        .route("/register", post(register_user))
        .route("/remove", post(delete_account))
        .route("/change-name", post(change_username))
        .route("/change-pass", post(change_password))
        .route("/sessions", post(create_session))
        .route("/me", post(me))
        .layer(Extension(key));

    let app = Router::new()
        .route("/", get(handler))
        .nest("/api/auth", auth_routes)
        .layer(Extension(Arc::clone(&sessions)))
        .layer(Extension(Arc::clone(&users_state)));

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

async fn register_user(
    extract::Json(payload): extract::Json<auth::User>,
    Extension(users_storage): Extension<UsersState>,
) -> Html<&'static str> {
    match &users_storage
        .write()
        .unwrap()
        .add_user(payload.username, payload.password)
    {
        Ok(_) => Html("Successful registration"),
        Err(auth::Error::UserAlreadyExists) => Html("User already exists"),
        Err(auth::Error::WeakPassword) => Html("Weak password"),
        _ => unreachable!(),
    }
}

async fn delete_account(
    extract::Json(payload): extract::Json<auth::UserName>,
    Extension(users_storage): Extension<UsersState>,
) -> Html<&'static str> {
    match &users_storage.write().unwrap().remove_user(payload.username) {
        Ok(_) => Html("Removed user"),
        Err(_) => Html("User not found"),
    }
}

async fn change_username(
    extract::Json(payload): extract::Json<auth::UserNameChange>,
    Extension(users_storage): Extension<UsersState>,
) -> Html<&'static str> {
    match &users_storage
        .write()
        .unwrap()
        .change_name(payload.username, payload.new_username)
    {
        Ok(_) => Html("Username changed"),
        Err(_) => Html("User not found"),
    }
}

async fn change_password(
    extract::Json(payload): extract::Json<auth::User>,
    Extension(users_storage): Extension<UsersState>,
) -> Html<&'static str> {
    match &users_storage
        .write()
        .unwrap()
        .change_pass(payload.username, payload.password)
    {
        Ok(_) => Html("Password changed"),
        Err(auth::Error::UserAlreadyExists) => Html("User already exists"),
        Err(auth::Error::WeakPassword) => Html("Weak password"),
        _ => unreachable!(),
    }
}

async fn create_session(
    Json(payload): Json<auth::User>,
    jar: SignedCookieJar,
    Extension(users_storage): Extension<UsersState>,
    Extension(session_storage): Extension<SessionState>,
) -> Result<SignedCookieJar, StatusCode> {
    if let Some(session_id) =
        authorize_and_create_session(&payload.username, &payload.password, users_storage).await
    {
        let _ = &session_storage.write().unwrap().sessions.insert(session_id.clone(), payload.username);
        Ok(jar.add(Cookie::new("session_id", session_id)))
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

async fn me(jar: SignedCookieJar, Extension(session_storage): Extension<SessionState>) -> Result<(), StatusCode> {
    if let Some(session_id) = jar.get("session_id") {
        if let Some(_) = &session_storage.read().unwrap().sessions.get(session_id.value()) {
            return Ok(())
        }
    }
    Err(StatusCode::UNAUTHORIZED)
}

async fn authorize_and_create_session(
    user: &str,
    pass: &str,
    users_storage: UsersState,
) -> Option<String> {
    match &users_storage.write().unwrap().verify(user, pass) {
        true => Some(Uuid::new_v4().to_string()),
        false => None,
    }
}
