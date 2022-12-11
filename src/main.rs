mod config;
mod logging;
mod manager;
mod signals;

fn main() {
    // Init logging
    logging::init();

    // Print program banner
    logging::print_banner();

    // Get the configuration
    let config = config::Config::load_from_file_with_fallback();

    // Get a signal handler to let us know when to exit
    let term_signal = signals::get_term_signal();

    match term_signal {
        Err(term_signal_err) => {
            log::error!("{}", term_signal_err);
            log::error!("Failed to register a termination signal. Aborting startup.");
        }
        Ok(term_signal) => {
            // Initialize and start the GPU fan manager
            let mut manager = manager::Manager::new(
                config.idle_temp,
                config.cooling_temp,
                config.refresh_delay,
                config.failsafe_shutdown_delay,
            );

            manager.start(term_signal);
        }
    }
}
