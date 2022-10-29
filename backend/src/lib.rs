pub mod administration;
pub mod auth;
pub mod database;
pub mod models;
pub mod routes;
pub mod schema;

use crate::database::get_connection_pool;
use axum::{
    middleware::{self},
    response::Html,
    routing::get,
    Extension, Router,
};
use dotenv::dotenv;
use std::net::{SocketAddr, TcpListener};
use tower_http::trace::TraceLayer;

pub fn app() -> Router {
    dotenv().ok();

    Router::new()
        .route("/", get(handler))
        .layer(middleware::from_fn(routes::auth::middleware))
        .nest("/api/auth", routes::auth::router())
        .nest("/api/admin", routes::admin::router())
        .layer(Extension(get_connection_pool()))
        .layer(TraceLayer::new_for_http())
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

pub async fn spawn_app() -> SocketAddr {
    let listener = TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], 3000))).unwrap();
    let addr = listener.local_addr().unwrap();

    tokio::spawn(async move {
        axum::Server::from_tcp(listener)
            .unwrap()
            .serve(app().into_make_service())
            .await
            .unwrap();
    });

    addr
}
