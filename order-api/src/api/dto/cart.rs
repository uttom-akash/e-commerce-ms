use crate::api::dto::product_ledger::ProductLedgerDto;
use crate::domain::models::cart::Cart;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct CartDto {
    pub cart_id: Uuid,
    pub order_id: Option<Uuid>,

    pub product_ledger: Vec<ProductLedgerDto>,
}

impl From<Cart> for CartDto {
    fn from(value: Cart) -> Self {
        CartDto {
            cart_id: value.cart_id,
            order_id: value.order_id,
            product_ledger: value.product_ledger.iter().map(|v| ProductLedgerDto::from(v.clone())).collect(),
        }
    }
}