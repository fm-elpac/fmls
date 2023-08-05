//! 配置项数据

use libfmlsc::r2c3p as p;

use super::super::hex::{hex_u16, hex_u32, hex_u64, hex_u8, n_u8};

/// 预定义的配置项 (K)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConfK {
    /// 不支持 / 未知
    None,
    /// `T`
    #[cfg(feature = "r2c3p-t")]
    T,
    /// `t`
    #[cfg(feature = "r2c3p-t")]
    T1,
    /// `cT`
    #[cfg(feature = "r2c3p-cc")]
    CT,
    /// `cR`
    #[cfg(feature = "r2c3p-cc")]
    CR,
    /// `cRd`
    #[cfg(feature = "r2c3p-cc")]
    CRd,
    /// `cTB`
    #[cfg(feature = "r2c3p-cc")]
    CTB,
    /// `cRB`
    #[cfg(feature = "r2c3p-cc")]
    CRB,
    /// `I`
    #[cfg(feature = "r2c3p-i")]
    I,
    /// `O`
    #[cfg(feature = "r2c3p-o")]
    O,
    /// `On`
    #[cfg(feature = "r2c3p-o")]
    On,
    /// `@`
    #[cfg(feature = "r2c3p-at")]
    At,
    /// `@s`N
    #[cfg(feature = "r2c3p-at")]
    AtS(u8),
    /// `@n`N
    #[cfg(feature = "r2c3p-at")]
    AtN(u8),
}

/// 读取配置项的名称 (K)
pub fn read_conf_k(k: &[u8]) -> ConfK {
    // `t`
    #[cfg(feature = "r2c3p-t")]
    if p::CONF_T_1 == k {
        return ConfK::T1;
    }
    // `I`
    #[cfg(feature = "r2c3p-i")]
    if p::CONF_I == k {
        return ConfK::I;
    }
    // `T`
    #[cfg(feature = "r2c3p-t")]
    if p::CONF_T == k {
        return ConfK::T;
    }

    #[cfg(feature = "r2c3p-cc")]
    {
        // `cT`
        if p::CONF_CT == k {
            return ConfK::CT;
        }
        // `cR`
        if p::CONF_CR == k {
            return ConfK::CR;
        }
        // `cRd`
        if p::CONF_CRD == k {
            return ConfK::CRd;
        }
        // `cTB`
        if p::CONF_CTB == k {
            return ConfK::CTB;
        }
        // `cRB`
        if p::CONF_CRB == k {
            return ConfK::CRB;
        }
    }

    #[cfg(feature = "r2c3p-o")]
    {
        // `O`
        if p::CONF_O == k {
            return ConfK::O;
        }
        // `On`
        if p::CONF_ON == k {
            return ConfK::On;
        }
    }

    #[cfg(feature = "r2c3p-at")]
    {
        // `@`
        if p::CONF_AT == k {
            return ConfK::At;
        }
        // TODO `@s`N, `@n`N
    }

    // 不支持 / 未知
    ConfK::None
}

/// 预定义配置项的值
#[derive(Debug, Clone, PartialEq)]
pub enum ConfV {
    /// 不支持 / 未知 / 格式错误
    None,
    /// `u8`: `m`, `O`, `On`, `@`
    #[cfg(any(feature = "r2c3p-at", feature = "r2c3p-o"))]
    U8(u8),
    /// `u16`: `t`
    #[cfg(feature = "r2c3p-t")]
    U16(u16),
    /// `u32`: `cT`, `cR`, `cRd`, `cTB`, `cRB`, `@s`N, `@n`N
    #[cfg(any(feature = "r2c3p-at", feature = "r2c3p-cc"))]
    U32(u32),
    /// `u64`: I
    #[cfg(feature = "r2c3p-i")]
    U64(u64),
}

/// 读取配置项的值 (V)
pub fn read_conf_v(k: ConfK, v: &[u8]) -> ConfV {
    match k {
        // `T` (hex(`Vec<u8>`)) => None
        #[cfg(feature = "r2c3p-t")]
        ConfK::T1 => {
            // hex(`u16`)
            if let Some(u) = hex_u16(v) {
                return ConfV::U16(u);
            }
        }
        #[cfg(feature = "r2c3p-cc")]
        ConfK::CT | ConfK::CR | ConfK::CRd | ConfK::CTB | ConfK::CRB => {
            // hex(`u32`)
            if let Some(u) = hex_u32(v) {
                return ConfV::U32(u);
            }
        }
        #[cfg(feature = "r2c3p-i")]
        ConfK::I => {
            // hex(`u64`)
            if let Some(u) = hex_u64(v) {
                return ConfV::U64(u);
            }
        }
        #[cfg(feature = "r2c3p-o")]
        ConfK::O | ConfK::On => {
            // hex(`u8`)
            if let Some(u) = hex_u8(v) {
                return ConfV::U8(u);
            }
        }
        #[cfg(feature = "r2c3p-at")]
        ConfK::At => {
            // n(`u8`)
            if let Some(u) = n_u8(v) {
                return ConfV::U8(u);
            }
        }
        #[cfg(feature = "r2c3p-at")]
        ConfK::AtS(_) | ConfK::AtN(_) => {
            // hex(`u32`)
            if let Some(u) = hex_u32(v) {
                return ConfV::U32(u);
            }
        }
        _ => {}
    }

    // 不支持 / 未知
    ConfV::None
}
