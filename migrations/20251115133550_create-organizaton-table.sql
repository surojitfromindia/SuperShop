-- Add migration script here
CREATE TYPE organization_status AS ENUM ('active', 'deleted', 'suspended');
CREATE SEQUENCE organization_public_seq;

-- create a function that create public facing id
CREATE
    OR REPLACE FUNCTION gen_org_public_id()
    RETURNS text AS
$$
DECLARE
    seq_val bigint;
BEGIN
    seq_val
        := nextval('organization_public_seq');
    RETURN to_char(NOW(), 'YYYYMM') || lpad(seq_val::text, 6, '0');
END;
$$ LANGUAGE plpgsql VOLATILE;

CREATE TABLE organizations
(
    id                 bigserial PRIMARY KEY,
    public_id          text UNIQUE         NOT NULL DEFAULT gen_org_public_id(),

    name               varchar(255)        NOT NULL,
    name_sl            varchar(255),

    status             organization_status NOT NULL DEFAULT 'active',

    created_by_user_id bigint              NOT NULL REFERENCES users (id),
    created_at         timestamptz         NOT NULL DEFAULT NOW(),
    updated_at         timestamptz         NOT NULL DEFAULT NOW()
);

CREATE TRIGGER organization_set_updated_at
    BEFORE UPDATE
    ON organizations
    FOR EACH ROW
EXECUTE FUNCTION set_updated_at();