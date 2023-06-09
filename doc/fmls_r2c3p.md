# fmls_r2c3p 协议 0.1.0

简单的近似单行文本的消息发送协议, 明文传输不加密:

- `r2`: 适用于低资源设备 (单片机 MCU)

- `c3`: 一条消息的校验码使用 crc32

- `p`: 点对点通信, 通过 UART (串口) 或 USB 发送.

设计参考:

- MIN 协议 <https://github.com/min-protocol/min>

fmls_r2c3p 协议的版本号: `0.1.0`

- 语义化版本 2.0.0 <https://semver.org/>

## 目录

- 1 消息格式
  - 1.1 消息转义传输
  - 1.2 接收消息的处理
- 2 消息类型
  - 2.1 请求响应消息的处理
  - 2.2 预定义的消息类型
    - 2.2.1 v
    - 2.2.2 V
    - 2.2.3 E
    - 2.2.4 K
    - 2.2.5 c
    - 2.2.6 C
  - 2.3 预定义的错误码
    - 2.3.1 E-1
    - 2.3.2 E-2
    - 2.3.3 E-3
    - 2.3.4 E-4
    - 2.3.5 E-5
  - 2.4 预定义的配置
    - 2.4.1 m (`u8`)
    - 2.4.2 T (`hex([u8])`)
    - 2.4.3 t (`hex(u16)`)
    - 2.4.4 cT (`hex(u32)`)
    - 2.4.5 cR (`hex(u32)`)
    - 2.4.6 cRd (`hex(u32)`)
    - 2.4.7 cTB (`hex(u32)`)
    - 2.4.8 cRB (`hex(u32)`)
    - 2.4.9 I (`hex(u64)`)
    - 2.4.10 O (`hex(u8)`)
    - 2.4.11 On (`hex(u8)`)
- 3 多通道模式
- 4 实现要求

## 1 消息格式

- 1 字节: **消息类型** (type)

- N 字节: **附加数据** (data)

  可为 0 字节.

- 4 字节 (UART): crc32 **校验码**, 对 `消息类型+附加数据` 进行计算.

  USB 无需 crc 校验码.

  如果消息长度 (消息类型+附加数据) 不超过 32 字节, 使用 crc16 校验码 (2 字节).

消息长度限制: 一条消息必须在 900ms 以内发送完毕.

### 1.1 消息转义传输

适用于 UART, USB 无需转义.

转义前的消息: 消息类型 + 附加数据 + crc 校验码

需要转义的字节: `\n` (0x0a), `\\` (0x5c)

- 如果遇到 `\n` 字节, 则发送 `\\n` (0x5c 0x6e)

- 如果遇到 `\\` 字节, 则发送 `\\s` (0x5c 0x73)

消息发送完毕后, 再发送一个 `\n` (0x0a) 字节表示消息结束.

### 1.2 接收消息的处理

如果收到转义错误 (不能识别的转义) 或 crc 校验错误的消息, 直接丢弃.

## 2 消息类型

- **请求消息** (request): 收到请求消息后必须发送响应消息.

  小写字母 (`a` ~ `z`), 取值 0x61 至 0x7a.

- **响应消息** (response): 用于回应收到的请求消息.

  大写字母 (`A` ~ `Z`), 取值 0x41 至 0x5a.

- **静默消息** (silent): 收到后无需发送响应消息.

  其余取值.

### 2.1 请求响应消息的处理

发送请求消息时, 必须一问一答, 在收到响应消息之前, 不能发送新的请求消息.

发送请求消息后, 如果 1 秒内未收到响应消息, 则认为发送失败, 自动重试发送.
如果发送 3 次后仍然失败, 不再重试, 向上层应用报告错误.

上层应用在使用请求响应消息时, 应该注意, 重复收到多条相同的请求消息,
不影响正常功能.

### 2.2 预定义的消息类型

应用不得将预定义的消息类型用于别的用途, 如果支持, 必须按照协议的规定使用.
对于没有预定义的消息类型, 应用可自由使用.

- 2.2.1 `v` 0x76 (请求消息) 获取设备固件版本信息

  对此消息的支持是必须的.

  此消息不检查 crc 是否正确, 且忽略附加数据.

  收到此消息后应该发送 `V` 消息.

- 2.2.2 `V` 0x56 (响应消息) 返回设备固件版本信息

  对此消息的支持是必须的.

  返回的内容有:
  - fmls_r2c3p 协议的版本号
  - 固件名称, 固件的版本号
  - 设备硬件信息, 设备的唯一编号 (如果支持)

  之间以 `\n` (0x0a) 分隔.

  此消息必须使用 crc32 校验码, 无论消息长度.

  比如: (请求+响应, 无 crc)

  ```
  v
  Vfmls_r2c3p 0.1.0\nsled 0.1.0\nch32v003 66665555
  ```

- 2.2.3 `E` 0x45 (响应消息) 返回错误

  对此消息的支持是必须的.

  附加数据格式:
  - 错误码 (十进制数字文本)
  - 空格 0x20 (在使用错误信息时需要)
  - 错误信息 (可选)

  比如 `E-2 32` 表示错误码 `-2`, 错误信息 `32`.

- 2.2.4 `K` 0x4b (响应消息) 表示成功 (ok)

  对此消息的支持是必须的.

  附加数据可选.

