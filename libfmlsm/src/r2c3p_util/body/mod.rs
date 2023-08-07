//! 消息的附加数据读取

use crate::r2c3p as p;

use crate::r2c3p_low::{index_of, n_u8};

#[cfg(feature = "r2c3p-c")]
mod conf;

#[cfg(feature = "r2c3p-c")]
pub use conf::{read_conf_v, ConfV};

/// 消息的附加数据的类型
#[derive(Debug, Clone, PartialEq)]
pub enum Body<'a> {
    /// 不支持 / 未知 / 无数据 / 格式错误
    None,
    /// `E`
    E {
        /// 错误码
        c: i16,
        /// 错误信息
        m: Option<&'a [u8]>,
    },
    /// `c`, `C`
    #[cfg(feature = "r2c3p-c")]
    C {
        /// 配置项名称
        k: &'a [u8],
        /// 配置项值
        v: Option<&'a [u8]>,
    },
    // TODO `V`
}

/// 读取附加数据
pub fn read_body<'a>(t: u8, b: &'a [u8]) -> Body<'a> {
    match t {
        // `E`
        p::MSGT_E => {
            // 寻找 ` ` 字符
            let (cb, m) = match index_of(b, p::BYTE_SPACE) {
                Some(i) => (&b[..i], Some(&b[(i + 1)..])),
                None => (&b[..], None),
            };
            // 错误码 < 0
            let c = if (cb.len() > 1) && (cb[0] == b'-') {
                match n_u8(&cb[1..]) {
                    Some(i) => Some(-(i as i16)),
                    None => None,
                }
            } else {
                match n_u8(cb) {
                    Some(i) => Some(i as i16),
                    None => None,
                }
            };
            if let Some(c) = c {
                return Body::E { c, m };
            }
        }
        // `c`, `C`
        #[cfg(feature = "r2c3p-c")]
        p::MSGT_C_R | p::MSGT_C => {
            // 寻找 `=` 字符
            let (k, v) = match index_of(b, p::BYTE_EQ) {
                Some(i) => (&b[..i], Some(&b[(i + 1)..])),
                None => (&b[..], None),
            };
            return Body::C { k, v };
        }
        _ => {}
    }
    // TODO `V`

    // 不支持 / 未知
    Body::None
}

#[cfg(test)]
mod test;
