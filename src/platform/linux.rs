#![cfg(target_os = "linux")]

use tao::{
    platform::unix::{WindowBuilderExtUnix, WindowExtUnix},
    window::{Window, WindowBuilder},
};
use wry::{WebView, WebViewBuilder, WebViewBuilderExtUnix};

use crate::platform::PlatformBuilder;

pub struct LinuxPlatform;

impl PlatformBuilder for LinuxPlatform {
    fn setup(&self) {
        // PLATFORM SPECIFIC:
        // Use x11 backend EVEN IF RUNNING UNDER WAYLAND
        // This is because there would be less bugs &
        // compatibility issues
        // #[cfg(target_os = "linux")]
        // unsafe {
        //     std::env::set_var("GDK_BACKEND", "x11");
        // }
    }
    fn setup_window(&self) -> WindowBuilder {
        WindowBuilder::new().with_default_vbox(true)
    }
    fn build_webview(&self, builder: WebViewBuilder<'_>, window: &Window) -> WebView {
        builder.build_gtk(window.default_vbox().unwrap()).unwrap()
    }
}
