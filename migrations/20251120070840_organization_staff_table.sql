-- Add migration script here
create type organization_staff_status as enum('active','deleted','suspended');

create table org_staff_public_counters
(
    organization_id        BIGINT       not null references organizations (id),
    last_value             BIGINT       not null default 0,
    organization_public_id varchar(255) not null,
    primary key (organization_id)
);

create table organization_staffs(
    id bigserial primary key ,
    name varchar(255) not null ,
    name_sl varchar(255),
    status organization_staff_status not null  default  'active',
    user_id bigint not null references users(id),
    created_at         timestamptz         NOT NULL DEFAULT NOW(),
    updated_at         timestamptz         NOT NULL DEFAULT NOW()

);

create or replace function gen_org_staff_public_id(org_id BIGINT) RETURNS TEXT
AS
$$
DECLARE
    prefix   TEXT;
    next_num BIGINT;
begin
    update org_staff_public_counters
    set last_value= last_value + 1
    where organization_id = org_id
    returning organization_public_id, last_value
        into prefix, next_num;
    return prefix || lpad(next_num::text, 5, '0');
end;
$$ language plpgsql;



create or replace function set_org_staff_public_id() returns trigger as
$$
begin
    if new.public_id is null then
        new.public_id := gen_org_staff_public_id(new.organiztion_id);
    end if;
    return new;
end;
$$ language plpgsql;


create trigger trg_set_org_staff_public_id
    before insert
    on organization_staffs
    for each row
execute function set_org_staff_public_id();


CREATE TRIGGER organization_staffs_set_updated_at
    BEFORE UPDATE
    ON organization_staffs
    FOR EACH ROW
EXECUTE FUNCTION set_updated_at();