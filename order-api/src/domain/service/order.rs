use crate::domain::error::CommonError;
use crate::domain::models::order::Order;
use async_trait::async_trait;

#[async_trait]
pub trait OrderService: Send + Sync {
    async fn create(&self, order: Order) -> Result<Order, CommonError>;
    async fn get(&self, order_id: uuid::Uuid) -> Result<Order, CommonError>;
}