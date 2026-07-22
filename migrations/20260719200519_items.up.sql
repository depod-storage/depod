create type item_status as enum (
    'Active',
    'Inactive',
    'Broken',
    'Service',
    'Rental'
);

create table items (
    id uuid primary key default gen_random_uuid(),

    model_id uuid not null references models(id) on delete restrict,
    project_id uuid not null references projects(id) on delete restrict,

    serial text not null unique check (length(trim(serial)) > 0),

    status item_status not null default 'Active'
);

create index items_model_idx on items(model_id);
create index items_project_idx on items(project_id);
