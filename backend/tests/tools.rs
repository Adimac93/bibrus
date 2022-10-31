use ::backend::app;
use reqwest::Client;
use std::net::{SocketAddr, TcpListener};

pub fn client() -> Client {
    Client::builder().cookie_store(true).build().unwrap()
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
