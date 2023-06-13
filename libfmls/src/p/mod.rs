//! 平台支持代码

// cargo feature `gnu` 和 `openwrt` 只能选择一个
#[cfg(all(feature = "gnu", feature = "openwrt"))]
compile_error!("cargo feature `gnu` 和 `openwrt` 只能选择一个");

// GNU/Linux
#[cfg(feature = "gnu")]
pub mod avahi;
#[cfg(feature = "gnu")]
pub mod gnu;

// OpenWrt
#[cfg(all(target_os = "linux", feature = "openwrt"))]
pub mod openwrt;
#[cfg(all(target_os = "linux", feature = "openwrt"))]
pub mod umdns;

// Android
#[cfg(target_os = "android")]
pub mod android;

// Windows
#[cfg(target_os = "windows")]
pub mod win32;

// TODO
