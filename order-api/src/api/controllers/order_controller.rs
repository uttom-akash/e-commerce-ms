use crate::api::dto::order::{NewOrderDto, OrderDto};
use crate::domain::error::ApiError;
use crate::domain::service::order::OrderService;
use actix_web::web::Json;
use actix_web::{web, Scope};

pub async fn create_order(
    order_service: web::Data<dyn OrderService>, new_order: web::Json<NewOrderDto>)
    -> Result<Json<OrderDto>, ApiError> {
    let res = order_service.create(new_order.into_inner().into()).await?;

    Ok(web::Json(OrderDto::from(res)))
}

pub async fn get_order(
    order_service: web::Data<dyn OrderService>, id: web::Path<uuid::Uuid>)
    -> Result<Json<OrderDto>, ApiError> {
    let res = order_service.get(id.into_inner()).await?;

    Ok(web::Json(OrderDto::from(res)))
}

pub fn order_endpoints() -> Scope {
    web::scope("/order")
        .route("/", web::post().to(create_order))
        .route("/{id}", web::get().to(get_order))
}