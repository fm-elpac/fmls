//! 接收消息

use crate::r2c3p::{BYTE_LF, MSGT_V_R};

#[cfg(feature = "r2c3p-crc16")]
use crate::r2c3p::MSGT_V;

use super::Unescape;

#[cfg(feature = "r2c3p-crc16")]
use super::{crc_len, Crc16};
#[cfg(feature = "r2c3p-crc32")]
use super::{CrcT, Fifo, C32};

#[derive(Debug, Clone, PartialEq)]
enum LowRecvCS {
    /// 正在接收消息数据 (消息类型+附加数据+CRC)
    D,
    /// 成功接收
    Ok,
    /// 错误, 丢弃消息 (等待结束字符重置状态)
    Err,
}

/// 接收消息 (不处理 CRC)
///
/// `N`: 内部接收缓冲区长度
#[derive(Debug, Clone)]
pub struct LowRecvC<const N: usize> {
    // 当前状态
    s: LowRecvCS,
    // 内部接收缓冲区
    b: [u8; N],
    // 消息的总长度 (取消转义之后, 不含结束字节) (消息类型+附加数据+CRC)
    m_len: usize,
    // 用于取消转义处理
    e: Unescape,

    // 消息丢弃计数
    crd: u32,
    // 消息成功接收计数
    cr: u32,
}

impl<const N: usize> LowRecvC<N> {
    pub fn new() -> Self {
        Self {
            s: LowRecvCS::D,
            b: [0; N],
            m_len: 0,
            e: Unescape::new(),
            crd: 0,
            cr: 0,
        }
    }

    /// 获取消息类型, 成功接收状态返回 `Some()`
    pub fn get_t(&self) -> Option<u8> {
        if LowRecvCS::Ok == self.s {
            Some(self.b[0])
        } else {
            None
        }
    }

    /// 消息太长错误
    pub fn get_e2(&self) -> bool {
        self.m_len > N
    }

    pub fn get_m_len(&self) -> usize {
        self.m_len
    }

    pub fn get_b0(&self) -> u8 {
        self.b[0]
    }

