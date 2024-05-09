CREATE TABLE projects (
    id   VARCHAR(6) NOT NULL PRIMARY KEY,
    name TEXT       NOT NULL UNIQUE
);

CREATE TABLE links (
    id         VARCHAR(6) NOT NULL PRIMARY KEY,
    project_id VARCHAR(6) NOT NULL,
    url        TEXT       NOT NULL
);
