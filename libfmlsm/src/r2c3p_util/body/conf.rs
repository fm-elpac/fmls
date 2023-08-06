//! 配置项数据

use libfmlsc::r2c3p as p;

use super::super::{hex_u16, hex_u32, hex_u64, hex_u8, n_u8};

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read_conf_k() {
        assert_eq!(read_conf_k(b""), ConfK::None);
        assert_eq!(read_conf_k(b"x"), ConfK::None);

        assert_eq!(read_conf_k(b"T"), ConfK::T);
        assert_eq!(read_conf_k(b"t"), ConfK::T1);
        assert_eq!(read_conf_k(b"cT"), ConfK::CT);
        assert_eq!(read_conf_k(b"cR"), ConfK::CR);
        assert_eq!(read_conf_k(b"cRd"), ConfK::CRd);
        assert_eq!(read_conf_k(b"cTB"), ConfK::CTB);
        assert_eq!(read_conf_k(b"cRB"), ConfK::CRB);
        assert_eq!(read_conf_k(b"I"), ConfK::I);
        assert_eq!(read_conf_k(b"O"), ConfK::O);
        assert_eq!(read_conf_k(b"On"), ConfK::On);
        assert_eq!(read_conf_k(b"@"), ConfK::At);
    }

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
