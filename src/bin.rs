// This part of source code is NOT compiled for mobile & etc
pub mod app;
pub mod platform;
pub mod utils;
pub mod entry_point;

/// Desktop entry point
fn main() {
    // PLATFORM SPECIFIC:
    // I don't know how we're running under mobile phone's here
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    {
        entry_point::init();
    }
}
