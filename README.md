# NVIDIA-ZERODB

A 0dB service for the `nvidia` driver.

Since the zero RPM fan mode doesn't always properly work with the `nvidia` driver, even when the GPU BIOS supports it (e.g. Zotac RTX 3080 AMP Holo). This service manually monitors GPU temperature via `nvidia-settings` and manually switches fan control from automatic to zero RPM, based on your configuration.

## WARNING

Disabling fans can potentially damage your hardware depending on your GPU type, cooling requirements and your configuration.

**By using this software you take full responsibility for the potential damage caused to your hardware.**

## Configuration

Edit `/etc/nvidia-zerodb.conf`:

```ini
[MAIN]
# Delay between checking the temperature in milliseconds
REFRESH_DELAY = 1000

[TEMPERATURES]
# Peak temperature in degrees C at which fans are disabled
MAX_TEMP = 50
# Minimal temperature in degrees C at which fans are active (after hitting MAX_TEMP)
MIN_TEMP = 40

[FAILSAFES]
# The time in milliseconds after a failed attempt to re-enable fans, after which system will be forced to shutdown
SHUTDOWN_DELAY = 30000
```

## Running

Due to `nvidia-settings` requiring a running instance of X.org server, it doesn't make sense to run the service as a root systemd service. On top of that, the additional functionality of D-Bus based desktop notifications depends on the service having access to user level D-Bus environment variables. That being said, the service will invoke `sudo` when running the actual fan control commands (since `nvidia-settings` requires it) so make sure that your user is whitelisted in `sudoers`.

Execute `systemctl --user start nvidia-zerodb.service` to start the daemon.

Optionally you can execute `systemctl --user enable nvidia-zerodb.service` to automatically start the service after a reboot.

Make sure to do a test run and check the logs with `journalctl --user --unit nvidia-zerodb.service` before enabling the service.


## License

Copyright © 2022, [Ruben Harutyunyan](https://github.com/Vagr9K/). Released under [GPLv3 license](./LICENSE).
