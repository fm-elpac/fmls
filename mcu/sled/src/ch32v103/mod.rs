//! ch32v103 设备支持代码

use ch32v1::ch32v103::Peripherals as P;

use create::sys_init::{init_gpioc, init_stk};

pub fn init(p: &P) {
    init_stk(p);
    init_gpioc(p);
    // TODO
}

/// 存储全局状态信息
#[derive(Debug, Default)]
pub struct G {
    // TODO
}

impl G {
    pub fn one_loop(&mut self, p: &P) {
        // TODO
    }
}
