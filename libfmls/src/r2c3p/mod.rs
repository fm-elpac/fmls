//! `fmls_r2c3p` 协议 (r2d)

pub use libfmlsc::r2c3p::P_VERSION;

mod error;
mod escape_crc;
mod msg;
mod server;

pub use error::MsgRecvErr;
pub use msg::{
    hex, ConfItem, Msg, MsgAt, MsgReq, MsgRes, MsgS, MsgType, MsgTypeReq, MsgTypeRes, MsgTypeS,
};
pub use server::{FeedResult, R2c3pServer, TC};
