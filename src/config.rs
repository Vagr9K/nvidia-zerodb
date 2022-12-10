use anyhow::{Context, Result};
use ini::Ini;

pub struct Config {
    pub idle_temp: u64,
    pub cooling_temp: u64,
    pub refresh_delay: u64,
    pub failsafe_shutdown_delay: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            idle_temp: 40,
            cooling_temp: 50,
            refresh_delay: 1000,
            failsafe_shutdown_delay: 30000,
        }
    }
}

impl Config {
    pub fn load_from_file() -> Result<Self> {
        let mut base_conf = Self::default();

        let file_conf_path = "/etc/nvidia-zerodb.conf";

        log::info!("Loading configuration from {}", file_conf_path);
        let file_conf = Ini::load_from_file(file_conf_path)
            .context("Failed to load configuration from file.")?;

        // Main section
        let main_conf_section = file_conf.section(Some("MAIN"));
        if let Some(main_conf_section_data) = main_conf_section {
            let refresh_delay_data = main_conf_section_data.get("REFRESH_DELAY");

            if let Some(refresh_delay) = refresh_delay_data {
                base_conf.refresh_delay = refresh_delay
                    .parse::<u64>()
                    .context("Failed to parse REFRESH_DELAY setting.")?;

                log::info!(
                    "Loaded REFRESH_DELAY from configuration file: {}",
                    refresh_delay
                );
            };
        };

        // Temperature section
        let temp_conf_section = file_conf.section(Some("TEMPERATURES"));
        if let Some(temp_conf_section_data) = temp_conf_section {
            let max_temp_data = temp_conf_section_data.get("MAX_TEMP");

            if let Some(max_temp) = max_temp_data {
                base_conf.cooling_temp = max_temp
                    .parse::<u64>()
                    .context("Failed to parse MAX_TEMP setting.")?;

                log::info!("Loaded MAX_TEMP from configuration file: {}", max_temp);
            };

            let min_temp_data = temp_conf_section_data.get("MIN_TEMP");

            if let Some(min_temp) = min_temp_data {
                base_conf.idle_temp = min_temp
                    .parse::<u64>()
                    .context("Failed to parse MIN_TEMP setting.")?;

                log::info!("Loaded MIN_TEMP from configuration file: {}", min_temp);
            };
        };

        // Failsafes section
        let failsafes_conf_section = file_conf.section(Some("FAILSAFES"));
        if let Some(failsafes_conf_section_data) = failsafes_conf_section {
            let shutdown_delay_data = failsafes_conf_section_data.get("SHUTDOWN_DELAY");

            if let Some(shutdown_delay) = shutdown_delay_data {
                base_conf.failsafe_shutdown_delay = shutdown_delay
                    .parse::<u64>()
                    .context("Failed to parse SHUTDOWN_DELAY setting.")?;

                log::info!(
                    "Loaded SHUTDOWN_DELAY from configuration file: {}",
                    shutdown_delay
                );
            };
        };

        Ok(base_conf)
    }

    pub fn load_from_file_with_fallback() -> Self {
        let file_config = Self::load_from_file();

        match file_config {
            Ok(config) => config,
            Err(err) => {
                log::warn!("Failed to load configuration from file: {:?}", err);
                log::warn!("Using default configuration.");
                Self::default()
            }
        }
    }
}
