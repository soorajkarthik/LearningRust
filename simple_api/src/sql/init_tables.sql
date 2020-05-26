create table if not exists book (
    id serial primary key,
    title varchar not null,
    authorId int not null check (authorId > 0),
    published boolean not null default false
);

create table if not exists author (
    id serial primary key,
    name varchar not null,
    country varchar not null
);