create view item_view as
select
    i.id,
    i.serial,
    i.status,
    b.name as brand_name,
    m.name as model_name,
    p.name as project_name,
    p.city as project_city
from items i
join models m on m.id = i.model_id
join brands b on b.id = m.brand_id
join projects p on p.id = i.project_id;
