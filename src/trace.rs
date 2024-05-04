use tracing::Level;
use tracing_panic::panic_hook;

pub fn setup_tracing() {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    std::panic::set_hook(Box::new(panic_hook));
}
