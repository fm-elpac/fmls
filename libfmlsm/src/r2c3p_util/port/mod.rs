//! 消息接收功能

use core::iter::Iterator;

use libfmlsc::r2c3p::BYTE_LF;

use super::escape_crc::Unescape;

#[cfg(feature = "r2c3p-crc16")]
use super::escape_crc::Crc16;
#[cfg(feature = "r2c3p-crc32")]
use super::escape_crc::{crc_len, Crc32};
#[cfg(feature = "r2c3p-crc16")]
use super::hex::{Fifo2, Fifo4};

mod conf;

pub use conf::ConfData;

// R2c3pPort 的内部状态 (接收状态)
#[derive(PartialEq)]
enum R2c3pPortS {
    /// 等待接收消息类型 (初始状态)
    T,
    /// 正在接收消息附加数据
    Data,
    /// 消息接收完毕 (成功接收)
    Ok,
    /// 错误, 丢弃消息 (等待结束字符来重置接收状态)
    Err,
}

/// `fmls_r2c3p` 协议的一个连接 (UART)
///
/// 通用功能 (不含内部接收缓冲区)
pub struct R2c3pPort<'a> {
    // 接收缓冲区的长度
    b_len: usize,

    // 接收状态
    s: R2c3pPortS,
    // 消息类型
    t: Option<u8>,
    // 消息附加数据长度
    m_len: usize,
    // 消息太长 (错误标志)
    e_2: bool,

    // 接收锁定状态 (锁定状态忽略接收)
    r_lock: bool,

    // 转义处理
    e: Unescape,
    // 缓存 crc 字节
    #[cfg(feature = "r2c3p-crc16")]
    f2: Fifo2,
    #[cfg(feature = "r2c3p-crc32")]
    f4: Fifo4,
    // 计算 crc
    #[cfg(feature = "r2c3p-crc16")]
    c16: Option<Crc16<'a>>,
    #[cfg(feature = "r2c3p-crc32")]
    c32: Option<Crc32<'a>>,
    // fix compile error for <'a>
    #[cfg(not(feature = "r2c3p-crc16"))]
    _a: &'a [u8],

    /// 预定义的配置项数据
    #[cfg(feature = "r2c3p-c")]
    conf: ConfData,
}

impl<'a> R2c3pPort<'a> {
    /// `b_len`: 接收缓冲区的长度
    pub const fn new(b_len: usize) -> Self {
        Self {
            b_len,
            s: R2c3pPortS::T,
            t: None,
            m_len: 0,
            e_2: false,
            r_lock: false,
            e: Unescape::new(),
            #[cfg(feature = "r2c3p-crc16")]
            f2: Fifo2::new(),
            #[cfg(feature = "r2c3p-crc32")]
            f4: Fifo4::new(),
            #[cfg(feature = "r2c3p-crc16")]
            c16: Some(Crc16::new()),
            #[cfg(feature = "r2c3p-crc32")]
            c32: Some(Crc32::new()),
            #[cfg(not(feature = "r2c3p-crc16"))]
            _a: b"",

            #[cfg(feature = "r2c3p-c")]
            conf: ConfData::new(),
        }
    }

    /// 获取消息类型 (只在成功接收状态有效, 其余都返回 None)
    pub fn get_t(&self) -> Option<u8> {
        if self.s == R2c3pPortS::Ok {
            self.t.clone()
        } else {
            None
        }
    }

    /// 获取消息附加数据长度 (当前接收长度)
    pub fn get_m_len(&self) -> usize {
        self.m_len
    }

    /// 返回是否消息太长
    pub fn get_e_2(&self) -> bool {
        self.e_2
    }

    /// 设置接收锁定状态
    pub fn lock(&mut self, r_lock: bool) {
        self.r_lock = r_lock;
    }

    /// 返回接收锁定状态
    pub fn get_lock(&self) -> bool {
        self.r_lock
    }

    /// 一次接收一个原始字节, 返回的数据需要放入缓冲区
    ///
    /// 处理转义, crc 计算等
    pub fn feed(&mut self, u: u8) -> Option<u8> {
        // TODO
        None
    }

    /// 重置接收状态
    fn reset(&mut self) {
        self.t = None;
        self.m_len = 0;
        self.e_2 = false;
        self.e = Unescape::new();
        #[cfg(feature = "r2c3p-crc16")]
        {
            self.f2 = Fifo2::new();
            self.c16 = Some(Crc16::new());
        }
        #[cfg(feature = "r2c3p-crc32")]
        {
            self.f4 = Fifo4::new();
            self.c32 = Some(Crc32::new());
        }
    }
}

/// `R2c3pPort*` 的统一接口
pub trait R2c3pPortT<'a> {
    /// 返回内部包装的 `R2c3pPort`
    fn get_p(&self) -> &R2c3pPort;
    fn get_p_mut(&mut self) -> &mut R2c3pPort<'a>;

    /// 获取消息类型
    ///
    /// 只有在成功接收消息的状态才会返回 `Some()`,
    /// 其余都返回 `None`
    fn get_t(&self) -> Option<u8> {
        self.get_p().get_t()
    }

    /// 获取消息附加数据的长度
    ///
    /// 只有在成功接收消息的状态才会返回 `Some()`,
    /// 其余都返回 `None`
    fn get_m_len(&self) -> Option<usize> {
        match self.get_t() {
            Some(_) => Some(self.get_p().get_m_len()),
            None => None,
        }
    }
    /// 消息太长 (E-2) 错误标志
    fn get_e_2(&self) -> bool {
        self.get_p().get_e_2()
    }
    /// 设置接收锁定状态
    fn lock(&mut self, r_lock: bool) {
        self.get_p_mut().lock(r_lock);
    }
    /// 返回接收锁定状态
    fn get_lock(&self) -> bool {
        self.get_p().get_lock()
    }
    /// 读取消息附加数据的一个字节
    fn get_m_b(&self, i: usize) -> u8;

    /// 一次接收一个字节
    fn feed(&mut self, u: u8);

    /// 读取全部消息附加数据
    fn get_body(&'a self) -> &'a [u8];
}

