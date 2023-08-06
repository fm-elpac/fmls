//! 预定义的配置项 (内置配置处理)

use libfmlsc::r2c3p as p;
use libfmlsc::r2c3p::ConfC;

use super::super::body::{read_body, read_conf_k, Body, ConfK};
use super::super::send::{CSender, ESender, MsgSender};
use super::super::{
    hex_u64, hex_u8, HexU32Sender, HexU64Sender, HexU8Sender, NU8Sender, VecSender,
};
use super::eat::Eat;

/// 预定义的配置数据
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ConfData {
    /// 传输质量监测计数器: `cT`, `cR`, `cRd`, `cTB`, `cRB`
    #[cfg(feature = "r2c3p-cc")]
    pub tc: ConfC,
    /// 配置项 `I`
    #[cfg(feature = "r2c3p-i")]
    pub i: u64,
    /// 配置项 `O`
    #[cfg(feature = "r2c3p-o")]
    pub o: u8,
    /// 配置项 `On`
    #[cfg(feature = "r2c3p-o")]
    pub on: u8,
    /// 配置项 `@`
    #[cfg(feature = "r2c3p-at")]
    pub at: u8,
}

impl ConfData {
    pub const fn new() -> Self {
        Self {
            #[cfg(feature = "r2c3p-cc")]
            tc: ConfC {
                t: 0,
                r: 0,
                rd: 0,
                tb: 0,
                rb: 0,
            },
            #[cfg(feature = "r2c3p-i")]
            i: 0,
            #[cfg(feature = "r2c3p-o")]
            o: 0,
            #[cfg(feature = "r2c3p-o")]
            on: 0,
            #[cfg(feature = "r2c3p-at")]
            at: 0,
        }
    }

    /// 处理 `c` 消息
    pub fn eat_c(&mut self, body: &[u8]) -> Option<Eat> {
        // 生成 `E-4` 错误
        fn e4() -> Option<Eat> {
            Some(Eat::E(MsgSender::new(
                p::MSGT_E,
                ESender::new(VecSender::new(p::EB_4), None),
            )))
        }
        // 生成 `E-5` 错误
        fn e5() -> Option<Eat> {
            Some(Eat::E(MsgSender::new(
                p::MSGT_E,
                ESender::new(VecSender::new(p::EB_5), None),
            )))
        }
        // 生成 `C` 消息
        #[cfg(feature = "r2c3p-at")]
        fn c_n8(c: &'static [u8], u: u8) -> Option<Eat> {
            Some(Eat::CN8(MsgSender::new(
                p::MSGT_C,
                CSender::new(c, NU8Sender::new(u)),
            )))
        }
        #[cfg(feature = "r2c3p-cc")]
        fn c_hex_u32(c: &'static [u8], u: u32) -> Option<Eat> {
            Some(Eat::CHexU32(MsgSender::new(
                p::MSGT_C,
                CSender::new(c, HexU32Sender::new(u)),
            )))
        }
        #[cfg(feature = "r2c3p-i")]
        fn c_hex_u64(c: &'static [u8], u: u64) -> Option<Eat> {
            Some(Eat::CHexU64(MsgSender::new(
                p::MSGT_C,
                CSender::new(c, HexU64Sender::new(u)),
            )))
        }
        #[cfg(feature = "r2c3p-o")]
        fn c_hex_u8(c: &'static [u8], u: u8) -> Option<Eat> {
            Some(Eat::CHexU8(MsgSender::new(
                p::MSGT_C,
                CSender::new(c, HexU8Sender::new(u)),
            )))
        }

        match read_body(p::MSGT_C_R, body) {
            Body::C { k, v } => {
                // 检查配置项
                match read_conf_k(k) {
                    // 应用代码应该处理的: `T`, `t`
                    #[cfg(feature = "r2c3p-cc")]
                    ConfK::CT => match v {
                        Some(_) => {
                            // 不支持设置值
                            e5()
                        }
                        None => {
                            // 返回计数器的值
                            c_hex_u32(p::CONF_CT, self.tc.t)
                        }
                    },
                    #[cfg(feature = "r2c3p-cc")]
                    ConfK::CR => match v {
                        Some(_) => e5(),
                        None => c_hex_u32(p::CONF_CR, self.tc.r),
                    },
                    #[cfg(feature = "r2c3p-cc")]
                    ConfK::CRd => match v {
                        Some(_) => e5(),
                        None => c_hex_u32(p::CONF_CRD, self.tc.rd),
                    },
                    #[cfg(feature = "r2c3p-cc")]
                    ConfK::CTB => match v {
                        Some(_) => e5(),
                        None => c_hex_u32(p::CONF_CTB, self.tc.tb),
                    },
                    #[cfg(feature = "r2c3p-cc")]
                    ConfK::CRB => match v {
                        Some(_) => e5(),
                        None => c_hex_u32(p::CONF_CRB, self.tc.rb),
                    },
                    #[cfg(feature = "r2c3p-i")]
                    ConfK::I => match v {
                        Some(v) => match hex_u64(v) {
                            Some(u) => {
                                // 设置值
                                self.i = u;
                                // 返回当前值
                                c_hex_u64(p::CONF_I, self.i)
                            }
                            None => {
                                // 消息解析错误
                                e4()
                            }
                        },
                        None => c_hex_u64(p::CONF_I, self.i),
                    },
                    #[cfg(feature = "r2c3p-o")]
                    ConfK::O => match v {
                        Some(v) => match hex_u8(v) {
                            Some(u) => {
                                self.o = u;
                                c_hex_u8(p::CONF_O, self.o)
                            }
                            None => e4(),
                        },
                        None => c_hex_u8(p::CONF_O, self.o),
                    },
                    #[cfg(feature = "r2c3p-o")]
                    ConfK::On => match v {
                        Some(v) => match hex_u8(v) {
                            Some(u) => {
                                self.on = u;
                                c_hex_u8(p::CONF_ON, self.on)
                            }
                            None => e4(),
                        },
                        None => c_hex_u8(p::CONF_ON, self.on),
                    },
                    #[cfg(feature = "r2c3p-at")]
                    ConfK::At => {
                        // 返回 `c@=0`
                        c_n8(p::CONF_AT, self.at)
                    }

                    _ => {
                        // 不支持的配置项
                        e5()
                    }
                }
            }
            _ => {
                // 消息附加数据格式错误, 返回 `E-4` 错误
                e4()
            }
        }
    }
}
