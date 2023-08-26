create table users (
    id uuid constraint users_pk primary key,
    name varchar(255) not null,
    password varchar(255) not null,
    email varchar(255) not null,
    born_date timestamp default now() not null,
    create_date timestamp default now() not null
);
