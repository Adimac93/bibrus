use reqwest::StatusCode;
use serde_json::json;
use uuid::Uuid;
mod tools;

#[tokio::test]
async fn register_and_login() {
    let addr = tools::spawn_app().await;
    let client = tools::client();

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

    let res = client
        .get(format!("http://{}/api/auth/greet", addr))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);
}
