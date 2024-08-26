-- This file should undo anything in `up.sql`
ALTER TABLE orders
ALTER COLUMN cart_id TYPE BIGINT;