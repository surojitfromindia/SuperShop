-- Add migration script here
alter table role_and_permission add column organization_id bigint not null  references organizations(id);