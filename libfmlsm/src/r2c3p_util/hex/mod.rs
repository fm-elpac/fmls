//! 字节处理工具

use core::iter::Iterator;

use libfmlsc::r2c3p::BYTE_HEX;

/// 当前字节位置
#[derive(Clone, Copy, PartialEq)]
enum Fifo2I {
    A0 = 0,
    A1 = 1,
}

impl Fifo2I {
    /// 输出下一个位置
    pub fn next(&self) -> Self {
        match self {
            Self::A0 => Self::A1,
            Self::A1 => Self::A0,
        }
    }
}

/// 先进先出队列, 容量 2 字节 (crc16)
pub struct Fifo2 {
    /// 缓冲区
    b: [u8; 2],
    /// 字节位置
    i: Fifo2I,
    /// 存满标志
    f: bool,
}

impl Fifo2 {
    pub const fn new() -> Self {
        Self {
            b: [0, 0],
            i: Fifo2I::A0,
            f: false,
        }
    }

    /// 喂给一个字节, 只有存满后才有输出
    pub fn feed(&mut self, u: u8) -> Option<u8> {
        // 保存下一个字节
        let ni = self.i.next();
        let nb = self.b[ni as usize];
        // 保存存满标志
        let f = self.f;
        // 设置存满标志
        if ni == Fifo2I::A0 {
            self.f = true;
        }
        // 更新字节位置
        self.i = ni;
        // 存储数据
        self.b[ni as usize] = u;

        if f {
            Some(nb)
        } else {
            None
        }
    }

    /// 取出缓存的值, 转换为 u16 (LE)
    ///
    /// 会给自己填充 0
    pub fn to_u16(&mut self) -> Option<u16> {
        let mut b: [u8; 2] = [0; 2];
        match self.feed(0) {
            Some(u) => {
                b[0] = u;
            }
            None => {
                return None;
            }
        }
        match self.feed(0) {
            Some(u) => {
                b[1] = u;
            }
            None => {
                return None;
            }
        }
        Some(u16::from_le_bytes(b))
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Fifo4I {
    A0 = 0,
    A1 = 1,
    A2 = 2,
    A3 = 3,
}

impl Fifo4I {
    pub fn next(&self) -> Self {
        match self {
            Self::A0 => Self::A1,
            Self::A1 => Self::A2,
            Self::A2 => Self::A3,
            Self::A3 => Self::A0,
        }
    }
}

/// 先进先出队列, 容量 4 字节 (crc32)
pub struct Fifo4 {
    b: [u8; 4],
    i: Fifo4I,
    f: bool,
}

impl Fifo4 {
    pub const fn new() -> Self {
        Self {
            b: [0, 0, 0, 0],
            i: Fifo4I::A0,
            f: false,
        }
    }

    /// 喂给一个字节, 只有存满后才有输出
    pub fn feed(&mut self, u: u8) -> Option<u8> {
        let ni = self.i.next();
        let nb = self.b[ni as usize];
        let f = self.f;
        if ni == Fifo4I::A0 {
            self.f = true;
        }
        self.i = ni;
        self.b[ni as usize] = u;
        if f {
            Some(nb)
        } else {
            None
        }
    }

    /// 取出缓存的值, 转换为 u32 (LE)
    ///
    /// 会给自己填充 0
    pub fn to_u32(&mut self) -> Option<u32> {
        let mut b: [u8; 4] = [0; 4];
        match self.feed(0) {
            Some(u) => {
                b[0] = u;
            }
            None => {
                return None;
            }
        }
        match self.feed(0) {
            Some(u) => {
                b[1] = u;
            }
            None => {
                return None;
            }
        }
        match self.feed(0) {
            Some(u) => {
                b[2] = u;
            }
            None => {
                return None;
            }
        }
        match self.feed(0) {
            Some(u) => {
                b[3] = u;
            }
            None => {
                return None;
            }
        }
        Some(u32::from_le_bytes(b))
    }
}

enum U16LeSenderS {
    /// 字节 0
    B0,
    /// 字节 1
    B1,
    /// 发送完毕
    None,
}

impl U16LeSenderS {
    /// 返回下一个状态
    pub fn next(&self) -> Self {
        match self {
            Self::B0 => Self::B1,
            Self::B1 => Self::None,
            Self::None => Self::None,
        }
    }
}

/// u16 LE (小尾字节序) 发送器 (crc16)
pub struct U16LeSender {
    u: u16,
    s: U16LeSenderS,
}

impl U16LeSender {
    pub fn new(u: u16) -> Self {
        Self {
            u,
            s: U16LeSenderS::B0,
        }
    }
}

impl Iterator for U16LeSender {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        let b = match self.s {
            U16LeSenderS::B0 => Some(self.u as u8),
            U16LeSenderS::B1 => Some((self.u >> 8) as u8),
            U16LeSenderS::None => None,
        };
        // 更新状态
        self.s = self.s.next();
        b
    }
}

enum U32LeSenderS {
    B0,
    B1,
    B2,
    B3,
    None,
}

impl U32LeSenderS {
    pub fn next(&self) -> Self {
        match self {
            Self::B0 => Self::B1,
            Self::B1 => Self::B2,
            Self::B2 => Self::B3,
            Self::B3 => Self::None,
            Self::None => Self::None,
        }
    }
}

/// u32 LE (小尾字节序) 发送器 (crc32)
pub struct U32LeSender {
    u: u32,
    s: U32LeSenderS,
}

impl U32LeSender {
    pub fn new(u: u32) -> Self {
        Self {
            u,
            s: U32LeSenderS::B0,
        }
    }
}

impl Iterator for U32LeSender {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        let b = match self.s {
            U32LeSenderS::B0 => Some(self.u as u8),
            U32LeSenderS::B1 => Some((self.u >> 8) as u8),
            U32LeSenderS::B2 => Some((self.u >> 16) as u8),
            U32LeSenderS::B3 => Some((self.u >> 24) as u8),
            U32LeSenderS::None => None,
        };
        self.s = self.s.next();
        b
    }
}

