CREATE TABLE cultures
(
    id              bigserial NOT NULL
        CONSTRAINT cultures_pk
            PRIMARY KEY,
    name            varchar   NOT NULL,
    scientific_name varchar   NOT NULL,
    description     varchar,
    create_date     timestamp NOT NULL DEFAULT NOW()
);