use busted::run_app;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    run_app().expect("Failed to start server").await
}
