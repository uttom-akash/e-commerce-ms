-- Your SQL goes here

ALTER TABLE orders
ADD CONSTRAINT unique_order_id UNIQUE (order_id);


CREATE TABLE cart (
    id SERIAL PRIMARY KEY,        -- Primary Key with auto-incrementing integer
    cart_id UUID NOT NULL UNIQUE,  -- Unique UUID for each cart
    order_id UUID,                 -- UUID referencing orders(order_id)
    FOREIGN KEY (order_id) REFERENCES orders(order_id)  -- Foreign Key constraint
);
