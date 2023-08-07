//! 消息默认处理

use core::iter::Iterator;

use crate::r2c3p as p;

use super::{
    send_e2_len, send_e3, send_msg_16, BArraySender, BStaticSender, HexArraySender, LowCSender,
    LowRecv, LowSend, MsgType, C16,
};

#[cfg(feature = "r2c3p-i")]
use super::hex_u64;
#[cfg(feature = "r2c3p-o")]
use super::hex_u8;
#[cfg(feature = "r2c3p-c")]
use super::{read_conf, read_conf_k, send_e4, send_e5, ConfK};

/// 消息默认处理可能返回的需要发送的消息
///
/// `N`: 缓冲区长度的数组长度 (用于发送消息太长错误).
/// 详见 `send_e2_len`
#[derive(Debug, Clone)]
pub enum LowEat<const N: usize> {
    /// 比如 `E-2 32`
    E2(LowSend<BArraySender<N>, C16, 2>),
    /// 比如 `E-3 c`
    E3(LowSend<BArraySender<4>, C16, 2>),
    /// 比如 `E-4`, `E-5`
    E(LowSend<BStaticSender, C16, 2>),
    /// 比如 `CcT=`
    #[cfg(feature = "r2c3p-cc")]
    CHex4(LowSend<LowCSender<HexArraySender<4>>, C16, 2>),
    /// 比如 `CI=`
    #[cfg(feature = "r2c3p-i")]
    CHex8(LowSend<LowCSender<HexArraySender<8>>, C16, 2>),
    /// 比如 `CO=`
    #[cfg(feature = "r2c3p-o")]
    CHex1(LowSend<LowCSender<HexArraySender<1>>, C16, 2>),
}

impl<const N: usize> Iterator for LowEat<N> {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        match self {
            LowEat::E2(s) => s.next(),
            LowEat::E3(s) => s.next(),
            LowEat::E(s) => s.next(),
            #[cfg(feature = "r2c3p-cc")]
            LowEat::CHex4(s) => s.next(),
            #[cfg(feature = "r2c3p-i")]
            LowEat::CHex8(s) => s.next(),
            #[cfg(feature = "r2c3p-o")]
            LowEat::CHex1(s) => s.next(),
        }
    }
}

/// 发送 `C` 消息 (v: `hex(u8)`)
pub fn send_c_u8(k: &'static [u8], v: u8) -> LowSend<LowCSender<HexArraySender<1>>, C16, 2> {
    let b: [u8; 1] = [v];
    send_msg_16(p::MSGT_C, LowCSender::new(k, Some(HexArraySender::new(b))))
}

/// 发送 `C` 消息 (v: `hex(u16)`)
pub fn send_c_u16(k: &'static [u8], v: u16) -> LowSend<LowCSender<HexArraySender<2>>, C16, 2> {
    let b: [u8; 2] = u16::to_be_bytes(v);
    send_msg_16(p::MSGT_C, LowCSender::new(k, Some(HexArraySender::new(b))))
}

/// 发送 `C` 消息 (v: `hex(u32)`)
pub fn send_c_u32(k: &'static [u8], v: u32) -> LowSend<LowCSender<HexArraySender<4>>, C16, 2> {
    let b: [u8; 4] = u32::to_be_bytes(v);
    send_msg_16(p::MSGT_C, LowCSender::new(k, Some(HexArraySender::new(b))))
}

/// 发送 `C` 消息 (v: `hex(u64)`)
pub fn send_c_u64(k: &'static [u8], v: u64) -> LowSend<LowCSender<HexArraySender<8>>, C16, 2> {
    let b: [u8; 8] = u64::to_be_bytes(v);
    send_msg_16(p::MSGT_C, LowCSender::new(k, Some(HexArraySender::new(b))))
}

/// 消息默认处理
///
/// `len`: 缓冲区长度 (用于发送消息太长错误)
#[derive(Debug, Clone)]
pub struct Eat<const M: usize> {
    /// 缓冲区长度数组
    len: [u8; M],

