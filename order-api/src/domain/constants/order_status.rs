pub enum OrderStatusType {
    None,
    New,
    Pending,        // Order has been placed but not yet processed
    Confirmed,      // Order has been confirmed and is being prepared
    Shipped,        // Order has been shipped to the customer
    Delivered,      // Order has been delivered to the customer
    Completed,      // Order has been completed and closed
    Cancelled,      // Order has been cancelled
    Returned,       // Order has been returned by the customer
    Refunded,       // Order has been refunded
}

impl OrderStatusType {
    // Optional: Add methods for OrderStatus if needed, e.g., to convert to string
    pub fn as_str(&self) -> &'static str {
        match self {
            OrderStatusType::None => "None",
            OrderStatusType::New => "New",
            OrderStatusType::Pending => "Pending",
            OrderStatusType::Confirmed => "Confirmed",
            OrderStatusType::Shipped => "Shipped",
            OrderStatusType::Delivered => "Delivered",
            OrderStatusType::Completed => "Completed",
            OrderStatusType::Cancelled => "Cancelled",
            OrderStatusType::Returned => "Returned",
            OrderStatusType::Refunded => "Refunded",
        }
    }
}
