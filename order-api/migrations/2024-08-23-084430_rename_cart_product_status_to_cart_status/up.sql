-- Your SQL goes here
ALTER TABLE cart_product_status RENAME TO cart_status;

ALTER TABLE carts RENAME COLUMN cart_product_status_id TO cart_status_id;

CREATE INDEX idx_carts_cart_status_id ON carts (cart_status_id);

DROP INDEX IF EXISTS idx_carts_cart_product_status_id;
