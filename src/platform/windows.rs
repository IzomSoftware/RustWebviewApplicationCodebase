#![cfg(target_os = "windows")]

use crate::platform::PlatformBuilder;

pub struct WindowsPlatform;

impl PlatformBuilder for WindowsPlatform {}