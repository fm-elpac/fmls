//! 低级别的 `fmls_r2c3p` 实现 (UART)

mod escape_crc;
mod hex;
mod msg_type;
mod recv;
mod send;
mod sender;

pub use escape_crc::{Escape, Unescape};
pub use hex::{hex_u16, hex_u32, hex_u64, hex_u8, index_of, n_u8, Fifo};
pub use msg_type::MsgType;
pub use recv::LowRecv;
pub use send::{send_msg_0, send_msg_32f, CrcT, LowSend, LowSendC, LowVSender, C0, C32F};
pub use sender::{BArraySender, BStaticSender, HexArraySender, NoneSender};

#[cfg(feature = "r2c3p-crc32")]
pub use escape_crc::Crc32;
#[cfg(feature = "r2c3p-crc16")]
pub use escape_crc::{crc_len, Crc16};
#[cfg(feature = "r2c3p-crc16")]
pub use send::{send_e2, send_e2_len, send_e3, send_e4, send_e5, send_msg_16, C16};
#[cfg(feature = "r2c3p-crc32")]
pub use send::{send_msg_32, send_v, C32};
