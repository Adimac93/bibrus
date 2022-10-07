// @generated automatically by Diesel CLI.

diesel::table! {
    schools (id) {
        id -> Uuid,
        name -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        login -> Varchar,
        password -> Varchar,
        schoolid -> Nullable<Uuid>,
    }
}

diesel::joinable!(users -> schools (schoolid));

diesel::allow_tables_to_appear_in_same_query!(
    schools,
    users,
);
