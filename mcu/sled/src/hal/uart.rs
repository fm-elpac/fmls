//! UART (串口) 操作

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
