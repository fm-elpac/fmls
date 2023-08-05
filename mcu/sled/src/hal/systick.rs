//! 系统时基定时器

use crate::P;

/// 读取系统时基定时器 (systick) 的计数值
pub fn read_stk(p: &P) -> u32 {
    p.PFIC.stk_cntl.read().cntl().bits()
}

/// 检查 1ms (不精确) 跳变, 基于 systick
pub fn read_stk_1(p: &P) -> bool {
    // systick 计数器为 1MHz (默认频率), 此处等待约 1ms (1024 周期)
    let t: u32 = 0x400;

    read_stk(p) & t != 0
}

/// 基于 systick 的定时器, 非阻塞
pub struct StkTimer {
    // 计数器比特翻转标志
    b: bool,
    // 当前计数器的值
    i: u32,
}

impl StkTimer {
    pub const fn default() -> Self {
        Self { b: false, i: 0 }
    }

    /// 检查计数器是否到时间
    ///
    /// `stk_1`: [`read_stk_1`] 的结果
    /// `t`: 等待的毫秒数 (不精确)
    ///
    /// 返回: true 已超时
    pub fn check(&mut self, stk_1: bool, t: u32) -> bool {
        // 产生了比特翻转
        if stk_1 != self.b {
            self.b = stk_1;
            self.i += 1;

            if self.i > t {
                self.i = 0;

                return true;
            }
        }
        false
    }
}
