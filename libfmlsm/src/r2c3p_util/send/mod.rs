//! 消息发送迭代器 (Iterator) 一次返回一个字节
//!
//! 用于避免动态内存分配及发送缓冲区

use core::iter::Iterator;

use crate::r2c3p::{BYTE_EQ, BYTE_LF, BYTE_SPACE, P_VERSION};
use crate::r2c3p_low::{BStaticSender, Escape, LowSendC, C0};

#[cfg(feature = "r2c3p-crc32")]
use crate::r2c3p::{MSGT_V, MSG_LEN_CRC16};

#[cfg(feature = "r2c3p-crc32")]
use crate::r2c3p_low::C32;
#[cfg(feature = "r2c3p-crc16")]
use crate::r2c3p_low::{BArraySender, CrcT, C16};

/// MsgSender 的内部状态
#[derive(Debug, Clone, PartialEq)]
enum MsgSenderS {
    /// 正在发送消息数据 (内部发送器 `LowSend<T, C0, 0>`)
    D,
    /// 正在发送 CRC
    Crc,
    /// 发送完毕
    None,
}

/// 发送一条消息的封装
///
/// 一次发送一个字节
#[derive(Debug, Clone)]
pub struct MsgSender<T: Iterator<Item = u8>> {
    s: MsgSenderS,
    // 内部发送器
    l: LowSendC<T, C0, 0>,
    // 转义处理
    e: Escape,

    // 消息类型
    #[cfg(feature = "r2c3p-crc32")]
    t: u8,
    // 发送长度
    #[cfg(feature = "r2c3p-crc16")]
    len: usize,
    // 计算 crc
    #[cfg(feature = "r2c3p-crc16")]
    c16: C16,
    #[cfg(feature = "r2c3p-crc32")]
    c32: C32,

    // 发送 crc
    #[cfg(feature = "r2c3p-crc16")]
    c16s: BArraySender<2>,
    #[cfg(feature = "r2c3p-crc32")]
    c32s: BArraySender<4>,
}

impl<T: Iterator<Item = u8>> MsgSender<T> {
    pub fn new(t: u8, d: T) -> Self {
        Self {
            s: MsgSenderS::D,
            l: LowSendC::new(t, d, C0::new()),
            e: Escape::new(),
            #[cfg(feature = "r2c3p-crc32")]
            t,
            #[cfg(feature = "r2c3p-crc16")]
            len: 0,
            #[cfg(feature = "r2c3p-crc16")]
            c16: C16::new(),
            #[cfg(feature = "r2c3p-crc32")]
            c32: C32::new(),

            #[cfg(feature = "r2c3p-crc16")]
            c16s: BArraySender::new([0; 2]),
            #[cfg(feature = "r2c3p-crc32")]
            c32s: BArraySender::new([0; 4]),
        }
    }

    // 发送一个字节时, 更新内部状态, 处理转义
    fn send_byte(&mut self, u: u8) -> u8 {
        // 计算 crc
        #[cfg(feature = "r2c3p-crc16")]
        {
            self.len += 1;
            self.c16.feed(u);

            #[cfg(feature = "r2c3p-crc32")]
            {
                self.c32.feed(u);
            }
        }

        // 处理转义
        self.e.feed(u)
    }

    // 判断发送 crc16 / crc32
    #[cfg(feature = "r2c3p-crc32")]
    fn use_crc32(&self) -> bool {
        // `V` 消息强制使用 crc32
        if (self.t == MSGT_V) || (self.len > MSG_LEN_CRC16 as usize) {
            true
        } else {
            false
        }
    }

    // 准备发送 crc
    fn setup_crc(&mut self) {
        #[cfg(feature = "r2c3p-crc32")]
        if self.use_crc32() {
            let c = self.c32.result();
            self.c32s = BArraySender::new(c);
            return;
        }

        #[cfg(feature = "r2c3p-crc16")]
        {
            // 计算 crc
            let c = self.c16.result();
            // 设置 crc 发送器
            self.c16s = BArraySender::new(c);
        }
    }

    // 获取要发送的 crc 的一个字节
    fn crc_next(&mut self) -> Option<u8> {
        #[cfg(feature = "r2c3p-crc32")]
        if self.use_crc32() {
            return self.c32s.next();
        }

        #[cfg(feature = "r2c3p-crc16")]
        {
            self.c16s.next()
        }
        #[cfg(not(feature = "r2c3p-crc16"))]
        {
            None
        }
    }
}

impl<T: Iterator<Item = u8>> Iterator for MsgSender<T> {
    type Item = u8;

    /// 当返回 `None` 时, 不一定发送完毕, 需要检查 `.done()`
    fn next(&mut self) -> Option<u8> {
        // 优先发送转义字符
        if let Some(b) = self.e.next() {
            return Some(b);
        }

        match self.s {
            MsgSenderS::D => {
                match self.l.next() {
                    Some(x) => {
                        let b = self.send_byte(x);
                        Some(b)
                    }
                    None => {
                        // 消息发送完毕
                        self.setup_crc();
                        self.s = MsgSenderS::Crc;
                        self.next()
                    }
                }
            }
            MsgSenderS::Crc => {
                match self.crc_next() {
                    Some(b) => {
                        // 处理转义
                        Some(self.e.feed(b))
                    }
                    None => {
                        // crc 发送完毕
                        self.s = MsgSenderS::None;
                        // 消息结束标志
                        Some(BYTE_LF)
                    }
                }
            }
            MsgSenderS::None => None,
        }
    }
}

