use std::fmt::format;
use std::net::TcpListener;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use busted::telemetry::*;

use busted::configurations::get_config;
use busted::startup::run_app;
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = get_config().expect("Failed to load configuration");
    let subscriber = get_subscriber(
        "INFO",
        &config.application.service_name,
        &config.application.exporter_url,
    );
    initialize_subscriber(subscriber);
    let address = format!("127.0.0.1:{}", config.application.port);
    let listener = TcpListener::bind(address).expect("Failed to bind to the address");
    let pool = PgPoolOptions::new().connect_lazy_with(config.database.with_db());
    run_app(listener, pool).expect("Failed to start server").await
}
