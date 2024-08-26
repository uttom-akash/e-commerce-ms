use crate::api::controllers::controller_registration::configure_endpoints;
use crate::container::configure_app_data;
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use actix_web::middleware::Logger;
use actix_web::App;
use actix_web::Error;
//use crate::api::controllers::urls::configure_services;
//use crate::api::middleware::ServiceContextMaintenanceCheck;
//use crate::container::configure_app_data;

pub fn create_app() -> App<
    impl ServiceFactory<
        ServiceRequest,
        Response=ServiceResponse<impl MessageBody>,
        Config=(),
        InitError=(),
        Error=Error,
    >,
> {
    App::new()
        .wrap(Logger::default())
        //.wrap(ServiceContextMaintenanceCheck)
        .configure(configure_app_data)
        .configure(configure_endpoints)
}