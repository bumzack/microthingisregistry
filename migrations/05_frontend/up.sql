CREATE TABLE frontend
(
    id              int          NOT NULL AUTO_INCREMENT,
    url             TEXT         NOT NULL,
    version_major   INT          NOT NULL DEFAULT 0,
    version_minor   INT          NOT NULL DEFAULT 0,
    version_patch   INT          NOT NULL DEFAULT 0,
    service_url     TEXT         NOT NULL,
    local_repo_path VARCHAR(255) NOT NULL,
    host_id         int,
    microservice_id VARCHAR(255) NOT NULL UNIQUE,
    technology_id   int          NOT NULL,

    FOREIGN KEY (host_id) REFERENCES host (id),
    FOREIGN KEY (microservice_id) REFERENCES microservice (microservice_id),
    FOREIGN KEY (technology_id) REFERENCES technology (id),

    PRIMARY KEY (ID)
);
