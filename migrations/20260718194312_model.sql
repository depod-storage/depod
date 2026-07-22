create type model_type as enum (
    'GPS',
    'TotalStation',
    'HandUnit',
    'Nivo'
);

create table models (
    id uuid primary key default gen_random_uuid(),
    brand_id uuid not null references brands(id) on delete restrict,
    name text not null check (length(trim(name)) > 0),
    ty model_type not null,
    constraint unique_model_in_brand unique(brand_id, name)
);

create index models_brand_idx on models(brand_id);
