//! 通用工具代码

use crate::P;

mod systick;
mod uart;

#[allow(dead_code)]
pub use systick::{read_stk, read_stk_1, wait_stk, StkTimer};
#[allow(dead_code)]
pub use uart::{uart1_写, uart1_可写, uart1_可读, uart1_读, UartEcho};

// 读取芯片唯一 ID (96 bit)
#[allow(dead_code)]
pub fn read_uid(p: &P) -> (u32, u32, u32) {
    let u1 = p.ESIG.uniid1.read().u_id().bits();
    let u2 = p.ESIG.uniid2.read().u_id().bits();
    let u3 = p.ESIG.uniid3.read().u_id().bits();
    (u1, u2, u3)
}
