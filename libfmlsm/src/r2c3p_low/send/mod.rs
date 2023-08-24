//! 发送消息

use core::fmt::Debug;
use core::iter::Iterator;

use crate::r2c3p::{
    BYTE_EQ, BYTE_LF, BYTE_SPACE, EB_2, EB_3, EB_4, EB_5, MSGT_E, MSGT_V, P_VERSION,
};

use super::{BArraySender, BStaticSender, Escape, HexArraySender};

#[cfg(feature = "r2c3p-crc16")]
use super::Crc16;
#[cfg(feature = "r2c3p-crc32")]
use super::Crc32;

#[derive(Debug, Clone, PartialEq)]
enum LowSendCS {
    // 正在发送消息类型
    T,
    // 正在发送附加数据
    D,
    // 正在发送 CRC
    C,
    // 发送完毕
    None,
}

/// 发送一条消息 (不含转义处理)
#[derive(Debug, Clone)]
pub struct LowSendC<T: Iterator<Item = u8>, C: CrcT<N>, const N: usize> {
    // 发送状态
    s: LowSendCS,
    // 用于计算 crc
    c: C,
    // 发送消息的附加数据
    d: T,
    // 消息类型
    t: u8,
    // 用于发送 crc
    cs: BArraySender<N>,
}

impl<T: Iterator<Item = u8>, C: CrcT<N>, const N: usize> LowSendC<T, C, N> {
    pub fn new(t: u8, d: T, c: C) -> Self {
        Self {
            s: LowSendCS::T,
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
        b
    }
}

impl<T: Iterator<Item = u8>, C: CrcT<N>, const N: usize> Iterator for LowSendC<T, C, N> {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        match self.s {
            LowSendCS::T => {
                let b = self.send_b(self.t);
                self.s = LowSendCS::D;

                Some(b)
            }
            LowSendCS::D => match self.d.next() {
                Some(b) => Some(self.send_b(b)),
                None => {
                    // 准备发送 crc
                    self.cs = BArraySender::new(self.c.result());
                    self.s = LowSendCS::C;
                    self.next()
                }
            },
            LowSendCS::C => match self.cs.next() {
                // 发送 CRC, 无需再计算 crc
                Some(b) => Some(b),
                None => {
                    // 发送完毕
                    self.s = LowSendCS::None;
                    None
                }
            },
            LowSendCS::None => None,
        }
    }
}

/// 发送一条消息
#[derive(Debug, Clone)]
pub struct LowSend<T: Iterator<Item = u8>, C: CrcT<N>, const N: usize> {
    // 发送状态 (true 表示发送完成)
    s: bool,
    // 内部发送器 (不含转义处理)
    c: LowSendC<T, C, N>,
    // 用于转义处理
    e: Escape,
}

impl<T: Iterator<Item = u8>, C: CrcT<N>, const N: usize> LowSend<T, C, N> {
    pub fn new(t: u8, d: T, c: C) -> Self {
        Self {
            s: false,
            c: LowSendC::new(t, d, c),
            e: Escape::new(),
        }
    }
}

impl<T: Iterator<Item = u8>, C: CrcT<N>, const N: usize> Iterator for LowSend<T, C, N> {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        // 优先发送转义字符
        if let Some(b) = self.e.next() {
            return Some(b);
        }

        if self.s {
            None
        } else {
            match self.c.next() {
                Some(b) => {
                    // 处理转义
                    Some(self.e.feed(b))
                }
                None => {
                    // 发送完毕
                    self.s = true;
                    // 发送消息结束字节
                    Some(BYTE_LF)
                }
            }
        }
    }
}

