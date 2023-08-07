//! `fmls_r2c3p` 协议工具

// TODO 支持 USB

mod body;
mod hex;
mod port;
mod send;

pub use body::{read_body, Body};
pub use hex::{
    HexU16Sender, HexU32Sender, HexU64Sender, HexU8Sender, NU8Sender, U16LeSender, U32LeSender,
};
pub use port::{Eat, R2c3pPort, R2c3pPort128, R2c3pPort32, R2c3pPort64, R2c3pPort8, R2c3pPortT};
pub use send::{CSender, ESender, MsgSender, VSender};

#[cfg(feature = "r2c3p-c")]
pub use body::{read_conf_k, read_conf_v, ConfK, ConfV};
#[cfg(feature = "r2c3p-c")]
pub use port::ConfData;
