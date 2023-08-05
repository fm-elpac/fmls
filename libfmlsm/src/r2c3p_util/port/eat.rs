//! 对接收消息的默认处理结果 (内置协议功能)

use core::iter::Iterator;

use super::super::hex::{
    HexU32Sender, HexU64Sender, HexU8Sender, NU8Sender, NoneSender, U8Sender, VecSender,
};
use super::super::send::{CSender, ESender, MsgSender};

/// 需要发送的响应消息 (发送器)
pub enum Eat<'a> {
    /// 比如 `E-2 32`
    E2(MsgSender<'a, ESender<VecSender, NU8Sender>>),
    /// 比如 `E-3 c`
    E3(MsgSender<'a, ESender<VecSender, U8Sender>>),
    /// 比如 `E-4`, `E-5`
    E(MsgSender<'a, ESender<VecSender, NoneSender>>),
    /// 比如 `CcT=`
    #[cfg(any(feature = "r2c3p-at", feature = "r2c3p-cc"))]
    CHexU32(MsgSender<'a, CSender<HexU32Sender>>),
    /// 比如 `CI=`
    #[cfg(feature = "r2c3p-i")]
    CHexU64(MsgSender<'a, CSender<HexU64Sender>>),
    /// 比如 `CO=`
    #[cfg(feature = "r2c3p-o")]
    CHexU8(MsgSender<'a, CSender<HexU8Sender>>),
    /// 比如 `C@=1`
    #[cfg(feature = "r2c3p-at")]
    CN8(MsgSender<'a, CSender<NU8Sender>>),
}

impl<'a> Eat<'a> {
    pub fn done(&self) -> bool {
        match self {
            Eat::E2(m) => m.done(),
            Eat::E3(m) => m.done(),
            Eat::E(m) => m.done(),
            #[cfg(any(feature = "r2c3p-at", feature = "r2c3p-cc"))]
            Eat::CHexU32(m) => m.done(),
            #[cfg(feature = "r2c3p-i")]
            Eat::CHexU64(m) => m.done(),
            #[cfg(feature = "r2c3p-o")]
            Eat::CHexU8(m) => m.done(),
            #[cfg(feature = "r2c3p-at")]
            Eat::CN8(m) => m.done(),
        }
    }
}

impl<'a> Iterator for Eat<'a> {
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