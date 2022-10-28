use crate::schema::{
    class_students, classes, grades, groups, schools, sessions, students, subjects, tasks,
    teachers, users,
};
use diesel::prelude::*;
use time::Date;
use uuid::Uuid;

#[derive(Queryable, Debug, PartialEq, Eq)]
pub struct User {
    pub id: Uuid,
    pub login: String,
    pub email: String,
    pub password: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub login: &'a str,
    pub email: &'a str,
    pub password: &'a str,
}

#[derive(Queryable)]
pub struct Session {
    pub id: Uuid,
    pub iat: std::time::SystemTime,
    pub user_id: Uuid,
}

#[derive(Insertable)]
#[diesel(table_name = sessions)]
pub struct NewSession {
    pub user_id: Uuid,
}

#[derive(Queryable)]
pub struct School {
    pub id: Uuid,
    pub name: String,
    pub place: String,
    pub school_type: String,
}

#[derive(Insertable)]
#[diesel(table_name = schools)]
pub struct NewSchool<'a> {
    pub name: &'a str,
    pub place: &'a str,
}

#[derive(Queryable)]
pub struct Group {
    pub id: Uuid,
    pub name: String,
    pub school_id: Uuid,
}

#[derive(Insertable)]
#[diesel(table_name = groups)]
pub struct NewGroup<'a> {
    pub name: &'a str,
    pub school_id: Uuid,
}

#[derive(Queryable)]
pub struct Subject {
    pub id: Uuid,
    pub name: String,
    pub school_id: Uuid,
}

#[derive(Insertable)]
#[diesel(table_name = subjects)]
pub struct NewSubject<'a> {
    pub name: &'a str,
    pub school_id: Uuid,
}

#[derive(Queryable)]
pub struct Student {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: Date,
    pub user_id: Uuid,
    pub group_id: Uuid,
    pub school_id: Uuid,
}

#[derive(Insertable)]
#[diesel(table_name = students)]
pub struct NewStudent<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub date_of_birth: Date,
    pub user_id: Uuid,
    pub group_id: Uuid,
    pub school_id: Uuid,
}

#[derive(Queryable)]
pub struct Teacher {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub user_id: Uuid,
    pub school_id: Uuid,
}

#[derive(Insertable)]
#[diesel(table_name = teachers)]
pub struct NewTeacher<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub user_id: Uuid,
    pub school_id: Uuid,
}

#[derive(Queryable)]
pub struct Class {
    pub id: Uuid,
    pub subject_id: Uuid,
    pub group_id: Uuid,
    pub teacher_id: Uuid,
}

#[derive(Insertable)]
#[diesel(table_name = classes)]
pub struct NewClass {
    pub subject_id: Uuid,
    pub group_id: Uuid,
    pub teacher_id: Uuid,
}

#[derive(Queryable, Identifiable)]
#[diesel(primary_key(class_id, student_id))]
pub struct ClassStudent {
    pub class_id: Uuid,
    pub student_id: Uuid,
}

#[derive(Insertable)]
#[diesel(table_name = class_students)]
pub struct NewClassStudent {
    pub class_id: Uuid,
    pub student_id: Uuid,
}

#[derive(Queryable)]
pub struct Task {
    pub id: Uuid,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = tasks)]
pub struct NewTask<'a> {
    pub name: &'a str,
}

#[derive(Queryable, Identifiable)]
#[diesel(primary_key(student_id, subject_id, task_id))]
pub struct Grade {
    pub value: f64,
    pub weight: i64,
    pub task_id: Uuid,
    pub student_id: Uuid,
    pub subject_id: Uuid,
    pub teacher_id: Uuid,
}
