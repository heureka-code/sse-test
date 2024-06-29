mod env_keys;
mod setup;
mod sse_route;

use actix_web::{middleware::Logger, App, HttpServer};
use setup::{get_app_port, get_cors};

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "0");
    env_logger::init();

    let allowed_origin_suffixes = crate::setup::cors::read_allowed_suffixes();
    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .wrap(get_cors(allowed_origin_suffixes.clone()))
            .service(sse_route::stream_updates)
    })
    .bind(("0.0.0.0", get_app_port()))?
    .workers(4)
    .run()
    .await?;
    Ok(())
}
