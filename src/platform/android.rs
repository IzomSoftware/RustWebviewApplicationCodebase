#![cfg(target_os = "android")]

use crate::platform::PlatformBuilder;

pub struct AndroidPlatform;

impl PlatformBuilder for AndroidPlatform {}