enum HexU8SenderS {
    H0,
    H1,
    None,
}

impl HexU8SenderS {
    pub fn next(&self) -> Self {
        match self {
            Self::H0 => Self::H1,
            Self::H1 => Self::None,
            Self::None => Self::None,
        }
    }
}

/// `hex(u8)` 发送器
pub struct HexU8Sender {
    s: HexU8SenderS,
    u: u8,
}

impl HexU8Sender {
    pub fn new(u: u8) -> Self {
        Self {
            s: HexU8SenderS::H0,
            u,
        }
    }
}

impl Iterator for HexU8Sender {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        let b = match self.s {
            HexU8SenderS::H0 => Some(BYTE_HEX[(self.u >> 4) as usize]),
            HexU8SenderS::H1 => Some(BYTE_HEX[(self.u & 0x0f) as usize]),
            HexU8SenderS::None => None,
        };
        self.s = self.s.next();
        b
    }
}

enum HexU16SenderS {
    B0,
    B1,
    None,
}

impl HexU16SenderS {
    pub fn next(&self) -> Self {
        match self {
            Self::B0 => Self::B1,
            Self::B1 => Self::None,
            Self::None => Self::None,
        }
    }
}

/// `hex(u16)` 发送器
pub struct HexU16Sender {
    s: HexU16SenderS,
    u: u16,
    h: HexU8Sender,
}

impl HexU16Sender {
    pub fn new(u: u16) -> Self {
        Self {
            s: HexU16SenderS::B0,
            u,
            h: HexU8Sender::new((u >> 8) as u8),
        }
    }
}

impl Iterator for HexU16Sender {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        match self.s {
            HexU16SenderS::B0 => match self.h.next() {
                Some(b) => Some(b),
                None => {
                    self.s = self.s.next();
                    self.h = HexU8Sender::new(self.u as u8);
                    self.h.next()
                }
            },
            HexU16SenderS::B1 => match self.h.next() {
                Some(b) => Some(b),
                None => {
                    self.s = self.s.next();
                    None
                }
            },
            HexU16SenderS::None => None,
        }
    }
}

enum HexU32SenderS {
    B0,
    B1,
    B2,
    B3,
    None,
}

impl HexU32SenderS {
    pub fn next(&self) -> Self {
        match self {
            Self::B0 => Self::B1,
            Self::B1 => Self::B2,
            Self::B2 => Self::B3,
            Self::B3 => Self::None,
            Self::None => Self::None,
        }
    }
}

/// `hex(u32)` 发送器
pub struct HexU32Sender {
    s: HexU32SenderS,
    u: u32,
    h: HexU8Sender,
}

impl HexU32Sender {
    pub fn new(u: u32) -> Self {
        Self {
            s: HexU32SenderS::B0,
            u,
            h: HexU8Sender::new((u >> 24) as u8),
        }
    }
}

impl Iterator for HexU32Sender {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        match self.s {
            HexU32SenderS::B0 => match self.h.next() {
                Some(b) => Some(b),
                None => {
                    self.s = self.s.next();
                    self.h = HexU8Sender::new((self.u >> 16) as u8);
                    self.h.next()
                }
            },
            HexU32SenderS::B1 => match self.h.next() {
                Some(b) => Some(b),
                None => {
                    self.s = self.s.next();
                    self.h = HexU8Sender::new((self.u >> 8) as u8);
                    self.h.next()
                }
            },
            HexU32SenderS::B2 => match self.h.next() {
                Some(b) => Some(b),
                None => {
                    self.s = self.s.next();
                    self.h = HexU8Sender::new(self.u as u8);
                    self.h.next()
                }
            },
            HexU32SenderS::B3 => match self.h.next() {
                Some(b) => Some(b),
                None => {
                    self.s = self.s.next();
                    None
                }
            },
            HexU32SenderS::None => None,
        }
    }
}