#[derive(PartialEq)]
enum VSenderS {
    /// 正在发送 p
    P,
    /// 正在发送 firmware
    Firmware,
    /// 正在发送 hardware
    Hardware,
    /// 正在发送 Extra
    Extra,
    /// 发送完毕
    None,
}

/// 发送 `V` 消息的数据部分
pub struct VSender<T: Iterator<Item = u8>, U: Iterator<Item = u8>> {
    s: VSenderS,
    firmware: &'static [u8],
    hardware: T,
    extra: Option<U>,
    v: BStaticSender,
}

impl<T: Iterator<Item = u8>, U: Iterator<Item = u8>> VSender<T, U> {
    pub fn new(firmware: &'static [u8], hardware: T, extra: Option<U>) -> Self {
        Self {
            s: VSenderS::P,
            firmware,
            hardware,
            extra,
            v: BStaticSender::new(P_VERSION),
        }
    }
}

impl<T: Iterator<Item = u8>, U: Iterator<Item = u8>> Iterator for VSender<T, U> {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        match self.s {
            VSenderS::P => match self.v.next() {
                Some(b) => Some(b),
                None => {
                    // 更新发送状态
                    self.v = BStaticSender::new(self.firmware);
                    self.s = VSenderS::Firmware;
                    // 发送分隔字节
                    Some(BYTE_LF)
                }
            },
            VSenderS::Firmware => match self.v.next() {
                Some(b) => Some(b),
                None => {
                    self.s = VSenderS::Hardware;
                    Some(BYTE_LF)
                }
            },
            VSenderS::Hardware => match self.hardware.next() {
                Some(b) => Some(b),
                None => {
                    if self.extra.is_some() {
                        self.s = VSenderS::Extra;
                        Some(BYTE_LF)
                    } else {
                        // 发送完毕
                        self.s = VSenderS::None;
                        None
                    }
                }
            },
            VSenderS::Extra => match &mut self.extra {
                Some(s) => match s.next() {
                    Some(b) => Some(b),
                    None => {
                        // 发送完毕
                        self.s = VSenderS::None;
                        None
                    }
                },
                None => None,
            },
            VSenderS::None => None,
        }
    }
}

#[derive(PartialEq)]
enum ESenderS {
    /// 正在发送错误码
    C,
    /// 正在发送错误信息
    M,
    /// 发送完毕
    None,
}

/// 发送 `E` 消息的数据部分
pub struct ESender<T: Iterator<Item = u8>, U: Iterator<Item = u8>> {
    s: ESenderS,
    /// 错误码发送器
    c: T,
    /// 错误信息发送器
    m: Option<U>,
}

impl<T: Iterator<Item = u8>, U: Iterator<Item = u8>> ESender<T, U> {
    pub fn new(c: T, m: Option<U>) -> Self {
        Self {
            s: ESenderS::C,
            c,
            m,
        }
    }
}

impl<T: Iterator<Item = u8>, U: Iterator<Item = u8>> Iterator for ESender<T, U> {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        match self.s {
            ESenderS::C => match self.c.next() {
                Some(b) => Some(b),
                None => {
                    if self.m.is_some() {
                        self.s = ESenderS::M;
                        // 发送空格字符
                        Some(BYTE_SPACE)
                    } else {
                        // 发送完毕
                        self.s = ESenderS::None;
                        None
                    }
                }
            },
            ESenderS::M => match &mut self.m {
                Some(m) => match m.next() {
                    Some(b) => Some(b),
                    None => {
                        self.s = ESenderS::None;
                        None
                    }
                },
                None => {
                    self.s = ESenderS::None;
                    None
                }
            },
            ESenderS::None => None,
        }
    }
}

#[derive(PartialEq)]
enum CSenderS {
    /// 正在发送 K
    K,
    /// 正在发送 V
    V,
    /// 发送完毕
    None,
}

/// 发送 `C` 消息的数据部分
pub struct CSender<T: Iterator<Item = u8>> {
    s: CSenderS,
    k: BStaticSender,
    v: T,
}

impl<T: Iterator<Item = u8>> CSender<T> {
    pub fn new(k: &'static [u8], v: T) -> Self {
        Self {
            s: CSenderS::K,
            k: BStaticSender::new(k),
            v,
        }
    }
}

impl<T: Iterator<Item = u8>> Iterator for CSender<T> {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        match self.s {
            CSenderS::K => match self.k.next() {
                Some(b) => Some(b),
                None => {
                    self.s = CSenderS::V;
                    // 发送等号
                    Some(BYTE_EQ)
                }
            },
            CSenderS::V => match self.v.next() {
                Some(b) => Some(b),
                None => {
                    self.s = CSenderS::None;
                    None
                }
            },
            CSenderS::None => None,
        }
    }
}

#[cfg(test)]
mod test;
