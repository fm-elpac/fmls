//! 消息接收功能

use crate::r2c3p as p;

use super::hex::NU8Sender;
use super::send::{ESender, MsgSender};
use crate::r2c3p_low::{BArraySender, BStaticSender, LowRecv, MsgType};

mod eat;

#[cfg(feature = "r2c3p-c")]
mod conf;

pub use eat::Eat;

#[cfg(feature = "r2c3p-c")]
pub use conf::ConfData;

/// `fmls_r2c3p` 协议的一个连接 (UART)
#[derive(Debug, Clone)]
pub struct R2c3pPort<const N: usize> {
    // 内部接收器
    r: LowRecv<N>,

    // 接收锁定状态 (锁定状态忽略接收)
    r_lock: bool,

    /// 预定义的配置项数据
    #[cfg(feature = "r2c3p-c")]
    conf: ConfData,
}

impl<const N: usize> R2c3pPort<N> {
    pub fn new() -> Self {
        Self {
            r: LowRecv::new(),
            r_lock: false,

            #[cfg(feature = "r2c3p-c")]
            conf: ConfData::new(),
        }
    }

    /// 获取消息类型 (只在成功接收状态有效, 其余都返回 None)
    pub fn get_t(&self) -> Option<u8> {
        self.r.get_t()
    }

    /// 返回是否消息太长
    pub fn get_e2(&self) -> bool {
        self.r.get_e2()
    }

