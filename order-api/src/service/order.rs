use crate::domain::error::CommonError;
use crate::domain::models::order::Order;
use crate::domain::repository::order::OrderRepository;
use crate::domain::service::cart::CartService;
use crate::domain::service::order::OrderService;
use async_trait::async_trait;
use rust_decimal::Decimal;
use std::sync::Arc;
use uuid::Uuid;

pub struct OrderServiceImpl {
    pub order_repository: Arc<dyn OrderRepository>,
    pub cart_service: Arc<dyn CartService>,
}

impl OrderServiceImpl {
    pub fn new(order_repository: Arc<dyn OrderRepository>, cart_service: Arc<dyn CartService>) -> Self {
        OrderServiceImpl {
            order_repository,
            cart_service,
        }
    }
}

#[async_trait]
impl OrderService for OrderServiceImpl {
    async fn create(&self, mut order: Order) -> Result<Order, CommonError> {
        let products_ledger = self.cart_service
            .list(order.cart_id)
            .await?;

        let total_price: Decimal = products_ledger.iter()
            .map(|pl| pl.price_per_unit.unwrap() * Decimal::from(pl.quantity.unwrap()))
            .sum();

        order.total_price = Some(total_price);

        let mut order_domain_model = self.order_repository
            .add(order)
            .await.map_err(|e| -> CommonError{ e.into() })?;


        if let Some(cart) = &mut order_domain_model.cart {
            cart.product_ledger = products_ledger;
        }

        Ok(order_domain_model)
    }

    async fn get(&self, order_id: Uuid) -> Result<Order, CommonError> {
        let order = self.order_repository
            .get(order_id)
            .await
            .map_err(|e| -> CommonError{ e.into() })?;

        Ok(order)
    }
}