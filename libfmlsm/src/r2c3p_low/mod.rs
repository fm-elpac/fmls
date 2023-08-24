//! 低级别的 `fmls_r2c3p` 实现 (UART)

mod eat;
mod escape_crc;
mod hex;
mod msg_type;
mod recv;
mod send;
mod sender;

#[cfg(feature = "r2c3p-c")]
mod conf;

pub use eat::{send_c_u16, send_c_u32, send_c_u64, send_c_u8, EatC, LowEatC};
pub use escape_crc::{Escape, Unescape};
pub use hex::{hex_u16, hex_u32, hex_u64, hex_u8, index_of, n_u8, Fifo};
pub use msg_type::MsgType;
pub use recv::{LowRecv, LowRecvC};
pub use send::{
    send0_e2, send0_e2_len, send0_e3, send0_e4, send0_e5, send0_msg, send0_v, send32f_msg,
    sendc_e2, sendc_e2_len, sendc_e3, sendc_e4, sendc_e5, sendc_msg, sendc_v, CrcT, LowCSender,
    LowSend, LowSendC, LowVSender, C0, C32F,
};
pub use sender::{BArraySender, BStaticSender, HexArraySender, NoneSender};

#[cfg(feature = "r2c3p-c")]
pub use conf::{read_conf, read_conf_k, ConfK};
#[cfg(feature = "r2c3p-crc16")]
pub use eat::{Eat, LowEat};
#[cfg(feature = "r2c3p-crc32")]
pub use escape_crc::Crc32;
#[cfg(feature = "r2c3p-crc16")]
pub use escape_crc::{crc_len, Crc16};
#[cfg(feature = "r2c3p-crc16")]
pub use send::{send16_msg, send_e2, send_e2_len, send_e3, send_e4, send_e5, C16};
#[cfg(feature = "r2c3p-crc32")]
pub use send::{send32_msg, send_v, C32};
