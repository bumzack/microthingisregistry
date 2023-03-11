CREATE TABLE frontend
(
    id              int          NOT NULL AUTO_INCREMENT,
    url             TEXT         NOT NULL,
    version_major   INT          NOT NULL,
    version_minor   INT          NOT NULL,
    version_patch   INT          NOT NULL,
    service_url     TEXT         NOT NULL,
    openapi_url     TEXT         NOT NULL,
    local_repo_path VARCHAR(255) NOT NULL,
    host_id         int,
    service_id      VARCHAR(255) NOT NULL,
    technology_id   int          NOT NULL,

    FOREIGN KEY (host_id) REFERENCES host (id),
    FOREIGN KEY (service_id) REFERENCES service (service_id),
    FOREIGN KEY (technology_id) REFERENCES technology (id),

    PRIMARY KEY (ID)
);
