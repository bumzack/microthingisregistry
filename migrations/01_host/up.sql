CREATE TABLE host
(
    id       int          NOT NULL AUTO_INCREMENT,
    hostname VARCHAR(255) NOT NULL,
    ip       VARCHAR(255) NOT NULL,
    port     INT          NOT NULL,
    PRIMARY KEY (ID)
);
