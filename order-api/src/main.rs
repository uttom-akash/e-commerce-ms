#![recursion_limit = "256"]

use actix_web::HttpServer;

use order_api::app::create_app;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(move || { create_app() })
        .bind(("127.0.0.1", 8081))?;
    server.run().await
}
