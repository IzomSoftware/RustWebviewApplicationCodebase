// This part of source code is NOT compiled for Desktop
#![cfg(any(target_os = "android", target_os = "ios"))]
pub mod app;
pub mod platform;
pub mod utils;
pub mod entry_point;

/// Panic handler
fn stop_unwind<F: FnOnce() -> T, T>(f: F) -> T {
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)) {
        Ok(t) => t,
        Err(err) => {
            eprintln!("{err}");
            std::process::abort()
        }
    }
}

/// Mobile Activity start handler
fn _start_app() {
    stop_unwind(|| entry_point::init());
}

/// C Compatible entry point for mobile
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn start_app() {
    // PLATFORM SPECIFIC:
    // Prepare Android bindings if is
    // Running under it.
    #[cfg(target_os = "android")]
    {
        tao::android_binding!(
            net_izom,
            rust_application_codebase,
            WryActivity,
            wry::android_setup, // pass the wry::android_setup function to tao which will invoke when the event loop is created
            _start_app
        );
        wry::android_binding!(net_izom, rust_application_codebase);
    }
    #[cfg(not(target_os = "android"))]
    _start_app()
}
