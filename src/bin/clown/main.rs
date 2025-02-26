use std::process;

use types::StreamingContext;

mod demo_futures;
mod logging;
mod server;
mod types;

#[tokio::main]
async fn main() {
    let config = server::load_config().await; // Arc?
    let _ = logging::setup_logging();
    let pid = process::id();
    let ip_addr = config.ip_addr.clone();
    log::info!("Server started as {pid} process, at {ip_addr}");
}
