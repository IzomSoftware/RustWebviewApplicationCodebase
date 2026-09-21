/// Init our logger
pub fn init() {
    // PLATFORM SPECIFIC:
    // Set up the Android logger if we're running under Android
    // I'm not really willing to change any format or whatever
    // for android, because if u connect to logcat u would understand
    // my point.
    #[cfg(target_os = "android")]
    {
        android_logger::init_once(
            android_logger::Config::default()
                .with_max_level(log::LevelFilter::Info)
                .with_tag("rust-webview-application-codebase"),
        )
    }
    // Setup env_logger if we're not running under Android
    #[cfg(not(target_os = "android"))]
    {
        use env_logger::Builder;
        use log::LevelFilter;
        use std::io::Write;

        Builder::new()
            .filter_level(LevelFilter::Debug)
            .target(env_logger::Target::Stdout)
            .format(|formatter, record| {
                writeln!(
                    formatter,
                    "[{}] [{}] {}",
                    record.module_path_static().unwrap_or(""),
                    record.level(),
                    record.args()
                )
            })
            .init()
    }
}
