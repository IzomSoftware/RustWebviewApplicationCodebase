use std::sync::LazyLock;

use crate::{logging_initializer, window_manager};
use tokio::runtime::Runtime;

// The Tokio runtime. this runtime could be initialized only once and that's the reason we're wrapping this
pub static RUNTIME: LazyLock<Runtime> = LazyLock::new(|| {
    Runtime::new()
        // # Safety
        //
        // We need the tokio runtime for the application to function
        // And this LazyLock would be poisoned if we don't crash here
        .expect("Couldn't initialize the tokio runtime")
});

/// The entry point
pub fn init() {
    // PLATFORM SPECIFIC:
    // Use x11 backend EVEN IF RUNNING UNDER WAYLAND
    // This is because there would be less bugs &
    // compatibility issues
    #[cfg(target_os = "linux")]
    unsafe {
        std::env::set_var("GDK_BACKEND", "x11");
    }

    logging_initializer::init();
    window_manager::init();
}
