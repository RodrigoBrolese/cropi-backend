CREATE TABLE pathogenics
(
    id              bigserial NOT NULL
        CONSTRAINT pathogenic_pk
            PRIMARY KEY,

    uid             uuid      NOT NULL UNIQUE,
    name            varchar   NOT NULL,
    scientific_name varchar   NOT NULL,
    description     varchar,
    create_date     timestamp DEFAULT NOW()
);

CREATE INDEX pathogenics_uid_idx
    ON pathogenics (uid);

CREATE TABLE pathogenic_cultures
(
    id            bigserial NOT NULL
        CONSTRAINT pathogenic_cultures_pk
            PRIMARY KEY,
    uid           uuid      NOT NULL UNIQUE,
    pathogenic_id bigint    NOT NULL,
    culture_id    bigint
                            NOT NULL,
    create_date   timestamp DEFAULT NOW()
);

CREATE INDEX pathogenic_cultures_uid_idx
    ON pathogenic_cultures (uid);

CREATE INDEX pathogenic_cultures_pathogenic_id_idx
    ON pathogenic_cultures (pathogenic_id);

CREATE INDEX pathogenic_cultures_culture_id_idx
    ON pathogenic_cultures (culture_id);