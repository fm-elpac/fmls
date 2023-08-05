//! 消息的附加数据读取

#[cfg(feature = "r2c3p-c")]
mod conf;

#[cfg(feature = "r2c3p-c")]
pub use conf::{read_conf_k, read_conf_v, ConfK, ConfV};

/// 消息的附加数据的类型
#[derive(Debug, Clone, PartialEq)]
pub enum Body<'a> {
    /// 不支持 / 未知 / 无数据
    None,
    /// `E`
    E {
        /// 错误码
        c: i8,
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

// TODO

#[cfg(test)]
mod test;
