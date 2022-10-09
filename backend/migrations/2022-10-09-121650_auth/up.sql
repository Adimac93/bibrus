create table users
(
    id uuid not null default gen_random_uuid() primary key,
    login varchar not null unique,
    email varchar not null unique,
    password varchar not null
);

create table sessions
(
    id uuid not null default gen_random_uuid() primary key,
    iat timestamp not null default now(),
    userID uuid not null,
    foreign key (userID) references users(id)
);