/// CRC 计算接口
pub trait CrcT<const N: usize>: Debug + Clone {
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
#[derive(Debug, Clone)]
pub struct C0 {}

impl C0 {
    pub fn new() -> Self {
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
#[derive(Debug, Clone)]
pub struct C16 {
    c: Option<Crc16>,
}

#[cfg(feature = "r2c3p-crc16")]
impl C16 {
    pub fn new() -> Self {
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
#[derive(Debug, Clone)]
pub struct C32 {
    c: Option<Crc32>,
}

#[cfg(feature = "r2c3p-crc32")]
impl C32 {
    pub fn new() -> Self {
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
#[derive(Debug, Clone)]
pub struct C32F {
    c: [u8; 4],
}

impl C32F {
    pub fn new(c: [u8; 4]) -> Self {
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

/// 发送一条消息 (不同 CRC 选择)
pub fn sendc_msg<T: Iterator<Item = u8>, C: CrcT<N>, const N: usize>(
    t: u8,
    d: T,
    c: C,
) -> LowSend<T, C, N> {
    LowSend::new(t, d, c)
}

/// 发送一条消息 (不使用 CRC)
pub fn send0_msg<T: Iterator<Item = u8>>(t: u8, d: T) -> LowSend<T, C0, 0> {
    sendc_msg(t, d, C0::new())
}

/// 发送一条消息 (使用 crc16)
#[cfg(feature = "r2c3p-crc16")]
pub fn send16_msg<T: Iterator<Item = u8>>(t: u8, d: T) -> LowSend<T, C16, 2> {
    sendc_msg(t, d, C16::new())
}

/// 发送一条消息 (使用 crc32)
#[cfg(feature = "r2c3p-crc32")]
pub fn send32_msg<T: Iterator<Item = u8>>(t: u8, d: T) -> LowSend<T, C32, 4> {
    sendc_msg(t, d, C32::new())
}

/// 发送一条消息 (使用固定 crc32 值)
pub fn send32f_msg<T: Iterator<Item = u8>>(t: u8, d: T, c: [u8; 4]) -> LowSend<T, C32F, 4> {
    sendc_msg(t, d, C32F::new(c))
}

#[derive(Debug, Clone, PartialEq)]
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
#[derive(Debug, Clone)]
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

/// 发送 `V` 消息 (不同 CRC 选择)
pub fn sendc_v<const N: usize, C: CrcT<M>, const M: usize>(
    firmware: &'static [u8],
    hardware_name: &'static [u8],
    hardware_id: [u8; N],
    c: C,
) -> LowSend<LowVSender<N>, C, M> {
    sendc_msg(
        MSGT_V,
        LowVSender::new(firmware, hardware_name, hardware_id),
        c,
    )
}

/// 发送 `V` 消息
#[cfg(feature = "r2c3p-crc32")]
pub fn send_v<const N: usize>(
    firmware: &'static [u8],
    hardware_name: &'static [u8],
    hardware_id: [u8; N],
) -> LowSend<LowVSender<N>, C32, 4> {
    sendc_v(firmware, hardware_name, hardware_id, C32::new())
}

/// 发送 `V` 消息 (无 CRC)
pub fn send0_v<const N: usize>(
    firmware: &'static [u8],
    hardware_name: &'static [u8],
    hardware_id: [u8; N],
) -> LowSend<LowVSender<N>, C0, 0> {
    sendc_v(firmware, hardware_name, hardware_id, C0::new())
}

/// 发送 `E-2` 消息 (不同 CRC 选择)
pub fn sendc_e2<C: CrcT<N>, const N: usize>(c: C) -> LowSend<BStaticSender, C, N> {
    sendc_msg(MSGT_E, BStaticSender::new(EB_2), c)
}

/// 发送 `E-2` 消息 (错误: 消息太长)
#[cfg(feature = "r2c3p-crc16")]
pub fn send_e2() -> LowSend<BStaticSender, C16, 2> {
    sendc_e2(C16::new())
}

/// 发送 `E-2` 消息 (无 CRC)
pub fn send0_e2() -> LowSend<BStaticSender, C0, 0> {
    sendc_e2(C0::new())
}

/// 发送 `E-2` 消息, 带缓冲区长度 (不同 CRC 选择)
pub fn sendc_e2_len<const N: usize, C: CrcT<M>, const M: usize>(
    mut len: [u8; N],
    c: C,
) -> LowSend<BArraySender<N>, C, M> {
    len[0..2].copy_from_slice(EB_2);
    len[2] = BYTE_SPACE;
    sendc_msg(MSGT_E, BArraySender::new(len), c)
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
pub fn send_e2_len<const N: usize>(len: [u8; N]) -> LowSend<BArraySender<N>, C16, 2> {
    sendc_e2_len(len, C16::new())
}

/// 发送 `E-2` 消息, 带缓冲区长度 (无 CRC)
pub fn send0_e2_len<const N: usize>(len: [u8; N]) -> LowSend<BArraySender<N>, C0, 0> {
    sendc_e2_len(len, C0::new())
}

/// 发送 `E-3` 消息 (不同 CRC 选择)
pub fn sendc_e3<C: CrcT<N>, const N: usize>(t: u8, c: C) -> LowSend<BArraySender<4>, C, N> {
    let mut b: [u8; 4] = [0; 4];
    b[0..2].copy_from_slice(EB_3);
    b[2] = BYTE_SPACE;
    b[3] = t;
    sendc_msg(MSGT_E, BArraySender::new(b), c)
}

/// 发送 `E-3` 消息 (错误: 未知的消息类型)
#[cfg(feature = "r2c3p-crc16")]
pub fn send_e3(t: u8) -> LowSend<BArraySender<4>, C16, 2> {
    sendc_e3(t, C16::new())
}

/// 发送 `E-3` 消息 (无 CRC)
pub fn send0_e3(t: u8) -> LowSend<BArraySender<4>, C0, 0> {
    sendc_e3(t, C0::new())
}

/// 发送 `E-4` 消息 (不同 CRC 选择)
pub fn sendc_e4<C: CrcT<N>, const N: usize>(c: C) -> LowSend<BStaticSender, C, N> {
    sendc_msg(MSGT_E, BStaticSender::new(EB_4), c)
}

/// 发送 `E-4` 消息 (错误: 错误的消息格式)
#[cfg(feature = "r2c3p-crc16")]
pub fn send_e4() -> LowSend<BStaticSender, C16, 2> {
    sendc_e4(C16::new())
}

/// 发送 `E-4` 消息 (无 CRC)
pub fn send0_e4() -> LowSend<BStaticSender, C0, 0> {
    sendc_e4(C0::new())
}

/// 发送 `E-5` 消息 (不同 CRC 选择)
pub fn sendc_e5<C: CrcT<N>, const N: usize>(c: C) -> LowSend<BStaticSender, C, N> {
    sendc_msg(MSGT_E, BStaticSender::new(EB_5), c)
}

/// 发送 `E-5` 消息 (错误: 错误的消息参数)
#[cfg(feature = "r2c3p-crc16")]
pub fn send_e5() -> LowSend<BStaticSender, C16, 2> {
    sendc_e5(C16::new())
}

/// 发送 `E-5` 消息 (无 CRC)
pub fn send0_e5() -> LowSend<BStaticSender, C0, 0> {
    sendc_e5(C0::new())
}

#[derive(Debug, Clone, PartialEq)]
enum LowCSenderS {
    /// 正在发送 K
    K,
    /// 正在发送 V
    V,
    /// 发送完毕
    None,
}

/// 发送 `C` 消息的数据部分
#[derive(Debug, Clone)]
pub struct LowCSender<T: Iterator<Item = u8>> {
    s: LowCSenderS,
    // k
    k: BStaticSender,
    // v
    v: Option<T>,
}

impl<T: Iterator<Item = u8>> LowCSender<T> {
    pub fn new(k: &'static [u8], v: Option<T>) -> Self {
        Self {
            s: LowCSenderS::K,
            k: BStaticSender::new(k),
            v,
        }
    }
}

impl<T: Iterator<Item = u8>> Iterator for LowCSender<T> {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        match self.s {
            LowCSenderS::K => match self.k.next() {
                Some(b) => Some(b),
                None => {
                    if self.v.is_some() {
                        self.s = LowCSenderS::V;
                        // 发送 `=`
                        Some(BYTE_EQ)
                    } else {
                        // 发送完毕
                        self.s = LowCSenderS::None;
                        None
                    }
                }
            },
            LowCSenderS::V => match &mut self.v {
                Some(s) => match s.next() {
                    Some(b) => Some(b),
                    None => {
                        self.s = LowCSenderS::None;
                        None
                    }
                },
                None => {
                    self.s = LowCSenderS::None;
                    None
                }
            },
            LowCSenderS::None => None,
        }
    }
}

#[cfg(test)]
mod test;
