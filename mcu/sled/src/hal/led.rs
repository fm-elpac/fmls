//! LED 闪烁

use super::{read_stk_1, StkTimer};
use crate::P;

/// 点亮 LED (闪烁)
pub fn led_on(p: &P) {
    // 引脚复位
    match () {
        #[cfg(feature = "ch32v003f4p6")]
        () => {
            p.GPIOC.outdr.modify(|_, w| w.odr1().clear_bit());
        }
        #[cfg(feature = "ch32v003j4m6")]
        () => {
            p.GPIOC.outdr.modify(|_, w| w.odr4().clear_bit());
        }
    }
}

/// 关闭 LED (闪烁)
pub fn led_off(p: &P) {
    // 引脚置位
    match () {
        #[cfg(feature = "ch32v003f4p6")]
        () => {
            p.GPIOC.outdr.modify(|_, w| w.odr1().set_bit());
        }
        #[cfg(feature = "ch32v003j4m6")]
        () => {
            p.GPIOC.outdr.modify(|_, w| w.odr4().set_bit());
        }
    }
}

/// LED 存储模块内部状态
pub struct Led {
    /// LED 亮的时间
    pub t_1: u32,
    /// LED 灭的时间
    pub t_0: u32,

    // led 当前状态: true 亮
    s: bool,

    t: StkTimer,
}

impl Led {
    // 默认值
    pub const fn default() -> Self {
        Self {
            t_1: 160,
            t_0: 817,
            s: false,
            t: StkTimer::default(),
        }
    }

    pub fn one_loop(&mut self, p: &P) -> bool {
        let mut on = false;
        if self.s {
            if self.t.check(read_stk_1(p), self.t_1) {
                led_off(p);
                self.s = false;
            }
        } else {
            if self.t.check(read_stk_1(p), self.t_0) {
                led_on(p);
                self.s = true;
                // 指示点亮
                on = true;
            }
        }
        on
    }
}
