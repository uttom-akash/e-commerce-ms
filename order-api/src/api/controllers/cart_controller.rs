use crate::api::dto::cart::CartDto;
use crate::api::dto::product_ledger::ProductDto;
use crate::domain::error::ApiError;
use crate::domain::models::product_ledger::ProductLedger;
use crate::domain::service::cart::CartService;
use actix_web::web::Json;
use actix_web::{web, Scope};

pub async fn create_or_add_to_cart(
    cart_service: web::Data<dyn CartService>, product: web::Json<ProductDto>)
    -> Result<Json<CartDto>, ApiError> {
    let cart: ProductLedger = product.into_inner().into();
    let res = cart_service.create_or_add_to_cart(cart).await?;

    let items = CartDto::from(res);

    Ok(web::Json(items))
}

pub fn cart_endpoints() -> Scope {
    web::scope("/carts")
        .route("/product", web::post().to(create_or_add_to_cart))
}