mod android;
mod linux;
mod windows;

#[cfg(target_os = "windows")]
pub use windows::WindowsPlatform as CurrentPlatform;

#[cfg(target_os = "linux")]
pub use linux::LinuxPlatform as CurrentPlatform;

#[cfg(target_os = "android")]
pub use android::AndroidPlatform as CurrentPlatform;

use crate::app::{self, app_window::ApplicationWindow, webview::WebviewApp};
use tao::{
    dpi::LogicalSize,
    event_loop::EventLoop,
    window::{Theme, Window, WindowBuilder},
};
use wry::{WebView, WebViewBuilder};

pub struct Platform<T> {
    platform_builder: T,
}

impl<T: PlatformBuilder> Platform<T> {
    pub fn new(platform_builder: T) -> Self {
        Self { platform_builder }
    }

    fn setup_app(&self) {
        app::logging::init();
        self.platform_builder.setup();
    }
    fn setup_window(&self) -> WindowBuilder {
        self.platform_builder.setup_window()
    }
    fn setup_webview(&self) -> WebViewBuilder<'static> {
        WebviewApp::new().builder
    }
    fn build_window(&self, event_loop: &EventLoop<()>) -> Window {
        self.setup_window()
            .with_title("Rust Webview Application Codebase")
            .with_inner_size(LogicalSize::new(1024, 768))
            .with_min_inner_size(LogicalSize::new(320, 240))
            .with_theme(Some(Theme::Dark))
            .build(event_loop)
            .unwrap()
    }

    pub fn build(&self, event_loop: EventLoop<()>) -> ApplicationWindow {
        self.setup_app();

        let window = self.build_window(&event_loop);

        ApplicationWindow {
            event_loop: Some(event_loop),
            webview: Some(
                self.platform_builder
                    .build_webview(self.setup_webview(), &window),
            ),
            window,
        }
    }
}

pub trait PlatformBuilder {
    fn setup(&self) {}
    fn setup_window(&self) -> WindowBuilder {
        WindowBuilder::new()
    }
    fn build_webview(&self, builder: WebViewBuilder<'_>, window: &Window) -> WebView {
        builder.build(window).unwrap()
    }
}
