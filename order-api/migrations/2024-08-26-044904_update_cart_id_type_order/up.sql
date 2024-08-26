-- Your SQL goes here
ALTER TABLE orders DROP COLUMN cart_id;
ALTER TABLE orders ADD COLUMN cart_id UUID;
