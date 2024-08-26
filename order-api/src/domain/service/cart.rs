use crate::domain::error::CommonError;
use crate::domain::models::cart::Cart;
use crate::domain::models::product_ledger::ProductLedger;
use async_trait::async_trait;

#[async_trait]
pub trait CartService: Send + Sync {
    async fn create_or_add_to_cart(&self, cart: ProductLedger) -> Result<Cart, CommonError>;
    async fn list(&self, cart_id: uuid::Uuid) -> Result<Vec<ProductLedger>, CommonError>;
}