create table users (
    id serial constraint users_pk primary key,
    uid uuid not null,
    name varchar(255) not null,
    password varchar(255) not null,
    email varchar(255) not null,
    born_date timestamp default now() not null,
    create_date timestamp default now() not null
);

create index users_uid_idx
    on users (uid);