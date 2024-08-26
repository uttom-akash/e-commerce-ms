use crate::domain::models::product_ledger_status::ProductLedgerStatus;
use serde::Serialize;

#[derive(Serialize)]
pub struct ProductLedgerStatusDto {
    pub id: i16,
    pub name: Option<String>,
}

impl From<ProductLedgerStatus> for ProductLedgerStatusDto {
    fn from(value: ProductLedgerStatus) -> Self {
        ProductLedgerStatusDto {
            id: value.id,
            name: value.name,
        }
    }
}

pub fn convert_option_product_ledger_status_to_dto(value: Option<ProductLedgerStatus>) -> Option<ProductLedgerStatusDto> {
    match value {
        Some(cart_status) => Some(ProductLedgerStatusDto::from(cart_status)),
        None => None
    }
}
