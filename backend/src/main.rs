use axum::{response::Html, routing::{get, post}, Router, extract, Extension};
use tower_http::add_extension::AddExtensionLayer;
use std::sync::{Arc, RwLock};
use std::net::SocketAddr;
use backend::auth;

type UsersState = Arc<RwLock<auth::Users>>;

#[tokio::main]
async fn main() {
    let users_state = Arc::new(RwLock::new(auth::Users::new()));
    // build our application with a route
    let app = Router::new()
    .route("/", get(handler))
    .route("/api/auth/register", post(register_user))
        .layer(AddExtensionLayer::new(Arc::clone(&users_state)))
    .route("/api/auth/remove", post(delete_account))
        .layer(AddExtensionLayer::new(Arc::clone(&users_state)))
    .route("/api/auth/change-name", post(change_username))
        .layer(AddExtensionLayer::new(Arc::clone(&users_state)))
    .route("/api/auth/change-pass", post(change_password))
        .layer(AddExtensionLayer::new(Arc::clone(&users_state)));

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

async fn register_user(extract::Json(payload): extract::Json<auth::User>, Extension(users_storage): Extension<UsersState>) -> Html<&'static str> {
    match &users_storage.write().unwrap().add_user(payload.username, payload.password) {
        Ok(_) => Html("Successful registration"),
        Err(_) => Html("You're a Failure."),
    }
}

async fn delete_account(extract::Json(payload): extract::Json<auth::UserName>, Extension(users_storage): Extension<UsersState>) -> Html<&'static str> {
    match &users_storage.write().unwrap().remove_user(payload.username) {
        Ok(_) => Html("Removed user"),
        Err(_) => Html("User not found"),
    }
}

async fn change_username(extract::Json(payload): extract::Json<auth::UserNameChange>, Extension(users_storage): Extension<UsersState>) -> Html<&'static str> {
    match &users_storage.write().unwrap().change_name(payload.username, payload.new_username) {
        Ok(_) => Html("Change username"),
        Err(_) => Html("User not found"),
    }
}

async fn change_password(extract::Json(payload): extract::Json<auth::User>, Extension(users_storage): Extension<UsersState>) -> Html<&'static str> {
    match &users_storage.write().unwrap().change_pass(payload.username, payload.password) {
        Ok(_) => Html("Change password"),
        Err(_) => Html("User not found"),
    }
}