    /// 获取附加数据
    pub fn get_body(&self) -> Option<&[u8]> {
        self.r.get_body()
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
            //self.conf.tc.t = self.conf.tc.t.wrapping_add(1);
        }
    }

    /// 更新传输质量计数器: 发送了一个字节
    pub fn c_tb(&mut self) {
        #[cfg(feature = "r2c3p-cc")]
        {
            //self.conf.tc.tb = self.conf.tc.tb.wrapping_add(1);
        }
    }

    /// 一次接收一个原始字节
    pub fn feed(&mut self, u: u8) {
        // 接收锁定
        if self.r_lock {
            return;
        }

        self.r.feed(u);
    }

    /// 对消息的默认处理
    ///
    /// 当应用代码不关心 (不处理) 一条接收的消息的时候,
    /// 应该调用此方法来进行默认处理.
    ///
    /// 如果应用已经自己处理了接收的消息, 不应该调用此方法.
    ///
    /// 返回需要发送的响应消息 (发送器)
    pub fn eat(&mut self) -> Option<Eat> {
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
        if self.get_e2() {
            // 对于过长的请求消息, 返回 `E-2` 错误
            if req {
                // 如果缓冲区长度小于 255 字节, 报告长度
                let m = if N < 0xff {
                    Some(NU8Sender::new(N as u8))
                } else {
                    None
                };

                return Some(Eat::E2(MsgSender::new(
                    p::MSGT_E,
                    ESender::new(BStaticSender::new(p::EB_2), m),
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
                    match self.r.get_body() {
                        Some(b) => {
                            return self.conf.eat_c(b);
                        }
                        None => {
                            // `E-4` 错误
                            return Some(Eat::E(MsgSender::new(
                                p::MSGT_E,
                                ESender::new(BStaticSender::new(p::EB_4), None),
                            )));
                        }
                    }
                }
                // 未知消息类型, 返回 `E-3` 错误
                _ => {
                    return Some(Eat::E3(MsgSender::new(
                        p::MSGT_E,
                        ESender::new(BStaticSender::new(p::EB_3), Some(BArraySender::new([t]))),
                    )));
                }
            }
        }
        // 如果不是请求消息, 直接丢弃
        None
    }
}

/// `R2c3pPort*` 的统一接口
pub trait R2c3pPortT<const N: usize> {
    /// 返回内部包装的 `R2c3pPort`
    fn get_p(&self) -> &R2c3pPort<N>;
    fn get_p_mut(&mut self) -> &mut R2c3pPort<N>;

    /// 获取消息类型
    ///
    /// 只有在成功接收消息的状态才会返回 `Some()`,
    /// 其余都返回 `None`
    fn get_t(&self) -> Option<u8> {
        self.get_p().get_t()
    }

    /// 消息太长 (E-2) 错误标志
    fn get_e2(&self) -> bool {
        self.get_p().get_e2()
    }
    /// 获取附加数据
    fn get_body(&self) -> Option<&[u8]> {
        self.get_p().get_body()
    }
    /// 设置接收锁定状态
    fn lock(&mut self, r_lock: bool) {
        self.get_p_mut().lock(r_lock);
    }
    /// 返回接收锁定状态
    fn get_lock(&self) -> bool {
        self.get_p().get_lock()
    }

    /// 更新传输质量计数器: 发送了一条消息
    fn c_t(&mut self) {
        self.get_p_mut().c_t();
    }

    /// 更新传输质量计数器: 发送了一个字节
    fn c_tb(&mut self) {
        self.get_p_mut().c_tb();
    }

    /// 一次接收一个字节
    fn feed(&mut self, u: u8) {
        self.get_p_mut().feed(u);
    }

    /// 对消息的默认处理
    ///
    /// 当应用代码不关心 (不处理) 一条接收的消息的时候,
    /// 应该调用此方法来进行默认处理.
    ///
    /// 如果应用已经自己处理了接收的消息, 不应该调用此方法.
    ///
    /// 返回需要发送的响应消息 (发送器)
    fn eat(&mut self) -> Option<Eat> {
        self.get_p_mut().eat()
    }
}

/// 含有 8 字节 (协议允许的最小值) 接收缓冲区
///
/// 能接收长度不超过 8 字节 (不含 CRC, 转义) 的消息
pub struct R2c3pPort8 {
    p: R2c3pPort<{ 8 + 2 }>,
}

impl R2c3pPort8 {
    pub fn new() -> Self {
        Self {
            p: R2c3pPort::new(),
        }
    }
}

impl R2c3pPortT<10> for R2c3pPort8 {
    fn get_p(&self) -> &R2c3pPort<10> {
        &self.p
    }

    fn get_p_mut(&mut self) -> &mut R2c3pPort<10> {
        &mut self.p
    }
}

/// 含有 32 字节 (使用 crc16) 接收缓冲区
pub struct R2c3pPort32 {
    p: R2c3pPort<{ 32 + 2 }>,
}

impl R2c3pPort32 {
    pub fn new() -> Self {
        Self {
            p: R2c3pPort::new(),
        }
    }
}

impl R2c3pPortT<34> for R2c3pPort32 {
    fn get_p(&self) -> &R2c3pPort<34> {
        &self.p
    }

    fn get_p_mut(&mut self) -> &mut R2c3pPort<34> {
        &mut self.p
    }
}

/// 含有 64 字节 (MCU 推荐值) 接收缓冲区
pub struct R2c3pPort64 {
    p: R2c3pPort<{ 64 + 4 }>,
}

impl R2c3pPort64 {
    pub fn new() -> Self {
        Self {
            p: R2c3pPort::new(),
        }
    }
}

impl R2c3pPortT<68> for R2c3pPort64 {
    fn get_p(&self) -> &R2c3pPort<68> {
        &self.p
    }

    fn get_p_mut(&mut self) -> &mut R2c3pPort<68> {
        &mut self.p
    }
}

/// 含有 128 字节 (UART 允许的最大值) 接收缓冲区
pub struct R2c3pPort128 {
    p: R2c3pPort<{ 128 + 4 }>,
}

impl R2c3pPort128 {
    pub fn new() -> Self {
        Self {
            p: R2c3pPort::new(),
        }
    }
}

impl R2c3pPortT<132> for R2c3pPort128 {
    fn get_p(&self) -> &R2c3pPort<132> {
        &self.p
    }

    fn get_p_mut(&mut self) -> &mut R2c3pPort<132> {
        &mut self.p
    }
}

#[cfg(test)]
mod test;
