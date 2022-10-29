use axum::{
    debug_handler, extract,
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::Html,
    routing::{get, post},
    Extension, Router,
};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use backend::{
    auth::{
        create_session, login_user, try_change_pass, try_create_new_user, try_get_session,
        AuthError,
    },
    database::{get_connection_pool, PgPool}, administration,
};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, str::FromStr};
use time::Duration;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;
use time::Date;

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

pub fn app() -> Router {
    dotenv().ok();

    let auth_routes = Router::new()
        .route("/register", post(post_register_user))
        .route("/login", post(post_login_user))
        .route("/change-pass", post(post_change_pass));

    let admin_routes = Router::new()
        .route("/school", post(post_create_school))
        .route("/student", post(post_create_student))
        .route("/teacher", post(post_create_teacher))
        .route("/subject", post(post_create_subject))
        .route("/group", post(post_create_group))
        .route("/class", post(post_create_class))
        .route("/class-student", post(post_create_class_student))
        .route("/grade", post(post_create_grade))
        .route("/task", post(post_create_task));

    Router::new()
        .route("/", get(handler))
        .layer(middleware::from_fn(auth_middleware))
        .nest("/api/auth", auth_routes)
        .nest("/api/admin", admin_routes)
        .layer(Extension(get_connection_pool()))
        .layer(TraceLayer::new_for_http())

}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

#[derive(Serialize, Deserialize, Debug)]
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

