-- Your SQL goes here
ALTER TABLE orders
ALTER COLUMN cart_id SET NOT NULL;

ALTER TABLE orders
ADD CONSTRAINT unique_cart_id UNIQUE (cart_id);
