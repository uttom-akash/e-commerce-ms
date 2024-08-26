-- This file should undo anything in `up.sql`
ALTER TABLE cart_status
ALTER COLUMN id TYPE INTEGER USING id::integer;
