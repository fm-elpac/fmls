//! UART 操作封装

use crate::P;

/// 检查 UART1 是否有数据可读 (收到数据)
pub fn uart1_可读(p: &P) -> bool {
    // RXNE 读数据寄存器非空标志
    p.USART1.statr.read().rxne().bit()
}

/// UART1 读取 1 字节数据
pub fn uart1_读(p: &P) -> u8 {
    p.USART1.datar.read().dr().bits() as u8
}

/// 检查 UART1 是否可写 (之前的数据已发送)
pub fn uart1_可写(p: &P) -> bool {
    // TXE 发送数据寄存器空标志
    p.USART1.statr.read().txe().bit()
}

pub fn uart1_写(p: &P, d: u8) {
    p.USART1.datar.write(|w| w.dr().variant(d as u16));
}

/// UART 回显功能
pub struct UartEcho {
    // UART 收到的数据, 用于 UART 回显
    r: Option<u8>,
}

impl UartEcho {
    // 默认值
    pub const fn default() -> Self {
        Self { r: None }
    }

    pub fn set_r(&mut self, r: Option<u8>) {
        self.r = r;
    }

    fn 发送(p: &P, d: u8, a: bool) {
        let o = if a {
            // 大小写英文字母转换
            match d {
                b'a'..=b'z' | b'A'..=b'Z' => d ^ 0x20,
                _ => d,
            }
        } else {
            d
        };
        uart1_写(p, o);
    }

    // `a`: 启用大小写字母转换
    pub fn one_loop(&mut self, p: &P, a: bool) {
        // 发送数据
        if let Some(d) = self.r {
            if uart1_可写(p) {
                Self::发送(p, d, a);
                self.r = None;
            }
        }

        // 接收数据
        if let None = self.r {
            if uart1_可读(p) {
                self.r = Some(uart1_读(p));
            }
        }
    }
}
