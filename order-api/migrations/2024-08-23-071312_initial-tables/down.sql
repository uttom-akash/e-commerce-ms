-- Drop indexes
DROP INDEX IF EXISTS idx_orders_cart_id;
DROP INDEX IF EXISTS idx_orders_order_status_id;
DROP INDEX IF EXISTS idx_carts_cart_product_status_id;

-- Drop tables
DROP TABLE IF EXISTS orders CASCADE;
DROP TABLE IF EXISTS carts CASCADE;
DROP TABLE IF EXISTS order_status CASCADE;
DROP TABLE IF EXISTS cart_product_status CASCADE;
