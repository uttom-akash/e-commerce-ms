use crate::api::dto::product_ledger_status::{convert_option_product_ledger_status_to_dto, ProductLedgerStatusDto};
use crate::domain::constants::product_ledger_status::ProductLedgerStatusType;
use crate::domain::models::product_ledger::ProductLedger;
use chrono::Utc;
use diesel::internal::derives::multiconnection::chrono;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[derive(Deserialize, Serialize)]
pub struct ProductDto {
    pub cart_id: uuid::Uuid,
    pub product_id: Option<uuid::Uuid>,
    pub quantity: Option<i16>,
    pub price_per_unit: Option<rust_decimal::Decimal>,
    pub discount: Option<rust_decimal::Decimal>,
}

impl Into<ProductLedger> for ProductDto {
    fn into(self) -> ProductLedger {
        ProductLedger {
            id: 0,
            product_id: self.product_id,
            quantity: self.quantity,
            price_per_unit: self.price_per_unit,
            discount: self.discount,
            cart_id: if self.cart_id == Uuid::default() {
                Uuid::new_v4()
            } else {
                self.cart_id
            },
            cart_status_id: Some(ProductLedgerStatusType::New as i16),
            created_at: Some(Utc::now().naive_utc()),
            modified_at: None,
            product_ledger_status: None,
        }
    }
}

#[derive(Serialize)]
pub struct ProductLedgerDto {
    pub cart_id: uuid::Uuid,
    pub product_id: Option<uuid::Uuid>,
    pub quantity: Option<i16>,
    pub price_per_unit: Option<rust_decimal::Decimal>,
    pub cart_status_id: Option<i16>,
    pub discount: Option<rust_decimal::Decimal>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub modified_at: Option<chrono::NaiveDateTime>,
    pub cart_status: Option<ProductLedgerStatusDto>,
}

impl From<ProductLedger> for ProductLedgerDto {
    fn from(cart: ProductLedger) -> Self {
        ProductLedgerDto {
            cart_id: cart.cart_id,
            product_id: cart.product_id,
            quantity: cart.quantity,
            price_per_unit: cart.price_per_unit,
            cart_status_id: cart.cart_status_id,
            discount: cart.discount,
            created_at: cart.created_at,
            modified_at: cart.modified_at,
            cart_status: convert_option_product_ledger_status_to_dto(cart.product_ledger_status),
        }
    }
}

