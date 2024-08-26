-- This file should undo anything in `up.sql`
-- Reverse the column rename
ALTER TABLE carts RENAME COLUMN cart_status_id TO cart_product_status_id;

-- Reverse the table rename
ALTER TABLE cart_status RENAME TO cart_product_status;

-- Drop the new index
DROP INDEX IF EXISTS idx_carts_cart_status_id;

-- Recreate the old index
CREATE INDEX idx_carts_cart_product_status_id
ON carts (cart_product_status_id);
