//! `fmls_r2c3p` 协议工具

// TODO 支持 USB

use crate::r2c3p_low::{
    hex_u16, hex_u32, hex_u64, hex_u8, index_of, n_u8, Escape, MsgType, NoneSender, Unescape,
};

#[cfg(feature = "r2c3p-crc32")]
use crate::r2c3p_low::Crc32;
#[cfg(feature = "r2c3p-crc16")]
use crate::r2c3p_low::{crc_len, Crc16};

mod body;
mod hex;
mod port;
mod send;

pub use body::{read_body, Body};
pub use hex::{
    Fifo2, Fifo4, HexU16Sender, HexU32Sender, HexU64Sender, HexU8Sender, NU8Sender, U16LeSender,
    U32LeSender, U8Sender, VecSender,
};
pub use port::{Eat, R2c3pPort, R2c3pPort128, R2c3pPort32, R2c3pPort64, R2c3pPort8, R2c3pPortT};
pub use send::{CSender, ESender, MsgSender, VSender};

#[cfg(feature = "r2c3p-c")]
pub use body::{read_conf_k, read_conf_v, ConfK, ConfV};
#[cfg(feature = "r2c3p-c")]
pub use port::ConfData;
