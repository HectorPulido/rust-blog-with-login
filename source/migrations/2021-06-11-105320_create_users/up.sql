-- Your SQL goes here
create table users (
    id SERIAL PRIMARY KEY,
    username varchar not null unique,
    email varchar not null unique,
    password varchar not null,
    is_admin BOOLEAN NOT NULL DEFAULT 'f',
    created_at timestamp not null default current_timestamp,
    updated_at timestamp not null default current_timestamp
);