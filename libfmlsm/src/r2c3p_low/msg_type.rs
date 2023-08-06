//! 消息类型判断

use crate::r2c3p as p;

/// 消息类型分类
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MsgType {
    /// 请求消息
    Req,
    /// 响应消息
    Res,
    /// 静默消息
    S,
}

// 判断消息类型分类
impl From<u8> for MsgType {
    fn from(t: u8) -> Self {
        match t {
            p::MSGT_REQ_S..=p::MSGT_REQ_E => Self::Req,
            p::MSGT_RES_S..=p::MSGT_RES_E => Self::Res,
            _ => Self::S,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // 常见消息类型测试
    #[test]
    fn msg_type() {
        assert_eq!(MsgType::from(b'v'), MsgType::Req);
        assert_eq!(MsgType::from(b'c'), MsgType::Req);
        assert_eq!(MsgType::from(b'V'), MsgType::Res);
        assert_eq!(MsgType::from(b'E'), MsgType::Res);
        assert_eq!(MsgType::from(b'K'), MsgType::Res);
        assert_eq!(MsgType::from(b'C'), MsgType::Res);
        assert_eq!(MsgType::from(b'@'), MsgType::S);
    }

    // 边界情况测试
    #[test]
    fn msg_type_b() {
        assert_eq!(MsgType::from(b'a'), MsgType::Req);
        assert_eq!(MsgType::from(b'z'), MsgType::Req);
        assert_eq!(MsgType::from(b'A'), MsgType::Res);
        assert_eq!(MsgType::from(b'Z'), MsgType::Res);
        assert_eq!(MsgType::from(b'1'), MsgType::S);

        assert_eq!(MsgType::from(b'a' - 1), MsgType::S);
        assert_eq!(MsgType::from(b'z' + 1), MsgType::S);
        assert_eq!(MsgType::from(b'A' - 1), MsgType::S);
        assert_eq!(MsgType::from(b'Z' + 1), MsgType::S);
    }
}
