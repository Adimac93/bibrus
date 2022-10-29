use self::schema::{
    class_students::dsl::*, classes::dsl::*, grades::dsl::*, groups::dsl::*, schools::dsl::*,
    students::dsl::*, subjects::dsl::*, tasks::dsl::*, teachers::dsl::*,
};
use crate::schema::{
    class_students, classes, grades, groups, schools, students, subjects, tasks, teachers,
};
use crate::{
    models::{Class, ClassStudent, Grade, Group, School, Student, Subject, Task, Teacher},
    schema,
};
use anyhow::{Context, Result};
use diesel::{delete, insert_into, update};
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, PooledConnection},
};
use thiserror::Error;
use time::{Date, Duration};
use uuid::Uuid;

pub type PgConn = PooledConnection<ConnectionManager<PgConnection>>;

pub fn create_school(
    conn: &mut PgConn,
    school_name: &str,
    school_place: &str,
    s_type: Option<&str>,
) -> Result<School, diesel::result::Error> {
    insert_into(schools)
        .values((
            schools::name.eq(school_name),
            schools::place.eq(school_place),
            schools::school_type.eq(s_type),
        ))
        .get_result::<School>(conn)
}

pub fn create_student(
    conn: &mut PgConn,
    student_first_name: &str,
    student_last_name: &str,
    student_date_of_birth: Date,
    student_school_id: Uuid,
    student_group_id: Uuid,
    student_user_id: Option<Uuid>,
) -> Result<Student, diesel::result::Error> {
    insert_into(students)
        .values((
            students::first_name.eq(student_first_name),
            students::last_name.eq(student_last_name),
            date_of_birth.eq(student_date_of_birth),
            students::school_id.eq(student_school_id),
            students::group_id.eq(student_group_id),
            students::user_id.eq(student_user_id),
        ))
        .get_result::<Student>(conn)
}

pub fn create_teacher(
    conn: &mut PgConn,
    teacher_first_name: &str,
    teacher_last_name: &str,
    teacher_user_id: Uuid,
    teacher_school_id: Uuid,
) -> Result<Teacher, diesel::result::Error> {
    insert_into(teachers)
        .values((
            teachers::first_name.eq(teacher_first_name),
            teachers::last_name.eq(teacher_last_name),
            teachers::school_id.eq(teacher_school_id),
            teachers::user_id.eq(teacher_user_id),
        ))
        .get_result::<Teacher>(conn)
}
pub fn create_subject(
    conn: &mut PgConn,
    subject_name: &str,
    school_uuid: Uuid,
) -> Result<Subject, diesel::result::Error> {
    insert_into(subjects)
        .values((
            subjects::name.eq(subject_name),
            subjects::school_id.eq(school_uuid),
        ))
        .get_result::<Subject>(conn)
}

pub fn create_group(
    conn: &mut PgConn,
    group_name: &str,
    school_uuid: Uuid,
) -> Result<Group, diesel::result::Error> {
    insert_into(groups)
        .values((
            groups::name.eq(group_name),
            groups::school_id.eq(school_uuid),
        ))
        .get_result::<Group>(conn)
}

pub fn create_class(
    conn: &mut PgConn,
    class_subject_id: Uuid,
    class_group_id: Uuid,
    class_teacher_id: Uuid,
) -> Result<Class, diesel::result::Error> {
    insert_into(classes)
        .values((
            classes::subject_id.eq(class_subject_id),
            classes::group_id.eq(class_group_id),
            classes::teacher_id.eq(class_teacher_id),
        ))
        .get_result::<Class>(conn)
}

pub fn add_student_to_class(
    conn: &mut PgConn,
    student_uuid: Uuid,
    class_uuid: Uuid,
) -> Result<ClassStudent, diesel::result::Error> {
    insert_into(class_students)
        .values((
            class_students::student_id.eq(student_uuid),
            class_students::class_id.eq(class_uuid),
        ))
        .get_result::<ClassStudent>(conn)
}

pub fn create_grade(
    conn: &mut PgConn,
    grade_value: f64,
    grade_weight: i32,
    grade_teacher_id: Uuid,
    grade_student_id: Uuid,
    grade_subject_id: Uuid,
    grade_task_id: Uuid,
) -> Result<Grade, diesel::result::Error> {
    insert_into(grades)
        .values((
            grades::value.eq(grade_value),
            grades::weight.eq(grade_weight),
            grades::teacher_id.eq(grade_teacher_id),
            grades::student_id.eq(grade_student_id),
            grades::subject_id.eq(grade_subject_id),
            grades::task_id.eq(grade_task_id),
        ))
        .get_result::<Grade>(conn)
}

pub fn create_task(conn: &mut PgConn, task_name: &str) -> Result<Task, diesel::result::Error> {
    insert_into(tasks)
        .values(tasks::name.eq(task_name))
        .get_result::<Task>(conn)
}
