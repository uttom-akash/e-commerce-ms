use crate::domain::models::cart::Cart;
use crate::domain::models::order_status::OrderStatus;

#[derive(Clone)]
pub struct Order {
    pub id: i32,
    pub order_id: uuid::Uuid,
    pub cart_id: uuid::Uuid,
    pub payment_id: Option<uuid::Uuid>,
    pub order_status_id: Option<i16>,
    pub address: Option<String>,
    pub shipment_id: Option<uuid::Uuid>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub modified_at: Option<chrono::NaiveDateTime>,
    pub description: Option<String>,
    pub total_price: Option<rust_decimal::Decimal>,

    pub cart: Option<Cart>,
    pub order_status: Option<OrderStatus>,
}