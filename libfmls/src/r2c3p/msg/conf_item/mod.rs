//! 配置项

use super::hex;
use libfmlsc::r2c3p as p;

/// 配置项的 K
#[derive(Clone, Debug, PartialEq)]
pub enum ConfItemK {
    /// 自定义配置项
    K(Vec<u8>),
    // 预定义配置
    M1,
    T,
    T1,
    C(&'static [u8]),
    I,
    O,
    On,
    At,
    AtS(u8),
    AtN(u8),
}

// K 转为 `Vec<u8>`
impl From<ConfItemK> for Vec<u8> {
    fn from(k: ConfItemK) -> Vec<u8> {
        match k {
            ConfItemK::K(b) => b.clone(),
            ConfItemK::M1 => Vec::from(p::CONF_M_1 as &[u8]),
            ConfItemK::T => Vec::from(p::CONF_T as &[u8]),
            ConfItemK::T1 => Vec::from(p::CONF_T_1 as &[u8]),
            ConfItemK::C(b) => Vec::from(b as &[u8]),
            ConfItemK::I => Vec::from(p::CONF_I as &[u8]),
            ConfItemK::O => Vec::from(p::CONF_O as &[u8]),
            ConfItemK::On => Vec::from(p::CONF_ON as &[u8]),
            ConfItemK::At => Vec::from(p::CONF_AT as &[u8]),
            ConfItemK::AtS(n) => {
                let mut v = Vec::from(p::CONF_ATS as &[u8]);
                let h = hex::u8_hex(n);
                v.extend_from_slice(&h);
                v
            }
            ConfItemK::AtN(n) => {
                let mut v = Vec::from(p::CONF_ATN as &[u8]);
                let h = hex::u8_hex(n);
                v.extend_from_slice(&h);
                v
            }
        }
    }
}

// `Vec<u8>` 转换为 K
impl From<Vec<u8>> for ConfItemK {
    fn from(b: Vec<u8>) -> ConfItemK {
        if Vec::from(p::CONF_M_1 as &[u8]) == b {
            ConfItemK::M1
        } else if Vec::from(p::CONF_T as &[u8]) == b {
            ConfItemK::T
        } else if Vec::from(p::CONF_T_1 as &[u8]) == b {
            ConfItemK::T1
        } else if Vec::from(p::CONF_I as &[u8]) == b {
            ConfItemK::I
        } else if Vec::from(p::CONF_O as &[u8]) == b {
            ConfItemK::O
        } else if Vec::from(p::CONF_ON as &[u8]) == b {
            ConfItemK::On
        } else if Vec::from(p::CONF_AT as &[u8]) == b {
            ConfItemK::At
        } else if Vec::from(p::CONF_CT as &[u8]) == b {
            ConfItemK::C(p::CONF_CT)
        } else if Vec::from(p::CONF_CR as &[u8]) == b {
            ConfItemK::C(p::CONF_CR)
        } else if Vec::from(p::CONF_CRD as &[u8]) == b {
            ConfItemK::C(p::CONF_CRD)
        } else if Vec::from(p::CONF_CTB as &[u8]) == b {
            ConfItemK::C(p::CONF_CTB)
        } else if Vec::from(p::CONF_CRB as &[u8]) == b {
            ConfItemK::C(p::CONF_CRB)
        } else {
            // TODO CONF_ATS
            // TODO CONF_ATN
            // TODO
            ConfItemK::K(b)
        }
    }
}

/// 配置项
#[derive(Clone, Debug, PartialEq)]
pub enum ConfItem {
    /// 自定义配置项
    K {
        /// K: 配置项名称
        k: Vec<u8>,
        /// V: 配置项的值
        v: Option<Vec<u8>>,
    },
    /// 预定义配置 `m`
    M1(Option<u8>),
    /// 预定义配置 `T`
    T(Option<Vec<u8>>),
    /// 预定义配置 `t`
    T1(Option<u16>),
    /// 预定义配置 `c*` 计数器
    C {
        /// `cT`, `cR`, `cRd`, `cTB`, `cRB`
        k: &'static [u8],
        /// 值
        v: Option<u32>,
    },
    /// 预定义配置 `I`
    I(Option<u64>),
    /// 预定义配置 `O`
    O(Option<u8>),
    /// 预定义配置 `On`
    On(Option<u8>),
    /// 预定义配置 `@`
    At(Option<u8>),
    /// 预定义配置 `@s`N
    AtS {
        /// 节点号
        n: u8,
        /// 值
        v: Option<u32>,
    },
    /// 预定义配置 `@n`N
    AtN {
        /// 节点号
        n: u8,
        /// 值
        v: Option<u32>,
    },
}

impl ConfItem {
    /// 返回 K
    pub fn k(&self) -> ConfItemK {
        match self {
            Self::K { k, .. } => ConfItemK::K(k.clone()),
            Self::M1(_) => ConfItemK::M1,
            Self::T(_) => ConfItemK::T,
            Self::T1(_) => ConfItemK::T1,
            Self::C { k, .. } => ConfItemK::C(k),
            Self::I(_) => ConfItemK::I,
            Self::O(_) => ConfItemK::O,
            Self::On(_) => ConfItemK::On,
            Self::At(_) => ConfItemK::At,
            Self::AtS { n, .. } => ConfItemK::AtS(*n),
            Self::AtN { n, .. } => ConfItemK::AtN(*n),
        }
    }

