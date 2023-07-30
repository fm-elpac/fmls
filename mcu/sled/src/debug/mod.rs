//! 调试功能

use crate::P;

pub extern "C" static mut DEBUG_P: u32 = 0;
pub extern "C" static mut DEBUG_STK: u32 = 0;

#[inline(never)]
#[no_mangle]
pub extern "C" fn keep_alive_debug(p: &P) {
    // PC3
    p.GPIOC.outdr.modify(|_, w| {
        if unsafe { DEBUG_P ^ DEBUG_STK == 0 } {
            w.odr3().set_bit()
        } else {
            w
        }
    });
}

// TODO
