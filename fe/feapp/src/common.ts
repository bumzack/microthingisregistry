export interface Backend {
    id: number,
    openapiclient?: string,
    service_url: String,
    openapi_url: String,
    local_repo_path: string,
    host_id?: number,
    microservice_id: string,
    technology_id: number,
    publish_as_frontend_package: boolean,
    api_client_prefix: string,
    api_client_package: string,
    version_major: number,
    version_minor: number,
    version_patch: number,
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