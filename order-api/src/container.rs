use std::sync::Arc;

use actix_web::web;
use actix_web::web::ServiceConfig;

use crate::domain::repository::cart::CartRepository;
use crate::domain::repository::order::OrderRepository;
use crate::domain::service::cart::CartService;
use crate::domain::service::order::OrderService;
use crate::infrastructure::database::postgresql::db_pool;
use crate::infrastructure::repository::cart::CartRepositoryImpl;
use crate::infrastructure::repository::order::OrderRepositoryImpl;
use crate::service::cart::CartServiceImpl;
use crate::service::order::OrderServiceImpl;

pub struct Container {
    pub cart_service: Arc<dyn CartService>,
    pub order_service: Arc<dyn OrderService>,
}

impl Container {
    pub fn new() -> Self {
        let pool = db_pool();

        //repository
        let cart_repository: Arc<dyn CartRepository> = Arc::new(
            CartRepositoryImpl::new(Arc::new(pool.clone()))
        );

        let order_repository: Arc<dyn OrderRepository> = Arc::new(
            OrderRepositoryImpl::new(Arc::new(pool.clone()))
        );

        //service
        let cart_service = Arc::new(
            CartServiceImpl { cart_repository }
        );

        let order_service = Arc::new(
            OrderServiceImpl { order_repository, cart_service: cart_service.clone() }
        );

        Container { cart_service, order_service }
    }
}

impl Default for Container {
    fn default() -> Self {
        Self::new()
    }
}

pub fn configure_app_data(config: &mut ServiceConfig) {
    let container = Container::new();

    config
        .app_data(web::Data::from(container.cart_service.clone()))
        .app_data(web::Data::from(container.order_service.clone()));
}