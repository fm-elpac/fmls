//! 消息的抽象及处理
//!
//! - [`MsgType`] 消息类型
//!
//!   `u8` 和消息类型的互相转换
//!
//! - [`Msg`] 一条消息 (含数据)
//!
//!   消息和 `Vec<u8>` (二进制数据) 的互相转换 (含错误处理)
//!
//! - [`ConfItem`] 配置项
//!
//!   配置项和 `Vec<u8>` (二进制数据) 的互相转换 (含错误处理)

use libfmlsc::r2c3p as p;

mod conf_item;
pub mod hex;
mod msg_type;

pub use conf_item::ConfItem;
use conf_item::ConfItemO;
pub use msg_type::{MsgType, MsgTypeReq, MsgTypeRes, MsgTypeS};

/// 一条消息
#[derive(Clone, Debug, PartialEq)]
pub enum Msg {
    /// 自定义消息
    A {
        /// 消息类型
        t: MsgType,
        /// 附加数据 (可选)
        data: Option<Vec<u8>>,
    },
    /// 预定义 请求消息
    Req(MsgReq),
    /// 预定义 响应消息
    Res(MsgRes),
    /// 预定义 静默消息
    S(MsgS),
}

impl Msg {
    /// 获取消息类型
    pub fn t(&self) -> MsgType {
        match self {
            Self::A { t, .. } => t.clone(),
            Self::Req(i) => i.t(),
            Self::Res(i) => i.t(),
            Self::S(i) => i.t(),
        }
    }

    /// `Vec<u8>` 转换为 Msg::A
    pub fn to_a(b: Vec<u8>) -> Self {
        if b.len() < 1 {
            return Self::A {
                t: MsgType::from(0),
                data: None,
            };
        }

        let data = if b.len() > 1 {
            Some(Vec::from(&b[1..]))
        } else {
            None
        };

        Self::A {
            t: MsgType::from(b[0]),
            data,
        }
    }
}

