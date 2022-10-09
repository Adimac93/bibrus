// @generated automatically by Diesel CLI.

diesel::table! {
    sessions (id) {
        id -> Uuid,
        iat -> Timestamp,
        userid -> Uuid,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        login -> Varchar,
        password -> Varchar,
    }
}

diesel::joinable!(sessions -> users (userid));

diesel::allow_tables_to_appear_in_same_query!(
    sessions,
    users,
);
