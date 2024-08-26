use crate::domain::models::product_ledger::ProductLedger;
use uuid::Uuid;

#[derive(Clone)]
pub struct Cart {
    pub id: i32,
    pub cart_id: Uuid,
    pub order_id: Option<Uuid>,

    pub product_ledger: Vec<ProductLedger>,
}