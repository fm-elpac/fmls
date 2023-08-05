//! 预定义的配置项 (内值配置处理)

use libfmlsc::r2c3p::ConfC;

/// 预定义的配置数据
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ConfData {
    /// 传输质量监测计数器: `cT`, `cR`, `cRd`, `cTB`, `cRB`
    #[cfg(feature = "r2c3p-cc")]
    pub tc: ConfC,
    /// 配置项 `I`
    #[cfg(feature = "r2c3p-i")]
    pub i: u64,
    /// 配置项 `O`
    #[cfg(feature = "r2c3p-o")]
    pub o: u8,
    /// 配置项 `On`
    #[cfg(feature = "r2c3p-o")]
    pub on: u8,
    /// 配置项 `@`
    #[cfg(feature = "r2c3p-at")]
    pub at: u8,
}

impl ConfData {
    pub const fn new() -> Self {
        Self {
            #[cfg(feature = "r2c3p-cc")]
            tc: ConfC {
                t: 0,
                r: 0,
                rd: 0,
                tb: 0,
                rb: 0,
            },
            #[cfg(feature = "r2c3p-i")]
            i: 0,
            #[cfg(feature = "r2c3p-o")]
            o: 0,
            #[cfg(feature = "r2c3p-o")]
            on: 0,
            #[cfg(feature = "r2c3p-at")]
            at: 0,
        }
    }
}
