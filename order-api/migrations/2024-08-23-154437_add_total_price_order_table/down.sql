-- This file should undo anything in `up.sql`
ALTER TABLE orders
DROP COLUMN IF EXISTS total_price;