- 2.2.5 `c` 0x63 (请求消息) 用于设备配置

  对此消息的支持是可选的.

  格式:

  - `cK` 获取当前的配置值 (配置项为 K)

  - `cK=V` 设置某个配置值 (配置项为 K, 值为 V)

    K 和 V 都可以是多个字节.

  收到此消息后, 如果无错误, 应该发送 `C` 消息. 如果有错误, 应该发送 `E` 消息.

- 2.2.6 `C` 0x43 (响应消息) 返回设备配置

  对此消息的支持是可选的.

  格式: `CK=V`

  比如: (# 表示注释)

  ```
  cm    # 获取配置项 m 的值
  Cm=0  # 当前 m 的值为 0
  cm=1  # 设置 m 的值为 1
  Cm=1  # 设置成功, m 的值现在为 1
  ```

### 2.3 预定义的错误码

错误码范围:

- `> 0` 应用定义

- `<= 0` 预定义

对于低端单片机, 可以使用 `i8` 作为错误码的数据类型. 对于更强大的设备,
可以按需使用 `i16` 或 `i32` 数据类型.

对这些预定义错误码的支持是必须的.

- 2.3.1 `E-1` 保留 (未知错误)

- 2.3.2 `E-2` 消息太长

  错误信息 (可选) 可以返回能够接收的最大消息长度.

  比如 `E-2 32` 表示最大可接收 32 字节长度的消息.

- 2.3.3 `E-3` 未知的消息类型

  错误信息 (可选) 可以返回具体针对哪个消息类型.

  比如 `E-3 c` 表示不支持 `c` 消息类型.

- 2.3.4 `E-4` 错误的消息格式

  表示消息的附加数据无法解析. 错误信息可选.

- 2.3.5 `E-5` 错误的消息参数

  消息的附加数据格式正确, 但是其内容无法接受. 错误信息可选.

### 2.4 预定义的配置

对这些配置项的支持是可选的. 如果不支持某个配置项, 当收到 `cK` (不含 `=`) 消息时,
应该返回 `E-5` 错误码, 表示不支持 `K` 配置项.

- 2.4.1 `m` 用于多通道模式 (`u8`), 默认值 `0`

  - 值 `0` 表示禁用此功能

  - 值 `1` 表示启用此功能

- 2.4.2 `T` 用于获取设备的系统时间 (`hex([u8])`)

  格式为 16 进制数字文本, 不限长度. 设备自定义的时间计数器.

  比如:

  ```
  cT
  CT=018905baffa7
  ```

- 2.4.3 `t` 用于获取设备的短的时间 (`hex(u16)`)

  格式与 `T` 相同. 可用于检测设备在线状态.

  比如: (# 表示注释)

  ```
  ct       # UART 方式, 本消息总长度 5 字节 (crc16)
  Ct=a8f8  # UART 方式, 本消息总长度 10 字节 (crc16)
  ```

- 2.4.4 `cT` 计数器 (`hex(u32)`): 总计发送的消息数量

  用于传输质量监测. 格式为 16 进制数字文本.

  比如:

  ```
  ccT
  CcT=00000149
  ```

- 2.4.5 `cR` 计数器 (`hex(u32)`): 总计成功接收的消息数量

  用于传输质量监测. 格式为 16 进制数字文本.

  比如:

  ```
  ccR
  CcR=000000c3
  ```

- 2.4.6 `cRd` 计数器 (`hex(u32)`): 总计丢弃的接收的消息数量

  用于传输质量监测. 格式为 16 进制数字文本.

  比如:

  ```
  ccRd
  CcRd=00000001
  ```

- 2.4.7 `cTB` 计数器 (`hex(u32)`): 总计发送的字节数

  用于传输质量监测. 格式为 16 进制数字文本.

  比如:

  ```
  ccTB
  CcTB=00002afc
  ```

- 2.4.8 `cRB` 计数器 (`hex(u32)`): 总计成功接收的字节数

  用于传输质量监测. 格式为 16 进制数字文本.

  比如:

  ```
  ccRB
  CcRB=00000eb2
  ```

- 2.4.9 `I` 用于重复设备检测 (`hex(u64)`)

  格式为 16 进制数字文本.

  比如:

  ```
  cI
  CI=e7a77e82c91d825d
  ```

  此值 (`u64`) 由上位机 (r1) 设置及读取, 设备 (MCU) 自己不应该修改.
  如果一个设备同时通过多个路径连接了上位机, 可以通过此值发现.

- 2.4.10 `O` 协议配置 (`hex(u8)`): 请求响应消息的超时时间

  格式为 16 进制数字文本.

  比如:

  ```
  cO
  CO=fa
  ```

  值 `0` 表示使用协议默认值 (1000ms). 值 `1` 表示超时时间为 4ms, 值 N 表示 N x
  4ms. 最大值 `0xff` 表示 1020ms.

  此配置可用于改善请求响应消息的传输性能 (延迟).

- 2.4.11 `On` 协议配置 (`hex(u8)`): 请求响应消息的重试次数

  格式为 16 进制数字文本.

  比如:

  ```
  cOn
  COn=03
  ```

  值 `0` 表示使用协议默认值 (3 次). 值 `1` 表示重试 1 次, 值 N 表示 N 次. 最大值
  `0xff` 表示 255 次.

  此配置可用于改善请求响应消息的传输性能.

## 3 多通道模式

这是一个可选功能.

TODO

## 4 实现要求

- 单片机 (MCU) 至少能够接收长度不超过 8 字节 (不含 crc) 的消息.

- fmls_r2c3p 协议的实现, 必须支持 `v`, `V`, `E`, `K` 消息类型.

TODO
