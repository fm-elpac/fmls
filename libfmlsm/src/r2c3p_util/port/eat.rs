//! 对接收消息的默认处理结果 (内置协议功能)

use core::iter::Iterator;

use super::super::send::{ESender, MsgSender};
use super::super::NU8Sender;
use crate::r2c3p_low::{BArraySender, BStaticSender, NoneSender};

#[cfg(any(
    feature = "r2c3p-cc",
    feature = "r2c3p-i",
    feature = "r2c3p-o",
    feature = "r2c3p-at"
))]
use super::super::send::CSender;
#[cfg(any(feature = "r2c3p-cc", feature = "r2c3p-at"))]
use super::super::HexU32Sender;
#[cfg(feature = "r2c3p-i")]
use super::super::HexU64Sender;
#[cfg(feature = "r2c3p-o")]
use super::super::HexU8Sender;

/// 需要发送的响应消息 (发送器)
pub enum Eat {
    /// 比如 `E-2 32`
    E2(MsgSender<ESender<BStaticSender, NU8Sender>>),
    /// 比如 `E-3 c`
    E3(MsgSender<ESender<BStaticSender, BArraySender<1>>>),
    /// 比如 `E-4`, `E-5`
    E(MsgSender<ESender<BStaticSender, NoneSender>>),
    /// 比如 `CcT=`
    #[cfg(any(feature = "r2c3p-at", feature = "r2c3p-cc"))]
    CHexU32(MsgSender<CSender<HexU32Sender>>),
    /// 比如 `CI=`
    #[cfg(feature = "r2c3p-i")]
    CHexU64(MsgSender<CSender<HexU64Sender>>),
    /// 比如 `CO=`
    #[cfg(feature = "r2c3p-o")]
    CHexU8(MsgSender<CSender<HexU8Sender>>),
    /// 比如 `C@=1`
    #[cfg(feature = "r2c3p-at")]
    CN8(MsgSender<CSender<NU8Sender>>),
}

impl Iterator for Eat {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        match self {
            Eat::E2(m) => m.next(),
            Eat::E3(m) => m.next(),
            Eat::E(m) => m.next(),
            #[cfg(any(feature = "r2c3p-at", feature = "r2c3p-cc"))]
            Eat::CHexU32(m) => m.next(),
            #[cfg(feature = "r2c3p-i")]
            Eat::CHexU64(m) => m.next(),
            #[cfg(feature = "r2c3p-o")]
            Eat::CHexU8(m) => m.next(),
            #[cfg(feature = "r2c3p-at")]
            Eat::CN8(m) => m.next(),
        }
    }
}
