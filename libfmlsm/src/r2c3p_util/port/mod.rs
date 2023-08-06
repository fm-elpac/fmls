//! 消息接收功能

use libfmlsc::r2c3p as p;

use super::hex::{NU8Sender, U8Sender, VecSender};
use super::send::{ESender, MsgSender};
use super::MsgType;
use super::Unescape;

#[cfg(feature = "r2c3p-crc16")]
use super::hex::{Fifo2, Fifo4};
#[cfg(feature = "r2c3p-crc32")]
use super::Crc32;
#[cfg(feature = "r2c3p-crc16")]
use super::{crc_len, Crc16};

mod eat;

#[cfg(feature = "r2c3p-c")]
mod conf;

pub use eat::Eat;

#[cfg(feature = "r2c3p-c")]
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
pub struct R2c3pPort {
    // 接收缓冲区的长度
    b_len: usize,

    // 接收状态
    s: R2c3pPortS,
    // 消息类型
    t: Option<u8>,
    // 原始接收的一条消息的总长度 (含消息类型+crc)
    r_len: usize,
    // 消息附加数据长度 (可能经过 crc 缓冲)
    m_len: usize,
    // 消息太长 (错误标志)
    e_2: bool,

    // 专用于接收 `vv` 消息
    vv: bool,

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
    c16: Option<Crc16>,
    #[cfg(feature = "r2c3p-crc32")]
    c32: Option<Crc32>,

    /// 预定义的配置项数据
    #[cfg(feature = "r2c3p-c")]
    conf: ConfData,
}

