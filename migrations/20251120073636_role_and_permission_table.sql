-- Add migration script here
CREATE TYPE role_status AS ENUM ('active', 'deleted', 'suspended');

CREATE TABLE role_and_permission (
                                     id BIGSERIAL PRIMARY KEY,
                                     public_id TEXT UNIQUE NOT NULL,
                                     name TEXT NOT NULL,
                                     name_sl TEXT NOT NULL,
                                     permissions JSONB NOT NULL,
                                     can_edit BOOLEAN NOT NULL DEFAULT FALSE,
                                     status role_status NOT NULL DEFAULT 'active',
                                    created_by_user_id bigint not null references users(id),
                                     created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                                     updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE role_public_counters
(
    organization_id        BIGINT       NOT NULL REFERENCES organizations (id),
    last_value             BIGINT       NOT NULL DEFAULT 0,
    organization_public_id VARCHAR(255) NOT NULL,
    PRIMARY KEY (organization_id)
);

CREATE OR REPLACE FUNCTION gen_role_public_id(org_id BIGINT)
    RETURNS TEXT AS
$$
DECLARE
    prefix   TEXT;
    next_num BIGINT;
BEGIN
    UPDATE role_public_counters
    SET last_value = last_value + 1
    WHERE organization_id = org_id
    RETURNING organization_public_id, last_value
        INTO prefix, next_num;

    RETURN prefix || LPAD(next_num::TEXT, 3, '0');
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION set_role_public_id()
    RETURNS TRIGGER AS
$$
BEGIN
    IF NEW.public_id IS NULL THEN
        NEW.public_id := gen_role_public_id(NEW.organization_id);
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_set_role_public_id
    BEFORE INSERT ON role_and_permission
    FOR EACH ROW
EXECUTE FUNCTION set_role_public_id();