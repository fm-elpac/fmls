//! 发送消息

use core::iter::Iterator;

use crate::r2c3p::{BYTE_LF, BYTE_SPACE, P_VERSION};

#[cfg(feature = "r2c3p-crc32")]
use crate::r2c3p::MSGT_V;
#[cfg(feature = "r2c3p-crc16")]
use crate::r2c3p::{EB_2, EB_3, EB_4, EB_5, MSGT_E};

use super::{BArraySender, BStaticSender, Escape, HexArraySender};

#[cfg(feature = "r2c3p-crc16")]
use super::Crc16;
#[cfg(feature = "r2c3p-crc32")]
use super::Crc32;

#[derive(PartialEq)]
enum LowSendS {
    // 正在发送消息类型
    T,
    // 正在发送附加数据
    D,
    // 正在发送 CRC
    C,
    // 发送完毕
    None,
}

/// 发送一条消息
pub struct LowSend<T: Iterator<Item = u8>, C: CrcT<N>, const N: usize> {
    // 发送状态
    s: LowSendS,
    // 用于转义处理
    e: Escape,
    // 用于计算 crc
    c: C,
    // 发送消息的附加数据
    d: T,
    // 消息类型
    t: u8,
    // 用于发送 crc
    cs: BArraySender<N>,
}

impl<T: Iterator<Item = u8>, C: CrcT<N>, const N: usize> LowSend<T, C, N> {
    pub fn new(t: u8, d: T, c: C) -> Self {
        Self {
            s: LowSendS::T,
            e: Escape::new(),
            c,
            d,
            t,
            cs: BArraySender::new([0; N]),
        }
    }

    // 发送一个字节
    fn send_b(&mut self, b: u8) -> u8 {
        // 计算 crc
        self.c.feed(b);
        // 处理转义
        self.e.feed(b)
    }
}

impl<T: Iterator<Item = u8>, C: CrcT<N>, const N: usize> Iterator for LowSend<T, C, N> {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        // 优先发送转义字符
        if let Some(b) = self.e.next() {
            return Some(b);
        }

        match self.s {
            LowSendS::T => {
                let b = self.send_b(self.t);
                self.s = LowSendS::D;

                Some(b)
            }
            LowSendS::D => match self.d.next() {
                Some(b) => Some(self.send_b(b)),
                None => {
                    // 准备发送 crc
                    self.cs = BArraySender::new(self.c.result());
                    self.s = LowSendS::C;
                    self.next()
                }
            },
            LowSendS::C => match self.cs.next() {
                Some(b) => Some(self.send_b(b)),
                None => {
                    self.s = LowSendS::None;
                    // 发送消息结束字节
                    Some(BYTE_LF)
                }
            },
            LowSendS::None => None,
        }
    }
}

/// CRC 计算接口
pub trait CrcT<const N: usize> {
    /// 重置 crc 计算
    fn reset(&mut self);

    /// 喂给一个字节
    fn feed(&mut self, b: u8);

    /// 获取计算结果
    ///
    /// 获取结果后, 需要手动 `reset()`
    fn result(&mut self) -> [u8; N];
}

/// 不计算 CRC
pub struct C0 {}

impl C0 {
    pub const fn new() -> Self {
        Self {}
    }
}

impl CrcT<0> for C0 {
    fn reset(&mut self) {}
    fn feed(&mut self, _b: u8) {}

    fn result(&mut self) -> [u8; 0] {
        []
    }
}

/// 计算 crc16 (封装)
#[cfg(feature = "r2c3p-crc16")]
pub struct C16 {
    c: Option<Crc16>,
}

#[cfg(feature = "r2c3p-crc16")]
impl C16 {
    pub const fn new() -> Self {
        Self {
            c: Some(Crc16::new()),
        }
    }
}

#[cfg(feature = "r2c3p-crc16")]
impl CrcT<2> for C16 {
    fn reset(&mut self) {
        self.c = Some(Crc16::new());
    }

    fn feed(&mut self, b: u8) {
        if let Some(c) = &mut self.c {
            c.feed(b);
        }
    }

    fn result(&mut self) -> [u8; 2] {
        let mut o: [u8; 2] = [0; 2];
        if let Some(c) = self.c.take() {
            o = c.result().to_le_bytes();
        }
        o
    }
}

/// 计算 crc32 (封装)
#[cfg(feature = "r2c3p-crc32")]
pub struct C32 {
    c: Option<Crc32>,
}

#[cfg(feature = "r2c3p-crc32")]
impl C32 {
    pub const fn new() -> Self {
        Self {
            c: Some(Crc32::new()),
        }
    }
}

#[cfg(feature = "r2c3p-crc32")]
impl CrcT<4> for C32 {
    fn reset(&mut self) {
        self.c = Some(Crc32::new());
    }

    fn feed(&mut self, b: u8) {
        if let Some(c) = &mut self.c {
            c.feed(b);
        }
    }

    fn result(&mut self) -> [u8; 4] {
        let mut o: [u8; 4] = [0; 4];
        if let Some(c) = self.c.take() {
            o = c.result().to_le_bytes();
        }
        o
    }
}

/// 使用固定 crc32 值
pub struct C32F {
    c: [u8; 4],
}

impl C32F {
    pub const fn new(c: [u8; 4]) -> Self {
        Self { c }
    }
}

impl CrcT<4> for C32F {
    fn reset(&mut self) {}
    fn feed(&mut self, _b: u8) {}

    fn result(&mut self) -> [u8; 4] {
        self.c
    }
}

/// 发送一条消息, 不使用 crc
pub fn send_msg_0<T: Iterator<Item = u8>>(t: u8, d: T) -> LowSend<T, C0, 0> {
    LowSend::new(t, d, C0::new())
}

