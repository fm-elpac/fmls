//! 消息默认处理

use core::iter::Iterator;

use crate::r2c3p as p;

use super::{
    sendc_e2_len, sendc_e3, sendc_msg, BArraySender, BStaticSender, CrcT, HexArraySender,
    LowCSender, LowRecv, LowSend, MsgType,
};

#[cfg(feature = "r2c3p-i")]
use super::hex_u64;
#[cfg(feature = "r2c3p-o")]
use super::hex_u8;
#[cfg(feature = "r2c3p-crc16")]
use super::C16;
#[cfg(feature = "r2c3p-c")]
use super::{read_conf, read_conf_k, sendc_e4, sendc_e5, ConfK};

/// 消息默认处理可能返回的需要发送的消息 (CRC16)
///
/// `N`: 缓冲区长度的数组长度 (用于发送消息太长错误).
/// 详见 `send_e2_len`
#[cfg(feature = "r2c3p-crc16")]
pub type LowEat<const N: usize> = LowEatC<N, C16, 2>;

/// 消息默认处理可能需要发送的消息 (不同 CRC 选择)
#[derive(Debug, Clone)]
pub enum LowEatC<const N: usize, C: CrcT<M>, const M: usize> {
    /// 比如 `E-2 32`
    E2(LowSend<BArraySender<N>, C, M>),
    /// 比如 `E-3 c`
    E3(LowSend<BArraySender<4>, C, M>),
    /// 比如 `E-4`, `E-5`
    E(LowSend<BStaticSender, C, M>),
    /// 比如 `CcT=`
    #[cfg(feature = "r2c3p-cc")]
    CHex4(LowSend<LowCSender<HexArraySender<4>>, C, M>),
    /// 比如 `CI=`
    #[cfg(feature = "r2c3p-i")]
    CHex8(LowSend<LowCSender<HexArraySender<8>>, C, M>),
    /// 比如 `CO=`
    #[cfg(feature = "r2c3p-o")]
    CHex1(LowSend<LowCSender<HexArraySender<1>>, C, M>),
}

impl<const N: usize, C: CrcT<M>, const M: usize> Iterator for LowEatC<N, C, M> {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        match self {
            LowEatC::E2(s) => s.next(),
            LowEatC::E3(s) => s.next(),
            LowEatC::E(s) => s.next(),
            #[cfg(feature = "r2c3p-cc")]
            LowEatC::CHex4(s) => s.next(),
            #[cfg(feature = "r2c3p-i")]
            LowEatC::CHex8(s) => s.next(),
            #[cfg(feature = "r2c3p-o")]
            LowEatC::CHex1(s) => s.next(),
        }
    }
}

/// 发送 `C` 消息 (v: `hex(u8)`)
pub fn send_c_u8<C: CrcT<N>, const N: usize>(
    k: &'static [u8],
    v: u8,
    c: C,
) -> LowSend<LowCSender<HexArraySender<1>>, C, N> {
    let b: [u8; 1] = [v];
    sendc_msg(
        p::MSGT_C,
        LowCSender::new(k, Some(HexArraySender::new(b))),
        c,
    )
}

/// 发送 `C` 消息 (v: `hex(u16)`)
pub fn send_c_u16<C: CrcT<N>, const N: usize>(
    k: &'static [u8],
    v: u16,
    c: C,
) -> LowSend<LowCSender<HexArraySender<2>>, C, N> {
    let b: [u8; 2] = u16::to_be_bytes(v);
    sendc_msg(
        p::MSGT_C,
        LowCSender::new(k, Some(HexArraySender::new(b))),
        c,
    )
}

/// 发送 `C` 消息 (v: `hex(u32)`)
pub fn send_c_u32<C: CrcT<N>, const N: usize>(
    k: &'static [u8],
    v: u32,
    c: C,
) -> LowSend<LowCSender<HexArraySender<4>>, C, N> {
    let b: [u8; 4] = u32::to_be_bytes(v);
    sendc_msg(
        p::MSGT_C,
        LowCSender::new(k, Some(HexArraySender::new(b))),
        c,
    )
}

/// 发送 `C` 消息 (v: `hex(u64)`)
pub fn send_c_u64<C: CrcT<N>, const N: usize>(
    k: &'static [u8],
    v: u64,
    c: C,
) -> LowSend<LowCSender<HexArraySender<8>>, C, N> {
    let b: [u8; 8] = u64::to_be_bytes(v);
    sendc_msg(
        p::MSGT_C,
        LowCSender::new(k, Some(HexArraySender::new(b))),
        c,
    )
}

