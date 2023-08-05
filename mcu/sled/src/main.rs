#![no_std]
#![no_main]
#![deny(unsafe_code)]

use panic_halt as _;
use riscv_rt::entry;

#[cfg(feature = "ch32v003")]
use ch32v0::ch32v003 as pac;
#[cfg(feature = "ch32v103")]
use ch32v1::ch32v103 as pac;

use pac::Peripherals as P;

mod conf;
mod hal;
mod r2c3p_c;

#[cfg(feature = "ch32v003")]
mod ch32v003;
#[cfg(feature = "ch32v003")]
use ch32v003::{init, G};

#[cfg(feature = "ch32v103")]
mod ch32v103;
#[cfg(feature = "ch32v103")]
use ch32v103::{init, G};

// 静态分配的全局数据
// 避免在栈空间动态分配
static mut SG: G = G::default();

#[entry]
#[allow(unsafe_code)]
fn main() -> ! {
    // 获取设备外设
    let p = unsafe { P::steal() };
    // 设备初始化
    init(&p);

    let g = unsafe { &mut SG };

    // 主循环
    loop {
        // 一次循环
        g.one_loop(&p);
    }
}
