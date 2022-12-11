mod failsafes;
mod gpu;

use failsafes::Failsafes;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use std::{thread, time};

#[derive(Debug, PartialEq)]
enum FanState {
    Idle,
    Cooling,
    Unknown,
}

#[derive(Debug)]
pub struct Manager {
    fan_state: FanState,
    idle_temp: u64,
    cooling_temp: u64,
    refresh_delay: u64,
    failsafe: Failsafes,
}

impl Manager {
    pub fn new(
        idle_temp: u64,
        cooling_temp: u64,
        refresh_delay: u64,
        failsafe_shutdown_delay: u64,
    ) -> Self {
        Manager {
            idle_temp,
            cooling_temp,
            refresh_delay,
            fan_state: FanState::Unknown,
            failsafe: Failsafes::new(failsafe_shutdown_delay),
        }
    }

    pub fn start(&mut self, term_signal: Arc<AtomicBool>) -> () {
        log::info!("Starting GPU manager.");
        log::info!("GPU manager settings: {:?}", self);

        // Regardless of our next steps, enable the fans nad give control to vBIOS as a starting point
        self.enable_fans();

        // Wait for 1000ms before starting the loop
        // This comes from an nvidia-settings limitation, where 2 or more instant commands may end up not registering
        thread::sleep(time::Duration::from_millis(1000));

        // Loop while we're supposed to run
        while !term_signal.load(Ordering::Relaxed) {
            // Get current temperature
            let temp_res = gpu::get_gpu_temp();

            if let Ok(temp) = temp_res {
                if temp < self.idle_temp {
                    if self.fan_state != FanState::Idle {
                        log::info!(
                            "Reached idle temperature. Disabling fans. Current temperature: {}",
                            temp
                        );

                        self.disable_fans();
                    }
                }

                if temp > self.cooling_temp {
                    if self.fan_state != FanState::Cooling {
                        log::info!(
                            "Reached cooling temperature. Enabling fans. Current temperature: {}",
                            temp
                        );

                        self.enable_fans();
                    }
                }
            } else {
                log::error!("Failed to determine GPU temperature.");

                log::warn!("Enabling GPU fans due to failure to read the GPU temperature.");
                self.enable_fans();
            }

            // Wait for the next refresh
            thread::sleep(time::Duration::from_millis(self.refresh_delay));
        }

        // Re-enable fans once done
        log::warn!("Finished running fan manager. Re-enabling fans.");
        self.enable_fans();
    }

    fn enable_fans(&mut self) {
        let fan_enable_res = gpu::enable_fans();

        if let Err(fan_err) = fan_enable_res {
            log::error!("Failed to enable fans: {:?}", fan_err);

            // Report the error to the failsafe systems
            self.failsafe.on_fan_enable_error();
        } else {
            log::info!("Successfully enabled the fans.");

            self.failsafe.on_fan_enable_success();

            self.fan_state = FanState::Cooling;
        }
    }

    fn disable_fans(&mut self) {
        let fan_disable_res = gpu::disable_fans();

        if let Err(fan_err) = fan_disable_res {
            log::error!("Failed to disable fans: {:?}", fan_err);

            // Report the error to the failsafe systems
            self.failsafe.on_fan_disable_error();
        } else {
            log::info!("Successfully disabled the fans.");

            self.failsafe.on_fan_disable_success();

            self.fan_state = FanState::Idle;
        }
    }
}

impl Drop for Manager {
    fn drop(&mut self) {
        log::warn!("Re-enabling GPU fan control as part of the cleanup process for Manager.");

        // Make absolutely sure we re-enable the fans
        while self.fan_state != FanState::Cooling {
            self.enable_fans();
            thread::sleep(time::Duration::from_millis(self.refresh_delay));
        }
    }
}
