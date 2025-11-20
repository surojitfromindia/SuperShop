-- Add migration script here
insert into _counter_public_id_branch
    (organization_id, last_value, organization_public_id)
values (1, 0, '202511000001');



insert into organization_branches (name, name_sl, organization_id, created_by_user_id, is_default)
values ('Head office', 'Head office', 1, 1, true);


insert into _counter_public_id_org_staff (organization_id, last_value, organization_public_id)
values (1, 1, '202511000001')