create extension if not exists pgcrypto;

create table users (
    id uuid primary key default gen_random_uuid(),
    username text not null unique,
    password_hash text not null
);
