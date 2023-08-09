//! 配置信息

/// 固件名称及版本
pub static FW_VER: &[u8] = b"sled 0.1.0";

#[cfg(all(
    feature = "ch32v003",
    not(feature = "ch32v003f4p6"),
    not(feature = "ch32v003j4m6")
))]
pub static HW_NAME: &[u8] = b"ch32v003";

/// 硬件名称 (MCU)
#[cfg(feature = "ch32v003f4p6")]
pub static HW_NAME: &[u8] = b"ch32v003f4p6";

#[cfg(feature = "ch32v003j4m6")]
pub static HW_NAME: &[u8] = b"ch32v003j4m6";

// TODO
