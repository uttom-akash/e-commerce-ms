-- Your SQL goes here
ALTER TABLE order_status
ALTER COLUMN id TYPE SMALLINT USING id::smallint;