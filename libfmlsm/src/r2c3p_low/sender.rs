//! 数据发送器 (一次发送一个字节)

use core::iter::Iterator;

use crate::r2c3p::BYTE_HEX;

/// 什么也不发送
pub struct NoneSender {}

impl NoneSender {
    pub const fn new() -> Self {
        Self {}
    }
}

impl Iterator for NoneSender {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        None
    }
}

/// 字节数组发送器 `[u8; N]`
pub struct BArraySender<const N: usize> {
    // 要发送的数据
    b: [u8; N],
    // 发送的位置
    i: usize,
}

impl<const N: usize> BArraySender<N> {
    pub fn new(b: [u8; N]) -> Self {
        Self { b, i: 0 }
    }
}

impl<const N: usize> Iterator for BArraySender<N> {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        if self.i < N {
            let b = Some(self.b[self.i]);
            self.i += 1;
            b
        } else {
            None
        }
    }
}

/// 字节静态引用发送器 `&'static [u8]`
pub struct BStaticSender {
    b: &'static [u8],
    i: usize,
}

impl BStaticSender {
    pub fn new(b: &'static [u8]) -> Self {
        Self { b, i: 0 }
    }
}

impl Iterator for BStaticSender {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        if self.i < self.b.len() {
            let b = Some(self.b[self.i]);
            self.i += 1;
            b
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum ByteHex {
    // 高位 (第一个 hex 字符)
    H,
    // 低位 (第二个 hex 字符)
    L,
}

/// `hex` 字节数组发送器 `[u8; N]`
pub struct HexArraySender<const N: usize> {
    b: [u8; N],
    i: usize,
    s: ByteHex,
}

impl<const N: usize> HexArraySender<N> {
    pub fn new(b: [u8; N]) -> Self {
        Self {
            b,
            i: 0,
            s: ByteHex::H,
        }
    }
}

impl<const N: usize> Iterator for HexArraySender<N> {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        if self.i < N {
            let b = self.b[self.i];
            match self.s {
                ByteHex::H => {
                    self.s = ByteHex::L;

                    Some(BYTE_HEX[(b >> 4) as usize])
                }
                ByteHex::L => {
                    self.s = ByteHex::H;
                    self.i += 1;

                    Some(BYTE_HEX[(b & 0x0f) as usize])
                }
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn none_sender() {
        let mut s = NoneSender::new();
        assert_eq!(s.next(), None);
        assert_eq!(s.next(), None);
    }
}
