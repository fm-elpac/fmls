//! 平台支持代码
//!
//! 此模块的对外接口做到平台无关

// 平台和 cargo feature 检查代码

// `gnu` 和 `openwrt` 不能同时使用
#[cfg(all(feature = "gnu", feature = "openwrt"))]
compile_error!("cargo feature `gnu` 和 `openwrt` 只能选择一个");
// 在 linux 上必须启用一个 `gnu` 或 `openwrt`
#[cfg(all(target_os = "linux", not(feature = "gnu"), not(feature = "openwrt")))]
compile_error!("cargo feature `gnu` 和 `openwrt` 请启用一个");
// 在 Windows 上不能启用 `gnu` 和 `openwrt`
#[cfg(all(target_os = "windows", feature = "gnu"))]
compile_error!("cargo feature `gnu` 不能在 Windows 上启用");
#[cfg(all(target_os = "windows", feature = "openwrt"))]
compile_error!("cargo feature `openwrt` 不能在 Windows 上启用");

// GNU/Linux
#[cfg(feature = "gnu")]
mod gnu;
#[cfg(feature = "gnu")]
pub use gnu::*;

// OpenWrt
#[cfg(all(target_os = "linux", feature = "openwrt"))]
mod openwrt;
#[cfg(all(target_os = "linux", feature = "openwrt"))]
pub use openwrt::*;

// Android
#[cfg(all(target_os = "android", not(feature = "gnu")))]
mod android;
#[cfg(all(target_os = "android", not(feature = "gnu")))]
pub use android::*;

// Windows
#[cfg(target_os = "windows")]
mod win;
#[cfg(target_os = "windows")]
pub use win::*;
