use anyhow::{Context, Result};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

// Setup signal handlers for signals that kill the process
pub fn get_term_signal() -> Result<Arc<AtomicBool>> {
    let term_signal = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::consts::SIGTERM, Arc::clone(&term_signal))
        .context("Failed to setup a SIGTERM hook.")?;
    signal_hook::flag::register(signal_hook::consts::SIGINT, Arc::clone(&term_signal))
        .context("Failed to setup a SIGINT hook.")?;
    signal_hook::flag::register(signal_hook::consts::SIGABRT, Arc::clone(&term_signal))
        .context("Failed to setup a SIGABRT hook.")?;

    // This atomic bool will switch to false, when any of these get triggered
    Ok(term_signal)
}
