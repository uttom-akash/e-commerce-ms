use strum_macros::{Display, EnumString, VariantNames};

#[derive(Debug, Clone, Copy, EnumString, VariantNames, Display)]
pub enum ProductLedgerStatusType {
    None,
    New,            // Cart is newly created
    Pending,        // Cart is pending some action (e.g., waiting for payment)
    Confirmed,      // Cart has been confirmed
    Shipped,        // Products in the cart have been shipped
    Completed,      // Cart transaction is completed
    Cancelled,      // Cart has been cancelled
}

impl ProductLedgerStatusType {
    // Optional: Add methods for CartStatus if needed, e.g., to convert to string
    pub fn as_str(&self) -> &'static str {
        match self {
            ProductLedgerStatusType::None => "None",
            ProductLedgerStatusType::New => "New",
            ProductLedgerStatusType::Pending => "Pending",
            ProductLedgerStatusType::Confirmed => "Confirmed",
            ProductLedgerStatusType::Shipped => "Shipped",
            ProductLedgerStatusType::Completed => "Completed",
            ProductLedgerStatusType::Cancelled => "Cancelled",
        }
    }
}