use crate::api::controllers::cart_controller::cart_endpoints;
use crate::api::controllers::order_controller::order_endpoints;
use actix_web::web::ServiceConfig;

pub fn configure_endpoints(config: &mut ServiceConfig) {
    config
        .service(cart_endpoints())
        .service(order_endpoints());
}