async fn auth_middleware<B>(
    req: Request<B>,
    _res: Next<B>,
) -> Result<Html<&'static str>, StatusCode> {
    let option_pool = req.extensions().get::<PgPool>();

    let pool;
    match option_pool {
        Some(x) => pool = x,
        None => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let cookie_header = req.headers().get("cookie");
    let session_cookie = match cookie_header {
        Some(header) => Cookie::parse(header.to_str().map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?)
            .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?,
        None => {
            println!("Invalid session");
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    let session_id = Uuid::from_str(session_cookie.value());
    match session_id {
        Ok(id) => {
            let mut conn = pool.get().map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?;
            match try_get_session(&mut conn, id) {
                Ok(user) => {
                    println!("{user:#?}");
                    Ok(Html("<h1>Session ok</h1>"))
                }
                // session expired
                Err(_e) => Err(StatusCode::GONE),
            }
        }
        // invalid uuid
        Err(_e) => Err(StatusCode::UNAUTHORIZED),
    }
}

#[derive(Deserialize)]
struct CreateSchool {
    pub name: String,
    pub place: String,
    pub school_type: Option<String>,
}

async fn post_create_school(
    extract::Json(payload): extract::Json<CreateSchool>,
    pool: Extension<PgPool>,
) -> Result<Html<&'static str>, StatusCode> {
    let mut conn = pool.get().map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?;

    let school = administration::create_school(
        &mut conn,
        &payload.name,
        &payload.place,
        payload.school_type.as_deref());

    match school {
        Ok(_) => Ok(Html("School created")),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[derive(Deserialize)]
struct CreateStudent {
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: Date,
    pub school_id: Uuid,
    pub group_id: Uuid,
    pub user_id: Option<Uuid>,
}

async fn post_create_student(
    extract::Json(payload): extract::Json<CreateStudent>,
    pool: Extension<PgPool>,
) -> Result<Html<&'static str>, StatusCode> {
    let mut conn = pool.get().map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?;

    let student = administration::create_student(
        &mut conn,
        &payload.first_name,
        &payload.last_name,
        payload.date_of_birth,
        payload.school_id,
        payload.group_id,
        payload.user_id);

    match student {
        Ok(_) => Ok(Html("Student created")),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[derive(Deserialize)]
struct CreateTeacher {
    pub first_name: String,
    pub last_name: String,
    pub school_id: Uuid,
    pub user_id: Uuid,
}

async fn post_create_teacher(
    extract::Json(payload): extract::Json<CreateTeacher>,
    pool: Extension<PgPool>,
) -> Result<Html<&'static str>, StatusCode> {
    let mut conn = pool.get().map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?;

    let teacher = administration::create_teacher(
        &mut conn,
        &payload.first_name,
        &payload.last_name,
        payload.user_id,
        payload.school_id,);

    match teacher {
        Ok(_) => Ok(Html("Teacher created")),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[derive(Deserialize)]
struct CreateSubject {
    pub name: String,
    pub school_id: Uuid,
}

async fn post_create_subject(
    extract::Json(payload): extract::Json<CreateSubject>,
    pool: Extension<PgPool>,
) -> Result<Html<&'static str>, StatusCode> {
    let mut conn = pool.get().map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?;

    let subject = administration::create_subject(
        &mut conn,
        &payload.name,
        payload.school_id,);

    match subject {
        Ok(_) => Ok(Html("Subject created")),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[derive(Deserialize)]
struct CreateGroup {
    pub name: String,
    pub school_id: Uuid,
}

async fn post_create_group(
    extract::Json(payload): extract::Json<CreateGroup>,
    pool: Extension<PgPool>,
) -> Result<Html<&'static str>, StatusCode> {
    let mut conn = pool.get().map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?;

    let group = administration::create_group(
        &mut conn,
        &payload.name,
        payload.school_id,);

    match group {
        Ok(_) => Ok(Html("Group created")),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[derive(Deserialize)]
struct CreateClass {
    pub subject_id: Uuid,
    pub group_id: Uuid,
    pub teacher_id: Uuid,
}

async fn post_create_class(
    extract::Json(payload): extract::Json<CreateClass>,
    pool: Extension<PgPool>,
) -> Result<Html<&'static str>, StatusCode> {
    let mut conn = pool.get().map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?;

    let class = administration::create_class(
        &mut conn,
        payload.subject_id,
        payload.group_id,
        payload.teacher_id);

    match class {
        Ok(_) => Ok(Html("Class created")),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[derive(Deserialize)]
struct CreateClassStudent {
    pub student_id: Uuid,
    pub class_id: Uuid,
}

async fn post_create_class_student(
    extract::Json(payload): extract::Json<CreateClassStudent>,
    pool: Extension<PgPool>,
) -> Result<Html<&'static str>, StatusCode> {
    let mut conn = pool.get().map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?;

    let class_student = administration::add_student_to_class(
        &mut conn,
        payload.student_id,
        payload.class_id);

    match class_student {
        Ok(_) => Ok(Html("Added student to class")),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[derive(Deserialize)]
struct CreateGrade {
    pub value: f64,
    pub weight: i32,
    pub teacher_id: Uuid,
    pub student_id: Uuid,
    pub subject_id: Uuid,
    pub task_id: Uuid,
}

async fn post_create_grade(
    extract::Json(payload): extract::Json<CreateGrade>,
    pool: Extension<PgPool>,
) -> Result<Html<&'static str>, StatusCode> {
    let mut conn = pool.get().map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?;

    let grade = administration::create_grade(
        &mut conn,
        payload.value,
        payload.weight,
        payload.teacher_id,
        payload.student_id,
        payload.subject_id,
        payload.task_id);

    match grade {
        Ok(_) => Ok(Html("Grade created")),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[derive(Deserialize)]
struct CreateTask {
    pub name: String,
}

async fn post_create_task(
    extract::Json(payload): extract::Json<CreateTask>,
    pool: Extension<PgPool>,
) -> Result<Html<&'static str>, StatusCode> {
    let mut conn = pool.get().map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?;

    let task = administration::create_task(&mut conn, &payload.name);

    match task {
        Ok(_) => Ok(Html("Task created")),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::Client;
    use std::net::TcpListener;

    async fn spawn_app() -> SocketAddr {
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

    #[tokio::test]
    async fn register_and_login() {
        let addr = spawn_app().await;
        let client = Client::new();

        let payload = AuthUser {
            login: format!("test_user_{}", Uuid::new_v4().to_string()),
            email: format!("{}@gmail.com", Uuid::new_v4().to_string()),
            password: "strong_pass12345".into(),
        };

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
}
