-- This file should undo anything in `up.sql`
ALTER TABLE orders
DROP CONSTRAINT unique_cart_id;

ALTER TABLE orders
ALTER COLUMN cart_id DROP NOT NULL;
