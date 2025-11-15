-- Add migration script here
CREATE TYPE branch_status AS ENUM ('active', 'deleted', 'suspended');


create table branch_public_counters
(
    organization_id        BIGINT       not null references organizations (id),
    last_value             BIGINT       not null default 0,
    organization_public_id varchar(255) not null,
    primary key (organization_id)
);

create table organization_branches
(
    id                 serial                               not null,
    public_id          text                                 not null,
    name               varchar(255)                         not null,
    name_sl            varchar(255)                         null,
    organization_id    bigint references organizations (id) not null,
    created_by_user_id BIGINT                               NOT NULL REFERENCES users (id),

    status             branch_status                        NOT NULL DEFAULT 'active',

    created_at         TIMESTAMPTZ                          NOT NULL DEFAULT NOW(),
    updated_at         TIMESTAMPTZ                          NOT NULL DEFAULT NOW()
);


create or replace function gen_branch_public_id(org_id BIGINT) RETURNS TEXT
AS
$$
DECLARE
    prefix   TEXT;
    next_num BIGINT;
begin
    update branch_public_counters
    set last_value= last_value + 1
    where organization_id = org_id
    returning organization_public_id, last_value
        into prefix, next_num;
    return prefix || lpad(next_num::text, 3, '0');
end;
$$ language plpgsql;


create or replace function set_branch_public_id() returns trigger as
$$
begin
    if new.public_id is null then
        new.public_id := gen_branch_public_id(new.organiztion_id);
    end if;
    return new;
end;
$$ language plpgsql;


-- set the public id generate trigger.
create trigger trg_set_branch_public_id
    before insert
    on organization_branches
    for each row
execute function set_branch_public_id()



