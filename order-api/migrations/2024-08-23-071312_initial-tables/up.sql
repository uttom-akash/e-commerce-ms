-- Create table for order_status
CREATE TABLE order_status (
    id SERIAL PRIMARY KEY,
    name VARCHAR
);

-- Create table for cart_product_status
CREATE TABLE cart_product_status (
    id SERIAL PRIMARY KEY,
    name VARCHAR
);

-- Create table for carts
CREATE TABLE carts (
    id SERIAL PRIMARY KEY,
    cart_id UUID NOT NULL,
    product_id UUID,
    quantity SMALLINT,
    price_per_unit DECIMAL,
    cart_product_status_id SMALLINT REFERENCES cart_product_status(id),
    discount DECIMAL,
    created_at TIMESTAMP,
    modified_at TIMESTAMP
);

-- Create an index on the foreign key cart_product_status_id
CREATE INDEX idx_carts_cart_product_status_id ON carts(cart_product_status_id);

-- Create table for orders
CREATE TABLE orders (
    id SERIAL PRIMARY KEY,
    order_id UUID NOT NULL,
    cart_id BIGINT REFERENCES carts(id),
    payment_id UUID,
    order_status_id SMALLINT REFERENCES order_status(id),
    address VARCHAR,
    shipment_id UUID,
    created_at TIMESTAMP,
    modified_at TIMESTAMP,
    description VARCHAR
);

-- Create an index on the foreign key cart_id
CREATE INDEX idx_orders_cart_id ON orders(cart_id);

-- Create an index on the foreign key order_status_id
CREATE INDEX idx_orders_order_status_id ON orders(order_status_id);




