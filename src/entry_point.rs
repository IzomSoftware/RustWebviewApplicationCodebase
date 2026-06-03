use std::sync::LazyLock;

use tao::event_loop::EventLoop;
use tokio::runtime::Runtime;

use crate::{app, platform::{self, CurrentPlatform}};

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
    // #[cfg(target_os = "linux")]
    // unsafe {
    //     std::env::set_var("GDK_BACKEND", "x11");
    // }

    app::logging::init();
    let platform = platform::Platform::new(CurrentPlatform);
    let app = platform.build(EventLoop::new());

    app.run();
}
