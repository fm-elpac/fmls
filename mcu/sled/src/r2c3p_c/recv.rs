//! 处理消息的接收

use libfmlsm::r2c3p as p;
use libfmlsm::r2c3p_low::LowRecv;

#[cfg(feature = "r2c3p-crc16")]
use libfmlsm::r2c3p_low::Eat;

use crate::hal::{read_uid, Led};
use crate::P;

use super::send::{make_d, make_v, Sender};

#[cfg(feature = "r2c3p-crc16")]
pub const E2_LEN: usize = 2 + 3;

#[cfg(feature = "not-mini")]
pub static E2_A: [u8; 5] = [0, 0, 0, b'6', b'4'];
#[cfg(all(not(feature = "not-mini"), feature = "r2c3p-crc16"))]
pub static E2_A: [u8; 5] = [0, 0, 0, b'3', b'2'];

#[derive(Debug, Clone)]
pub struct Recv {
    // 使用 64 字节接收缓冲区 (crc32)
    #[cfg(feature = "not-mini")]
    recv: LowRecv<{ 64 + 4 }>,

    // 使用 32 字节接收缓冲区 (crc16)
    #[cfg(not(feature = "not-mini"))]
    recv: LowRecv<{ 32 + 2 }>,

    // 默认消息处理
    #[cfg(feature = "r2c3p-crc16")]
    eat: Eat<E2_LEN>,

    // 标记正在发送
    sending: bool,
    // 标记闪烁
    led_o: bool,
}

impl Default for Recv {
    fn default() -> Self {
        Self {
            recv: LowRecv::new(),
            sending: false,
            led_o: false,
            #[cfg(feature = "r2c3p-crc16")]
            eat: Eat::new(E2_A),
        }
    }
}

impl Recv {
    /// 通知 LED 灯闪烁
    pub fn led_on(&mut self) {
        self.led_o = true;
    }

    /// 通知消息发送完毕
    pub fn send_end(&mut self) {
        self.sending = false;
    }

    /// 从 UART 接收一个字节
    pub fn feed(&mut self, u: u8) {
        self.recv.feed(u);
    }

    /// 检查是否有消息要发送
    pub fn check(&mut self, p: &P, led: &mut Led) -> Option<Sender> {
        if self.sending {
            return None;
        }
        // 发送 `.` 消息
        if self.led_o {
            self.led_o = false;
            return Some(make_d());
        }

        // 检查成功接收消息
        if let Some(t) = self.recv.get_t() {
            // 检查消息太长
            if !self.recv.get_e2() {
                if let Some(s) = self.on_msg(t, p, led) {
                    if s.is_some() {
                        self.sending = true;
                    }
                    return s;
                }
            }

            // 默认消息处理
            #[cfg(feature = "r2c3p-crc16")]
            if let Some(m) = self.eat.eat(&self.recv) {
                return Some(Sender::Eat(m));
            }
        }
        None
    }

    /// 收到一条消息的处理
    ///
    /// 如果不处理, 返回 None
    fn on_msg(&mut self, t: u8, p: &P, _led: &mut Led) -> Option<Option<Sender>> {
        match t {
            // 处理 `V` 消息
            p::MSGT_V_R => Some(Some(make_v(read_uid(p)))),

            _ => None,
        }
    }
}
