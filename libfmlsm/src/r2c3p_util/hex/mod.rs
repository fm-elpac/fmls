//! 字节处理工具

use core::iter::Iterator;

use crate::r2c3p::BYTE_HEX;
use crate::r2c3p_low::{BArraySender, HexArraySender};

/// u16 LE (小尾字节序) 发送器 (crc16)
#[derive(Debug, Clone)]
pub struct U16LeSender {
    s: BArraySender<2>,
}

impl U16LeSender {
    pub fn new(u: u16) -> Self {
        let b = u16::to_le_bytes(u);
        Self {
            s: BArraySender::new(b),
        }
    }
}

impl Iterator for U16LeSender {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        self.s.next()
    }
}

/// u32 LE (小尾字节序) 发送器 (crc32)
#[derive(Debug, Clone)]
pub struct U32LeSender {
    s: BArraySender<4>,
}

impl U32LeSender {
    pub fn new(u: u32) -> Self {
        let b = u32::to_le_bytes(u);
        Self {
            s: BArraySender::new(b),
        }
    }
}

impl Iterator for U32LeSender {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        self.s.next()
    }
}

/// `hex(u8)` 发送器
#[derive(Debug, Clone)]
pub struct HexU8Sender {
    s: HexArraySender<1>,
}

impl HexU8Sender {
    pub fn new(u: u8) -> Self {
        Self {
            s: HexArraySender::new([u]),
        }
    }
}

impl Iterator for HexU8Sender {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        self.s.next()
    }
}

/// `hex(u16)` 发送器
#[derive(Debug, Clone)]
pub struct HexU16Sender {
    s: HexArraySender<2>,
}

impl HexU16Sender {
    pub fn new(u: u16) -> Self {
        let b = u16::to_be_bytes(u);
        Self {
            s: HexArraySender::new(b),
        }
    }
}

impl Iterator for HexU16Sender {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        self.s.next()
    }
}

/// `hex(u32)` 发送器
#[derive(Debug, Clone)]
pub struct HexU32Sender {
    s: HexArraySender<4>,
}

impl HexU32Sender {
    pub fn new(u: u32) -> Self {
        let b = u32::to_be_bytes(u);
        Self {
            s: HexArraySender::new(b),
        }
    }
}

impl Iterator for HexU32Sender {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        self.s.next()
    }
}

/// `hex(u64)` 发送器
#[derive(Debug, Clone)]
pub struct HexU64Sender {
    s: HexArraySender<8>,
}

impl HexU64Sender {
    pub fn new(u: u64) -> Self {
        let b = u64::to_be_bytes(u);
        Self {
            s: HexArraySender::new(b),
        }
    }
}

impl Iterator for HexU64Sender {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        self.s.next()
    }
}

/// 10 进制数字发送
#[derive(Debug, Clone)]
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

#[cfg(test)]
mod test;
