//! ch32v003 设备支持代码

use ch32v0::ch32v003::Peripherals as P;

use crate::hal::{init_gpioc, init_gpiod, init_stk, init_uart1, led_off};
use crate::r2c3p_c::R2c3pClient;

#[cfg(feature = "ch32v003j4m6")]
use crate::hal::init_gpioa;

pub fn init(p: &P) {
    init_stk(p);
    init_gpioc(p);
    init_gpiod(p);
    #[cfg(feature = "ch32v003j4m6")]
    init_gpioa(p);

    match () {
        #[cfg(feature = "ch32v003f4p6")]
        () => {
            // 配置 PC1 引脚
            p.GPIOC.cfglr.modify(|_, w| {
                // 推挽输出 10MHz
                w.mode1().variant(0b01).cnf1().variant(0b00)
            });

            // PD5 USART1_TX (USART1_RM=00)
            p.GPIOD.cfglr.modify(|_, w| {
                // 推挽复用输出
                w.mode5()
                    .variant(0b10) // 输出模式 2MHz
                    .cnf5()
                    .variant(0b10) // 复用功能推挽输出模式
            });
            // PD6 USART1_RX (USART1_RM=00)
            p.GPIOD.cfglr.modify(|_, w| {
                // 带上拉输入
                w.mode6()
                    .variant(0b00) // 输入模式
                    .cnf6()
                    .variant(0b10) // 带有上下拉模式
            });
            p.GPIOD.outdr.modify(|_, w| {
                w.odr6().set_bit() // 上拉输入
            });
        }

        #[cfg(feature = "ch32v003j4m6")]
        () => {
            // 配置 PC4 引脚
            p.GPIOC.cfglr.modify(|_, w| {
                // 推挽输出 10MHz
                w.mode4().variant(0b01).cnf4().variant(0b00)
            });

            // TODO PD6 USART1_TX (USART1_RM=10)
            // TODO PC1 USART1_RX (USART1_RM=11)
        }
    }

    init_uart1(p);

    // 配置结束
    match () {
        #[cfg(feature = "ch32v003f4p6")]
        () => {
            // 配置 PC2 引脚
            p.GPIOC.cfglr.modify(|_, w| {
                // 推挽输出 2MHz
                w.mode2().variant(0b10).cnf2().variant(0b00)
            });
        }
        #[cfg(feature = "ch32v003j4m6")]
        () => {
            // 配置 PA2 引脚
            p.GPIOA.cfglr.modify(|_, w| {
                // 推挽输出 2MHz
                w.mode2().variant(0b10).cnf2().variant(0b00)
            });
        }
    }

    led_off(p);
}

/// 存储全局状态信息
pub struct G {
    c: R2c3pClient,
}

impl G {
    // 默认值
    pub const fn default() -> Self {
        Self {
            c: R2c3pClient::default(),
        }
    }

    pub fn one_loop(&mut self, p: &P) {
        self.c.one_loop(p);
    }
}
