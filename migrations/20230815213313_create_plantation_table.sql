CREATE TABLE plantations
(
    id          bigserial NOT NULL
        CONSTRAINT plantation_pk
            PRIMARY KEY,
    uid         uuid      NOT NULL UNIQUE,
    user_id     bigint    NOT NULL,
    culture_id  bigint    NOT NULL,
    station_id  bigint    NULL,
    alias       varchar   NULL,
    location    point     NOT NULL,
    area        float8    NOT NULL,
    create_date timestamp DEFAULT NOW(),
    update_date timestamp DEFAULT NOW()
);

CREATE INDEX plantations_culture_id_idx
    ON plantations (culture_id);

CREATE INDEX plantations_station_id_idx
    ON plantations (station_id);

CREATE INDEX plantations_user_id_idx
    ON plantations (user_id);

CREATE INDEX plantations_uid_idx
    ON plantations (uid);


CREATE TABLE plantation_pathogenic_occurrences
(
    id              bigserial NOT NULL
        CONSTRAINT plantation_pathogenic_occurrences_pk
            PRIMARY KEY,
    uid             uuid      NOT NULL UNIQUE,
    user_id         bigint    NOT NULL,
    plantation_id   bigint    NOT NULL,
    pathogenic_id   bigint    NOT NULL,
    image           varchar   NULL,
    occurrence_date timestamp NOT NULL,
    temperature     float8    NULL,
    humidity        float8    NULL,
    create_date     timestamp DEFAULT NOW(),
    update_date     timestamp DEFAULT NOW()
);

CREATE INDEX plantation_pathogenic_occurrences_plantation_id_idx
    ON plantation_pathogenic_occurrences (plantation_id);

CREATE INDEX plantation_pathogenic_occurrences_pathogenic_id_idx
    ON plantation_pathogenic_occurrences (pathogenic_id);

CREATE INDEX plantation_pathogenic_occurrences_uid_idx
    ON plantation_pathogenic_occurrences (uid);
    
CREATE INDEX plantation_pathogenic_occurrences_user_id_idx
    ON plantation_pathogenic_occurrences (user_id);