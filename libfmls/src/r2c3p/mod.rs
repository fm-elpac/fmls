//! fmls_r2c3p 协议

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

impl From<u8> for MsgType {
    fn from(i: u8) -> Self {
        match i {
            p::MSGT_REQ_S..=p::MSGT_REQ_E => Self::Req(MsgTypeReq::from(i)),
            p::MSGT_RES_S..=p::MSGT_RES_E => Self::Res(MsgTypeRes::from(i)),
            _ => Self::S(MsgTypeS::from(i)),
        }
    }
}

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
    /// 预定义类型 `v` 请求消息
    V,
    /// 预定义类型 `c` 请求消息
    C,
}

impl From<u8> for MsgTypeReq {
    fn from(i: u8) -> Self {
        match i {
            p::MSGT_V_R => Self::V,
            p::MSGT_C_R => Self::C,
            _ => Self::Type(i),
        }
    }
}

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
}

impl From<u8> for MsgTypeS {
    fn from(i: u8) -> Self {
        Self::Type(i)
    }
}

impl From<MsgTypeS> for u8 {
    fn from(t: MsgTypeS) -> u8 {
        match t {
            MsgTypeS::Type(i) => i,
        }
    }
}

/// 一条消息
#[derive(Clone, Debug, PartialEq)]
pub enum Msg {
    /// 自定义消息
    A(MsgA),
    /// 预定义 请求消息
    Req(MsgReq),
    /// 预定义 响应消息
    Res(MsgRes),
}

impl Msg {
    /// 获取消息类型
    pub fn t(&self) -> MsgType {
        match self {
            Msg::A(i) => i.t(),
            Msg::Req(i) => i.t(),
            Msg::Res(i) => i.t(),
        }
    }
}

impl From<Msg> for Vec<u8> {
    fn from(m: Msg) -> Vec<u8> {
        match m {
            Msg::A(i) => i.into(),
            Msg::Req(i) => i.into(),
            Msg::Res(i) => i.into(),
        }
    }
}

/// 自定义消息
#[derive(Clone, Debug, PartialEq)]
pub struct MsgA {
    /// 消息类型
    pub t: MsgType,
    /// 附加数据 (可选)
    pub data: Option<Vec<u8>>,
}

impl MsgA {
    /// 获取消息类型
    pub fn t(&self) -> MsgType {
        self.t
    }
}

impl From<MsgA> for Vec<u8> {
    fn from(m: MsgA) -> Vec<u8> {
        let mut v: Vec<u8> = vec![m.t.into()];
        match m.data {
            Some(b) => {
                v.extend_from_slice(&b);
            }
            None => {}
        }
        v
    }
}

impl From<Vec<u8>> for MsgA {
    fn from(b: Vec<u8>) -> Self {
        if b.len() < 1 {
            return Self {
                t: MsgType::from(0),
                data: None,
            };
        }

        let data = if b.len() > 1 {
            Some(Vec::from(&b[1..]))
        } else {
            None
        };

        Self {
            t: MsgType::from(b[0]),
            data,
        }
    }
}

/// 预定义 请求消息
#[derive(Clone, Debug, PartialEq)]
pub enum MsgReq {
    V,
    C(ConfItem),
}

impl MsgReq {
    /// 获取消息类型
    pub fn t(&self) -> MsgType {
        match self {
            MsgReq::V => MsgType::Req(MsgTypeReq::V),
            MsgReq::C(_) => MsgType::Req(MsgTypeReq::C),
        }
    }
}

impl From<MsgReq> for Vec<u8> {
    fn from(m: MsgReq) -> Vec<u8> {
        match m {
            MsgReq::V => vec![MsgTypeReq::V.into()],
            MsgReq::C(i) => {
                let mut v = vec![MsgTypeReq::C.into()];
                let b: Vec<u8> = i.into();
                v.extend_from_slice(&b);
                v
            }
        }
    }
}

/// 预定义 响应消息
#[derive(Clone, Debug, PartialEq)]
pub enum MsgRes {
    /// `V` 版本信息
    V(MsgResV),
    /// `E` (错误码, 错误信息)
    E((i8, Option<Vec<u8>>)),
    /// `K` 附加数据
    K(Option<Vec<u8>>),
    /// `CK=V`
    C(ConfItem),
}

