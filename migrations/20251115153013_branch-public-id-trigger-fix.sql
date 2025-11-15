-- Add migration script here
create or replace function set_branch_public_id() returns trigger as
$$
begin
    if new.public_id is null then
        new.public_id := gen_branch_public_id(new.organization_id);
    end if;
    return new;
end;
$$ language plpgsql;