/// 发送一条消息, 使用 crc16
#[cfg(feature = "r2c3p-crc16")]
pub fn send_msg_16<T: Iterator<Item = u8>>(t: u8, d: T) -> LowSend<T, C16, 2> {
    LowSend::new(t, d, C16::new())
}

/// 发送一条消息, 使用 crc32
#[cfg(feature = "r2c3p-crc32")]
pub fn send_msg_32<T: Iterator<Item = u8>>(t: u8, d: T) -> LowSend<T, C32, 4> {
    LowSend::new(t, d, C32::new())
}

/// 发送一条消息, 使用固定 crc32 值
pub fn send_msg_32f<T: Iterator<Item = u8>>(t: u8, d: T, c: [u8; 4]) -> LowSend<T, C32F, 4> {
    LowSend::new(t, d, C32F::new(c))
}

#[derive(Clone, Copy, PartialEq)]
enum LowVSenderS {
    /// 正在发送 p
    P,
    /// 正在发送 firmware
    F,
    /// 正在发送硬件名称 (hardware)
    HN,
    /// 正在发送硬件编号 (hex)
    HH,
    /// 发送完毕
    None,
}

/// 发送 `V` 消息的数据部分 (不支持 extra)
pub struct LowVSender<const N: usize> {
    s: LowVSenderS,
    // 固件名称
    f: &'static [u8],
    // 硬件名称
    hn: &'static [u8],
    // 静态字节发送器
    b: BStaticSender,
    // 发送硬件编号 (hex)
    h: HexArraySender<N>,
}

impl<const N: usize> LowVSender<N> {
    pub fn new(f: &'static [u8], hn: &'static [u8], h: [u8; N]) -> Self {
        Self {
            s: LowVSenderS::P,
            f,
            hn,
            b: BStaticSender::new(P_VERSION),
            h: HexArraySender::new(h),
        }
    }
}

impl<const N: usize> Iterator for LowVSender<N> {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        match self.s {
            LowVSenderS::P => match self.b.next() {
                Some(b) => Some(b),
                None => {
                    // 准备发送 固件名称
                    self.b = BStaticSender::new(self.f);
                    self.s = LowVSenderS::F;
                    // 发送分隔字节
                    Some(BYTE_LF)
                }
            },
            LowVSenderS::F => match self.b.next() {
                Some(b) => Some(b),
                None => {
                    // 准备发送 硬件名称
                    self.b = BStaticSender::new(self.hn);
                    self.s = LowVSenderS::HN;
                    // 发送分隔字节
                    Some(BYTE_LF)
                }
            },
            LowVSenderS::HN => match self.b.next() {
                Some(b) => Some(b),
                None => {
                    // 准备发送 硬件编号
                    self.s = LowVSenderS::HH;
                    // 发送空格
                    Some(BYTE_SPACE)
                }
            },
            LowVSenderS::HH => match self.h.next() {
                Some(b) => Some(b),
                None => {
                    // 发送完毕
                    self.s = LowVSenderS::None;

                    None
                }
            },
            LowVSenderS::None => None,
        }
    }
}

/// 发送 `V` 消息
#[cfg(feature = "r2c3p-crc32")]
pub fn send_v<const N: usize>(
    firmware: &'static [u8],
    hardware_name: &'static [u8],
    hardware_id: [u8; N],
) -> LowSend<LowVSender<N>, C32, 4> {
    send_msg_32(
        MSGT_V,
        LowVSender::new(firmware, hardware_name, hardware_id),
    )
}

/// 发送 `E-2` 消息 (错误: 消息太长)
#[cfg(feature = "r2c3p-crc16")]
pub fn send_e2() -> LowSend<BStaticSender, C16, 2> {
    send_msg_16(MSGT_E, BStaticSender::new(EB_2))
}

/// 发送 `E-2` 消息, 带缓冲区长度, 比如 `E-2 32`
///
/// `N`: 缓冲区数字的长度 + 3
///
/// 比如:
///
/// ```
/// # use libfmlsm::r2c3p_low::send_e2_len;
/// let mut b: [u8; 5] = [0; 5];
/// b[3..].copy_from_slice(b"32");
/// send_e2_len(b);  // `E-2 32`
/// ```
#[cfg(feature = "r2c3p-crc16")]
pub fn send_e2_len<const N: usize>(mut len: [u8; N]) -> LowSend<BArraySender<N>, C16, 2> {
    len[0..2].copy_from_slice(EB_2);
    len[2] = BYTE_SPACE;
    send_msg_16(MSGT_E, BArraySender::new(len))
}

/// 发送 `E-3` 消息 (错误: 未知的消息类型)
#[cfg(feature = "r2c3p-crc16")]
pub fn send_e3(t: u8) -> LowSend<BArraySender<4>, C16, 2> {
    let mut b: [u8; 4] = [0; 4];
    b[0..2].copy_from_slice(EB_3);
    b[2] = BYTE_SPACE;
    b[3] = t;
    send_msg_16(MSGT_E, BArraySender::new(b))
}

/// 发送 `E-4` 消息 (错误: 错误的消息格式)
#[cfg(feature = "r2c3p-crc16")]
pub fn send_e4() -> LowSend<BStaticSender, C16, 2> {
    send_msg_16(MSGT_E, BStaticSender::new(EB_4))
}

/// 发送 `E-5` 消息 (错误: 错误的消息参数)
#[cfg(feature = "r2c3p-crc16")]
pub fn send_e5() -> LowSend<BStaticSender, C16, 2> {
    send_msg_16(MSGT_E, BStaticSender::new(EB_5))
}

#[cfg(test)]
mod test;
