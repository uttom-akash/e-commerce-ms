-- This file should undo anything in `up.sql`
ALTER TABLE product_ledger RENAME TO carts;
ALTER TABLE product_ledger_status RENAME TO cart_status;