/// 含有 8 字节 (协议允许的最小值) 接收缓冲区
///
/// 能接收长度不超过 8 字节 (不含 CRC, 转义) 的消息
pub struct R2c3pPort8<'a> {
    p: R2c3pPort<'a>,
    // 内部缓冲区
    b: [u8; 8 + 2],
}

impl<'a> R2c3pPort8<'a> {
    pub const fn new() -> Self {
        const B_LEN: usize = 8;
        Self {
            p: R2c3pPort::new(B_LEN),
            b: [0; B_LEN + 2],
        }
    }
}

impl<'a> R2c3pPortT<'a> for R2c3pPort8<'a> {
    fn get_p(&self) -> &R2c3pPort {
        &self.p
    }

    fn get_p_mut(&mut self) -> &mut R2c3pPort<'a> {
        &mut self.p
    }

    fn get_m_b(&self, i: usize) -> u8 {
        self.b[i]
    }

    fn feed(&mut self, u: u8) {
        if let Some(b) = self.p.feed(u) {
            // 存储数据到缓冲区
            self.b[self.p.get_m_len()] = b;
        }
    }

    fn get_body(&'a self) -> &'a [u8] {
        match self.get_m_len() {
            Some(len) => &self.b[0..len],
            // empty
            None => &self.b[0..0],
        }
    }
}

/// 含有 32 字节 (使用 crc16) 接收缓冲区
pub struct R2c3pPort32<'a> {
    p: R2c3pPort<'a>,
    b: [u8; 32 + 2],
}

impl<'a> R2c3pPort32<'a> {
    pub const fn new() -> Self {
        const B_LEN: usize = 32;
        Self {
            p: R2c3pPort::new(B_LEN),
            b: [0; B_LEN + 2],
        }
    }
}

impl<'a> R2c3pPortT<'a> for R2c3pPort32<'a> {
    fn get_p(&self) -> &R2c3pPort {
        &self.p
    }

    fn get_p_mut(&mut self) -> &mut R2c3pPort<'a> {
        &mut self.p
    }

    fn get_m_b(&self, i: usize) -> u8 {
        self.b[i]
    }

    fn feed(&mut self, u: u8) {
        if let Some(b) = self.p.feed(u) {
            self.b[self.p.get_m_len()] = b;
        }
    }

    fn get_body(&'a self) -> &'a [u8] {
        match self.get_m_len() {
            Some(len) => &self.b[0..len],
            // empty
            None => &self.b[0..0],
        }
    }
}

/// 含有 64 字节 (MCU 推荐值) 接收缓冲区
pub struct R2c3pPort64<'a> {
    p: R2c3pPort<'a>,
    b: [u8; 64 + 2],
}

impl<'a> R2c3pPort64<'a> {
    pub const fn new() -> Self {
        const B_LEN: usize = 64;
        Self {
            p: R2c3pPort::new(B_LEN),
            b: [0; B_LEN + 2],
        }
    }
}

impl<'a> R2c3pPortT<'a> for R2c3pPort64<'a> {
    fn get_p(&self) -> &R2c3pPort {
        &self.p
    }

    fn get_p_mut(&mut self) -> &mut R2c3pPort<'a> {
        &mut self.p
    }

    fn get_m_b(&self, i: usize) -> u8 {
        self.b[i]
    }

    fn feed(&mut self, u: u8) {
        if let Some(b) = self.p.feed(u) {
            self.b[self.p.get_m_len()] = b;
        }
    }

    fn get_body(&'a self) -> &'a [u8] {
        match self.get_m_len() {
            Some(len) => &self.b[0..len],
            // empty
            None => &self.b[0..0],
        }
    }
}

/// 含有 128 字节 (UART 允许的最大值) 接收缓冲区
pub struct R2c3pPort128<'a> {
    p: R2c3pPort<'a>,
    b: [u8; 128 + 2],
}

impl<'a> R2c3pPort128<'a> {
    pub const fn new() -> Self {
        const B_LEN: usize = 128;
        Self {
            p: R2c3pPort::new(B_LEN),
            b: [0; B_LEN + 2],
        }
    }
}

impl<'a> R2c3pPortT<'a> for R2c3pPort128<'a> {
    fn get_p(&self) -> &R2c3pPort {
        &self.p
    }

    fn get_p_mut(&mut self) -> &mut R2c3pPort<'a> {
        &mut self.p
    }

    fn get_m_b(&self, i: usize) -> u8 {
        self.b[i]
    }

    fn feed(&mut self, u: u8) {
        if let Some(b) = self.p.feed(u) {
            self.b[self.p.get_m_len()] = b;
        }
    }

    fn get_body(&'a self) -> &'a [u8] {
        match self.get_m_len() {
            Some(len) => &self.b[0..len],
            // empty
            None => &self.b[0..0],
        }
    }
}

// TODO

#[cfg(test)]
mod test;
