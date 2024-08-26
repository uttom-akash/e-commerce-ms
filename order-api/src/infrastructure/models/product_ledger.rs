use crate::domain::models::product_ledger::ProductLedger;
use crate::schema::product_ledger;
use diesel::internal::derives::multiconnection::chrono;
use diesel::Insertable;
use diesel::Queryable;

#[derive(Queryable)]
pub struct ProductLedgerDiesel {
    pub id: i32,
    pub cart_id: uuid::Uuid,
    pub product_id: Option<uuid::Uuid>,
    pub quantity: Option<i16>,
    pub price_per_unit: Option<rust_decimal::Decimal>,
    pub cart_status_id: Option<i16>,
    pub discount: Option<rust_decimal::Decimal>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub modified_at: Option<chrono::NaiveDateTime>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = product_ledger)]
pub struct NewProductLedgerDiesel {
    pub cart_id: uuid::Uuid,
    pub product_id: Option<uuid::Uuid>,
    pub quantity: Option<i16>,
    pub price_per_unit: Option<rust_decimal::Decimal>,
    pub cart_status_id: Option<i16>,
    pub discount: Option<rust_decimal::Decimal>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub modified_at: Option<chrono::NaiveDateTime>,
}

impl From<ProductLedger> for NewProductLedgerDiesel {
    fn from(cart: ProductLedger) -> Self {
        NewProductLedgerDiesel {
            cart_id: cart.cart_id,
            product_id: cart.product_id,
            quantity: cart.quantity,
            price_per_unit: cart.price_per_unit,
            cart_status_id: cart.cart_status_id,
            discount: cart.discount,
            created_at: cart.created_at,
            modified_at: cart.modified_at,
        }
    }
}

impl Into<ProductLedger> for ProductLedgerDiesel {
    fn into(self) -> ProductLedger {
        ProductLedger {
            id: self.id,
            cart_id: self.cart_id,
            product_id: self.product_id,
            quantity: self.quantity,
            price_per_unit: self.price_per_unit,
            cart_status_id: self.cart_status_id,
            discount: self.discount,
            created_at: self.created_at,
            modified_at: self.modified_at,
            product_ledger_status: None,
        }
    }
}