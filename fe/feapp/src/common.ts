export interface Backend {
    pub id: number,
    pub openapiclient?: string,
    pub service_url: String,
    pub openapi_url: String,
    pub local_repo_path: string,
    pub host_id?: number, 
    pub microservice_id: string,
    pub technology_id: number,
    pub publish_as_frontend_package: boolean,
    pub api_client_prefix: string,
    pub api_client_package: string,
    pub version_major: number,
    pub version_minor: number,
    pub version_patch: number,
}

export interface Frontend {
     id: number,
     url: string,
     version_major: number,
     version_minor: number,
     version_patch: number,
     service_url: string,
     local_repo_path: string,
     host_id?: number, 
     microservice_id: string,
     technology_id: number,
}