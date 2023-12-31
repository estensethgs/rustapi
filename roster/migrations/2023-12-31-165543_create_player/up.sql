-- Your SQL goes here
create table players (
    id serial primary key,
    name text not null
);

create table alts (
    id serial primary key,
    name text not null,
    player_id int not null references players(id)
);



