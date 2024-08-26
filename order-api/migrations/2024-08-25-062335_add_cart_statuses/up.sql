-- Your SQL goes here
INSERT INTO cart_status (id, name) VALUES
    (0, 'None'),        -- Cart is not assigned any status
    (1, 'New'),         -- Cart is newly created
    (2, 'Pending'),     -- Cart is pending some action (e.g., waiting for payment)
    (3, 'Confirmed'),   -- Cart has been confirmed
    (4, 'Shipped'),     -- Products in the cart have been shipped
    (5, 'Completed'),   -- Cart transaction is completed
    (6, 'Cancelled');   -- Cart has been cancelled
