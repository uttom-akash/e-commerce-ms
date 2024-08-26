-- This file should undo anything in `up.sql`
DELETE FROM cart_status
WHERE name IN ('None', 'New', 'Pending', 'Confirmed', 'Shipped', 'Completed', 'Cancelled');