impl MsgRes {
    /// 获取消息类型
    pub fn t(&self) -> MsgType {
        match self {
            MsgRes::V(_) => MsgType::Res(MsgTypeRes::V),
            MsgRes::E(_) => MsgType::Res(MsgTypeRes::E),
            MsgRes::K(_) => MsgType::Res(MsgTypeRes::K),
            MsgRes::C(_) => MsgType::Res(MsgTypeRes::C),
        }
    }
}

impl From<MsgRes> for Vec<u8> {
    fn from(m: MsgRes) -> Vec<u8> {
        let mut v: Vec<u8> = Vec::new();
        match m {
            MsgRes::V(i) => {
                v.push(MsgTypeRes::V.into());
                let b: Vec<u8> = i.into();
                v.extend_from_slice(&b);
            }
            MsgRes::E(i) => {
                v.push(MsgTypeRes::E.into());
                v.extend_from_slice(&Vec::from(format!("{}", i.0)));
                match i.1 {
                    Some(e) => {
                        v.push(p::BYTE_SPACE);
                        v.extend_from_slice(&e);
                    }
                    None => {}
                }
            }
            MsgRes::K(i) => {
                v.push(MsgTypeRes::K.into());
                match i {
                    Some(e) => {
                        v.extend_from_slice(&e);
                    }
                    None => {}
                }
            }
            MsgRes::C(i) => {
                v.push(MsgTypeRes::C.into());
                let s = i.is_set();
                let b: Vec<u8> = i.into();
                v.extend_from_slice(&b);
                if !s {
                    v.push(p::BYTE_EQ);
                }
            }
        }
        v
    }
}

/// 配置项
#[derive(Clone, Debug, PartialEq)]
pub enum ConfItem {
    /// 自定义配置项 (K, V)
    K((Vec<u8>, Option<Vec<u8>>)),
    /// 预定义配置 `m`
    M1(Option<u8>),
    /// 预定义配置 `T`
    T(Option<Vec<u8>>),
    /// 预定义配置 `t`
    T1(Option<u16>),
    /// 预定义配置 `c*` 计数器 (K, V)
    C((&'static [u8], Option<u32>)),
}

impl ConfItem {
    /// 返回 K
    pub fn k(&self) -> &[u8] {
        match self {
            ConfItem::K(i) => &i.0,
            ConfItem::M1(_) => p::CONF_M_1,
            ConfItem::T(_) => p::CONF_T,
            ConfItem::T1(_) => p::CONF_T_1,
            ConfItem::C(i) => i.0,
        }
    }

    /// 返回 V
    pub fn v(&self) -> Option<Vec<u8>> {
        match self {
            ConfItem::K(i) => i.1.clone(),
            ConfItem::M1(i) => match i {
                Some(t) => Some(Vec::from(format!("{}", t))),
                None => None,
            },
            ConfItem::T(i) => i.clone(),
            ConfItem::T1(i) => match i {
                Some(t) => Some(Vec::from(format!("{:04x}", t))),
                None => None,
            },
            ConfItem::C(i) => match i.1 {
                Some(n) => Some(Vec::from(format!("{:08x}", n))),
                None => None,
            },
        }
    }

    /// 返回是否为 `cK=V` 格式
    pub fn is_set(&self) -> bool {
        match self {
            ConfItem::K(i) => i.1.is_some(),
            ConfItem::M1(i) => i.is_some(),
            ConfItem::T(i) => i.is_some(),
            ConfItem::T1(i) => i.is_some(),
            ConfItem::C(i) => i.1.is_some(),
        }
    }
}

impl From<ConfItem> for Vec<u8> {
    fn from(i: ConfItem) -> Vec<u8> {
        let mut v: Vec<u8> = Vec::new();
        v.extend_from_slice(i.k());
        let d = i.v();
        match d {
            Some(b) => {
                v.push(p::BYTE_EQ);
                v.extend_from_slice(&b);
            }
            None => {}
        }
        v
    }
}

impl From<Vec<u8>> for ConfItem {
    fn from(b: Vec<u8>) -> Self {
        let eq = b.iter().position(|n| *n == p::BYTE_EQ);
        let (k, v) = match eq {
            Some(i) => (&b[..i], Some(Vec::from(&b[(i + 1)..]))),
            None => {
                let v: Option<Vec<u8>> = None;
                (&b[..], v)
            }
        };

        // 由于无法返回错误, 此处不能进行预定义配置项的解析
        ConfItem::K((Vec::from(k), v))
    }
}

/// 预定义响应消息 `V`
#[derive(Clone, Debug, PartialEq)]
pub struct MsgResV {
    /// 原始附加数据
    pub raw: Option<Vec<u8>>,
    /// fmls_r2c3p 协议的版本
    pub p: Vec<u8>,
    /// 固件名称及版本
    pub firmware: Vec<u8>,
    /// 硬件信息及编号
    pub hardware: Vec<u8>,
}

impl From<MsgResV> for Vec<u8> {
    fn from(m: MsgResV) -> Vec<u8> {
        let mut v: Vec<u8> = Vec::new();
        v.extend_from_slice(p::P_VERSION);
        v.push(p::BYTE_LF);
        v.extend_from_slice(&m.firmware);
        v.push(p::BYTE_LF);
        v.extend_from_slice(&m.hardware);
        v
    }
}

// server
mod server;

pub use server::R2c3pServer;

// TODO

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn msg_type() {
        assert_eq!(MsgType::from(b'v'), MsgType::Req(MsgTypeReq::V));
        assert_eq!(MsgType::from(b'V'), MsgType::Res(MsgTypeRes::V));
        assert_eq!(MsgType::from(b'E'), MsgType::Res(MsgTypeRes::E));
        assert_eq!(MsgType::from(b'K'), MsgType::Res(MsgTypeRes::K));
        assert_eq!(MsgType::from(b'c'), MsgType::Req(MsgTypeReq::C));
        assert_eq!(MsgType::from(b'C'), MsgType::Res(MsgTypeRes::C));

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
        assert_eq!(
            <MsgType as Into<u8>>::into(MsgType::S(MsgTypeS::Type(b'6'))),
            b'6'
        );
    }

    #[test]
    fn msg_type_b() {
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
            MsgType::from(b'A' - 1),
            MsgType::S(MsgTypeS::Type(b'A' - 1))
        );
        assert_eq!(
            MsgType::from(b'Z' + 1),
            MsgType::S(MsgTypeS::Type(b'Z' + 1))
        );
    }

