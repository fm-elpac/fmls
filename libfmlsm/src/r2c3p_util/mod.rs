//! `fmls_r2c3p` 协议工具

// TODO 支持 USB

mod body;
mod escape_crc;
mod hex;
mod msg_type;
mod port;
mod send;

pub use hex::{
    Fifo2, Fifo4, HexU16Sender, HexU32Sender, HexU64Sender, HexU8Sender, NU8Sender, NoneSender,
    U16LeSender, U32LeSender, VecSender,
};
pub use msg_type::MsgType;
pub use port::{
    BodyReader, R2c3pPort, R2c3pPort128, R2c3pPort32, R2c3pPort64, R2c3pPort8, R2c3pPortT,
};
pub use send::{MsgSender, VSender};