/// 消息默认处理
///
/// `len`: 缓冲区长度 (用于发送消息太长错误)
#[derive(Debug, Clone)]
pub struct EatC<const N: usize> {
    /// 缓冲区长度数组
    len: [u8; N],

    // 处理 `cI` 消息
    #[cfg(feature = "r2c3p-i")]
    c_i: u64,
    // 处理 `cO`, `cOn` 消息
    #[cfg(feature = "r2c3p-o")]
    pub c_o: u8,
    #[cfg(feature = "r2c3p-o")]
    pub c_on: u8,
}

impl<const N: usize> EatC<N> {
    pub fn new(len: [u8; N]) -> Self {
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
    pub fn eat<const T: usize, C: CrcT<M>, const M: usize>(
        &mut self,
        r: &LowRecv<T>,
        c: C,
    ) -> Option<LowEatC<N, C, M>> {
        // 检查是否成功接收消息
        if let Some(t) = r.get_t() {
            // 是否为请求消息
            let req = MsgType::from(t) == MsgType::Req;

            // 消息太长
            if r.get_e2() {
                if req {
                    // 对于过长的请求消息, 返回 `E-2` 错误
                    return Some(LowEatC::E2(sendc_e2_len(self.len, c)));
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
                            return Some(self.eat_c(r, read_conf_k(k), v, c));
                        }
                        None => {
                            // `E-4` 错误
                            return Some(LowEatC::E(sendc_e4(c)));
                        }
                    }
                }
                _ => {
                    // 未知的请求消息, 返回 `E-3` 错误
                    if req {
                        return Some(LowEatC::E3(sendc_e3(t, c)));
                    }
                }
            }
        }

        None
    }

    // 处理 `c` 消息
    #[cfg(feature = "r2c3p-c")]
    fn eat_c<const T: usize, C: CrcT<M>, const M: usize>(
        &mut self,
        r: &LowRecv<T>,
        k: ConfK,
        v: Option<&[u8]>,
        c: C,
    ) -> LowEatC<N, C, M> {
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
                            return LowEatC::E(sendc_e4(c));
                        }
                    }
                }
                // 返回当前值
                LowEatC::CHex8(send_c_u64(p::CONF_I, self.c_i, c))
            }

            #[cfg(feature = "r2c3p-o")]
            ConfK::O => {
                if let Some(v) = v {
                    match hex_u8(v) {
                        Some(u) => {
                            self.c_o = u;
                        }
                        None => return LowEatC::E(sendc_e4(c)),
                    }
                }
                LowEatC::CHex1(send_c_u8(p::CONF_O, self.c_o, c))
            }
            #[cfg(feature = "r2c3p-o")]
            ConfK::On => {
                if let Some(v) = v {
                    match hex_u8(v) {
                        Some(u) => {
                            self.c_on = u;
                        }
                        None => return LowEatC::E(sendc_e4(c)),
                    }
                }
                LowEatC::CHex1(send_c_u8(p::CONF_ON, self.c_on, c))
            }

            #[cfg(feature = "r2c3p-cc")]
            ConfK::CR => match v {
                Some(_) => {
                    // 禁止设置值
                    LowEatC::E(sendc_e5(c))
                }
                None => {
                    // 返回计数器的值
                    LowEatC::CHex4(send_c_u32(p::CONF_CR, r.get_cr(), c))
                }
            },
            #[cfg(feature = "r2c3p-cc")]
            ConfK::CRd => match v {
                Some(_) => {
                    // 禁止设置值
                    LowEatC::E(sendc_e5(c))
                }
                None => {
                    // 返回计数器的值
                    LowEatC::CHex4(send_c_u32(p::CONF_CRD, r.get_crd(), c))
                }
            },

            _ => {
                // 不支持的配置项
                LowEatC::E(sendc_e5(c))
            }
        }
    }
}

/// 消息默认处理 (crc16)
///
/// `len`: 缓冲区长度 (用于发送消息太长错误)
#[cfg(feature = "r2c3p-crc16")]
#[derive(Debug, Clone)]
pub struct Eat<const N: usize>(EatC<N>);

#[cfg(feature = "r2c3p-crc16")]
impl<const N: usize> Eat<N> {
    pub fn new(len: [u8; N]) -> Self {
        Self(EatC::new(len))
    }

    #[cfg(feature = "r2c3p-i")]
    pub fn get_ci(&self) -> u64 {
        self.0.get_ci()
    }

    /// 默认处理一条消息
    pub fn eat<const T: usize>(&mut self, r: &LowRecv<T>) -> Option<LowEatC<N, C16, 2>> {
        self.0.eat(r, C16::new())
    }
}
