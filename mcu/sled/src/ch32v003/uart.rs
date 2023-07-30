//! UART 功能

use crate::led::Led;
use crate::util::UartEcho;
use crate::P;

/// UART 存储模块内部状态
pub struct Uart {
    pub e: UartEcho,
}

impl Uart {
    // 默认值
    pub const fn default() -> Self {
        Self {
            e: UartEcho::default(),
        }
    }

    pub fn one_loop(&mut self, p: &P, _led: &mut Led) {
        self.e.one_loop(p, true);
    }
}