    /// 获取附加数据, 成功接收状态返回 `Some()`
    pub fn get_body(&self) -> Option<&[u8]> {
        if LowRecvCS::Ok == self.s {
            // 检查太长
            if self.get_e2() {
                return None;
            }
            if self.m_len > 1 {
                return Some(&self.b[1..self.m_len]);
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

    /// 丢弃当前消息
    pub fn reset(&mut self) {
        if self.m_len > 0 {
            if LowRecvCS::Ok == self.s {
                self.cr -= 1;
            }
            self.msg_end_drop();
        } else {
            self.s = LowRecvCS::D;
        }
        self.e = Unescape::new();
    }

    /// (内部) 重置接收状态
    fn reset_i(&mut self) {
        self.b[0] = 0;
        self.m_len = 0;
    }

    /// 丢弃一条消息
    fn msg_end_drop(&mut self) {
        self.crd += 1;
        self.reset_i();
        self.s = LowRecvCS::D;
    }

    /// 消息结束
    fn msg_end(&mut self) {
        // 重置转义
        self.e = Unescape::new();
        // 接收错误, 丢弃
        if LowRecvCS::Err == self.s {
            self.msg_end_drop();
            return;
        }
        // 检查消息长度
        if self.m_len < 2 {
            self.msg_end_drop();
            return;
        }

        // 成功接收了一条消息
        self.cr += 1;
        self.s = LowRecvCS::Ok;
    }

    /// 获取 4 个字节之前的数据
    pub fn get_b4(&self) -> Option<u8> {
        if self.m_len > 4 {
            Some(self.b[self.m_len - 5])
        } else {
            None
        }
    }

    /// 一次接收一个字节
    pub fn feed(&mut self, b: u8) -> Option<u8> {
        // 消息结束字节
        if BYTE_LF == b {
            self.msg_end();
            return None;
        }
        // 错误状态, 丢弃这条消息
        if LowRecvCS::Err == self.s {
            return None;
        }

        // 处理转义
        let b = match self.e.feed(b) {
            Ok(b) => match b {
                Some(b) => b,
                None => {
                    // 转义模式, 等待下一个字节
                    return None;
                }
            },
            Err(_) => {
                // 转义错误, 丢弃这条消息
                self.s = LowRecvCS::Err;
                return None;
            }
        };

        match self.s {
            LowRecvCS::D | LowRecvCS::Ok => {
                if LowRecvCS::Ok == self.s {
                    // 重置状态
                    self.reset_i();
                    self.s = LowRecvCS::D;
                }

                // 接收的字节放入缓冲区
                if self.m_len < N {
                    self.b[self.m_len] = b;
                }
                self.m_len += 1;
            }
            _ => unreachable!(),
        }

        Some(b)
    }
}

/// 接收消息 (CRC)
///
/// `N`: 内部接收缓冲区长度, 可接收的实际消息长度需要减去 CRC
#[derive(Debug, Clone)]
pub struct LowRecv<const N: usize> {
    r: LowRecvC<N>,

    // 用于计算 crc32
    #[cfg(feature = "r2c3p-crc32")]
    c: C32,
    // 用于太长的消息计算 crc32
    #[cfg(feature = "r2c3p-crc32")]
    f4: Fifo<4>,
}

impl<const N: usize> LowRecv<N> {
    pub fn new() -> Self {
        Self {
            r: LowRecvC::new(),

            #[cfg(feature = "r2c3p-crc32")]
            c: C32::new(),
            #[cfg(feature = "r2c3p-crc32")]
            f4: Fifo::new(),
        }
    }

    /// 获取消息类型, 成功接收状态返回 `Some()`
    pub fn get_t(&self) -> Option<u8> {
        self.r.get_t()
    }

    /// 消息太长错误
    pub fn get_e2(&self) -> bool {
        self.r.get_e2()
    }

    /// 返回 crc 长度
    fn crc_l(&self) -> usize {
        let m_len = self.r.get_m_len();
        // `vv` 消息
        if m_len == 1 {
            return 0;
        }

        #[cfg(feature = "r2c3p-crc16")]
        {
            let use_crc32 = (m_len > 0) && (self.r.get_b0() == MSGT_V);
            match crc_len(m_len, use_crc32) {
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
        match self.r.get_body() {
            Some(b) => {
                if self.is_vv(b) {
                    return None;
                }
                let b_end = b.len() - self.crc_l();
                if b_end > 0 {
                    return Some(&b[..b_end]);
                }
                None
            }
            None => None,
        }
    }

    /// 返回成功接收消息的计数
    pub fn get_cr(&self) -> u32 {
        self.r.get_cr()
    }

    /// 返回丢弃的消息的计数
    pub fn get_crd(&self) -> u32 {
        self.r.get_crd()
    }

    /// 丢弃当前消息
    pub fn reset(&mut self) {
        self.reset_i();
        self.r.reset();
    }

    /// (内部) 重置接收状态
    fn reset_i(&mut self) {
        #[cfg(feature = "r2c3p-crc32")]
        {
            self.c.reset();
        }
    }

    /// 检查 `vv` 消息
    fn is_vv(&self, b: &[u8]) -> bool {
        match self.r.get_m_len() {
            2 => {
                if (b.len() == 1) && (b[0] == MSGT_V_R) {
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    #[cfg(feature = "r2c3p-crc32")]
    fn check_crc32(&mut self) {
        // 计算的 crc 的值
        let cc = self.c.result();
        // 接收的 crc 的值, 考虑太长的消息
        let cr = self.f4.get();
        if cc != cr {
            // crc 错误
            self.reset();
            return;
        }
    }

    /// 消息结束
    fn msg_end(&mut self, t: u8) {
        // 检查 crc
        #[cfg(feature = "r2c3p-crc16")]
        {
            let m_len = self.r.get_m_len();
            match self.r.get_body() {
                Some(b) => {
                    // `vv` 消息
                    if self.is_vv(b) {
                        return;
                    }

                    let use_crc32 = MSGT_V == b[0];
                    match crc_len(m_len, use_crc32) {
                        Some(len) => match len {
                            // crc16
                            2 => {
                                // 计算 crc16
                                let mut c = Crc16::new();
                                c.feed(t);
                                for i in &b[0..(m_len - 3)] {
                                    c.feed(*i);
                                }
                                let cc = c.result().to_le_bytes();
                                // 接收的 crc 的值, 处理消息太长
                                let end = if m_len <= (N - 1) { m_len } else { N };
                                let cr = &b[(end - 3)..(end - 1)];
                                if cc != cr {
                                    // crc 错误
                                    self.reset();
                                    return;
                                }
                            }
                            // crc32
                            #[cfg(feature = "r2c3p-crc32")]
                            4 => self.check_crc32(),
                            _ => {
                                // crc 错误
                                self.reset();
                                return;
                            }
                        },
                        _ => {
                            // crc 错误
                            self.reset();
                            return;
                        }
                    }
                }
                None => {
                    // 处理 e2
                    #[cfg(feature = "r2c3p-crc32")]
                    if self.r.get_e2() {
                        self.check_crc32();
                    }
                }
            }
            // 重置 crc32
            #[cfg(feature = "r2c3p-crc32")]
            {
                self.c.reset();
            }
        }
    }

    /// 一次接收一个字节
    pub fn feed(&mut self, b: u8) {
        if let Some(b) = self.r.feed(b) {
            // 实时计算 crc32
            #[cfg(feature = "r2c3p-crc32")]
            {
                // 用于太长的消息检查 crc32
                self.f4.feed(b);

                if let Some(b) = self.r.get_b4() {
                    self.c.feed(b);
                }
            }
        }
        // 检查消息接收完毕
        if let Some(t) = self.r.get_t() {
            self.msg_end(t);
        }
    }
}

#[cfg(test)]
mod test;
