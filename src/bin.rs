// This part of source code ONLY works for desktop,
// Even tho it technically gets executed on all
// Platforms.
pub mod entry_point;
pub mod logging_initializer;
pub mod assets_bundled_manager;
pub mod window_manager;
pub mod webview_manager;

/// Desktop entry point
fn main() {
    // PLATFORM SPECIFIC:
    // I don't know how the fuck we're running under mobile phone's here
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    {
        entry_point::init();
    }
}
