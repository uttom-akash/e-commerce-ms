use crate::domain::error::CommonError;
use crate::domain::models::cart::Cart;
use crate::domain::models::product_ledger::ProductLedger;
use crate::domain::repository::cart::CartRepository;
use crate::domain::service::cart::CartService;
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;

pub struct CartServiceImpl {
    pub cart_repository: Arc<dyn CartRepository>,
}

impl CartServiceImpl {
    pub fn new(cart_repository: Arc<dyn CartRepository>) -> Self {
        CartServiceImpl {
            cart_repository
        }
    }
}

#[async_trait]
impl CartService for CartServiceImpl {
    async fn create_or_add_to_cart(&self, cart: ProductLedger) -> Result<Cart, CommonError> {
        let mut cart = self.cart_repository.add(cart)
            .await
            .map_err(|e| -> CommonError{ e.into() })?;

        cart.product_ledger = self.list(cart.cart_id)
            .await?;

        Ok(cart)
    }

    async fn list(&self, cart_id: Uuid) -> Result<Vec<ProductLedger>, CommonError> {
        let items = self.cart_repository.list(cart_id)
            .await
            .map_err(|e| -> CommonError{ e.into() })?;

        Ok(items)
    }
}