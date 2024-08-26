use crate::domain::models::order::Order;
use crate::schema::orders;
use diesel::{Insertable, Queryable};

#[derive(Queryable)]
pub struct OrderDiesel {
    pub id: i32,
    pub order_id: uuid::Uuid,
    pub payment_id: Option<uuid::Uuid>,
    pub order_status_id: Option<i16>,
    pub address: Option<String>,
    pub shipment_id: Option<uuid::Uuid>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub modified_at: Option<chrono::NaiveDateTime>,
    pub description: Option<String>,
    pub total_price: Option<rust_decimal::Decimal>,
    pub cart_id: uuid::Uuid,
}

impl Into<Order> for OrderDiesel {
    fn into(self) -> Order {
        Order {
            id: self.id,
            order_id: self.order_id,
            cart_id: self.cart_id,
            payment_id: self.payment_id,
            order_status_id: self.order_status_id,
            address: self.address,
            shipment_id: self.shipment_id,
            created_at: self.created_at,
            modified_at: self.modified_at,
            description: self.description,
            total_price: self.total_price,
            cart: None,
            order_status: None,
        }
    }
}

#[derive(Insertable, Clone)]
#[diesel(table_name = orders)]
pub struct NewOrderDiesel {
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
}

impl From<Order> for NewOrderDiesel {
    fn from(value: Order) -> Self {
        NewOrderDiesel {
            order_id: value.order_id,
            cart_id: value.cart_id,
            payment_id: value.payment_id,
            order_status_id: value.order_status_id,
            address: value.address,
            shipment_id: value.shipment_id,
            created_at: value.created_at,
            modified_at: value.modified_at,
            description: value.description,
            total_price: value.total_price,
        }
    }
}