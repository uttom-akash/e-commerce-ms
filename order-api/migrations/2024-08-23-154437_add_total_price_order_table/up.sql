-- Your SQL goes here
ALTER TABLE orders
ADD COLUMN total_price NUMERIC DEFAULT 0;