enum HexU64SenderS {
    B0,
    B1,
    B2,
    B3,
    B4,
    B5,
    B6,
    B7,
    None,
}

impl HexU64SenderS {
    pub fn next(&self) -> Self {
        match self {
            Self::B0 => Self::B1,
            Self::B1 => Self::B2,
            Self::B2 => Self::B3,
            Self::B3 => Self::B4,
            Self::B4 => Self::B5,
            Self::B5 => Self::B6,
            Self::B6 => Self::B7,
            Self::B7 => Self::None,
            Self::None => Self::None,
        }
    }
}

/// `hex(u64)` 发送器
pub struct HexU64Sender {
    s: HexU64SenderS,
    u: u64,
    h: HexU8Sender,
}

impl HexU64Sender {
    pub fn new(u: u64) -> Self {
        Self {
            s: HexU64SenderS::B0,
            u,
            h: HexU8Sender::new((u >> 56) as u8),
        }
    }
}

impl Iterator for HexU64Sender {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        match self.s {
            HexU64SenderS::B0 => match self.h.next() {
                Some(b) => Some(b),
                None => {
                    self.s = self.s.next();
                    self.h = HexU8Sender::new((self.u >> 48) as u8);
                    self.h.next()
                }
            },
            HexU64SenderS::B1 => match self.h.next() {
                Some(b) => Some(b),
                None => {
                    self.s = self.s.next();
                    self.h = HexU8Sender::new((self.u >> 40) as u8);
                    self.h.next()
                }
            },
            HexU64SenderS::B2 => match self.h.next() {
                Some(b) => Some(b),
                None => {
                    self.s = self.s.next();
                    self.h = HexU8Sender::new((self.u >> 32) as u8);
                    self.h.next()
                }
            },
            HexU64SenderS::B3 => match self.h.next() {
                Some(b) => Some(b),
                None => {
                    self.s = self.s.next();
                    self.h = HexU8Sender::new((self.u >> 24) as u8);
                    self.h.next()
                }
            },
            HexU64SenderS::B4 => match self.h.next() {
                Some(b) => Some(b),
                None => {
                    self.s = self.s.next();
                    self.h = HexU8Sender::new((self.u >> 16) as u8);
                    self.h.next()
                }
            },
            HexU64SenderS::B5 => match self.h.next() {
                Some(b) => Some(b),
                None => {
                    self.s = self.s.next();
                    self.h = HexU8Sender::new((self.u >> 8) as u8);
                    self.h.next()
                }
            },
            HexU64SenderS::B6 => match self.h.next() {
                Some(b) => Some(b),
                None => {
                    self.s = self.s.next();
                    self.h = HexU8Sender::new(self.u as u8);
                    self.h.next()
                }
            },
            HexU64SenderS::B7 => match self.h.next() {
                Some(b) => Some(b),
                None => {
                    self.s = self.s.next();
                    None
                }
            },
            HexU64SenderS::None => None,
        }
    }
}

/// 根据引用发送字节
pub struct VecSender {
    v: &'static [u8],
    i: usize,
}

impl VecSender {
    pub fn new(v: &'static [u8]) -> Self {
        Self { v, i: 0 }
    }
}

impl Iterator for VecSender {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        if self.i < self.v.len() {
            let b = self.v[self.i];
            self.i += 1;
            Some(b)
        } else {
            None
        }
    }
}

/// 10 进制数字发送
pub struct NU8Sender {
    // 剩余要发送的数字
    u: u8,
    // 发送完毕
    done: bool,
    // 发送十位标志
    f: bool,
}

impl NU8Sender {
    pub fn new(u: u8) -> Self {
        Self {
            u,
            done: false,
            f: false,
        }
    }
}

impl Iterator for NU8Sender {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        if self.done {
            None
        } else {
            if (!self.f) && (self.u < 10) {
                // 发送个位数字
                let b = BYTE_HEX[self.u as usize];
                self.done = true;
                Some(b)
            } else if self.u < 100 {
                self.f = false;
                // 发送十位数字
                let n = self.u / 10;
                self.u = self.u - n * 10;
                Some(BYTE_HEX[n as usize])
            } else {
                // 标记应该发送十位
                self.f = true;
                // 发送百位数字
                let n = self.u / 100;
                self.u = self.u - n * 100;
                Some(BYTE_HEX[n as usize])
            }
        }
    }
}

/// 发送一个字节
pub struct U8Sender {
    u: u8,
    // 发送完毕标志
    done: bool,
}

impl U8Sender {
    pub fn new(u: u8) -> Self {
        Self { u, done: false }
    }
}

impl Iterator for U8Sender {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        if self.done {
            None
        } else {
            self.done = true;
            Some(self.u)
        }
    }
}

#[cfg(test)]
mod test;
