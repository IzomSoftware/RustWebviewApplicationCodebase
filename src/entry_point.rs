use std::sync::LazyLock;

use tao::event_loop::EventLoop;
use tokio::runtime::Runtime;

use crate::{platform::{self, CurrentPlatform}};

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
    let platform = platform::Platform::new(CurrentPlatform);
    let app = platform.build(EventLoop::new());

    app.run();
}
