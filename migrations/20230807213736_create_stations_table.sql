CREATE TABLE stations
(
    id          bigserial    NOT NULL
        CONSTRAINT stations_pk
            PRIMARY KEY,
    city        varchar(255) NOT NULL,
    uf          varchar(2)   NOT NULL,
    location    point        NOT NULL,
    status      bool         NOT NULL DEFAULT TRUE,
    inmet_code  varchar(6)   NULL,
    create_date timestamp    NOT NULL DEFAULT NOW(),
    update_date timestamp    NOT NULL DEFAULT NOW()
);

CREATE INDEX stations_inmet_code_idx
    ON stations (inmet_code);