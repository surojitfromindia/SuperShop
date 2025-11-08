-- Add migration script here
alter table user_credentials rename column password_hash to hash_password;