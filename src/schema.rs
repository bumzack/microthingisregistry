// @generated automatically by Diesel CLI.

diesel::table! {
    backend (id) {
        id -> Integer,
        openapiclient -> Nullable<Text>,
        service_url -> Text,
        openapi_url -> Text,
        local_repo_path -> Varchar,
        host_id -> Nullable<Integer>,
        microservice_id -> Varchar,
        technology_id -> Integer,
        publish_as_frontend_package -> Bool,
        api_client_prefix -> Varchar,
        api_client_package -> Varchar,
        version_major -> Integer,
        version_minor -> Integer,
        version_patch -> Integer,

    }
}

diesel::table! {
    frontend (id) {
        id -> Integer,
        url -> Text,
        version_major -> Integer,
        version_minor -> Integer,
        version_patch -> Integer,
        service_url -> Text,
        local_repo_path -> Varchar,
        host_id -> Nullable<Integer>,
        microservice_id -> Varchar,
        technology_id -> Integer,
    }
}

diesel::table! {
    host (id) {
        id -> Integer,
        hostname -> Varchar,
        ip -> Varchar,
        port -> Integer,
    }
}

diesel::table! {
    microservice (id) {
        id -> Integer,
        microservice_id -> Varchar,
    }
}

diesel::table! {
    technology (id) {
        id -> Integer,
        name -> Varchar,
    }
}

diesel::joinable!(backend -> host (host_id));
diesel::joinable!(backend -> technology (technology_id));
diesel::joinable!(frontend -> host (host_id));
diesel::joinable!(frontend -> technology (technology_id));

diesel::allow_tables_to_appear_in_same_query!(backend, frontend, host, microservice, technology,);
