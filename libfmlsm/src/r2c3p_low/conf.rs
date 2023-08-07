//! 对配置项的支持

use crate::r2c3p as p;

use super::index_of;

/// 读取配置消息 (`c`, `C`) 附加数据
///
/// 返回 `(k, v)`
pub fn read_conf(d: &[u8]) -> (&[u8], Option<&[u8]>) {
    match index_of(d, p::BYTE_EQ) {
        Some(i) => (&d[..i], Some(&d[(i + 1)..])),
        None => (d, None),
    }
}

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
}