// 消息转换为 `Vec<u8>`
impl From<Msg> for Vec<u8> {
    fn from(m: Msg) -> Vec<u8> {
        match m {
            Msg::A { t, data } => {
                let mut v: Vec<u8> = vec![t.into()];
                if let Some(b) = data {
                    v.extend_from_slice(&b);
                }
                v
            }
            Msg::Req(i) => i.into(),
            Msg::Res(i) => i.into(),
            Msg::S(i) => i.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct MsgO(pub Option<Msg>);

// `Vec<u8>` 转换为消息 (带错误处理)
// 如果出错, 返回 None
impl From<Vec<u8>> for MsgO {
    fn from(b: Vec<u8>) -> MsgO {
        // 首先解析为 自定义消息
        let m = Msg::to_a(b);
        let (t, v) = match m.clone() {
            Msg::A { t, data } => (t, data),
            _ => {
                return MsgO(None);
            }
        };

        // 根据消息类型分别处理
        match t {
            MsgType::Req(r) => match r {
                MsgTypeReq::Type(_) => MsgO(Some(m)),
                _ => {
                    let o: MsgReqO = (r, v).into();
                    match o.0 {
                        Some(m) => MsgO(Some(Msg::Req(m))),
                        None => MsgO(None),
                    }
                }
            },
            MsgType::Res(r) => match r {
                MsgTypeRes::Type(_) => MsgO(Some(m)),
                _ => {
                    let o: MsgResO = (r, v).into();
                    match o.0 {
                        Some(m) => MsgO(Some(Msg::Res(m))),
                        None => MsgO(None),
                    }
                }
            },
            MsgType::S(s) => match s {
                MsgTypeS::Type(_) => MsgO(Some(m)),
                _ => {
                    let o: MsgSO = (s, v).into();
                    match o.0 {
                        Some(m) => MsgO(Some(Msg::S(m))),
                        None => MsgO(None),
                    }
                }
            },
        }
    }
}

/// 预定义 请求消息
#[derive(Clone, Debug, PartialEq)]
pub enum MsgReq {
    /// `v` 获取设备固件版本信息
    V,
    /// `vv` 特殊的请求消息, 发送时不添加 crc
    Vv,
    /// `c` 设备配置
    C(ConfItem),
}

impl MsgReq {
    /// 获取消息类型
    pub fn t(&self) -> MsgType {
        match self {
            Self::V => MsgType::Req(MsgTypeReq::V),
            Self::Vv => MsgType::Req(MsgTypeReq::V),
            Self::C(_) => MsgType::Req(MsgTypeReq::C),
        }
    }
}

// 请求消息转换为 `Vec<u8>`
impl From<MsgReq> for Vec<u8> {
    fn from(m: MsgReq) -> Vec<u8> {
        match m {
            MsgReq::V => vec![MsgTypeReq::V.into()],
            MsgReq::Vv => vec![MsgTypeReq::V.into(), p::MSGT_V_R],
            MsgReq::C(i) => {
                let mut v = vec![MsgTypeReq::C.into()];
                let b: Vec<u8> = i.into();
                v.extend_from_slice(&b);
                v
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct MsgReqO(pub Option<MsgReq>);

// `(MsgTypeReq, Option<Vec<u8>>)` 转换为请求消息 (带错误处理)
// 如果出错, 返回 None
impl From<(MsgTypeReq, Option<Vec<u8>>)> for MsgReqO {
    fn from(tv: (MsgTypeReq, Option<Vec<u8>>)) -> MsgReqO {
        let (t, v) = tv;
        match t {
            MsgTypeReq::V => MsgReqO(Some(MsgReq::V)),
            MsgTypeReq::C => match v {
                Some(b) => {
                    let oc: ConfItemO = b.into();
                    match oc.0 {
                        Some(c) => MsgReqO(Some(MsgReq::C(c))),
                        None => MsgReqO(None),
                    }
                }
                None => MsgReqO(None),
            },
            _ => MsgReqO(None),
        }
    }
}

/// 预定义 响应消息
#[derive(Clone, Debug, PartialEq)]
pub enum MsgRes {
    /// `V` 设备固件版本信息
    V {
        /// fmls_r2c3p 协议的版本
        p: String,
        /// 固件名称及版本
        firmware: String,
        /// 硬件信息及编号
        hardware: String,
        /// 额外的自定义内容 (可选)
        extra: Option<String>,

        /// 原始附加数据
        raw: Option<Vec<u8>>,
    },
    /// `E` 错误信息
    E {
        /// 错误码
        c: i32,
        /// 错误信息
        m: Option<Vec<u8>>,
    },
    /// `K` 成功, 附加数据可选
    K(Option<Vec<u8>>),
    /// `CK=V` 设备配置
    C(ConfItem),
}

impl MsgRes {
    /// 获取消息类型
    pub fn t(&self) -> MsgType {
        match self {
            Self::V { .. } => MsgType::Res(MsgTypeRes::V),
            Self::E { .. } => MsgType::Res(MsgTypeRes::E),
            Self::K(_) => MsgType::Res(MsgTypeRes::K),
            Self::C(_) => MsgType::Res(MsgTypeRes::C),
        }
    }
}

// 响应消息转换为 `Vec<u8>`
impl From<MsgRes> for Vec<u8> {
    fn from(m: MsgRes) -> Vec<u8> {
        match m {
            MsgRes::V {
                p,
                firmware,
                hardware,
                extra,
                ..
            } => {
                let mut v = vec![MsgTypeRes::V.into()];
                if p.len() > 0 {
                    v.extend_from_slice(&Vec::from(p));
                } else {
                    // 使用默认协议版本号
                    v.extend_from_slice(p::P_VERSION);
                }
                v.push(p::BYTE_LF);
                v.extend_from_slice(&Vec::from(firmware));
                v.push(p::BYTE_LF);
                v.extend_from_slice(&Vec::from(hardware));
                if let Some(i) = extra {
                    v.push(p::BYTE_LF);
                    v.extend_from_slice(&Vec::from(i));
                }
                v
            }
            MsgRes::E { c, m } => {
                let mut v = vec![MsgTypeRes::E.into()];
                v.extend_from_slice(&Vec::from(format!("{}", c)));
                if let Some(i) = m {
                    v.push(p::BYTE_SPACE);
                    v.extend_from_slice(&i);
                }
                v
            }
            MsgRes::K(i) => {
                let mut v = vec![MsgTypeRes::K.into()];
                if let Some(d) = i {
                    v.extend_from_slice(&d);
                }
                v
            }
            MsgRes::C(i) => {
                let mut v = vec![MsgTypeRes::C.into()];
                let s = i.is_set();
                let b: Vec<u8> = i.into();
                v.extend_from_slice(&b);
                // 若配置信息为空, 响应消息 `CK=` 应该包含字符 `=`
                if !s {
                    v.push(p::BYTE_EQ);
                }
                v
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct MsgResO(pub Option<MsgRes>);

// `(MsgTypeRes, Option<Vec<u8>>)` 转换为响应消息 (带错误处理)
// 如果出错, 返回 None
impl From<(MsgTypeRes, Option<Vec<u8>>)> for MsgResO {
    fn from(tv: (MsgTypeRes, Option<Vec<u8>>)) -> MsgResO {
        let (t, v) = tv;
        match t {
            MsgTypeRes::V => match v {
                Some(raw) => {
                    let s1 = String::from_utf8_lossy(&raw).into_owned();
                    let mut s = s1.lines();
                    let p = match s.next() {
                        Some(l) => l.to_string(),
                        None => "".to_string(),
                    };
                    let firmware = match s.next() {
                        Some(l) => l.to_string(),
                        None => "".to_string(),
                    };
                    let hardware = match s.next() {
                        Some(l) => l.to_string(),
                        None => "".to_string(),
                    };
                    let r = s.collect::<Vec<_>>().join("\n");
                    let extra = if r.len() > 0 { Some(r) } else { None };

                    MsgResO(Some(MsgRes::V {
                        raw: Some(raw),
                        p,
                        firmware,
                        hardware,
                        extra,
                    }))
                }
                None => MsgResO(None),
            },
            MsgTypeRes::E => match v {
                Some(b) => {
                    let sp = b.iter().position(|n| p::BYTE_SPACE == *n);
                    let (n, m) = match sp {
                        Some(i) => (&b[..i], Some(Vec::from(&b[(i + 1)..]))),
                        None => (&b[..], None),
                    };
                    match i32::from_str_radix(String::from_utf8_lossy(n).into_owned().as_str(), 10)
                    {
                        Ok(c) => MsgResO(Some(MsgRes::E { c, m })),
                        _ => MsgResO(None),
                    }
                }
                None => MsgResO(None),
            },
            MsgTypeRes::K => MsgResO(Some(MsgRes::K(v))),
            MsgTypeRes::C => match v {
                Some(b) => {
                    let oc: ConfItemO = b.into();
                    match oc.0 {
                        Some(c) => MsgResO(Some(MsgRes::C(c))),
                        None => MsgResO(None),
                    }
                }
                None => MsgResO(None),
            },
            _ => MsgResO(None),
        }
    }
}

/// 预定义 静默消息
#[derive(Clone, Debug, PartialEq)]
pub enum MsgS {
    /// `@` 用于 r2 集线器
    At(Option<MsgAt>),
}

impl MsgS {
    /// 获取消息类型
    pub fn t(&self) -> MsgType {
        match self {
            Self::At(_) => MsgType::S(MsgTypeS::At),
        }
    }
}

// 静默消息转换为 `Vec<u8>`
impl From<MsgS> for Vec<u8> {
    fn from(m: MsgS) -> Vec<u8> {
        match m {
            MsgS::At(i) => {
                let mut v = vec![MsgTypeS::At.into()];
                if let Some(a) = i {
                    let b: Vec<u8> = a.into();
                    v.extend_from_slice(&b);
                }
                v
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct MsgSO(pub Option<MsgS>);

// `(MsgTypeS, Option<Vec<u8>>)` 转换为静默消息 (带错误处理)
// 如果出错, 返回 None
impl From<(MsgTypeS, Option<Vec<u8>>)> for MsgSO {
    fn from(tv: (MsgTypeS, Option<Vec<u8>>)) -> MsgSO {
        let (t, v) = tv;
        match t {
            MsgTypeS::At => match v {
                Some(b) => MsgSO(Some(MsgS::At(Some(b.into())))),
                None => MsgSO(Some(MsgS::At(None))),
            },
            _ => MsgSO(None),
        }
    }
}

/// 预定义 静默消息 `@` 的附加数据
#[derive(Clone, Debug, PartialEq)]
pub struct MsgAt {
    /// 节点号
    pub n: u8,
    /// 承载的数据
    pub d: Vec<u8>,
}

// MsgAt 转换为 `Vec<u8>`
impl From<MsgAt> for Vec<u8> {
    fn from(a: MsgAt) -> Vec<u8> {
        let mut v = vec![a.n];
        v.extend_from_slice(&a.d);
        v
    }
}

// `Vec<u8>` 转换为 MsgAt
impl From<Vec<u8>> for MsgAt {
    fn from(b: Vec<u8>) -> Self {
        if b.len() < 1 {
            return Self {
                n: 0,
                d: Vec::new(),
            };
        }

        let d = if b.len() > 1 {
            Vec::from(&b[1..])
        } else {
            Vec::new()
        };

        Self { n: b[0], d }
    }
}

#[cfg(test)]
mod test;
