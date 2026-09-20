// This part of source code is NOT compiled for Desktop
#![cfg(any(target_os = "android", target_os = "ios"))]
pub mod app;
pub mod entry_point;
pub mod platform;
pub mod utils;

/// Panic handler
fn stop_unwind<F: FnOnce() -> T, T>(f: F) -> T {
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)) {
        Ok(t) => t,
        Err(err) => {
            eprintln!("{}", err
                .downcast_ref::<&'static str>()
                .map(|s| (*s).to_string())
                .or_else(|| err.downcast_ref::<String>().cloned())
                .unwrap_or_else(|| "".to_string()));
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
            rust_webview_application_codebase,
            Rust,
            wry::android_setup, // pass the wry::android_setup function to tao which will invoke when the event loop is created
            _start_app
        );
        wry::android_binding!(net_izom, rust_webview_application_codebase);
    }
    #[cfg(not(target_os = "android"))]
    _start_app()
}
