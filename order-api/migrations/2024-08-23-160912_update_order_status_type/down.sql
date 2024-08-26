-- This file should undo anything in `up.sql`
ALTER TABLE order_status
ALTER COLUMN id TYPE INTEGER USING id::integer;