//! r2c3p 支持

use crate::hal::{uart1_写, uart1_可写, uart1_可读, uart1_读, Led};
use crate::P;

#[cfg(feature = "r2c3p")]
mod recv;
#[cfg(feature = "r2c3p")]
mod send;

#[cfg(feature = "r2c3p")]
use recv::Recv;
#[cfg(feature = "r2c3p")]
use send::Sender;

/// r2c3p 客户端
pub struct R2c3pClient {
    led: Led,

    #[cfg(feature = "r2c3p")]
    s: Option<Sender>,
    #[cfg(feature = "r2c3p")]
    r: Recv,
}

impl R2c3pClient {
    pub const fn default() -> Self {
        Self {
            led: Led::default(),
            #[cfg(feature = "r2c3p")]
            s: None,
            #[cfg(feature = "r2c3p")]
            r: Recv::default(),
        }
    }

    pub fn one_loop(&mut self, p: &P) {
        // LED 灯闪烁处理
        let on = self.led.one_loop(p);

        #[cfg(feature = "r2c3p")]
        {
            if on {
                self.r.led_on();
            }

            // 检查串口并发送
            if let Some(s) = &mut self.s {
                // 首先检查串口空闲
                if uart1_可写(p) {
                    // 发送一个字节
                    match s.next() {
                        Some(b) => {
                            uart1_写(p, b);
                        }
                        None => {
                            // 发送完毕
                            self.r.send_end();
                            self.s = None;
                        }
                    }
                }
            }

            // 从串口接收
            if uart1_可读(p) {
                let b = uart1_读(p);
                self.r.feed(b);
            }

            // 只有消息发送完毕后, 才检查是否有新消息需要发送
            if self.s.is_none() {
                self.s = self.r.check(p, &mut self.led);
            }
        }
    }
}
