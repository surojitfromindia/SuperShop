-- Add migration script here
CREATE TYPE staff_role_status AS ENUM ('active', 'deleted', 'suspended');


CREATE TABLE organization_staff_roles (
                                          id BIGSERIAL PRIMARY KEY,
                                          organization_id BIGINT NOT NULL REFERENCES organizations(id),

                                          staff_id BIGINT NOT NULL REFERENCES organization_staffs(id),
                                          role_id  BIGINT NOT NULL REFERENCES role_and_permission(id),

                                          status staff_role_status NOT NULL DEFAULT 'active',

                                          created_by_user_id BIGINT NOT NULL REFERENCES users(id),

                                          created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                                          updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

                                          UNIQUE (organization_id, staff_id, role_id)
);