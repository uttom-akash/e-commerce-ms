use crate::domain::models::order_status::OrderStatus;
use serde::Serialize;

#[derive(Serialize)]
pub struct OrderStatusDto {
    pub id: i16,
    pub name: Option<String>,
}

impl From<OrderStatus> for OrderStatusDto {
    fn from(value: OrderStatus) -> Self {
        OrderStatusDto {
            id: value.id,
            name: value.name,
        }
    }
}

pub fn convert_option_order_status_to_dto(value: Option<OrderStatus>) -> Option<OrderStatusDto> {
    match value {
        Some(cart_status) => Some(OrderStatusDto::from(cart_status)),
        None => None
    }
}