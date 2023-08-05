//! 系统初始化

use crate::P;

/// 初始化系统时基定时器 (systick)
pub fn init_stk(p: &P) {
    p.PFIC.stk_ctlr.modify(|_, w| {
        w.stre()
            .clear_bit() // 向上计数到比较值后继续向上计数
            .stclk()
            .clear_bit() // HCLK/8 做时基 (1MHz)
            .stie()
            .clear_bit() // 关闭计数器中断
            .ste()
            .set_bit() // 启动系统计数器 STK
    });
}

/// 启用 GPIOA
pub fn init_gpioa(p: &P) {
    p.RCC.apb2pcenr.modify(|_, w| w.iopaen().set_bit());
}

/// 启用 GPIOC
pub fn init_gpioc(p: &P) {
    p.RCC.apb2pcenr.modify(|_, w| w.iopcen().set_bit());
}

/// 启用 GPIOD
pub fn init_gpiod(p: &P) {
    p.RCC.apb2pcenr.modify(|_, w| w.iopden().set_bit());
}

/// 启用 UART1
pub fn init_uart1(p: &P) {
    p.RCC.apb2pcenr.modify(|_, w| w.usart1en().set_bit());

    let u = &p.USART1;

    // 波特率 9600
    //
    // 波特率计算公式
    //   波特率 = HCLK / (16 * USARTDIV)
    //   USARTDIV = DIV_M + (DIV_F / 16)
    //
    // HCLK 频率 8MHz (默认)
    // 则 USARTDIV = 52.08333333333333
    // 则 DIV_M = 52, DIV_F = 1
    // 实际 USARTDIV = 52.0625
    // 实际 波特率 9603.8415366146, 误差 0.04%
    u.brr.modify(|_, w| {
        w.div_mantissa()
            .variant(52) // 这 12 位定义了分频器除法因子的整数部分
            .div_fraction()
            .variant(1) // 这 4 位定义了分频器除法因子的小数部分
    });
    // 工作模式设置
    u.ctlr1.modify(|_, w| {
        w.m()
            .clear_bit() // 8 个数据位
            .pce()
            .clear_bit() // 关闭奇偶校验
            .te()
            .set_bit() // 发送使能
            .re()
            .set_bit() // 接收使能
    });
    u.ctlr2.modify(|_, w| {
        w.linen()
            .clear_bit() // 关闭 LIN 模式
            .stop()
            .variant(0) // 1 个停止位
    });
    u.ctlr3.modify(|_, w| {
        w.ctse()
            .clear_bit()
            .rtse()
            .clear_bit() // 关闭硬件流控
            .scen()
            .clear_bit() // 关闭 智能卡模式
            .hdsel()
            .clear_bit() // 关闭 半双工模式
            .iren()
            .clear_bit() // 关闭 红外模式
    });

    // 启用 UART
    u.ctlr1.modify(|_, w| w.ue().set_bit());
}
