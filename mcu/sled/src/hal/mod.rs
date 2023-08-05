//! 硬件抽象层

use crate::P;

mod led;
mod sys_init;
mod systick;
mod uart;

pub use led::{led_off, led_on, Led};
pub use systick::{read_stk, read_stk_1, StkTimer};
pub use uart::{uart1_写, uart1_可写, uart1_可读, uart1_读};

#[allow(dead_code)]
pub use sys_init::{init_gpioa, init_gpioc, init_gpiod, init_stk, init_uart1};

// 读取芯片唯一 ID (96 bit)
pub fn read_uid(p: &P) -> (u32, u32, u32) {
    let u1 = p.ESIG.uniid1.read().u_id().bits();
    let u2 = p.ESIG.uniid2.read().u_id().bits();
    let u3 = p.ESIG.uniid3.read().u_id().bits();
    (u1, u2, u3)
}
