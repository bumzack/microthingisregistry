CREATE TABLE frontend
(
    id            int                 NOT NULL AUTO_INCREMENT,
    frontendid    VARCHAR(255) UNIQUE NOT NULL,
    url           TEXT                NOT NULL,
    version_major INT                 NOT NULL,
    version_minor INT                 NOT NULL,
    verion_patch  INT                 NOT NULL,
    PRIMARY KEY (ID)
)