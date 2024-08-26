-- This file should undo anything in `up.sql`
ALTER TABLE product_ledger
DROP CONSTRAINT fk_cart_id_product_ledger;

ALTER TABLE orders
DROP CONSTRAINT fk_cart_id_orders;
