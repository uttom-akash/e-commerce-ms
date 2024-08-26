use crate::domain::error::RepositoryError;
use crate::domain::models::cart::Cart;
use crate::domain::models::product_ledger::ProductLedger;
use async_trait::async_trait;

#[async_trait]
pub trait CartRepository: Send + Sync {
    async fn add(&self, product: ProductLedger) -> Result<Cart, RepositoryError>;
    async fn list(&self, cart_id: uuid::Uuid) -> Result<Vec<ProductLedger>, RepositoryError>;
}