//! 消息转义处理及 crc 计算
//!
//! 用于 UART 方式传输

use core::iter::Iterator;

#[cfg(feature = "r2c3p-crc16")]
use crc::{Crc, Digest};

use libfmlsc::r2c3p::{BYTE_B, BYTE_LF, BYTE_N, BYTE_S};

#[cfg(feature = "r2c3p-crc32")]
use libfmlsc::r2c3p::CRC_32;
#[cfg(feature = "r2c3p-crc16")]
use libfmlsc::r2c3p::{CRC_16, MSG_LEN_CRC16};

/// 转义处理
///
/// 用于发送消息之前, 一次处理一个字节
pub struct Escape {
    /// 转义时, 用于发送的下一个字节
    next_byte: Option<u8>,
}

impl Escape {
    pub fn new() -> Self {
        Self { next_byte: None }
    }

    /// 一次喂给一个字节
    ///
    /// 应该先用 `.next()` 检查是否发送完毕, 然后再投喂
    pub fn feed(&mut self, b: u8) -> u8 {
        // assert: self.next_byte == None
        match b {
            BYTE_LF => {
                self.next_byte = Some(BYTE_N);
                BYTE_B
            }
            BYTE_B => {
                self.next_byte = Some(BYTE_S);
                BYTE_B
            }
            _ => b,
        }
    }
}

impl Iterator for Escape {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        self.next_byte.take()
    }
}

/// 取消转义处理
///
/// 用于接收消息之后, 一次处理一个字节
pub struct Unescape {
    /// 转义模式标志
    ef: bool,
}

impl Unescape {
    pub fn new() -> Self {
        Self { ef: false }
    }

    /// 一次投喂一个字节
    /// 如果出错, 返回 `Err(())`
    pub fn feed(&mut self, b: u8) -> Result<Option<u8>, ()> {
        if self.ef {
            self.ef = false;

            match b {
                BYTE_N => Ok(Some(BYTE_LF)),
                BYTE_S => Ok(Some(BYTE_B)),
                _ => Err(()),
            }
        } else {
            match b {
                BYTE_LF => Err(()),
                BYTE_B => {
                    self.ef = true;
                    Ok(None)
                }
                _ => Ok(Some(b)),
            }
        }
    }
}

#[cfg(feature = "r2c3p-crc16")]
const C16: Crc<u16> = Crc::<u16>::new(&CRC_16);
#[cfg(feature = "r2c3p-crc32")]
const C32: Crc<u32> = Crc::<u32>::new(&CRC_32);

/// 计算 crc16
#[cfg(feature = "r2c3p-crc16")]
pub struct Crc16<'a> {
    d: Digest<'a, u16>,
}

impl<'a> Crc16<'a> {
    pub fn new() -> Self {
        Self { d: C16.digest() }
    }

    /// 一次喂给一个字节
    pub fn feed(&mut self, b: u8) {
        let a: [u8; 1] = [b];
        self.d.update(&a);
    }

    /// 获取计算结果
    pub fn result(self) -> u16 {
        self.d.finalize()
    }
}

/// 计算 crc32
#[cfg(feature = "r2c3p-crc32")]
pub struct Crc32<'a> {
    d: Digest<'a, u32>,
}

impl<'a> Crc32<'a> {
    pub fn new() -> Self {
        Self { d: C32.digest() }
    }

    /// 一次喂给一个字节
    pub fn feed(&mut self, b: u8) {
        let a: [u8; 1] = [b];
        self.d.update(&a);
    }

    /// 获取计算结果
    pub fn result(self) -> u32 {
        self.d.finalize()
    }
}

/// 根据数据长度选择 crc 类型
/// 如果出错返回 None
#[cfg(feature = "r2c3p-crc32")]
pub fn crc_len(len: usize, use_crc32: bool) -> Option<usize> {
    let u32b = core::mem::size_of::<u32>();
    if use_crc32 || len > (MSG_LEN_CRC16 as usize + u32b) {
        return Some(u32b);
    }

    let u16b = core::mem::size_of::<u16>();
    if len <= (MSG_LEN_CRC16 as usize + u16b) {
        return Some(u16b);
    }

    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_escape() {
        let mut e = Escape::new();
        assert_eq!(e.next(), None);
        // 普通数据
        assert_eq!(e.feed(b'1'), b'1');
        assert_eq!(e.next(), None);
        assert_eq!(e.feed(b'2'), b'2');
        assert_eq!(e.next(), None);
        assert_eq!(e.feed(b'a'), b'a');
        assert_eq!(e.next(), None);
        // 需要转义
        assert_eq!(e.feed(b'\n'), b'\\');
        assert_eq!(e.next(), Some(b'n'));
        assert_eq!(e.feed(b'3'), b'3');
        assert_eq!(e.next(), None);
        assert_eq!(e.feed(b'\\'), b'\\');
        assert_eq!(e.next(), Some(b's'));
        assert_eq!(e.feed(b'b'), b'b');
        assert_eq!(e.next(), None);
    }

    #[test]
    fn test_unescape() {
        let mut u = Unescape::new();
        // 普通数据
        assert_eq!(u.feed(b'a'), Ok(Some(b'a')));
        assert_eq!(u.feed(b'1'), Ok(Some(b'1')));
        assert_eq!(u.feed(b'2'), Ok(Some(b'2')));
        // 转义处理
        assert_eq!(u.feed(b'\\'), Ok(None));
        assert_eq!(u.feed(b'n'), Ok(Some(b'\n')));
        assert_eq!(u.feed(b'3'), Ok(Some(b'3')));
        assert_eq!(u.feed(b'\\'), Ok(None));
        assert_eq!(u.feed(b's'), Ok(Some(b'\\')));
        assert_eq!(u.feed(b'b'), Ok(Some(b'b')));
    }

    #[test]
    fn test_crc16() {
        let mut c = Crc16::new();
        c.feed(b'v');
        assert_eq!(c.result(), 0xe681);
    }

    #[test]
    fn test_crc32() {
        let mut c = Crc32::new();
        for _ in 0..33 {
            c.feed(b'V');
        }
        assert_eq!(c.result(), 0x14c7ad9e);
    }

    #[test]
    fn test_crc_len() {
        assert_eq!(crc_len(3, false), Some(2));
        assert_eq!(crc_len(33, false), Some(2));
        assert_eq!(crc_len(34, false), Some(2));
        assert_eq!(crc_len(35, false), None);
        assert_eq!(crc_len(36, false), None);
        assert_eq!(crc_len(37, false), Some(4));
        assert_eq!(crc_len(38, false), Some(4));
    }
}