    /// 返回 V (`Vec<u8>`)
    pub fn v(&self) -> Option<Vec<u8>> {
        match self {
            Self::K { v, .. } => v.clone(),
            Self::M1(v) | Self::At(v) => match v {
                Some(u) => Some(Vec::from(format!("{}", u))),
                None => None,
            },
            Self::T(v) => match v {
                Some(b) => Some(hex::v_hex(b)),
                None => None,
            },
            Self::T1(v) => match v {
                Some(u) => Some(hex::u16_hex(*u)),
                None => None,
            },
            Self::C { v, .. } | Self::AtS { v, .. } | Self::AtN { v, .. } => match v {
                Some(u) => Some(hex::u32_hex(*u)),
                None => None,
            },
            Self::I(v) => match v {
                Some(u) => Some(hex::u64_hex(*u)),
                None => None,
            },
            Self::O(v) | Self::On(v) => match v {
                Some(u) => Some(hex::u8_hex(*u)),
                None => None,
            },
        }
    }

    /// 是否为 `cK=V` 格式
    pub fn is_set(&self) -> bool {
        match self {
            Self::K { v, .. } => v.is_some(),
            Self::M1(v) => v.is_some(),
            Self::T(v) => v.is_some(),
            Self::T1(v) => v.is_some(),
            Self::C { v, .. } => v.is_some(),
            Self::I(v) => v.is_some(),
            Self::O(v) => v.is_some(),
            Self::On(v) => v.is_some(),
            Self::At(v) => v.is_some(),
            Self::AtS { v, .. } => v.is_some(),
            Self::AtN { v, .. } => v.is_some(),
        }
    }
}

// 配置项转换为 `Vec<u8>`
impl From<ConfItem> for Vec<u8> {
    fn from(i: ConfItem) -> Vec<u8> {
        let mut v: Vec<u8> = i.k().into();
        if let Some(b) = i.v() {
            v.push(p::BYTE_EQ);
            v.extend_from_slice(&b);
        }
        v
    }
}

// `Vec<u8>` 转换为配置项
impl From<Vec<u8>> for ConfItem {
    fn from(b: Vec<u8>) -> Self {
        let eq = b.iter().position(|n| *n == p::BYTE_EQ);
        let (k, v) = match eq {
            Some(i) => (&b[..i], Some(Vec::from(&b[(i + 1)..]))),
            None => (&b[..], None),
        };

        // 由于无法返回错误, 此处不能进行预定义配置项的解析
        Self::K { k: Vec::from(k), v }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ConfItemO(pub Option<ConfItem>);

// `Vec<u8>` 转换为配置项 (带错误处理)
// 如果出错, 返回 None
impl From<Vec<u8>> for ConfItemO {
    fn from(b: Vec<u8>) -> ConfItemO {
        fn map_v<T, F: Fn(Vec<u8>) -> Option<T>, G: Fn(Option<T>) -> ConfItem>(
            v: Option<Vec<u8>>,
            f1: F,
            f2: G,
        ) -> ConfItemO {
            match v {
                Some(b) => match f1(b) {
                    Some(u) => ConfItemO(Some(f2(Some(u)))),
                    _ => ConfItemO(None),
                },
                None => ConfItemO(Some(f2(None))),
            }
        }

        // 首先转换为 自定义配置项
        let c: ConfItem = b.into();
        let (kb, v) = match c {
            ConfItem::K { k, v } => (k, v),
            _ => {
                return ConfItemO(None);
            }
        };
        // 根据 k 进行不同处理
        let k: ConfItemK = kb.into();
        match k {
            ConfItemK::K(k) => ConfItemO(Some(ConfItem::K { k, v })),
            ConfItemK::M1 => map_v::<u8, _, _>(v, |b| hex::n_u8(b), |u| ConfItem::M1(u)),
            ConfItemK::T => map_v::<Vec<u8>, _, _>(v, |b| hex::hex_v(&b), |u| ConfItem::T(u)),
            ConfItemK::T1 => map_v::<u16, _, _>(v, |b| hex::hex_u16(b), |u| ConfItem::T1(u)),
            ConfItemK::C(k) => map_v::<u32, _, _>(v, |b| hex::hex_u32(b), |v| ConfItem::C { k, v }),
            ConfItemK::I => map_v::<u64, _, _>(v, |b| hex::hex_u64(b), |u| ConfItem::I(u)),
            ConfItemK::O => map_v::<u8, _, _>(v, |b| hex::hex_u8(b), |u| ConfItem::O(u)),
            ConfItemK::On => map_v::<u8, _, _>(v, |b| hex::hex_u8(b), |u| ConfItem::On(u)),
            ConfItemK::At => map_v::<u8, _, _>(v, |b| hex::n_u8(b), |u| ConfItem::At(u)),
            ConfItemK::AtS(n) => {
                map_v::<u32, _, _>(v, |b| hex::hex_u32(b), |v| ConfItem::AtS { n, v })
            }
            ConfItemK::AtN(n) => {
                map_v::<u32, _, _>(v, |b| hex::hex_u32(b), |v| ConfItem::AtN { n, v })
            }
        }
    }
}

#[cfg(test)]
mod test;
