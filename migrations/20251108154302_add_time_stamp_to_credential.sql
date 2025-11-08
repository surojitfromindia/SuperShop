-- Add migration script here
alter table user_credentials
    add column created_at TIMESTAMPTZ NOT NULL DEFAULT NOW();
alter table user_credentials
    add updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW();



CREATE TRIGGER users_cred_set_updated_at
    BEFORE UPDATE
    ON user_credentials
    FOR EACH ROW
EXECUTE FUNCTION set_updated_at();