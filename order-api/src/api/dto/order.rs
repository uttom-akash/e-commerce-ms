use crate::api::dto::cart::CartDto;
use crate::api::dto::order_status::{convert_option_order_status_to_dto, OrderStatusDto};
use crate::domain::constants::order_status::OrderStatusType;
use crate::domain::models::order::Order;
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct OrderDto {
    pub order_id: uuid::Uuid,
    pub cart_id: uuid::Uuid,
    pub payment_id: Option<uuid::Uuid>,
    pub address: Option<String>,
    pub shipment_id: Option<uuid::Uuid>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub modified_at: Option<chrono::NaiveDateTime>,
    pub description: Option<String>,
    pub total_price: Option<rust_decimal::Decimal>,

    pub cart: Option<CartDto>,
    pub order_status: Option<OrderStatusDto>,
}

impl From<Order> for OrderDto {
    fn from(value: Order) -> Self {
        OrderDto {
            order_id: value.order_id,
            cart_id: value.cart_id,
            payment_id: value.payment_id,
            address: value.address,
            shipment_id: value.shipment_id,
            created_at: value.created_at,
            modified_at: value.modified_at,
            description: value.description,
            total_price: value.total_price,

            cart: Some(CartDto::from(value.cart.unwrap())),
            order_status: convert_option_order_status_to_dto(value.order_status),
        }
    }
}

#[derive(Deserialize)]
pub struct NewOrderDto {
    pub order_id: uuid::Uuid,
    pub cart_id: uuid::Uuid,
    pub address: Option<String>,
    pub description: Option<String>,
}

impl Into<Order> for NewOrderDto {
    fn into(self) -> Order {
        Order {
            id: 0,
            order_id: self.order_id,
            cart_id: self.cart_id,
            payment_id: None,
            order_status_id: Some(OrderStatusType::New as i16),
            address: self.address,
            shipment_id: None,
            created_at: Some(Utc::now().naive_utc()),
            modified_at: None,
            description: self.description,
            total_price: None,
            cart: None,
            order_status: None,
        }
    }
}