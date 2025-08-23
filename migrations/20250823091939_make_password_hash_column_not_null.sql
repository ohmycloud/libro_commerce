-- Add migration script here
-- Make `password_hash` mandatory
UPDATE users set password_hash = 'test' WHERE id = 1;

ALTER TABLE users
ALTER COLUMN password_hash
SET
    NOT NULL;
