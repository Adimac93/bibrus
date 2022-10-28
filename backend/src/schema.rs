// @generated automatically by Diesel CLI.

diesel::table! {
    class_students (class_id, student_id) {
        class_id -> Uuid,
        student_id -> Uuid,
    }
}

diesel::table! {
    classes (id) {
        id -> Uuid,
        subject_id -> Nullable<Uuid>,
        group_id -> Nullable<Uuid>,
        teacher_id -> Uuid,
    }
}

diesel::table! {
    grades (student_id, subject_id, task_id) {
        value -> Nullable<Float8>,
        weight -> Nullable<Int4>,
        task_id -> Uuid,
        student_id -> Uuid,
        subject_id -> Uuid,
        teacher_id -> Uuid,
    }
}

diesel::table! {
    groups (id) {
        id -> Uuid,
        name -> Varchar,
        school_id -> Uuid,
    }
}

diesel::table! {
    schools (id) {
        id -> Uuid,
        name -> Varchar,
        place -> Varchar,
        school_type -> Nullable<Varchar>,
    }
}

diesel::table! {
    sessions (id) {
        id -> Uuid,
        iat -> Timestamp,
        user_id -> Uuid,
    }
}

diesel::table! {
    students (id) {
        id -> Uuid,
        first_name -> Varchar,
        last_name -> Varchar,
        date_of_birth -> Date,
        user_id -> Nullable<Uuid>,
        group_id -> Uuid,
        school_id -> Uuid,
    }
}

diesel::table! {
    subjects (id) {
        id -> Uuid,
        name -> Varchar,
        school_id -> Uuid,
    }
}

diesel::table! {
    tasks (id) {
        id -> Uuid,
        name -> Nullable<Varchar>,
    }
}

diesel::table! {
    teachers (id) {
        id -> Uuid,
        first_name -> Varchar,
        last_name -> Varchar,
        user_id -> Uuid,
        school_id -> Uuid,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        login -> Varchar,
        email -> Varchar,
        password -> Varchar,
    }
}

diesel::joinable!(class_students -> classes (class_id));
diesel::joinable!(class_students -> students (student_id));
diesel::joinable!(classes -> groups (group_id));
diesel::joinable!(classes -> subjects (subject_id));
diesel::joinable!(classes -> teachers (teacher_id));
diesel::joinable!(grades -> students (student_id));
diesel::joinable!(grades -> subjects (subject_id));
diesel::joinable!(grades -> tasks (task_id));
diesel::joinable!(grades -> teachers (teacher_id));
diesel::joinable!(groups -> schools (school_id));
diesel::joinable!(sessions -> users (user_id));
diesel::joinable!(students -> groups (group_id));
diesel::joinable!(students -> schools (school_id));
diesel::joinable!(students -> users (user_id));
diesel::joinable!(subjects -> schools (school_id));
diesel::joinable!(teachers -> schools (school_id));
diesel::joinable!(teachers -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    class_students,
    classes,
    grades,
    groups,
    schools,
    sessions,
    students,
    subjects,
    tasks,
    teachers,
    users,
);
