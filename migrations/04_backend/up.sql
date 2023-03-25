CREATE TABLE backend
(
    id                          int          NOT NULL AUTO_INCREMENT,
    openapiclient               TEXT,
    service_url                 TEXT         NOT NULL,
    openapi_url                 TEXT         NOT NULL,
    local_repo_path             VARCHAR(255) NOT NULL,
    host_id                     int,
    microservice_id             VARCHAR(255) NOT NULL UNIQUE,
    technology_id               int          NOT NULL,
    publish_as_frontend_package BOOLEAN      NOT NULL DEFAULT false,
    api_client_prefix           VARCHAR(255) NOT NULL UNIQUE,
    api_client_package          VARCHAR(255) NOT NULL UNIQUE,

    version_major   INT          NOT NULL DEFAULT  0,
    version_minor   INT          NOT NULL DEFAULT  0,
    version_patch   INT          NOT NULL DEFAULT  0,

    FOREIGN KEY (host_id) REFERENCES host (id),
    FOREIGN KEY (microservice_id) REFERENCES microservice (microservice_id),
    FOREIGN KEY (technology_id) REFERENCES technology (id),
    PRIMARY KEY (ID)
);
