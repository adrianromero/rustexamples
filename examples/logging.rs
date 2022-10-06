fn main() {
    env_logger::init();

    // RUST_LOG=trace cargo run --example logging
    log::trace!("Executing query: {}", "pepe");
    log::debug!("Executing query: {}", "debug");
    log::info!("Executing info.");
    log::warn!("Executing warn.");
    log::error!("Executing error.");
}
