CREATE TABLE backend
(
    id            int                 NOT NULL AUTO_INCREMENT,
    backendid     VARCHAR(255) UNIQUE NOT NULL,
    openapiclient TEXT,
    PRIMARY KEY (ID)
)