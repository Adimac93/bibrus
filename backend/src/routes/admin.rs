use crate::{administration, database::PgPool};
use axum::{extract, http::StatusCode, response::Html, routing::post, Extension, Router};
use serde::Deserialize;
use time::Date;
use uuid::Uuid;

pub fn router() -> Router {
    Router::new()
        .route("/school", post(post_create_school))
        .route("/student", post(post_create_student))
        .route("/teacher", post(post_create_teacher))
        .route("/subject", post(post_create_subject))
        .route("/group", post(post_create_group))
        .route("/class", post(post_create_class))
        .route("/class-student", post(post_create_class_student))
        .route("/grade", post(post_create_grade))
        .route("/task", post(post_create_task))
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
        payload.school_type.as_deref(),
    );

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
        payload.user_id,
    );

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
        payload.school_id,
    );

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

    let subject = administration::create_subject(&mut conn, &payload.name, payload.school_id);

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

    let group = administration::create_group(&mut conn, &payload.name, payload.school_id);

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
        payload.teacher_id,
    );

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

    let class_student =
        administration::add_student_to_class(&mut conn, payload.student_id, payload.class_id);

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
        payload.task_id,
    );

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
