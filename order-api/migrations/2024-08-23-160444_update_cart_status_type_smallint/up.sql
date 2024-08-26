-- Your SQL goes here
ALTER TABLE cart_status
ALTER COLUMN id TYPE SMALLINT USING id::smallint;