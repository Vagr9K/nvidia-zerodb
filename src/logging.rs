pub fn init() {
    let mut builder = pretty_env_logger::formatted_builder();
    builder.filter_level(log::LevelFilter::Info);
    builder.init();
}

pub fn print_banner() {
    log::info!("nvidia-zerodb");

    log::info!("Build information:");

    log::info!("Version: {}", env!("VERGEN_BUILD_SEMVER"));

    log::info!("Source version: {}", env!("VERGEN_GIT_SEMVER"));
    log::info!("Source branch: {}", env!("VERGEN_GIT_BRANCH"));
    log::info!("Source timestamp: {}", env!("VERGEN_GIT_COMMIT_TIMESTAMP"));

    log::info!("Build timestamp: {}", env!("VERGEN_BUILD_TIMESTAMP"));
    log::info!("Build profile: {}", env!("VERGEN_CARGO_PROFILE"));
    log::info!("Build target: {}", env!("VERGEN_CARGO_TARGET_TRIPLE"));

    log::info!("Rust version: {}", env!("VERGEN_RUSTC_SEMVER"));
    log::info!("Rust LLVM version: {}", env!("VERGEN_RUSTC_LLVM_VERSION"));
    log::info!("Rust host arch: {}", env!("VERGEN_RUSTC_HOST_TRIPLE"));
    log::info!("Rust host OS: {}", env!("VERGEN_SYSINFO_OS_VERSION"));
}
