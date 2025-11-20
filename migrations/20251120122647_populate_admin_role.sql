-- Add migration script here


alter table organization_branches
    add column is_default bool default false;


-- create prime org
insert into organizations (name, name_sl, created_by_user_id)
VALUES ('Prime', 'Prime', 1);


-- create the counter tables
insert into _counter_public_id_role (organization_id, last_value, organization_public_id)
VALUES (1,
        0,
        '202511000001');

-- insert role from prime.
insert into role_and_permission (name, name_sl, created_by_user_id, organization_id, can_edit, permissions)
values ('Admin', 'Admin', 1, 1, false, '{
  "branch_create": true,
  "branch_update": true,
  "branch_delete": true
}'::jsonb);