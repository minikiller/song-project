-- Add migration script here
drop table if exists tb_card;

create table tb_card 
(
    id serial primary key,
    card varchar(18) not null,
    time TIMESTAMP default now()
);