CREATE TABLE cultures
(
    id              bigserial NOT NULL
        CONSTRAINT cultures_pk
            PRIMARY KEY,
    uid             uuid      NOT NULL UNIQUE,
    name            varchar   NOT NULL,
    scientific_name varchar   NOT NULL,
    description     varchar,
    create_date     timestamp DEFAULT NOW()
);

CREATE INDEX cultures_uid_idx
    ON cultures (uid);