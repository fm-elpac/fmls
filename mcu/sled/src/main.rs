#![no_std]
#![no_main]
#![deny(unsafe_code)]

use core::arch::asm;

use panic_halt as _;
use riscv_rt::{entry, pre_init};

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
static mut SG: Option<G> = None;

#[allow(unsafe_code)]
#[entry]
fn main() -> ! {
    // 获取设备外设
    let p = unsafe { P::steal() };
    // 设备初始化
    init(&p);

    let g = unsafe {
        SG = Some(G::default());
        SG.as_mut().unwrap()
    };

    // 主循环
    loop {
        // 一次循环
        g.one_loop(&p);
    }
}

/// 发送 `V` 消息的固定 CRC32 值 ("tesT")
///
/// 这个值放在 `.data` 段 (RAM 可读写) 之中,
/// 可以在 `.hex` (Intel HEX) 文件中直接看到 (搜索) 并修改.
/// 修改 `.hex` 文件之后, 可以使用 `ihex_util.js` 计算行尾的校验码.
///
/// 实现在不重新编译固件的情况下, 修改 crc32 的值, 并刷写到设备
#[cfg(not(feature = "not-mini"))]
#[allow(unsafe_code)]
#[no_mangle]
pub static mut VC: [u8; 4] = [0x74, 0x65, 0x73, 0x54];

#[cfg(not(feature = "not-mini"))]
#[allow(unsafe_code)]
fn read_vc() -> [u8; 4] {
    unsafe { VC }
}

extern "C" {
    /// 堆起始地址 (在 `.bss` 和 `.data` 节之后)
    /// 在本程序中, 对应栈的最低地址
    static _sheap: u8;
}

/// 清空栈的内存区域
/// 用于分析栈空间使用情况
#[allow(unsafe_code)]
#[pre_init]
unsafe fn clear_stack() {
    // 获取当前栈指针
    let sp: usize;
    asm!("mv {}, sp", out(reg) sp);

    // 填充数据
    const FILL: u8 = b'-';
    // 填充整个栈区域
    let mut p: *mut u8 = &_sheap as *const u8 as *mut u8;
    while p < ((sp - 8) as *mut u8) {
        *p = FILL;
        p = p.add(1);
    }
}
