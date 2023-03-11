CREATE TABLE backend
(
    id              int          NOT NULL AUTO_INCREMENT,
    openapiclient   TEXT,
    service_url     TEXT         NOT NULL,
    openapi_url     TEXT         NOT NULL,
    local_repo_path VARCHAR(255) NOT NULL,
    host_id         int,
    microservice_id VARCHAR(255) NOT NULL UNIQUE,
    technology_id   int          NOT NULL,
    publish_as_frontend_package   BOOLEAN          NOT NULL  DEFAULT  false,
    FOREIGN KEY (host_id) REFERENCES host (id),
    FOREIGN KEY (microservice_id) REFERENCES microservice (microservice_id),
    FOREIGN KEY (technology_id) REFERENCES technology (id),
    PRIMARY KEY (ID)
);
