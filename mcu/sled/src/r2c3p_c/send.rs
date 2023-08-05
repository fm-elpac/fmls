//! 处理消息的发送

use core::iter::Iterator;

use libfmlsm::r2c3p as p;
use libfmlsm::r2c3p_util::{Eat, HexU32Sender, MsgSender, NoneSender, VSender, VecSender};

use crate::conf::{FW_VER, HW_NAME};

/// 所有可发送的消息
pub enum Sender {
    /// 默认消息处理
    P(Eat),
    /// 发送 `V` 消息
    V(MsgSender<VSender<HwSender, NoneSender>>),
    /// 发送 `.` 消息
    D(MsgSender<NoneSender>),
}

impl Sender {
    pub fn done(&self) -> bool {
        match self {
            Sender::P(s) => s.done(),
            Sender::V(s) => s.done(),
            Sender::D(s) => s.done(),
        }
    }
}

impl Iterator for Sender {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        match self {
            Sender::P(s) => s.next(),
            Sender::V(s) => s.next(),
            Sender::D(s) => s.next(),
        }
    }
}

/// 发送 `V` 消息
pub fn send_v(uid: (u32, u32, u32)) -> Sender {
    Sender::V(MsgSender::new(
        p::MSGT_V,
        VSender::new(FW_VER, HwSender::new(uid), None),
    ))
}

#[derive(PartialEq)]
enum HwSenderS {
    /// 正在发送硬件名称
    Name,
    /// 正在发送唯一序号
    Uid(u8),
    /// 发送完毕
    None,
}

/// 发送 `V` 消息的硬件信息 (hardware) 部分
pub struct HwSender {
    s: HwSenderS,
    n: VecSender,
    u: HexU32Sender,
    uid: (u32, u32, u32),
}

impl HwSender {
    pub fn new(uid: (u32, u32, u32)) -> Self {
        Self {
            s: HwSenderS::Name,
            n: VecSender::new(HW_NAME),
            u: HexU32Sender::new(uid.0),
            uid,
        }
    }
}

impl Iterator for HwSender {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        match self.s {
            HwSenderS::Name => match self.n.next() {
                Some(b) => Some(b),
                None => {
                    self.s = HwSenderS::Uid(0);
                    // 发送空格
                    Some(p::BYTE_SPACE)
                }
            },
            HwSenderS::Uid(i) => match self.u.next() {
                Some(b) => Some(b),
                None => match i {
                    0 => {
                        self.u = HexU32Sender::new(self.uid.1);
                        self.s = HwSenderS::Uid(1);
                        self.u.next()
                    }
                    1 => {
                        self.u = HexU32Sender::new(self.uid.2);
                        self.s = HwSenderS::Uid(2);
                        self.u.next()
                    }
                    _ => {
                        self.s = HwSenderS::None;
                        None
                    }
                },
            },
            HwSenderS::None => None,
        }
    }
}

/// 发送 `.` 消息
pub fn send_d() -> Sender {
    Sender::D(MsgSender::new(b'.', NoneSender::new()))
}
