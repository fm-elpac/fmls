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

/// 什么也不发送
pub struct NoneSender {}

impl NoneSender {
    pub fn new() -> Self {
        Self {}
    }
}

impl Iterator for NoneSender {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        None
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

/// `hex(u8)` 读取 (16 进制)
///
/// 如果格式错误, 返回 `None`
pub fn hex_u8(b: &[u8]) -> Option<u8> {
    if b.len() != 2 {
        return None;
    }

    // 转换一个 hex 字符
    fn h(x: u8) -> Option<u8> {
        match x {
            b'0'..=b'9' => Some(x - b'0'),
            b'a'..=b'f' => Some(x - b'a' + 10),
            _ => None,
        }
    }

    if let Some(x) = h(b[0]) {
        if let Some(y) = h(b[1]) {
            let u = (x << 4) | y;
            return Some(u);
        }
    }

    None
}

/// 判断长度是否为 2 的倍数
///
/// 不为 2 返回 true
fn len_n2(h: &[u8]) -> bool {
    // 检查最低位为 0
    h.len() & 1 != 0
}

/// `hex(u16)` 读取 (16 进制)
pub fn hex_u16(h: &[u8]) -> Option<u16> {
    if len_n2(h) || h.len() < 2 {
        return None;
    }

    let mut u: u16 = 0;
    for i in 0..2 {
        let end: usize = i * 2 + 2;
        if end > h.len() {
            break;
        }
        match hex_u8(&h[(i * 2)..end]) {
            Some(x) => {
                u = (u << 8) | (x as u16);
            }
            None => {
                return None;
            }
        }
    }
    Some(u)
}

/// `hex(u32)` 读取 (16 进制)
pub fn hex_u32(h: &[u8]) -> Option<u32> {
    if len_n2(h) || h.len() < 2 {
        return None;
    }

    let mut u: u32 = 0;
    for i in 0..4 {
        let end: usize = i * 2 + 2;
        if end > h.len() {
            break;
        }
        match hex_u8(&h[(i * 2)..end]) {
            Some(x) => {
                u = (u << 8) | (x as u32);
            }
            None => {
                return None;
            }
        }
    }
    Some(u)
}

/// `hex(u64)` 读取 (16 进制)
pub fn hex_u64(h: &[u8]) -> Option<u64> {
    if len_n2(h) || h.len() < 2 {
        return None;
    }

    let mut u: u64 = 0;
    for i in 0..8 {
        let end: usize = i * 2 + 2;
        if end > h.len() {
            break;
        }
        match hex_u8(&h[(i * 2)..end]) {
            Some(x) => {
                u = (u << 8) | (x as u64);
            }
            None => {
                return None;
            }
        }
    }
    Some(u)
}

/// n(`u8`) 读取 (10 进制)
pub fn n_u8(h: &[u8]) -> Option<u8> {
    // 转换一个 10 进制字符
    fn d(x: u8) -> Option<u8> {
        match x {
            b'0'..=b'9' => Some(x - b'0'),
            _ => None,
        }
    }

    match h.len() {
        // 1 位数 (0 ~ 9)
        1 => d(h[0]),
        // 2 位数 (10 ~ 99)
        2 => {
            if let Some(x) = d(h[0]) {
                if let Some(y) = d(h[1]) {
                    return Some(x * 10 + y);
                }
            }
            None
        }
        // 3 位数 (100 ~ 255)
        3 => {
            // 首先转换成 u32
            if let Some(x) = d(h[0]) {
                if let Some(y) = d(h[1]) {
                    if let Some(z) = d(h[2]) {
                        let u: u32 = ((x as u32) * 100) + ((y as u32) * 10) + (z as u32);
                        // 最大值 255
                        if u <= 0xff {
                            return Some(u as u8);
                        }
                    }
                }
            }
            None
        }
        _ => None,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn fifo2() {
        let mut f = Fifo2::new();
        assert_eq!(f.feed(1), None);
        assert_eq!(f.feed(2), None);
        assert_eq!(f.feed(3), Some(1));
        assert_eq!(f.feed(4), Some(2));
        assert_eq!(f.feed(5), Some(3));
        assert_eq!(f.feed(6), Some(4));
        assert_eq!(f.feed(7), Some(5));
        assert_eq!(f.feed(8), Some(6));
    }

    #[test]
    fn fifo4() {
        let mut f = Fifo4::new();
        assert_eq!(f.feed(1), None);
        assert_eq!(f.feed(2), None);
        assert_eq!(f.feed(3), None);
        assert_eq!(f.feed(4), None);
        assert_eq!(f.feed(5), Some(1));
        assert_eq!(f.feed(6), Some(2));
        assert_eq!(f.feed(7), Some(3));
        assert_eq!(f.feed(8), Some(4));
        assert_eq!(f.feed(9), Some(5));
        assert_eq!(f.feed(10), Some(6));
        assert_eq!(f.feed(11), Some(7));
        assert_eq!(f.feed(12), Some(8));
        assert_eq!(f.feed(13), Some(9));
        assert_eq!(f.feed(14), Some(10));
        assert_eq!(f.feed(15), Some(11));
        assert_eq!(f.feed(16), Some(12));
    }

    #[test]
    fn u16le_sender() {
        let mut s = U16LeSender::new(0x1234);
        assert_eq!(s.next(), Some(0x34));
        assert_eq!(s.next(), Some(0x12));
        assert_eq!(s.next(), None);
        assert_eq!(s.next(), None);
    }

    #[test]
    fn u32le_sender() {
        let mut s = U32LeSender::new(0x1234abcd);
        assert_eq!(s.next(), Some(0xcd));
        assert_eq!(s.next(), Some(0xab));
        assert_eq!(s.next(), Some(0x34));
        assert_eq!(s.next(), Some(0x12));
        assert_eq!(s.next(), None);
        assert_eq!(s.next(), None);
    }

    #[test]
    fn hex_u8_sender() {
        let mut s = HexU8Sender::new(0x1c);
        assert_eq!(s.next(), Some(b'1'));
        assert_eq!(s.next(), Some(b'c'));
        assert_eq!(s.next(), None);
        assert_eq!(s.next(), None);
    }

    #[test]
    fn hex_u16_sender() {
        let mut s = HexU16Sender::new(0x12ab);
        assert_eq!(s.next(), Some(b'1'));
        assert_eq!(s.next(), Some(b'2'));
        assert_eq!(s.next(), Some(b'a'));
        assert_eq!(s.next(), Some(b'b'));
        assert_eq!(s.next(), None);
        assert_eq!(s.next(), None);
    }

    #[test]
    fn hex_u32_sender() {
        let mut s = HexU32Sender::new(0x1234abcd);
        assert_eq!(s.next(), Some(b'1'));
        assert_eq!(s.next(), Some(b'2'));
        assert_eq!(s.next(), Some(b'3'));
        assert_eq!(s.next(), Some(b'4'));
        assert_eq!(s.next(), Some(b'a'));
        assert_eq!(s.next(), Some(b'b'));
        assert_eq!(s.next(), Some(b'c'));
        assert_eq!(s.next(), Some(b'd'));
        assert_eq!(s.next(), None);
        assert_eq!(s.next(), None);
    }

    #[test]
    fn hex_u64_sender() {
        let mut s = HexU64Sender::new(0x1234567890abcdef);
        assert_eq!(s.next(), Some(b'1'));
        assert_eq!(s.next(), Some(b'2'));
        assert_eq!(s.next(), Some(b'3'));
        assert_eq!(s.next(), Some(b'4'));
        assert_eq!(s.next(), Some(b'5'));
        assert_eq!(s.next(), Some(b'6'));
        assert_eq!(s.next(), Some(b'7'));
        assert_eq!(s.next(), Some(b'8'));
        assert_eq!(s.next(), Some(b'9'));
        assert_eq!(s.next(), Some(b'0'));
        assert_eq!(s.next(), Some(b'a'));
        assert_eq!(s.next(), Some(b'b'));
        assert_eq!(s.next(), Some(b'c'));
        assert_eq!(s.next(), Some(b'd'));
        assert_eq!(s.next(), Some(b'e'));
        assert_eq!(s.next(), Some(b'f'));
        assert_eq!(s.next(), None);
        assert_eq!(s.next(), None);
    }

    #[test]
    fn vec_sender() {
        let mut s = VecSender::new(b"test 666");
        assert_eq!(s.next(), Some(b't'));
        assert_eq!(s.next(), Some(b'e'));
        assert_eq!(s.next(), Some(b's'));
        assert_eq!(s.next(), Some(b't'));
        assert_eq!(s.next(), Some(b' '));
        assert_eq!(s.next(), Some(b'6'));
        assert_eq!(s.next(), Some(b'6'));
        assert_eq!(s.next(), Some(b'6'));
        assert_eq!(s.next(), None);
        assert_eq!(s.next(), None);
    }

    #[test]
    fn none_sender() {
        let mut s = NoneSender::new();
        assert_eq!(s.next(), None);
        assert_eq!(s.next(), None);
    }

    #[test]
    fn n_u8_sender() {
        let mut s = NU8Sender::new(0);
        assert_eq!(s.next(), Some(b'0'));
        assert_eq!(s.next(), None);
        assert_eq!(s.next(), None);

        let mut s = NU8Sender::new(1);
        assert_eq!(s.next(), Some(b'1'));
        assert_eq!(s.next(), None);
        assert_eq!(s.next(), None);

        let mut s = NU8Sender::new(2);
        assert_eq!(s.next(), Some(b'2'));
        assert_eq!(s.next(), None);
        assert_eq!(s.next(), None);

        let mut s = NU8Sender::new(9);
        assert_eq!(s.next(), Some(b'9'));
        assert_eq!(s.next(), None);
        assert_eq!(s.next(), None);

        let mut s = NU8Sender::new(10);
        assert_eq!(s.next(), Some(b'1'));
        assert_eq!(s.next(), Some(b'0'));
        assert_eq!(s.next(), None);
        assert_eq!(s.next(), None);

        let mut s = NU8Sender::new(11);
        assert_eq!(s.next(), Some(b'1'));
        assert_eq!(s.next(), Some(b'1'));
        assert_eq!(s.next(), None);
        assert_eq!(s.next(), None);

        let mut s = NU8Sender::new(90);
        assert_eq!(s.next(), Some(b'9'));
        assert_eq!(s.next(), Some(b'0'));
        assert_eq!(s.next(), None);
        assert_eq!(s.next(), None);

        let mut s = NU8Sender::new(99);
        assert_eq!(s.next(), Some(b'9'));
        assert_eq!(s.next(), Some(b'9'));
        assert_eq!(s.next(), None);
        assert_eq!(s.next(), None);

        let mut s = NU8Sender::new(100);
        assert_eq!(s.next(), Some(b'1'));
        assert_eq!(s.next(), Some(b'0'));
        assert_eq!(s.next(), Some(b'0'));
        assert_eq!(s.next(), None);
        assert_eq!(s.next(), None);

        let mut s = NU8Sender::new(101);
        assert_eq!(s.next(), Some(b'1'));
        assert_eq!(s.next(), Some(b'0'));
        assert_eq!(s.next(), Some(b'1'));
        assert_eq!(s.next(), None);
        assert_eq!(s.next(), None);

        let mut s = NU8Sender::new(123);
        assert_eq!(s.next(), Some(b'1'));
        assert_eq!(s.next(), Some(b'2'));
        assert_eq!(s.next(), Some(b'3'));
        assert_eq!(s.next(), None);
        assert_eq!(s.next(), None);

        let mut s = NU8Sender::new(199);
        assert_eq!(s.next(), Some(b'1'));
        assert_eq!(s.next(), Some(b'9'));
        assert_eq!(s.next(), Some(b'9'));
        assert_eq!(s.next(), None);
        assert_eq!(s.next(), None);

        let mut s = NU8Sender::new(200);
        assert_eq!(s.next(), Some(b'2'));
        assert_eq!(s.next(), Some(b'0'));
        assert_eq!(s.next(), Some(b'0'));
        assert_eq!(s.next(), None);
        assert_eq!(s.next(), None);

        let mut s = NU8Sender::new(222);
        assert_eq!(s.next(), Some(b'2'));
        assert_eq!(s.next(), Some(b'2'));
        assert_eq!(s.next(), Some(b'2'));
        assert_eq!(s.next(), None);
        assert_eq!(s.next(), None);

        let mut s = NU8Sender::new(254);
        assert_eq!(s.next(), Some(b'2'));
        assert_eq!(s.next(), Some(b'5'));
        assert_eq!(s.next(), Some(b'4'));
        assert_eq!(s.next(), None);
        assert_eq!(s.next(), None);

        let mut s = NU8Sender::new(255);
        assert_eq!(s.next(), Some(b'2'));
        assert_eq!(s.next(), Some(b'5'));
        assert_eq!(s.next(), Some(b'5'));
        assert_eq!(s.next(), None);
        assert_eq!(s.next(), None);
    }

    #[test]
    fn test_hex_u8() {
        assert_eq!(hex_u8(b""), None);
        assert_eq!(hex_u8(b"0"), None);
        assert_eq!(hex_u8(b"123"), None);
        assert_eq!(hex_u8(b"0x"), None);

        assert_eq!(hex_u8(b"00"), Some(0x00));
        assert_eq!(hex_u8(b"ff"), Some(0xff));

        assert_eq!(hex_u8(b"01"), Some(0x01));
        assert_eq!(hex_u8(b"23"), Some(0x23));
        assert_eq!(hex_u8(b"45"), Some(0x45));
        assert_eq!(hex_u8(b"67"), Some(0x67));
        assert_eq!(hex_u8(b"89"), Some(0x89));
        assert_eq!(hex_u8(b"ab"), Some(0xab));
        assert_eq!(hex_u8(b"cd"), Some(0xcd));
        assert_eq!(hex_u8(b"ef"), Some(0xef));
    }

    #[test]
    fn test_hex_u16() {
        assert_eq!(hex_u16(b""), None);
        assert_eq!(hex_u16(b"123"), None);

        assert_eq!(hex_u16(b"0123"), Some(0x123));
        assert_eq!(hex_u16(b"4567"), Some(0x4567));
        assert_eq!(hex_u16(b"89ab"), Some(0x89ab));
        assert_eq!(hex_u16(b"cdef"), Some(0xcdef));
    }

    #[test]
    fn test_hex_u32() {
        assert_eq!(hex_u32(b""), None);
        assert_eq!(hex_u32(b"123"), None);

        assert_eq!(hex_u32(b"01234567"), Some(0x1234567));
        assert_eq!(hex_u32(b"89abcdef"), Some(0x89abcdef));
    }

    #[test]
    fn test_hex_u64() {
        assert_eq!(hex_u64(b""), None);
        assert_eq!(hex_u64(b"123"), None);

        assert_eq!(hex_u64(b"123456789abcdef0"), Some(0x123456789abcdef0));
    }

    #[test]
    fn test_n_u8() {
        assert_eq!(n_u8(b""), None);
        assert_eq!(n_u8(b"x"), None);

        assert_eq!(n_u8(b"0"), Some(0));
        assert_eq!(n_u8(b"1"), Some(1));
        assert_eq!(n_u8(b"2"), Some(2));
        assert_eq!(n_u8(b"9"), Some(9));
        assert_eq!(n_u8(b"10"), Some(10));
        assert_eq!(n_u8(b"11"), Some(11));
        assert_eq!(n_u8(b"90"), Some(90));
        assert_eq!(n_u8(b"99"), Some(99));
        assert_eq!(n_u8(b"100"), Some(100));
        assert_eq!(n_u8(b"101"), Some(101));
        assert_eq!(n_u8(b"123"), Some(123));
        assert_eq!(n_u8(b"199"), Some(199));
        assert_eq!(n_u8(b"200"), Some(200));
        assert_eq!(n_u8(b"222"), Some(222));
        assert_eq!(n_u8(b"254"), Some(254));
        assert_eq!(n_u8(b"255"), Some(255));
        assert_eq!(n_u8(b"256"), None);
        assert_eq!(n_u8(b"257"), None);
        assert_eq!(n_u8(b"1000"), None);
    }
}
