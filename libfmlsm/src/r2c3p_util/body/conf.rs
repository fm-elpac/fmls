//! 配置项数据

use crate::r2c3p_low::ConfK;

#[cfg(feature = "r2c3p-t")]
use crate::r2c3p_low::hex_u16;
#[cfg(any(feature = "r2c3p-cc", feature = "r2c3p-at"))]
use crate::r2c3p_low::hex_u32;
#[cfg(feature = "r2c3p-i")]
use crate::r2c3p_low::hex_u64;
#[cfg(feature = "r2c3p-o")]
use crate::r2c3p_low::hex_u8;
#[cfg(feature = "r2c3p-at")]
use crate::r2c3p_low::n_u8;

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read_conf_v() {
        assert_eq!(read_conf_v(ConfK::None, b""), ConfV::None);

        assert_eq!(read_conf_v(ConfK::T, b"1234"), ConfV::None);
        assert_eq!(read_conf_v(ConfK::T1, b"ab12"), ConfV::U16(0xab12));
        assert_eq!(read_conf_v(ConfK::CT, b"123456ab"), ConfV::U32(0x123456ab));
        assert_eq!(read_conf_v(ConfK::CR, b"789cdef0"), ConfV::U32(0x789cdef0));
        assert_eq!(read_conf_v(ConfK::CRd, b"ab123450"), ConfV::U32(0xab123450));
        assert_eq!(read_conf_v(ConfK::CTB, b"89cdef67"), ConfV::U32(0x89cdef67));
        assert_eq!(read_conf_v(ConfK::CRB, b"0010a000"), ConfV::U32(0x10a000));
        assert_eq!(
            read_conf_v(ConfK::I, b"123456789abcdef0"),
            ConfV::U64(0x123456789abcdef0)
        );
        assert_eq!(read_conf_v(ConfK::O, b"0a"), ConfV::U8(0x0a));
        assert_eq!(read_conf_v(ConfK::On, b"1c"), ConfV::U8(0x1c));
        assert_eq!(read_conf_v(ConfK::At, b"1"), ConfV::U8(1));
        assert_eq!(
            read_conf_v(ConfK::AtS(0), b"0123456a"),
            ConfV::U32(0x123456a)
        );
        assert_eq!(
            read_conf_v(ConfK::AtN(0), b"789bcdef"),
            ConfV::U32(0x789bcdef)
        );
    }
}
