mod windows;
mod linux;
mod android;

#[cfg(target_os = "windows")]
pub use windows::WindowsPlatform as CurrentPlatform;

#[cfg(target_os = "linux")]
pub use linux::LinuxPlatform as CurrentPlatform;

#[cfg(target_os = "android")]
pub use android::AndroidPlatform as CurrentPlatform;

use tao::{
    dpi::LogicalSize,
    event_loop::EventLoop,
    window::{Theme, Window, WindowBuilder},
};
use wry::{WebView, WebViewBuilder};
use crate::app::{webview::WebviewApp, app_window::ApplicationWindow};

pub struct Platform<T> {
    platform_builder: T,
}

impl<T: PlatformBuilder> Platform<T> {
    pub fn new(platform_builder: T) -> Self {
        Self { platform_builder }
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
            .build(event_loop).unwrap()
    }

    pub fn build(&self, event_loop: EventLoop<()>) -> ApplicationWindow {
        let window = self.build_window(&event_loop);

        ApplicationWindow {
            event_loop: Some(event_loop),
            webview: Some(self.platform_builder.build_webview(self.setup_webview(), &window)),
            window,
        }
    }
}

pub trait PlatformBuilder {
    fn setup() {}
    fn setup_window(&self) -> WindowBuilder {
        WindowBuilder::new()
    }
    fn build_webview(&self, builder: WebViewBuilder<'_>, window: &Window) -> WebView {
        builder.build(window).unwrap()
    }
}
