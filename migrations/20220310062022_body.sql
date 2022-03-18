-- Add migration script here
drop table if exists body;

create table body 
(
    id serial primary key,
    device_id INT not null,
    content varchar(1024) not null,
    time TIMESTAMP default now()
);

insert into body
(id,device_id,content,time)
values
(1,1,'First course','2022-01-17 05:40:00');

insert into body
(id,device_id,content,time)
values
(2,1,'Second course','2022-01-17 05:40:00');