pub mod rest_models {
    use serde::Deserialize;
    use serde::Serialize;

    /// An API error serializable to JSON.
    #[derive(Serialize, Debug)]
    pub struct ErrorMessage {
        pub(crate) code: u16,
        pub(crate) message: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct NewHostPost {
        pub hostname: String,
        pub ip: String,
        pub port: i32,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct NewTechnologyPost {
        pub name: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct NewBackendPost {
        pub service_url: String,
        pub openapi_url: String,
        pub local_repo_path: String,
        pub microservice_id: String,
        pub technology_id: i32,
        pub publish_as_frontend_package: bool,
        pub api_client_prefix: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct UpdateBackendOpenApiPut {
        pub openapiclient: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct NewFrontendPost {
        pub url: String,
        pub service_url: String,
        pub local_repo_path: String,
        pub microservice_id: String,
        pub technology_id: i32,
    }
}
