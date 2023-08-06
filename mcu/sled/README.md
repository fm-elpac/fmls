# sled

点亮一颗 LED.

支持的硬件:

- ch32v103

- ch32v003 单片机

## ch32v103

TODO

## ch32v003

关于 ch32v003 单片机的编译工具准备, 以及固件刷写方法, 请见
[ch32v003.md](./ch32v003.md).

编译:

```sh
> cargo +rv32e build --release
```

UART 波特率: 9600

r2c3p 测试:

```
vv
Vfmls_r2c3p 0.1.0\nsled 0.1.0\nch32v003f4p6 cdab84aa49bc9a12ffffffff
```

固件对 flash 空间的占用: 8392 字节 (52%) (`libfmlsm::r2c3p_util` 2023-08-06)

固件对 flash 空间的占用: 5192 字节 (32%) (`libfmlsm::r2c3p_low` 2023-08-07)
节省了 3200 字节 (19%)

---

引脚使用: (ch32v003f4p6 TSSOP20)

| 编号 | 名称       | 说明                             |
| :--: | :--------- | :------------------------------- |
|  2   | PD5/A5/UTX | UART 发送                        |
|  3   | PD6/A6/URX | UART 接收                        |
|  7   | VSS        | 接地                             |
|  9   | VDD        | 电源 (3.3V)                      |
|  11  | PC1        | 闪烁 LED: 此灯不停闪烁           |
|  12  | PC2        | 状态 LED: 初始化完成后常亮       |
|  18  | PD1/SWIO   | 下载/调试 (连接 WCH-LINKE SWDIO) |

引脚使用: (ch32v003j4m6 SOP8)

TODO (UART)

| 编号 | 名称     | 说明                             |
| :--: | :------- | :------------------------------- |
|  1   | PD6/UTX_ | UART 发送                        |
|  2   | VSS      | 接地                             |
|  3   | PA2      | 状态 LED: 初始化完成后常亮       |
|  4   | VDD      | 电源 (3.3V)                      |
|  5   | PC1/URX_ | UART 接收                        |
|  7   | PC4      | 闪烁 LED: 此灯不停闪烁           |
|  8   | PD1/SWIO | 下载/调试 (连接 WCH-LINKE SWDIO) |

---

TODO
