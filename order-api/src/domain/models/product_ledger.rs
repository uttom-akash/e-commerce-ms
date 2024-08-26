use crate::domain::models::product_ledger_status::ProductLedgerStatus;
use diesel::internal::derives::multiconnection::chrono;

#[derive(Clone)]
pub struct ProductLedger {
    pub id: i32,
    pub cart_id: uuid::Uuid,
    pub product_id: Option<uuid::Uuid>,
    pub quantity: Option<i16>,
    pub price_per_unit: Option<rust_decimal::Decimal>,
    pub cart_status_id: Option<i16>,
    pub discount: Option<rust_decimal::Decimal>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub modified_at: Option<chrono::NaiveDateTime>,

    pub product_ledger_status: Option<ProductLedgerStatus>,
}