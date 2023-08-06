//! 接收消息

use crate::r2c3p::{BYTE_LF, MSGT_V, MSGT_V_R};

use super::Unescape;

#[cfg(feature = "r2c3p-crc16")]
use super::{crc_len, Crc16};
#[cfg(feature = "r2c3p-crc32")]
use super::{CrcT, C32};

#[derive(PartialEq)]
enum LowRecvS {
    /// 正在接收消息数据 (消息类型+附加数据+CRC)
    D,
    /// 成功接收
    Ok,
    /// 错误, 丢弃消息 (等待结束字符重置状态)
    Err,
}

/// 接收消息
///
/// `N`: 内部接收缓冲区长度, 可接收的实际消息长度需要减去 crc
pub struct LowRecv<const N: usize> {
    // 当前状态
    s: LowRecvS,
    // 内部接收缓冲区
    b: [u8; N],
    // 消息的总长度 (取消转义之后, 不含结束字节) (消息类型+附加数据+CRC)
    m_len: usize,
    // 用于取消转义处理
    e: Unescape,
    // 用于计算 crc32
    #[cfg(feature = "r2c3p-crc32")]
    c: C32,

    // 消息丢弃计数
    crd: u32,
    // 消息成功接收计数
    cr: u32,
}

impl<const N: usize> LowRecv<N> {
    pub const fn new() -> Self {
        Self {
            s: LowRecvS::D,
            b: [0; N],
            m_len: 0,
            e: Unescape::new(),
            #[cfg(feature = "r2c3p-crc32")]
            c: C32::new(),
            crd: 0,
            cr: 0,
        }
    }

    /// 获取消息类型, 成功接收状态返回 `Some()`
    pub fn get_t(&self) -> Option<u8> {
        if LowRecvS::Ok == self.s {
            Some(self.b[0])
        } else {
            None
        }
    }

    /// 消息太长错误
    pub fn get_e2(&self) -> bool {
        self.m_len > N
    }

    /// 返回 crc 长度
    fn crc_l(&self) -> usize {
        // `vv` 消息
        if self.m_len == 1 {
            return 0;
        }

        #[cfg(feature = "r2c3p-crc16")]
        {
            let use_crc32 = (self.m_len > 0) && (self.b[0] == MSGT_V);
            match crc_len(self.m_len, use_crc32) {
                Some(l) => l,
                None => 0,
            }
        }
        #[cfg(not(feature = "r2c3p-crc16"))]
        {
            0
        }
    }

    /// 获取附加数据, 成功接收状态返回 `Some()`
    pub fn get_body(&self) -> Option<&[u8]> {
        if LowRecvS::Ok == self.s {
            let b_end = self.m_len - self.crc_l();
            if b_end > 1 {
                return Some(&self.b[1..b_end]);
            }
        }
        None
    }

    /// 返回成功接收消息的计数
    pub fn get_cr(&self) -> u32 {
        self.cr
    }

    /// 返回丢弃的消息的计数
    pub fn get_crd(&self) -> u32 {
        self.crd
    }

    /// 重置接收状态
    fn reset(&mut self) {
        self.b[0] = 0;
        self.m_len = 0;

        #[cfg(feature = "r2c3p-crc32")]
        {
            self.c.reset();
        }
    }

    /// 丢弃一条消息
    fn msg_end_drop(&mut self) {
        self.crd += 1;
        self.reset();
        self.s = LowRecvS::D;
    }

    /// 消息结束
    fn msg_end(&mut self) {
        // 重置转义
        self.e = Unescape::new();

        // 接收错误, 丢弃
        if LowRecvS::Err == self.s {
            self.msg_end_drop();
            return;
        }
        // 检查 `vv` 消息
        if (self.m_len == 2) && (self.b[0] == MSGT_V_R) && (self.b[1] == MSGT_V_R) {
            // 修正消息长度
            self.m_len = 1;
            // 成功接收消息
            self.cr += 1;
            self.s = LowRecvS::Ok;
            return;
        }

        // 检查 crc
        #[cfg(feature = "r2c3p-crc16")]
        {
            let use_crc32 = MSGT_V == self.b[0];
            match crc_len(self.m_len, use_crc32) {
                Some(len) => match len {
                    // crc16
                    2 => {
                        // 计算 crc16
                        let mut c = Crc16::new();
                        for i in &self.b[0..(self.m_len - 2)] {
                            c.feed(*i);
                        }
                        let cc = c.result().to_le_bytes();
                        // 接收的 crc 的值
                        let cr = &self.b[(self.m_len - 2)..self.m_len];
                        if cc != cr {
                            // crc 错误
                            self.msg_end_drop();
                            return;
                        }
                    }
                    // crc32
                    #[cfg(feature = "r2c3p-crc32")]
                    4 => {
                        // 计算的 crc 的值
                        let cc = self.c.result();
                        // 接收的 crc 的值
                        let cr = &self.b[(self.m_len - 4)..self.m_len];
                        if cc != cr {
                            // crc 错误
                            self.msg_end_drop();
                            return;
                        }
                    }
                    _ => {
                        // crc 错误
                        self.msg_end_drop();
                        return;
                    }
                },
                _ => {
                    // crc 错误
                    self.msg_end_drop();
                    return;
                }
            }
        }
        // 检查 crc 通过, 成功接收了一条消息
        self.cr += 1;
        self.s = LowRecvS::Ok;
    }

    /// 一次接收一个字节
    pub fn feed(&mut self, b: u8) {
        // 消息结束字节
        if BYTE_LF == b {
            self.msg_end();
            return;
        }
        // 错误状态, 丢弃这条消息
        if LowRecvS::Err == self.s {
            return;
        }

        // 处理转义
        let b = match self.e.feed(b) {
            Ok(b) => match b {
                Some(b) => b,
                None => {
                    // 转义模式, 等待下一个字节
                    return;
                }
            },
            Err(_) => {
                // 转义错误, 丢弃这条消息
                self.s = LowRecvS::Err;
                return;
            }
        };

        match self.s {
            LowRecvS::D | LowRecvS::Ok => {
                if LowRecvS::Ok == self.s {
                    // 重置状态
                    self.reset();
                    self.s = LowRecvS::D;
                }

                // 接收的字节放入缓冲区
                self.b[self.m_len] = b;
                self.m_len += 1;

                // 实时计算 crc32
                #[cfg(feature = "r2c3p-crc32")]
                if self.m_len > 4 {
                    self.c.feed(self.b[self.m_len - 5]);
                }
            }
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod test;
