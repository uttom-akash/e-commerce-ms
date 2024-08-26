use crate::domain::models::cart::Cart;
use crate::schema::cart;
use diesel::{Insertable, Queryable};
use uuid::Uuid;

#[derive(Queryable)]
pub struct CartDiesel {
    pub id: i32,
    pub cart_id: Uuid,
    pub order_id: Option<Uuid>,
}

#[derive(Insertable)]
#[diesel(table_name = cart)]
pub struct NewCartDiesel {
    pub cart_id: Uuid,
}

impl Into<Cart> for CartDiesel {
    fn into(self) -> Cart {
        Cart {
            id: self.id,
            cart_id: self.cart_id,
            order_id: self.order_id,
            product_ledger: vec![],
        }
    }
}