//! 消息发送迭代器 (Iterator) 一次返回一个字节
//!
//! 用于避免动态内存分配及发送缓冲区

use core::iter::Iterator;

use libfmlsc::r2c3p::{BYTE_EQ, BYTE_LF, BYTE_SPACE, P_VERSION};

#[cfg(feature = "r2c3p-crc32")]
use libfmlsc::r2c3p::MSGT_V;
#[cfg(feature = "r2c3p-crc16")]
use libfmlsc::r2c3p::MSG_LEN_CRC16;

use super::hex::VecSender;
use super::Escape;

#[cfg(feature = "r2c3p-crc16")]
use super::hex::U16LeSender;
#[cfg(feature = "r2c3p-crc32")]
use super::hex::U32LeSender;
#[cfg(feature = "r2c3p-crc16")]
use super::Crc16;
#[cfg(feature = "r2c3p-crc32")]
use super::Crc32;

/// MsgSender 的内部状态
#[derive(PartialEq)]
enum MsgSenderS {
    /// 正在发送消息类型
    Head,
    /// 正在发送消息数据
    Body,
    /// 正在发送 CRC
    #[cfg(feature = "r2c3p-crc16")]
    Crc,
    /// 正在发送消息结束标志
    End,
    /// 发送完毕
    None,
}

/// 发送一条消息的封装
///
/// 一次发送一个字节
pub struct MsgSender<T: Iterator<Item = u8>> {
    s: MsgSenderS,
    // 消息类型
    t: u8,
    // 消息附加数据
    i: T,
    // 转义处理
    e: Escape,
    // 发送长度
    #[cfg(feature = "r2c3p-crc16")]
    len: usize,
    // 计算 crc
    #[cfg(feature = "r2c3p-crc16")]
    c16: Option<Crc16>,
    #[cfg(feature = "r2c3p-crc32")]
    c32: Option<Crc32>,

    // 发送 crc
    #[cfg(feature = "r2c3p-crc16")]
    c16s: U16LeSender,
    #[cfg(feature = "r2c3p-crc32")]
    c32s: U32LeSender,
}

impl<T: Iterator<Item = u8>> MsgSender<T> {
    pub fn new(t: u8, i: T) -> Self {
        Self {
            s: MsgSenderS::Head,
            t,
            i,
            e: Escape::new(),
            #[cfg(feature = "r2c3p-crc16")]
            len: 0,
            #[cfg(feature = "r2c3p-crc16")]
            c16: Some(Crc16::new()),
            #[cfg(feature = "r2c3p-crc32")]
            c32: Some(Crc32::new()),

            #[cfg(feature = "r2c3p-crc16")]
            c16s: U16LeSender::new(0),
            #[cfg(feature = "r2c3p-crc32")]
            c32s: U32LeSender::new(0),
        }
    }

    /// 返回是否发送完毕
    pub fn done(&self) -> bool {
        self.s == MsgSenderS::None
    }

    // 发送一个字节时, 更新内部状态, 处理转义
    fn send_byte(&mut self, u: u8) -> u8 {
        // 计算 crc
        #[cfg(feature = "r2c3p-crc16")]
        {
            self.len += 1;
            self.c16.as_mut().unwrap().feed(u);

            #[cfg(feature = "r2c3p-crc32")]
            {
                self.c32.as_mut().unwrap().feed(u);
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

    #[cfg(feature = "r2c3p-crc16")]
    fn setup_crc16(&mut self) {
        // 计算 crc
        let c = self.c16.take().unwrap().result();
        // 设置 crc 发送器
        self.c16s = U16LeSender::new(c);
    }

    #[cfg(feature = "r2c3p-crc32")]
    fn setup_crc32(&mut self) {
        let c = self.c32.take().unwrap().result();
        self.c32s = U32LeSender::new(c);
    }

    // 准备发送 crc
    #[cfg(feature = "r2c3p-crc16")]
    fn setup_crc(&mut self) {
        #[cfg(feature = "r2c3p-crc32")]
        if self.use_crc32() {
            self.setup_crc32();
            return;
        }

        self.setup_crc16();
    }

    // 获取要发送的 crc 的一个字节
    #[cfg(feature = "r2c3p-crc16")]
    fn crc_next(&mut self) -> Option<u8> {
        #[cfg(feature = "r2c3p-crc32")]
        if self.use_crc32() {
            return self.c32s.next();
        }

        self.c16s.next()
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
            MsgSenderS::Head => {
                // 要发送的字节
                let b = self.send_byte(self.t);
                // 更新发送状态
                self.s = MsgSenderS::Body;

                Some(b)
            }
            MsgSenderS::Body => {
                match self.i.next() {
                    Some(x) => {
                        let b = self.send_byte(x);
                        Some(b)
                    }
                    None => {
                        // 消息发送完毕
                        #[cfg(feature = "r2c3p-crc16")]
                        {
                            self.setup_crc();
                            self.s = MsgSenderS::Crc;
                            None
                        }
                        #[cfg(not(feature = "r2c3p-crc16"))]
                        {
                            self.s = MsgSenderS::End;
                            None
                        }
                    }
                }
            }
            #[cfg(feature = "r2c3p-crc16")]
            MsgSenderS::Crc => {
                match self.crc_next() {
                    Some(b) => {
                        // 处理转义
                        Some(self.e.feed(b))
                    }
                    None => {
                        // crc 发送完毕
                        self.s = MsgSenderS::End;
                        None
                    }
                }
            }
            MsgSenderS::End => {
                // 发送完毕
                self.s = MsgSenderS::None;
                // 消息结束标志
                Some(BYTE_LF)
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
    v: VecSender,
}

impl<T: Iterator<Item = u8>, U: Iterator<Item = u8>> VSender<T, U> {
    pub fn new(firmware: &'static [u8], hardware: T, extra: Option<U>) -> Self {
        Self {
            s: VSenderS::P,
            firmware,
            hardware,
            extra,
            v: VecSender::new(P_VERSION),
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
                    self.v = VecSender::new(self.firmware);
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
    k: VecSender,
    v: T,
}

impl<T: Iterator<Item = u8>> CSender<T> {
    pub fn new(k: &'static [u8], v: T) -> Self {
        Self {
            s: CSenderS::K,
            k: VecSender::new(k),
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
