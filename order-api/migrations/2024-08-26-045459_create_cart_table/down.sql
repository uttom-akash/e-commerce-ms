-- This file should undo anything in `up.sql`
ALTER TABLE orders
DROP CONSTRAINT unique_order_id;

DROP TABLE cart;