    // 处理 `cI` 消息
    #[cfg(feature = "r2c3p-i")]
    c_i: u64,
    // 处理 `cO`, `cOn` 消息
    #[cfg(feature = "r2c3p-o")]
    pub c_o: u8,
    #[cfg(feature = "r2c3p-o")]
    pub c_on: u8,
}

impl<const M: usize> Eat<M> {
    pub const fn new(len: [u8; M]) -> Self {
        Self {
            len,
            #[cfg(feature = "r2c3p-i")]
            c_i: 0,
            #[cfg(feature = "r2c3p-o")]
            c_o: 0,
            #[cfg(feature = "r2c3p-o")]
            c_on: 0,
        }
    }

    #[cfg(feature = "r2c3p-i")]
    pub fn get_ci(&self) -> u64 {
        self.c_i
    }

    /// 默认处理一条消息
    pub fn eat<const N: usize>(&mut self, r: &LowRecv<N>) -> Option<LowEat<M>> {
        // 检查是否成功接收消息
        if let Some(t) = r.get_t() {
            // 是否为请求消息
            let req = MsgType::from(t) == MsgType::Req;

            // 消息太长
            if r.get_e2() {
                if req {
                    // 对于过长的请求消息, 返回 `E-2` 错误
                    return Some(LowEat::E2(send_e2_len(self.len)));
                } else {
                    // 如果不是请求消息, 丢弃
                    return None;
                }
            }

            // 检查消息类型
            match t {
                // 处理 `c` 消息
                #[cfg(feature = "r2c3p-c")]
                p::MSGT_C_R => {
                    match r.get_body() {
                        Some(b) => {
                            let (k, v) = read_conf(b);
                            return Some(self.eat_c(r, read_conf_k(k), v));
                        }
                        None => {
                            // `E-4` 错误
                            return Some(LowEat::E(send_e4()));
                        }
                    }
                }
                _ => {
                    // 未知的请求消息, 返回 `E-3` 错误
                    if req {
                        return Some(LowEat::E3(send_e3(t)));
                    }
                }
            }
        }

        None
    }

    // 处理 `c` 消息
    #[cfg(feature = "r2c3p-c")]
    fn eat_c<const N: usize>(&mut self, r: &LowRecv<N>, k: ConfK, v: Option<&[u8]>) -> LowEat<M> {
        match k {
            #[cfg(feature = "r2c3p-i")]
            ConfK::I => {
                if let Some(v) = v {
                    match hex_u64(v) {
                        Some(u) => {
                            // 设置值
                            self.c_i = u;
                        }
                        None => {
                            // 消息解析错误
                            return LowEat::E(send_e4());
                        }
                    }
                }
                // 返回当前值
                LowEat::CHex8(send_c_u64(p::CONF_I, self.c_i))
            }

            #[cfg(feature = "r2c3p-o")]
            ConfK::O => {
                if let Some(v) = v {
                    match hex_u8(v) {
                        Some(u) => {
                            self.c_o = u;
                        }
                        None => return LowEat::E(send_e4()),
                    }
                }
                LowEat::CHex1(send_c_u8(p::CONF_O, self.c_o))
            }
            #[cfg(feature = "r2c3p-o")]
            ConfK::On => {
                if let Some(v) = v {
                    match hex_u8(v) {
                        Some(u) => {
                            self.c_on = u;
                        }
                        None => return LowEat::E(send_e4()),
                    }
                }
                LowEat::CHex1(send_c_u8(p::CONF_ON, self.c_on))
            }

            #[cfg(feature = "r2c3p-cc")]
            ConfK::CR => match v {
                Some(_) => {
                    // 禁止设置值
                    LowEat::E(send_e5())
                }
                None => {
                    // 返回计数器的值
                    LowEat::CHex4(send_c_u32(p::CONF_CR, r.get_cr()))
                }
            },
            #[cfg(feature = "r2c3p-cc")]
            ConfK::CRd => match v {
                Some(_) => {
                    // 禁止设置值
                    LowEat::E(send_e5())
                }
                None => {
                    // 返回计数器的值
                    LowEat::CHex4(send_c_u32(p::CONF_CRD, r.get_crd()))
                }
            },

            _ => {
                // 不支持的配置项
                LowEat::E(send_e5())
            }
        }
    }
}
