//! `fmls_r2c3p` 协议工具

// TODO 支持 USB

mod body;
mod escape_crc;
mod hex;
mod msg_type;
mod port;
mod send;

pub use body::{read_body, Body};
pub use hex::{
    hex_u16, hex_u32, hex_u64, hex_u8, index_of, n_u8, Fifo2, Fifo4, HexU16Sender, HexU32Sender,
    HexU64Sender, HexU8Sender, NU8Sender, NoneSender, U16LeSender, U32LeSender, U8Sender,
    VecSender,
};
pub use msg_type::MsgType;
pub use port::{Eat, R2c3pPort, R2c3pPort128, R2c3pPort32, R2c3pPort64, R2c3pPort8, R2c3pPortT};
pub use send::{CSender, ESender, MsgSender, VSender};

#[cfg(feature = "r2c3p-c")]
pub use body::{read_conf_k, read_conf_v, ConfK, ConfV};
#[cfg(feature = "r2c3p-c")]
pub use port::ConfData;