    #[test]
    fn msg_to_u8() {
        // Msg::Req
        assert_eq!(<Msg as Into<Vec<u8>>>::into(Msg::Req(MsgReq::V)), b"v");
        assert_eq!(
            <Msg as Into<Vec<u8>>>::into(Msg::Req(MsgReq::C(ConfItem::M1(None)))),
            b"cm"
        );
        assert_eq!(
            <Msg as Into<Vec<u8>>>::into(Msg::Req(MsgReq::C(ConfItem::M1(Some(1))))),
            b"cm=1"
        );
        assert_eq!(
            <Msg as Into<Vec<u8>>>::into(Msg::Req(MsgReq::C(ConfItem::T(None)))),
            b"cT"
        );
        assert_eq!(
            <Msg as Into<Vec<u8>>>::into(Msg::Req(MsgReq::C(ConfItem::T1(None)))),
            b"ct"
        );
        assert_eq!(
            <Msg as Into<Vec<u8>>>::into(Msg::Req(MsgReq::C(ConfItem::C((p::CONF_CT, None))))),
            b"ccT"
        );

        // Msg::Res
        assert_eq!(
            <Msg as Into<Vec<u8>>>::into(Msg::Res(MsgRes::V(MsgResV {
                raw: None,
                p: Vec::new(),
                firmware: <Vec<u8> as From<&[u8]>>::from(b"sled 0.1.0"),
                hardware: <Vec<u8> as From<&[u8]>>::from(b"ch32v003 666"),
            }))),
            b"Vfmls_r2c3p 0.1.0\nsled 0.1.0\nch32v003 666"
        );
        assert_eq!(
            <Msg as Into<Vec<u8>>>::into(Msg::Res(MsgRes::E((
                -2,
                Some(<Vec<u8> as From<&[u8]>>::from(b"32"))
            )))),
            b"E-2 32"
        );
        assert_eq!(
            <Msg as Into<Vec<u8>>>::into(Msg::Res(MsgRes::K(None))),
            b"K"
        );
        assert_eq!(
            <Msg as Into<Vec<u8>>>::into(Msg::Res(MsgRes::C(ConfItem::M1(Some(0))))),
            b"Cm=0"
        );
        assert_eq!(
            <Msg as Into<Vec<u8>>>::into(Msg::Res(MsgRes::C(ConfItem::T(Some(
                <Vec<u8> as From<&[u8]>>::from(b"000011112222")
            ))))),
            b"CT=000011112222"
        );
        assert_eq!(
            <Msg as Into<Vec<u8>>>::into(Msg::Res(MsgRes::C(ConfItem::T1(Some(0xac03))))),
            b"Ct=ac03"
        );
        assert_eq!(
            <Msg as Into<Vec<u8>>>::into(Msg::Res(MsgRes::C(ConfItem::C((
                p::CONF_CR,
                Some(0x10ce)
            ))))),
            b"CcR=000010ce"
        );

        // ConfItem::K
        assert_eq!(
            <Msg as Into<Vec<u8>>>::into(Msg::Req(MsgReq::C(ConfItem::K((
                <Vec<u8> as From<&[u8]>>::from(b"a"),
                None
            ))))),
            b"ca"
        );
        assert_eq!(
            <Msg as Into<Vec<u8>>>::into(Msg::Res(MsgRes::C(ConfItem::K((
                <Vec<u8> as From<&[u8]>>::from(b"a"),
                Some(<Vec<u8> as From<&[u8]>>::from(b"1")),
            ))))),
            b"Ca=1"
        );

        // MsgA
        assert_eq!(
            <Msg as Into<Vec<u8>>>::into(Msg::A(MsgA {
                t: MsgType::from(b'_'),
                data: None,
            })),
            b"_"
        );
        assert_eq!(
            <Msg as Into<Vec<u8>>>::into(Msg::A(MsgA {
                t: MsgType::from(b'_'),
                data: Some(<Vec<u8> as From<&[u8]>>::from(b"test 666")),
            })),
            b"_test 666"
        );
    }

