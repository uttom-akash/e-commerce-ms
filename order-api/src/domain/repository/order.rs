use crate::domain::error::RepositoryError;
use crate::domain::models::order::Order;
use async_trait::async_trait;

#[async_trait]
pub trait OrderRepository: Send + Sync {
    async fn add(&self, order: Order) -> Result<Order, RepositoryError>;
    async fn get(&self, order_id: uuid::Uuid) -> Result<Order, RepositoryError>;
}