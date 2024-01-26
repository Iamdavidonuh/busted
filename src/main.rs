use std::fmt::format;

use busted::configurations::get_config;
use busted::run_app;
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = get_config().expect("Failed to load configuration");
    let address = format!("127.0.0.1:{}", config.application_port);
    run_app(&address).expect("Failed to start server").await
}
