-- Add migration script here

ALTER TABLE organization_staffs
    ADD COLUMN organization_id BIGINT REFERENCES organizations(id),
    ADD COLUMN public_id VARCHAR(255);