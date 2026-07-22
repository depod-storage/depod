create table app_config (
    id int primary key,

    bootstrap boolean not null default false,

    check (id = 1)
);

insert into app_config (id) values (1);
