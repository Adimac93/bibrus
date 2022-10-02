use axum::{response::Html, routing::{get, post}, Router, extract, Extension};
use tower_http::{add_extension::AddExtensionLayer};
use tower::ServiceBuilder;
use std::{net::SocketAddr};
use serde::{Deserialize};
use backend::auth;
use std::sync::Arc;
use std::sync::RwLock;

type UsersState = Arc<RwLock<auth::Users>>;

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
    .route("/", get(handler))
    .route("/api/auth/register", post(register_user))
        .layer(
        ServiceBuilder::new()
        .layer(AddExtensionLayer::new(Arc::new(RwLock::new(auth::Users::new()))))
        .into_inner()
    );

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
struct RegisterUser {
    username: String,
    password: String,
}

async fn register_user(extract::Json(payload): extract::Json<RegisterUser>, Extension(users_storage): Extension<UsersState>) -> Html<&'static str> {
    match &users_storage.write().unwrap().add_user(payload.username, payload.password) {
        Ok(_) => {
            println!("{:?}", users_storage);
            Html("Successful registration")
        },
        Err(_) => Html("You're a Failure."),
    }
}