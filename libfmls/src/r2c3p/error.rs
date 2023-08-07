//! 错误信息

use std::error::Error;
use std::fmt::{Display, Formatter};

/// 接收消息的错误
#[derive(Debug, Clone, PartialEq)]
pub enum MsgRecvErr {
    /// E-2 消息太长
    E2,
    /// E-4 错误的消息格式
    /// (消息原始数据)
    E4(Vec<u8>),
}

impl Error for MsgRecvErr {}

impl Display for MsgRecvErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}