impl R2c3pPort {
    /// `b_len`: 接收缓冲区的长度
    pub const fn new(b_len: usize) -> Self {
        Self {
            b_len,
            s: R2c3pPortS::T,
            t: None,
            r_len: 0,
            m_len: 0,
            e_2: false,
            vv: false,
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

    /// 返回预定义的配置数据的当前值
    #[cfg(feature = "r2c3p-c")]
    pub fn get_conf(&self) -> ConfData {
        self.conf.clone()
    }

    /// 更新传输质量计数器: 发送了一条消息
    pub fn c_t(&mut self) {
        #[cfg(feature = "r2c3p-cc")]
        {
            self.conf.tc.t = self.conf.tc.t.wrapping_add(1);
        }
    }

    /// 更新传输质量计数器: 发送了一个字节
    pub fn c_tb(&mut self) {
        #[cfg(feature = "r2c3p-cc")]
        {
            self.conf.tc.tb = self.conf.tc.tb.wrapping_add(1);
        }
    }

    /// 更新传输质量计数器: 成功接收了一条消息
    fn c_r(&mut self) {
        #[cfg(feature = "r2c3p-cc")]
        {
            self.conf.tc.r = self.conf.tc.r.wrapping_add(1);
        }
    }

    /// 更新传输质量计数器: 丢弃了一条消息
    fn c_rd(&mut self) {
        #[cfg(feature = "r2c3p-cc")]
        {
            self.conf.tc.rd = self.conf.tc.rd.wrapping_add(1);
        }
    }

    /// 更新传输质量计数器: 成功接收了字节
    fn c_rb(&mut self, n: u32) {
        #[cfg(feature = "r2c3p-cc")]
        {
            self.conf.tc.rb = self.conf.tc.rb.wrapping_add(n);
        }
    }

    /// 一次接收一个原始字节, 返回的数据需要放入缓冲区
    ///
    /// 处理转义, crc 计算等
    pub fn feed(&mut self, u: u8) -> Option<u8> {
        // 消息结束标志
        if p::BYTE_LF == u {
            self.msg_end();
            return None;
        }
        // 错误状态, 丢弃这条消息
        if R2c3pPortS::Err == self.s {
            return None;
        }

        // 转义处理 (取消转义)
        let b = match self.e.feed(u) {
            Ok(b) => match b {
                Some(b) => b,
                None => {
                    // 进入转义模式, 等待接收下一个字节
                    return None;
                }
            },
            Err(_) => {
                // 转义错误, 丢弃这条消息
                self.s = R2c3pPortS::Err;
                return None;
            }
        };
        // 接收锁定
        if self.r_lock {
            return None;
        }

        match self.s {
            R2c3pPortS::T | R2c3pPortS::Ok => {
                if R2c3pPortS::Ok == self.s {
                    self.reset();
                }

                // 保存消息类型
                self.t = Some(b);
                // 等待接收附加消息数据
                self.s = R2c3pPortS::Data;

                self.crc_feed(b);
                self.r_len += 1;
                // 消息类型字节不放入缓冲区
                None
            }
            R2c3pPortS::Data => {
                // 处理 `vv` 消息
                if (Some(p::MSGT_V_R) == self.t) && (1 == self.r_len) && (b'v' == b) {
                    self.vv = true;
                } else {
                    self.vv = false;
                }

                let o = self.crc_feed(b);
                self.r_len += 1;
                // 忽略吐出的第一个字节 (消息类型)
                if o.is_some() && (self.r_len > 3) {
                    self.m_len += 1;
                }

                // 检查消息长度
                if self.m_len > (self.b_len + 2) {
                    // 错误: 消息太长
                    self.e_2 = true;
                    return None;
                }
                // 这个字节应该放入缓冲区
                if self.r_len > 3 {
                    o
                } else {
                    None
                }
            }
            R2c3pPortS::Err => {
                // unreachable!()
                None
            }
        }
    }

    // 喂入接收的一个字节数据
    fn crc_feed(&mut self, b: u8) -> Option<u8> {
        #[cfg(feature = "r2c3p-crc16")]
        {
            #[cfg(feature = "r2c3p-crc32")]
            {
                // 缓冲 crc 字节 (最后 4 个字节不计算)
                if let Some(b) = self.f4.feed(b) {
                    if let Some(c) = &mut self.c32 {
                        // 计算 crc32
                        c.feed(b);
                    }
                }
            }

            // 最后 2 个字节不计算
            let b16 = self.f2.feed(b);
            if let Some(b) = &b16 {
                if let Some(c) = &mut self.c16 {
                    // 计算 crc16
                    c.feed(*b);
                }
            }
            // 返回的数据放入缓冲区
            b16
        }
        #[cfg(not(feature = "r2c3p-crc16"))]
        {
            Some(b)
        }
    }

    // 消息接收结束时, 因错误而丢弃一条消息
    fn msg_end_drop(&mut self) {
        self.reset();
        self.s = R2c3pPortS::T;
        self.c_rd();
    }

    // 消息接收完成
    fn msg_end(&mut self) {
        // 重置转义状态
        self.e = Unescape::new();
        // 接收错误, 丢弃这条消息的处理
        if self.s == R2c3pPortS::Err {
            self.msg_end_drop();
            return;
        }
        // 处理 `vv` 消息
        if self.vv {
            self.s = R2c3pPortS::Ok;
            self.m_len = 0;
            self.vv = false;
            // 成功接收了一条消息
            self.c_r();
            return;
        }

        // 检查 crc
        #[cfg(feature = "r2c3p-crc16")]
        {
            let use_crc32 = Some(p::MSGT_V) == self.t;
            match crc_len(self.r_len, use_crc32) {
                Some(len) => match len {
                    // crc16
                    2 => {
                        // 获取接收的 crc16 值
                        match self.f2.to_u16() {
                            Some(c16) => {
                                if self.c16.take().unwrap().result() != c16 {
                                    // 错误, 丢弃
                                    self.msg_end_drop();
                                    return;
                                }
                            }
                            None => {
                                // 错误, 丢弃
                                self.msg_end_drop();
                                return;
                            }
                        }
                    }
                    // crc32
                    #[cfg(feature = "r2c3p-crc32")]
                    4 => {
                        // 获取接收的 crc32 值
                        match self.f4.to_u32() {
                            Some(c32) => {
                                if self.c32.take().unwrap().result() != c32 {
                                    // 错误, 丢弃
                                    self.msg_end_drop();
                                    return;
                                } else {
                                    // 修复接收的消息的长度 (len(crc32) - len(crc16))
                                    self.m_len -= 2;
                                }
                            }
                            None => {
                                // 错误, 丢弃
                                self.msg_end_drop();
                                return;
                            }
                        }
                    }
                    _ => {
                        // 错误, 丢弃
                        self.msg_end_drop();
                        return;
                    }
                },
                None => {
                    // 错误, 丢弃
                    self.msg_end_drop();
                    return;
                }
            }
        }

        // 检查 crc 通过
        self.s = R2c3pPortS::Ok;
        // 检查消息太长
        if self.m_len > self.b_len {
            self.e_2 = true;
        }
        // 成功接收了一条消息
        self.c_r();
        self.c_rb(self.m_len as u32);
    }

    /// 重置接收状态
    fn reset(&mut self) {
        self.t = None;
        self.r_len = 0;
        self.m_len = 0;
        self.e_2 = false;
        self.vv = false;
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

    /// 对消息的默认处理
    ///
    /// 当应用代码不关心 (不处理) 一条接收的消息的时候,
    /// 应该调用此方法来进行默认处理.
    ///
    /// 如果应用已经自己处理了接收的消息, 不应该调用此方法.
    ///
    /// 返回需要发送的响应消息 (发送器)
    pub fn eat(&mut self, body: &[u8]) -> Option<Eat> {
        // 检查是否成功接收消息
        let t = match self.get_t() {
            Some(t) => t,
            None => {
                // 忽略处理
                return None;
            }
        };
        // 解除接收锁定
        self.lock(false);

        // 检查是否为请求消息
        let req = MsgType::from(t) == MsgType::Req;
        // 检查消息太长
        if self.get_e_2() {
            // 对于过长的请求消息, 返回 `E-2` 错误
            if req {
                // 如果缓冲区长度小于 255 字节, 报告长度
                let m = if self.b_len < 0xff {
                    Some(NU8Sender::new(self.b_len as u8))
                } else {
                    None
                };

                return Some(Eat::E2(MsgSender::new(
                    p::MSGT_E,
                    ESender::new(VecSender::new(p::EB_2), m),
                )));
            }
            // 如果不是请求消息, 直接丢弃
            return None;
        }

        if req {
            // 检查消息类型
            match t {
                // 处理 `c` 消息
                #[cfg(feature = "r2c3p-c")]
                p::MSGT_C_R => {
                    return self.conf.eat_c(body);
                }
                // 未知消息类型, 返回 `E-3` 错误
                _ => {
                    return Some(Eat::E3(MsgSender::new(
                        p::MSGT_E,
                        ESender::new(VecSender::new(p::EB_3), Some(U8Sender::new(t))),
                    )));
                }
            }
        }
        // 如果不是请求消息, 直接丢弃
        None
    }
}

/// `R2c3pPort*` 的统一接口
pub trait R2c3pPortT {
    /// 返回内部包装的 `R2c3pPort`
    fn get_p(&self) -> &R2c3pPort;
    fn get_p_mut(&mut self) -> &mut R2c3pPort;

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

    /// 更新传输质量计数器: 发送了一条消息
    fn c_t(&mut self) {
        self.get_p_mut().c_t();
    }

    /// 更新传输质量计数器: 发送了一个字节
    fn c_tb(&mut self) {
        self.get_p_mut().c_tb();
    }

    /// 一次接收一个字节
    fn feed(&mut self, u: u8);

    /// 读取全部消息附加数据
    fn get_body(&self) -> &[u8];

    /// 对消息的默认处理
    ///
    /// 当应用代码不关心 (不处理) 一条接收的消息的时候,
    /// 应该调用此方法来进行默认处理.
    ///
    /// 如果应用已经自己处理了接收的消息, 不应该调用此方法.
    ///
    /// 返回需要发送的响应消息 (发送器)
    fn eat(&mut self) -> Option<Eat>;
}

/// 含有 8 字节 (协议允许的最小值) 接收缓冲区
///
/// 能接收长度不超过 8 字节 (不含 CRC, 转义) 的消息
pub struct R2c3pPort8 {
    p: R2c3pPort,
    // 内部缓冲区
    b: [u8; 8 + 2],
}

impl R2c3pPort8 {
    pub const fn new() -> Self {
        const B_LEN: usize = 8;
        Self {
            p: R2c3pPort::new(B_LEN),
            b: [0; B_LEN + 2],
        }
    }
}

impl R2c3pPortT for R2c3pPort8 {
    fn get_p(&self) -> &R2c3pPort {
        &self.p
    }

    fn get_p_mut(&mut self) -> &mut R2c3pPort {
        &mut self.p
    }

    fn get_m_b(&self, i: usize) -> u8 {
        self.b[i]
    }

    fn feed(&mut self, u: u8) {
        if let Some(b) = self.p.feed(u) {
            // 存储数据到缓冲区
            self.b[self.p.get_m_len() - 1] = b;
        }
    }

    fn get_body(&self) -> &[u8] {
        match self.get_m_len() {
            Some(len) => &self.b[0..len],
            // empty
            None => &self.b[0..0],
        }
    }

    fn eat(&mut self) -> Option<Eat> {
        let b = match self.get_m_len() {
            Some(len) => &self.b[0..len],
            None => &self.b[0..0],
        };
        self.p.eat(b)
    }
}

/// 含有 32 字节 (使用 crc16) 接收缓冲区
pub struct R2c3pPort32 {
    p: R2c3pPort,
    b: [u8; 32 + 2],
}

impl R2c3pPort32 {
    pub const fn new() -> Self {
        const B_LEN: usize = 32;
        Self {
            p: R2c3pPort::new(B_LEN),
            b: [0; B_LEN + 2],
        }
    }
}

impl R2c3pPortT for R2c3pPort32 {
    fn get_p(&self) -> &R2c3pPort {
        &self.p
    }

    fn get_p_mut(&mut self) -> &mut R2c3pPort {
        &mut self.p
    }

    fn get_m_b(&self, i: usize) -> u8 {
        self.b[i]
    }

    fn feed(&mut self, u: u8) {
        if let Some(b) = self.p.feed(u) {
            self.b[self.p.get_m_len() - 1] = b;
        }
    }

    fn get_body(&self) -> &[u8] {
        match self.get_m_len() {
            Some(len) => &self.b[0..len],
            // empty
            None => &self.b[0..0],
        }
    }

    fn eat(&mut self) -> Option<Eat> {
        let b = match self.get_m_len() {
            Some(len) => &self.b[0..len],
            None => &self.b[0..0],
        };
        self.p.eat(b)
    }
}

/// 含有 64 字节 (MCU 推荐值) 接收缓冲区
pub struct R2c3pPort64 {
    p: R2c3pPort,
    b: [u8; 64 + 2],
}

impl R2c3pPort64 {
    pub const fn new() -> Self {
        const B_LEN: usize = 64;
        Self {
            p: R2c3pPort::new(B_LEN),
            b: [0; B_LEN + 2],
        }
    }
}

impl R2c3pPortT for R2c3pPort64 {
    fn get_p(&self) -> &R2c3pPort {
        &self.p
    }

    fn get_p_mut(&mut self) -> &mut R2c3pPort {
        &mut self.p
    }

    fn get_m_b(&self, i: usize) -> u8 {
        self.b[i]
    }

    fn feed(&mut self, u: u8) {
        if let Some(b) = self.p.feed(u) {
            self.b[self.p.get_m_len() - 1] = b;
        }
    }

    fn get_body(&self) -> &[u8] {
        match self.get_m_len() {
            Some(len) => &self.b[0..len],
            // empty
            None => &self.b[0..0],
        }
    }

    fn eat(&mut self) -> Option<Eat> {
        let b = match self.get_m_len() {
            Some(len) => &self.b[0..len],
            None => &self.b[0..0],
        };
        self.p.eat(b)
    }
}

/// 含有 128 字节 (UART 允许的最大值) 接收缓冲区
pub struct R2c3pPort128 {
    p: R2c3pPort,
    b: [u8; 128 + 2],
}

impl R2c3pPort128 {
    pub const fn new() -> Self {
        const B_LEN: usize = 128;
        Self {
            p: R2c3pPort::new(B_LEN),
            b: [0; B_LEN + 2],
        }
    }
}

impl R2c3pPortT for R2c3pPort128 {
    fn get_p(&self) -> &R2c3pPort {
        &self.p
    }

    fn get_p_mut(&mut self) -> &mut R2c3pPort {
        &mut self.p
    }

    fn get_m_b(&self, i: usize) -> u8 {
        self.b[i]
    }

    fn feed(&mut self, u: u8) {
        if let Some(b) = self.p.feed(u) {
            self.b[self.p.get_m_len() - 1] = b;
        }
    }

    fn get_body(&self) -> &[u8] {
        match self.get_m_len() {
            Some(len) => &self.b[0..len],
            // empty
            None => &self.b[0..0],
        }
    }

    fn eat(&mut self) -> Option<Eat> {
        let b = match self.get_m_len() {
            Some(len) => &self.b[0..len],
            None => &self.b[0..0],
        };
        self.p.eat(b)
    }
}

#[cfg(test)]
mod test;
