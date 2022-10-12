-- Add migration script here
create extension if not exists "uuid-ossp";

create table users (
    id uuid default uuid_generate_v4() primary key,
    fname varchar(255) not null,
    lname varchar(255) not null,
    email varchar(255) not null unique,
    password varchar(255) not null,
    image varchar(255) null,
    phone_number varchar(255) null
);