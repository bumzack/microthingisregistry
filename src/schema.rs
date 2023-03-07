// @generated automatically by Diesel CLI.

diesel::table! {
    backend (id) {
        id -> Integer,
        backendid -> Varchar,
        openapiclient -> Nullable<Text>,
    }
}

diesel::table! {
    frontend (id) {
        id -> Integer,
        frontendid -> Varchar,
        url -> Text,
        version_major -> Integer,
        version_minor -> Integer,
        verion_patch -> Integer,
    }
}

diesel::table! {
    technology (id) {
        id -> Integer,
        name -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    backend,
    frontend,
    technology,
);
