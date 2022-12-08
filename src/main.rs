mod config;
mod logging;
mod manager;

fn main() -> ! {
    // Init logging
    logging::init();

    // Print program banner
    logging::print_banner();

    // Get the configuration
    let config = config::Config::load_from_file_with_fallback();

    // Initialize and start the GPU fan manager
    let mut manager = manager::Manager::new(
        config.idle_temp,
        config.cooling_temp,
        config.refresh_delay,
        config.failsafe_shutdown_delay,
    );
    manager.start();
}
