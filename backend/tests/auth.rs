use backend::spawn_app;
use reqwest::{Client, StatusCode};
use serde_json::json;
use std::str::FromStr;
use uuid::Uuid;

#[tokio::test]
async fn register_and_login() {
    let addr = spawn_app().await;
    let client = Client::new();

    let payload = json!({
        "login": format!("test_user_{}", Uuid::new_v4().to_string()),
        "email": format!("{}@gmail.com", Uuid::new_v4().to_string()),
        "password": "strong_pass12345",
    });

    let res = client
        .post(format!("http://{}/api/auth/register", addr))
        .json(&payload)
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);

    let res = client
        .post(format!("http://{}/api/auth/login", addr))
        .json(&payload)
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);

    let _session_id = match res.cookies().find(|x| x.name() == "session_id") {
        Some(cookie) => Uuid::from_str(cookie.value()).unwrap(),
        None => panic!(),
    };
}
