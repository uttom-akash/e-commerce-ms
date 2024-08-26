-- Your SQL goes here
ALTER TABLE product_ledger
ADD CONSTRAINT fk_cart_id_product_ledger FOREIGN KEY (cart_id) REFERENCES cart(cart_id);

ALTER TABLE orders
ADD CONSTRAINT fk_cart_id_orders FOREIGN KEY (cart_id) REFERENCES cart(cart_id);
