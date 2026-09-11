#![cfg(target_os = "windows")]

use crate::platform::PlatformBuilder;

pub struct WindowsPlatform;

impl PlatformBuilder for WindowsPlatform {
    fn setup_window(&self) -> WindowBuilder {
        WindowBuilder::new()
            .with_desktop_default_size()
    }
}