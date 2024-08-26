-- This file should undo anything in `up.sql`
ALTER TABLE orders DROP COLUMN cart_id;
ALTER TABLE orders ADD COLUMN cart_id INT4;