    #[test]
    fn u8_to_msg() {
        // MsgA
        assert_eq!(
            MsgA::from(<Vec<u8> as From<&[u8]>>::from(b"_666")),
            MsgA {
                t: MsgType::S(MsgTypeS::Type(b'_')),
                data: Some(<Vec<u8> as From<&[u8]>>::from(b"666")),
            }
        );
        assert_eq!(
            MsgA::from(<Vec<u8> as From<&[u8]>>::from(b"2")),
            MsgA {
                t: MsgType::S(MsgTypeS::Type(b'2')),
                data: None,
            }
        );

        // ConfItem
        assert_eq!(
            ConfItem::from(<Vec<u8> as From<&[u8]>>::from(b"a")),
            ConfItem::K((<Vec<u8> as From<&[u8]>>::from(b"a"), None))
        );
        assert_eq!(
            ConfItem::from(<Vec<u8> as From<&[u8]>>::from(b"a=1")),
            ConfItem::K((
                <Vec<u8> as From<&[u8]>>::from(b"a"),
                Some(<Vec<u8> as From<&[u8]>>::from(b"1"))
            ))
        );
    }

    #[test]
    fn msg_to_type() {
        assert_eq!(
            Msg::A(MsgA {
                t: MsgType::from(0),
                data: None,
            })
            .t(),
            MsgType::S(MsgTypeS::Type(0))
        );
        assert_eq!(Msg::Req(MsgReq::V).t(), MsgType::Req(MsgTypeReq::V));
        assert_eq!(
            Msg::Req(MsgReq::C(ConfItem::T1(None))).t(),
            MsgType::Req(MsgTypeReq::C)
        );
        assert_eq!(
            Msg::Res(MsgRes::V(MsgResV {
                raw: None,
                p: Vec::new(),
                firmware: Vec::new(),
                hardware: Vec::new(),
            }))
            .t(),
            MsgType::Res(MsgTypeRes::V)
        );
        assert_eq!(
            Msg::Res(MsgRes::E((1, None))).t(),
            MsgType::Res(MsgTypeRes::E)
        );
        assert_eq!(Msg::Res(MsgRes::K(None)).t(), MsgType::Res(MsgTypeRes::K));
        assert_eq!(
            Msg::Res(MsgRes::C(ConfItem::T1(None))).t(),
            MsgType::Res(MsgTypeRes::C)
        );
    }
}
