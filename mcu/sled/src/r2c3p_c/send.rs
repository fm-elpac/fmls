//! 处理消息的发送

use core::iter::Iterator;

use libfmlsm::r2c3p_low::{LowSend, LowVSender, NoneSender};

#[cfg(not(feature = "not-mini"))]
use libfmlsm::r2c3p::MSGT_V;
#[cfg(not(feature = "not-mini"))]
use libfmlsm::r2c3p_low::{send_msg_0, C0};
#[cfg(feature = "r2c3p-crc16")]
use libfmlsm::r2c3p_low::{send_msg_16, C16};
#[cfg(feature = "not-mini")]
use libfmlsm::r2c3p_low::{send_v, C32};

use crate::conf::{FW_VER, HW_NAME};

/// 所有可发送的消息
pub enum Sender {
    /// 发送 `V` 消息
    #[cfg(feature = "not-mini")]
    V(LowSend<LowVSender<12>, C32, 4>),
    #[cfg(not(feature = "not-mini"))]
    V(LowSend<LowVSender<12>, C0, 0>),
    /// 发送 `.` 消息
    #[cfg(feature = "r2c3p-crc16")]
    D(LowSend<NoneSender, C16, 2>),
    #[cfg(not(feature = "r2c3p-crc16"))]
    D(LowSend<NoneSender, C0, 0>),
}

impl Iterator for Sender {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        match self {
            Sender::V(s) => s.next(),
            Sender::D(s) => s.next(),
        }
    }
}

/// 发送 `V` 消息
pub fn make_v(uid: (u32, u32, u32)) -> Sender {
    let mut id: [u8; 12] = [0; 12];
    id[0..4].copy_from_slice(&u32::to_le_bytes(uid.0));
    id[4..8].copy_from_slice(&u32::to_le_bytes(uid.1));
    id[8..12].copy_from_slice(&u32::to_le_bytes(uid.2));

    #[cfg(feature = "not-mini")]
    {
        Sender::V(send_v(FW_VER, HW_NAME, id))
    }
    #[cfg(not(feature = "not-mini"))]
    {
        Sender::V(send_msg_0(MSGT_V, LowVSender::new(FW_VER, HW_NAME, id)))
    }
}

/// 发送 `.` 消息
pub fn make_d() -> Sender {
    #[cfg(feature = "r2c3p-crc16")]
    {
        Sender::D(send_msg_16(b'.', NoneSender::new()))
    }
    #[cfg(not(feature = "r2c3p-crc16"))]
    {
        Sender::D(send_msg_0(b'.', NoneSender::new()))
    }
}
