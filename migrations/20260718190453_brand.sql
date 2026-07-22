create table brands (
    id uuid primary key default gen_random_uuid(),
    name text not null unique,
    constraint brands_name_not_empty check (length(trim(name)) > 0)
);
