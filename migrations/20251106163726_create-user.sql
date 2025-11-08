create extension if not exists "uuid-ossp";

BEGIN ;


--- global update trigger.
CREATE OR REPLACE FUNCTION set_updated_at()
    RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;


CREATE SEQUENCE user_public_seq;

-- create a function that create public facing id
CREATE OR REPLACE FUNCTION gen_user_public_id()
    RETURNS text AS
$$
DECLARE
    seq_val bigint;
BEGIN
    seq_val := nextval('user_public_seq');
    RETURN to_char(NOW(), 'YYYYMMDD')||lpad(seq_val::text, 6, '0');
END;
$$ LANGUAGE plpgsql VOLATILE;

CREATE TYPE user_status AS ENUM ('active', 'deleted', 'suspended');

CREATE TABLE users
(
    id         bigserial PRIMARY KEY,
    public_id  text UNIQUE  NOT NULL DEFAULT gen_user_public_id(),
    first_name VARCHAR(100) NOT NULL,
    last_name  VARCHAR(100) NOT NULL,
    email      VARCHAR(255) UNIQUE,
    phone      VARCHAR(60),
    is_active  BOOLEAN      NOT NULL DEFAULT TRUE,
    status user_status NOT NULL DEFAULT 'active',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);



CREATE TRIGGER users_set_updated_at
    BEFORE UPDATE ON users
    FOR EACH ROW
EXECUTE FUNCTION set_updated_at();






COMMIT ;