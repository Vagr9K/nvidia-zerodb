use anyhow::{bail, Context, Result};
use std::process::Command;
use std::str;

// Read current GPU temperature
pub fn get_gpu_temp() -> Result<u64> {
    log::debug!("Querying GPU temperature.");

    // Run the temperature read command
    let proc_res = Command::new("nvidia-settings")
        .arg("-c")
        .arg("0")
        .arg("-q")
        .arg("gpucoretemp")
        .arg("-t")
        .output()
        .context("Failed to execute GPU temperature query command.")?;

    // Make sure we didn't run into an error
    if !proc_res.status.success() {
        bail!("GPU temperature query command didn't exit properly.")
    }

    // Separate the relevant part
    let out_string = str::from_utf8(&proc_res.stdout)
        .context("Failed to convert process output to a valid string.")?;

    let temp_text = out_string.strip_suffix("\n").with_context(|| {
        format!(
            "Failed to strip suffix from the output string: {}",
            out_string
        )
    })?;

    // Parse and return
    let temp_num = temp_text
        .parse::<u64>()
        .with_context(|| format!("Failed to parse temperature report output: {}", temp_text))?;

    log::debug!("GPU temperature query response: {}", temp_num);

    Ok(temp_num)
}

// Disables GPU fans by switching them to manual control and settings the lowest possible PWM value
pub fn disable_fans() -> Result<()> {
    log::info!("Disabling fans by taking over fan control.");

    // Run the command for switching off the fans
    let proc_res = Command::new("sudo")
        .arg("nvidia-settings")
        .arg("-c")
        .arg("0")
        .arg("-a")
        .arg("[gpu:0]/GPUFanControlState=1")
        .arg("-a")
        .arg("[fan:0]/GPUTargetFanSpeed=30")
        .arg("-a")
        .arg("[fan:1]/GPUTargetFanSpeed=30")
        .output()
        .expect("Failed to disable GPU fans.");

    // Make sure we didn't run into an error
    if !proc_res.status.success() {
        bail!("GPU fan disable command didn't exit properly.")
    }

    // Convert output to a string
    let out_string = str::from_utf8(&proc_res.stdout)
        .context("Failed to convert fan disable command output to a string.")?;

    // Calculate the expected output
    let expected_output = format!("\n  Attribute 'GPUFanControlState' ([gpu:0]) assigned value 1.\n\n  Attribute 'GPUTargetFanSpeed' ([fan:0]) assigned value 30.\n\n  Attribute 'GPUTargetFanSpeed' ([fan:1]) assigned value 30.\n\n",  );

    // Make sure that we got the expected output
    if out_string != expected_output {
        bail!(
            "Unexpected output when trying to disable fans: {}",
            out_string
        )
    };

    Ok(())
}

// Switches GPU fan control to vBIOS
pub fn enable_fans() -> Result<()> {
    log::info!("Enabling fans by giving away control to vBIOS.");

    // Run the command for switching fan contorl to vBIOS
    let proc_res = Command::new("sudo")
        .arg("nvidia-settings")
        .arg("-c")
        .arg("0")
        .arg("-a")
        .arg("[gpu:0]/GPUFanControlState=0")
        .arg("-t")
        .output()
        .expect("Failed to re-enable GPU fans.");

    // Make sure we didn't run into an error
    if !proc_res.status.success() {
        bail!("GPU vBIOS control command didn't exit properly.")
    }

    // Convert output to a string
    let out_string = str::from_utf8(&proc_res.stdout)
        .context("Failed to convert fan disable command output to a string.")?;

    // Calculate the expected output
    let expected_output =
        format!("\n  Attribute 'GPUFanControlState' ([gpu:0]) assigned value 0.\n\n",);

    // Make sure that we got the expected output
    if out_string != expected_output {
        bail!(
            "Unexpected output when trying to enable fans: {}",
            out_string
        )
    };

    Ok(())
}
