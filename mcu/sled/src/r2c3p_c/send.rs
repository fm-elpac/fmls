//! 处理消息的发送

use core::iter::Iterator;

use libfmlsm::r2c3p_low::{LowSend, LowVSender, NoneSender};

#[cfg(not(feature = "not-mini"))]
use libfmlsm::r2c3p::MSGT_V;
#[cfg(not(feature = "r2c3p-crc16"))]
use libfmlsm::r2c3p_low::{send0_msg, C0};
#[cfg(feature = "r2c3p-crc16")]
use libfmlsm::r2c3p_low::{send16_msg, LowEat, C16};
#[cfg(not(feature = "not-mini"))]
use libfmlsm::r2c3p_low::{send32f_msg, C32F};
#[cfg(feature = "not-mini")]
use libfmlsm::r2c3p_low::{send_v, C32};

use crate::conf::{FW_VER, HW_NAME};

#[cfg(feature = "r2c3p-crc16")]
use super::recv::E2_LEN;

/// 所有可发送的消息
#[derive(Debug, Clone)]
pub enum Sender {
    /// 默认消息处理
    #[cfg(feature = "r2c3p-crc16")]
    Eat(LowEat<E2_LEN>),
    /// 发送 `V` 消息
    #[cfg(feature = "not-mini")]
    V(LowSend<LowVSender<12>, C32, 4>),
    #[cfg(not(feature = "not-mini"))]
    V(LowSend<LowVSender<12>, C32F, 4>),
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
            #[cfg(feature = "r2c3p-crc16")]
            Sender::Eat(s) => s.next(),
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
        Sender::V(send32f_msg(
            MSGT_V,
            LowVSender::new(FW_VER, HW_NAME, id),
            crate::read_vc(),
        ))
    }
}

/// 发送 `.` 消息
pub fn make_d() -> Sender {
    #[cfg(feature = "r2c3p-crc16")]
    {
        Sender::D(send16_msg(b'.', NoneSender::new()))
    }
    #[cfg(not(feature = "r2c3p-crc16"))]
    {
        Sender::D(send0_msg(b'.', NoneSender::new()))
    }
}
