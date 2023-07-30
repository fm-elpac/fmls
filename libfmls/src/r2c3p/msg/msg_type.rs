//! 消息类型

use libfmlsc::r2c3p as p;

/// 消息类型
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MsgType {
    /// 请求消息
    Req(MsgTypeReq),
    /// 响应消息
    Res(MsgTypeRes),
    /// 静默消息
    S(MsgTypeS),
}

// 单个字节 (`u8`) 转换为消息类型
impl From<u8> for MsgType {
    fn from(i: u8) -> Self {
        match i {
            p::MSGT_REQ_S..=p::MSGT_REQ_E => Self::Req(MsgTypeReq::from(i)),
            p::MSGT_RES_S..=p::MSGT_RES_E => Self::Res(MsgTypeRes::from(i)),
            _ => Self::S(MsgTypeS::from(i)),
        }
    }
}

// 消息类型转换为 `u8`
impl From<MsgType> for u8 {
    fn from(t: MsgType) -> u8 {
        match t {
            MsgType::Req(i) => i.into(),
            MsgType::Res(i) => i.into(),
            MsgType::S(i) => i.into(),
        }
    }
}

/// 消息类型 请求消息
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MsgTypeReq {
    /// 自定义类型
    Type(u8),
    /// 预定义类型 `v`
    V,
    /// 预定义类型 `c`
    C,
}

// `u8` 转换为请求消息类型
impl From<u8> for MsgTypeReq {
    fn from(i: u8) -> Self {
        match i {
            p::MSGT_V_R => Self::V,
            p::MSGT_C_R => Self::C,
            _ => Self::Type(i),
        }
    }
}

// 请求消息类型转换为 `u8`
impl From<MsgTypeReq> for u8 {
    fn from(t: MsgTypeReq) -> u8 {
        match t {
            MsgTypeReq::Type(i) => i,
            MsgTypeReq::V => p::MSGT_V_R,
            MsgTypeReq::C => p::MSGT_C_R,
        }
    }
}

/// 消息类型 响应消息
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MsgTypeRes {
    /// 自定义类型
    Type(u8),
    /// 预定义类型 `V`
    V,
    /// 预定义类型 `E`
    E,
    /// 预定义类型 `K`
    K,
    /// 预定义类型 `C`
    C,
}

// `u8` 转换为响应消息类型
impl From<u8> for MsgTypeRes {
    fn from(i: u8) -> Self {
        match i {
            p::MSGT_V => Self::V,
            p::MSGT_E => Self::E,
            p::MSGT_K => Self::K,
            p::MSGT_C => Self::C,
            _ => Self::Type(i),
        }
    }
}

// 响应消息类型转换为 `u8`
impl From<MsgTypeRes> for u8 {
    fn from(t: MsgTypeRes) -> u8 {
        match t {
            MsgTypeRes::Type(i) => i,
            MsgTypeRes::V => p::MSGT_V,
            MsgTypeRes::E => p::MSGT_E,
            MsgTypeRes::K => p::MSGT_K,
            MsgTypeRes::C => p::MSGT_C,
        }
    }
}

/// 消息类型 静默消息
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MsgTypeS {
    /// 自定义类型
    Type(u8),
    /// 预定义类型 `@`
    At,
}

// `u8` 转换为静默消息类型
impl From<u8> for MsgTypeS {
    fn from(i: u8) -> Self {
        match i {
            p::MSGT_AT => Self::At,
            _ => Self::Type(i),
        }
    }
}

// 静默消息类型转换为 `u8`
impl From<MsgTypeS> for u8 {
    fn from(t: MsgTypeS) -> u8 {
        match t {
            MsgTypeS::Type(i) => i,
            MsgTypeS::At => p::MSGT_AT,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // MsgType 转为 `u8`
    #[test]
    fn type_u8() {
        assert_eq!(
            <MsgType as Into<u8>>::into(MsgType::Req(MsgTypeReq::V)),
            b'v'
        );
        assert_eq!(
            <MsgType as Into<u8>>::into(MsgType::Req(MsgTypeReq::C)),
            b'c'
        );
        assert_eq!(
            <MsgType as Into<u8>>::into(MsgType::Req(MsgTypeReq::Type(b'a'))),
            b'a'
        );
        assert_eq!(
            <MsgType as Into<u8>>::into(MsgType::Res(MsgTypeRes::V)),
            b'V'
        );
        assert_eq!(
            <MsgType as Into<u8>>::into(MsgType::Res(MsgTypeRes::E)),
            b'E'
        );
        assert_eq!(
            <MsgType as Into<u8>>::into(MsgType::Res(MsgTypeRes::K)),
            b'K'
        );
        assert_eq!(
            <MsgType as Into<u8>>::into(MsgType::Res(MsgTypeRes::C)),
            b'C'
        );
        assert_eq!(
            <MsgType as Into<u8>>::into(MsgType::Res(MsgTypeRes::Type(b'A'))),
            b'A'
        );
        assert_eq!(<MsgType as Into<u8>>::into(MsgType::S(MsgTypeS::At)), b'@');
        assert_eq!(
            <MsgType as Into<u8>>::into(MsgType::S(MsgTypeS::Type(b'6'))),
            b'6'
        );
    }

    // `u8` 转为 MsgType
    #[test]
    fn u8_type() {
        assert_eq!(MsgType::from(b'v'), MsgType::Req(MsgTypeReq::V));
        assert_eq!(MsgType::from(b'V'), MsgType::Res(MsgTypeRes::V));
        assert_eq!(MsgType::from(b'E'), MsgType::Res(MsgTypeRes::E));
        assert_eq!(MsgType::from(b'K'), MsgType::Res(MsgTypeRes::K));
        assert_eq!(MsgType::from(b'c'), MsgType::Req(MsgTypeReq::C));
        assert_eq!(MsgType::from(b'C'), MsgType::Res(MsgTypeRes::C));
        assert_eq!(MsgType::from(b'@'), MsgType::S(MsgTypeS::At));
    }

    // 边界情况
    #[test]
    fn u8_b() {
        assert_eq!(MsgType::from(b'a'), MsgType::Req(MsgTypeReq::Type(b'a')));
        assert_eq!(MsgType::from(b'z'), MsgType::Req(MsgTypeReq::Type(b'z')));
        assert_eq!(MsgType::from(b'A'), MsgType::Res(MsgTypeRes::Type(b'A')));
        assert_eq!(MsgType::from(b'Z'), MsgType::Res(MsgTypeRes::Type(b'Z')));
        assert_eq!(MsgType::from(b'1'), MsgType::S(MsgTypeS::Type(b'1')));

        assert_eq!(
            MsgType::from(b'a' - 1),
            MsgType::S(MsgTypeS::Type(b'a' - 1))
        );
        assert_eq!(
            MsgType::from(b'z' + 1),
            MsgType::S(MsgTypeS::Type(b'z' + 1))
        );
        assert_eq!(
            MsgType::from(b'A' - 2),
            MsgType::S(MsgTypeS::Type(b'A' - 2))
        );
        assert_eq!(
            MsgType::from(b'Z' + 1),
            MsgType::S(MsgTypeS::Type(b'Z' + 1))
        );
    }
}
