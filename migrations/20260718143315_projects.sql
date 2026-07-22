create table projects (
    id uuid primary key default gen_random_uuid(),
    name text not null check (length(trim(name)) > 0),
    city text not null check (length(trim(city)) > 0),
    created_at timestamptz not null default now(),
    constraint unique_project_in_city unique(name, city)
);
