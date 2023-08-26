CREATE TABLE plantations
(
    id          uuid NOT NULL
        CONSTRAINT plantation_pk
            PRIMARY KEY,
    user_id     uuid      NOT NULL,
    culture_id  bigint    NOT NULL,
    station_id  bigint    NULL,
    alias       varchar   NULL,
    location    point     NOT NULL,
    area        float8    NOT NULL,
    create_date timestamp NOT NULL DEFAULT NOW(),
    update_date timestamp DEFAULT NOW()
);

CREATE INDEX plantations_culture_id_idx
    ON plantations (culture_id);

CREATE INDEX plantations_station_id_idx
    ON plantations (station_id);

CREATE INDEX plantations_user_id_idx
    ON plantations (user_id);


CREATE TABLE plantation_pathogenic_occurrences
(
    id              uuid NOT NULL
        CONSTRAINT plantation_pathogenic_occurrences_pk
            PRIMARY KEY,
    user_id         uuid    NOT NULL,
    plantation_id   uuid    NOT NULL,
    pathogenic_id   bigint    NOT NULL,
    image           varchar   NULL,
    occurrence_date timestamp NOT NULL,
    temperature     float8    NULL,
    humidity        float8    NULL,
    create_date     timestamp NOT NULL DEFAULT NOW(),
    update_date     timestamp DEFAULT NOW()
);

CREATE INDEX plantation_pathogenic_occurrences_plantation_id_idx
    ON plantation_pathogenic_occurrences (plantation_id);

CREATE INDEX plantation_pathogenic_occurrences_pathogenic_id_idx
    ON plantation_pathogenic_occurrences (pathogenic_id);
    
CREATE INDEX plantation_pathogenic_occurrences_user_id_idx
    ON plantation_pathogenic_occurrences (user_id);
