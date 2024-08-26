-- Your SQL goes here
-- Add a unique constraint on cart_id and product_id
ALTER TABLE carts
ADD CONSTRAINT unique_cart_product UNIQUE (cart_id, product_id);
