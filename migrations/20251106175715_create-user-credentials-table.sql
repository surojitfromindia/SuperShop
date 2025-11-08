
CREATE TABLE user_credentials(
    user_id bigserial primary key references users(id),
    password_hash TEXT not null
)
