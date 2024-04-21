use tracing::error;
use wallet_hooks::api;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    match api::start("127.0.0.1:3000").await {
        Ok(_) => {},
        Err(e) => error!("Cannot run api: {}", e),
    };
}
