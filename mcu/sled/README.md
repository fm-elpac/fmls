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

### flash 空间占用

固件对 flash 空间的占用: 5132 字节 (32%) (`libfmlsm::r2c3p_low` 2023-08-08)

添加默认消息处理: 6628 字节 (41%) (`libfmlsm::r2c3p_low` 2023-08-08)

只使用 crc16 (`r2c3p`, `r2c3p-crc16`): 5094 字节 (32%)

### 引脚使用

- ch32v003f4p6 TSSOP20

| 编号 | 名称       | 说明                             |
| :--: | :--------- | :------------------------------- |
|  2   | PD5/A5/UTX | UART 发送                        |
|  3   | PD6/A6/URX | UART 接收                        |
|  7   | VSS        | 接地                             |
|  9   | VDD        | 电源 (3.3V)                      |
|  11  | PC1        | 闪烁 LED: 此灯不停闪烁           |
|  12  | PC2        | 状态 LED: 初始化完成后常亮       |
|  18  | PD1/SWIO   | 下载/调试 (连接 WCH-LINKE SWDIO) |

- ch32v003j4m6 SOP8

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

### 栈内存空间使用分析

- 不开启 r2c3p 功能:

  ```sh
  > cargo +rv32e build --release --no-default-features --features ch32v003f4p6
  ```

  此时最大使用的栈地址 (最低地址): `0x07b0` (1968 字节)

- 开启 r2c3p, crc16 功能:

  ```sh
  > cargo +rv32e build --release --no-default-features --features ch32v003f4p6,r2c3p,r2c3p-crc16
  ```

  此时最大栈使用的地址: `0x06d8` (1752 字节)

  栈多用了 216 字节.

---

TODO
