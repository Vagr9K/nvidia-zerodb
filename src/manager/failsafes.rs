use notify_rust::Notification;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct Failsafes {
    shutdown_delay: Duration,
    last_fan_enable_error_time: Option<Instant>,
}

impl Failsafes {
    pub fn new(shutdown_delay: u64) -> Self {
        Failsafes {
            shutdown_delay: Duration::from_millis(shutdown_delay),
            last_fan_enable_error_time: None,
        }
    }

    pub fn on_fan_enable_error(&mut self) {
        let current_time = Instant::now();
        let last_time = self.last_fan_enable_error_time.get_or_insert(current_time);

        let time_diff = current_time.duration_since(*last_time);

        if time_diff == Duration::ZERO {
            log::warn!("Issuing a notification about failing to enable the fans.");

            let res = Notification::new()
                .summary("nvidia-zerodb: Failure to enable fans.")
                .body(
                    "nvidia-zerodb: Failed to enable fans due to an error. Check logs for details.",
                )
                .appname("nvidia-zerodb")
                .timeout(0)
                .show();

            if let Err(notification_err) = res {
                log::error!(
                    "Failed to issue a notification about failing to enable the fans: {:?}",
                    notification_err
                );
            }
        } else if time_diff > self.shutdown_delay {
            log::error!(
                "Failed to re-enable fans during recovery period. Shutting down the computer."
            );
            let status = system_shutdown::force_shutdown();

            if let Err(shutdown_err) = status {
                log::error!("Failed to shutdown the computer: {:?}", shutdown_err);
            }
        }
    }

    pub fn on_fan_enable_success(&mut self) {
        self.last_fan_enable_error_time = None;
    }

    pub fn on_fan_disable_error(&mut self) {
        log::warn!("Issuing a notification about failing to disable the fans.");

        let res = Notification::new()
            .summary("nvidia-zerodb: Failure to disable fans.")
            .body("nvidia-zerodb: Failed to disable fans due to an error. Check logs for details.")
            .appname("nvidia-zerodb")
            .timeout(0)
            .show();

        if let Err(notification_err) = res {
            log::error!(
                "Failed to issue a notification about failing to disable the fans: {:?}",
                notification_err
            );
        }
    }

    pub fn on_fan_disable_success(&mut self) {
        // No-OP
    }
}
