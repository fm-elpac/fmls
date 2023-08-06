# fmls_r2c3p 协议 0.1.0

简单的近似单行文本的消息发送协议, 明文传输不加密:

- `r2`: 适用于低资源设备 (单片机 MCU)

- `c3`: 一条消息的校验码使用 crc32

- `p`: 点对点通信, 通过 UART (串口) 或 USB 发送.

设计参考:

- MIN 协议 <https://github.com/min-protocol/min>

fmls_r2c3p 协议的版本号: `0.1.0-a1`

- 语义化版本 2.0.0 <https://semver.org/>

> The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD",
> "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL" in this document are to be
> interpreted as described in
> [RFC 2119](https://www.rfc-editor.org/rfc/rfc2119).

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
    - 2.2.7 @
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
    - 2.4.12 @ (`u8`)
    - 2.4.13 @sN (`hex(u32)`)
    - 2.4.14 @nN (`hex(u32)`)
- 3 实现要求
- 4 下层协议参数
  - 4.1 UART
  - 4.2 USB
  - 4.3 塑料光纤 UART 远距离传输
- 5 r2c3p over TLS/TCP
  - 5.1 TLS 证书验证
  - 5.2 传输格式
- 6 星形级联枚举
  - 6.1 节点号
  - 6.2 枚举探测
  - 6.3 消息透传
  - 6.4 消息接收缓冲区
- 7 上位机 (fmls-r2d) 对 UART 接口的处理

## 1 消息格式

- 1 字节: **消息类型** (type)

- N 字节: **附加数据** (data)

  可为 0 字节.

- 4 字节 (UART): crc32 **校验码**, 对 `消息类型+附加数据` 进行计算.

  USB 无需 crc 校验码.

  如果消息长度 (消息类型+附加数据) 不超过 32 字节, (MUST) 使用 crc16 校验码 (2
  字节).

  crc 以小尾字节序添加 (LE).

消息长度限制: 一条消息必须 (MUST) 在 900ms 以内发送完毕. 以 UART 方式传输时,
建议 (SHOULD) 一条消息的长度不超过 128 字节.

- crc32 计算方式: CRC-32/ISO-HDLC

  <https://reveng.sourceforge.io/crc-catalogue/all.htm>

  > ## CRC-32/ISO-HDLC
  > `width=32 poly=0x04c11db7 init=0xffffffff refin=true refout=true xorout=0xffffffff check=0xcbf43926 residue=0xdebb20e3 name="CRC-32/ISO-HDLC"`
  > - Class: attested
  > - Alias: **CRC-32**, CRC-32/ADCCP, CRC-32/V-42, CRC-32/XZ, PKZIP
  > - HDLC is defined in ISO/IEC 13239.
  > - ITU-T Recommendation V.42 (March 2002)
  > - AUTOSAR (25 November 2021), AUTOSAR Classic Platform release R21-11,
    > Specification of CRC Routines
  > - Lasse Collin, Igor Pavlov et al. (27 August 2009), The .xz file format,
    > version 1.0.4
  > - IETF RFC 1662 (July 1994)
  > - PKWARE Inc. (1 February 1993), PKZIP 2.04g
  > - Frank J. T. Wojcik, Guy Eric Schalnat, Andreas Dilger, Glenn
    > Randers-Pehrson et al. (15 October 1999), libpng 1.0.5
  > - Lasse Collin, Igor Pavlov et al. (21 May 2011), XZ Utils 5.0.3
  > - Lammert Bies (August 2011), CRC calculator
  > - PVL Team (25 October 2008), CRC .NET control, version 14.0.0.0
  > - Dr Ross N. Williams (19 August 1993), "A Painless Guide to CRC Error
    > Detection Algorithms"
  > - Emil Lenchak, Texas Instruments, Inc. (June 2018), CRC Implementation With
    > MSP430
  > - Berndt M. Gammel (29 October 2006), Matpack 1.9.1 class MpCRC
    > documentation
  > - Cisco Systems (September 2013), Meraki Air Marshal white paper
  > - Ryan Luecke, James Lyons (11 October 2011), CRC32 Checksums; The Good, The
    > Bad, And The Ugly
  > - Unique effective solution of codeword set
  > - Created: 30 March 2005
  > - Updated: 7 May 2022

- crc16 计算方式: CRC-16/ARC

  <https://reveng.sourceforge.io/crc-catalogue/all.htm>

  > ## CRC-16/ARC
  > `width=16 poly=0x8005 init=0x0000 refin=true refout=true xorout=0x0000 check=0xbb3d residue=0x0000 name="CRC-16/ARC"`
  > - Class: attested
  > - Alias: ARC, **CRC-16**, CRC-16/LHA, CRC-IBM
  > - AUTOSAR (25 November 2021), AUTOSAR Classic Platform release R21-11,
    > Specification of CRC Routines
  > - System Enhancement Associates (24 October 1986), ARC 5.20
  > - Haruyasu Yoshizaki (10 January 1996), LHA 2.55E
  > - Rahul Dhesi (19 April 1996), ZOO 2.1a
  > - Lammert Bies (August 2011), CRC calculator
  > - PVL Team (25 October 2008), CRC .NET control, version 14.0.0.0
  > - Dr Ross N. Williams (19 August 1993), "A Painless Guide to CRC Error
    > Detection Algorithms"
  > - Emil Lenchak, Texas Instruments, Inc. (June 2018), CRC Implementation With
    > MSP430
  > - Altera Corporation (April 1999), crc MegaCore Function Data Sheet, version
    > 2 (courtesy of the Internet Archive)
  > - Unique effective solution of codeword set
  > - Created: 30 March 2005
  > - Updated: 7 May 2022

### 1.1 消息转义传输

适用于 UART, USB 无需转义.

转义前的消息: 消息类型 + 附加数据 + crc 校验码

需要转义的字节: `\n` (0x0a), `\\` (0x5c)

- 如果遇到 `\n` 字节, 则发送 `\\n` (0x5c 0x6e)

- 如果遇到 `\\` 字节, 则发送 `\\s` (0x5c 0x73)

消息发送完毕后, 再发送一个 `\n` (0x0a) 字节表示消息结束.

### 1.2 接收消息的处理

如果收到转义错误 (不能识别的转义) 或 crc 校验错误的消息, (MUST) 直接丢弃.

## 2 消息类型

- **请求消息** (request): 收到请求消息后必须 (MUST) 发送响应消息. 但是,
  如果实现的能接收的最大消息长度不足 32 字节 (使用 crc16), 且接收的消息太长,
  在收到请求消息后, 可以 (MAY) 不发送 `E-2` (消息太长) 错误消息.

  小写字母 (`a` ~ `z`), 取值 0x61 至 0x7a.

- **响应消息** (response): 用于回应收到的请求消息.

  大写字母 (`A` ~ `Z`), 取值 0x41 至 0x5a.

  注意: 只有在收到请求消息后, 才能发送响应消息. 如果没有收到请求消息, 不得 (MUST
  NOT) 发送响应消息.

- **静默消息** (silent): 收到后无需发送响应消息.

  其余取值.

### 2.1 请求响应消息的处理

发送请求消息时, 必须 (MUST) 一问一答, 在收到响应消息之前, 不能 (MUST NOT)
发送新的请求消息.

发送请求消息后, 如果 1 秒内未收到响应消息, 则认为发送失败, 自动重试发送.
如果发送 3 次后仍然失败, 不再重试, 向上层应用报告错误.

上层应用在使用请求响应消息时, 应该注意, 重复收到多条相同的请求消息, (MUST)
不影响正常功能.

### 2.2 预定义的消息类型

应用不得 (MUST NOT) 将预定义的消息类型用于别的用途, 如果支持, 必须 (MUST)
按照协议的规定使用. 对于没有预定义的消息类型, 应用可自由使用.

- 2.2.1 `v` 0x76 (请求消息) 获取设备固件版本信息

  对此消息的支持是必须的 (MUST).

  如果消息长度为 2 字节, 且内容为 `vv`, 则不 (MUST NOT) 检查 crc. 其余情况需要
  (MUST) 检查 crc.

  (向后兼容) 这是因为, 不同版本的 fmls_r2c3p 协议, 可能使用不同的 crc 计算方式.
  而在接收 `V` 消息之前, 不知道协议的版本号. 如果 `v` 消息必须检查 crc,
  就会遇到先有鸡还是先有蛋的问题.

  不检查 crc 的消息为 2 字节的 `vv` 消息, 这是为了避免因为传输错误 (比特翻转)
  而导致别的消息被误认为 `v` 消息. 采用这种设计, 则单个比特翻转错误,
  不可能导致别的消息被误认为 `v` 消息. 当然, 如果同时发生多个比特翻转错误,
  仍然有可能导致误认, 但概率较小.

  收到此消息后应该 (MUST) 发送 `V` 消息.

- 2.2.2 `V` 0x56 (响应消息) 返回设备固件版本信息

  对此消息的支持是必须的 (MUST).

  返回的内容有:
  - fmls_r2c3p 协议的版本号
  - 固件名称, 固件的版本号
  - 设备硬件信息, 设备的唯一编号 (如果支持)

  之间以 `\n` (0x0a) 分隔. 此处的文本 (MUST) 使用 UTF-8 编码.

  以上 3 行内容是必须的 (MUST). 可以 (MAY) 在之后添加自定义内容 (以 `\n` 分隔),
  但要注意消息总长度不应 (SHOULD NOT) 超过 256 字节.

  此消息必须 (MUST) 使用 crc32 校验码, 无论消息长度.

  比如: (请求+响应, 无 crc)

  ```
  v
  Vfmls_r2c3p 0.1.0\nsled 0.1.0\nch32v003 66665555
  ```

- 2.2.3 `E` 0x45 (响应消息) 返回错误

  对此消息的支持是必须的 (MUST).

  附加数据格式:
  - 错误码 (十进制数字文本)
  - 空格 0x20 (在使用错误信息时需要)
  - 错误信息 (可选) (MAY)

  比如 `E-2 32` 表示错误码 `-2`, 错误信息 `32`.

- 2.2.4 `K` 0x4b (响应消息) 表示成功 (ok)

  对此消息的支持是必须的 (MUST).

  附加数据可选 (MAY).

- 2.2.5 `c` 0x63 (请求消息) 用于设备配置

  对此消息的支持是可选的 (MAY).

  格式:

  - `cK` 获取当前的配置值 (配置项为 K)

  - `cK=V` 设置某个配置值 (配置项为 K, 值为 V)

    K 和 V 都可以是多个字节.

  收到此消息后, 如果无错误, 应该 (MUST) 发送 `C` 消息. 如果有错误, 应该 (MUST)
  发送 `E` 消息.

- 2.2.6 `C` 0x43 (响应消息) 返回设备配置

  对此消息的支持是可选的 (MAY).

  格式: `CK=V`

  比如: (# 表示注释)

  ```
  cm    # 获取配置项 m 的值
  Cm=0  # 当前 m 的值为 0
  cm=1  # 设置 m 的值为 1
  Cm=1  # 设置成功, m 的值现在为 1
  ```

- 2.2.7 `@` 0x40 (静默消息) 用于 r2 集线器 (星形级联枚举)

  对此消息的支持是可选的 (MAY).

  附加数据格式: 1 字节 (节点号, 可选) + N 字节 (承载的数据)

### 2.3 预定义的错误码

错误码范围:

- `> 0` 应用定义

- `<= 0` 预定义

对于低端单片机, 可以 (MAY) 使用 `i8` 作为错误码的数据类型. 对于更强大的设备,
可以按需使用 `i16` 或 `i32` 数据类型.

对这些预定义错误码的支持是必须的 (MUST).

- 2.3.1 `E-1` 保留 (未知错误)

- 2.3.2 `E-2` 消息太长

  错误信息 (可选) (MAY) 可以返回能够接收的最大消息长度.

  比如 `E-2 32` 表示最大可接收 32 字节长度的消息.

- 2.3.3 `E-3` 未知的消息类型

  错误信息 (可选) (MAY) 可以返回具体针对哪个消息类型.

  比如 `E-3 c` 表示不支持 `c` 消息类型.

- 2.3.4 `E-4` 错误的消息格式

  表示消息的附加数据无法解析. 错误信息可选 (MAY).

- 2.3.5 `E-5` 错误的消息参数

  消息的附加数据格式正确, 但是其内容无法接受. 错误信息可选 (MAY).

### 2.4 预定义的配置

对这些配置项的支持是可选的 (MAY). 如果不支持某个配置项, 当收到 `cK` (不含 `=`)
消息时, 应该 (SHOULD) 返回 `E-5` 错误码, 表示不支持 `K` 配置项.

- 2.4.1 `m` 用于多通道模式 (`u8`), 默认值 `0`

  - 值 `0` 表示禁用此功能

  - 值 `1` 表示启用此功能

  TODO

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

  此值 (`u64`) 由上位机 (r1) 设置及读取, 设备 (MCU) 自己不应该 (MUST NOT) 修改.
  如果一个设备同时通过多个路径连接了上位机, 可以通过此值发现.

  设备重启 (启动) 之后, 应该 (SHOULD) 将此值设为 0. 上位机可以通过读取 0
  值来检测设备意外重启.

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

- 2.4.12 `@` 用于检测下级设备是否为 r2 集线器 (`u8`)

  - 值 `0` 表示不是 r2 集线器

  - 值 `1` 表示是 r2 集线器

  比如:

  ```
  c@
  C@=1
  ```

- 2.4.13 `@s`N r2 集线器: 下行接口的传输速度 (`hex(u32)`)

  格式: `@s` hex(u8) `=` hex(u32)

  其中 N (u8) 是节点号. 值的单位: 字节/秒, 默认值 960 (波特率 9600)

  比如:

  ```
  c@s01
  C@s01=000003c0
  ```

- 2.4.14 `@n`N r2 集线器: 下行接口的缓冲区长度 (`hex(u32)`)

  格式: `@n` hex(u8) `=` hex(u32)

  其中 N (u8) 是节点号. 值的单位: 字节, 默认值 800 (允许通过的最大消息长度)

  比如:

  ```
  c@n01
  C@n01=00000320
  ```

## 3 实现要求

fmls_r2c3p 的实现必须 (MUST) 满足这些要求:

- 单片机 (MCU) 至少能够接收长度不超过 8 字节 (不含 crc) 的消息.

- 支持 `v`, `V`, `E`, `K` 消息类型.

## 4 下层协议参数

建议 (SHOULD) 使用下列参数配置下层协议, 方便各设备的连接, 而无需手动配置.

- 4.1 UART

  - 波特率: 9600

  - 数据格式: 8N1 (8 个数据位, 1 个停止位, 无校验位)

  - 电压: 3.3V (TTL)

- 4.2 USB

  支持的速率:

  - USB 12Mbps (USB 1.1 FS)

  - USB 480Mbps (USB 2.0 HS)

  TODO

- 4.3 塑料光纤 UART 远距离传输

  此处的参数是可选的 (MAY).

  - 传输距离: 10m ("远距离" 是相对的)

    波特率: 9600

  - 塑料光纤: SI-POF (PMMA 纤芯材料)

    价格: 约 0.6 元/m (单根, 直径 1mm)

    损耗: 约 230dB/km (10m 损耗约 2.3dB)

  - 发射/接收装置: 红外 LED / 光敏二极管

    价格: 约 0.42 元/对 (发射+接收, 直径 3mm)

    波长: 940nm

    电压不超过 5V, 电流不超过 30mA

## 5 r2c3p over TLS/TCP

对此功能的支持是可选的 (MAY).

如果一个设备, 通过 IP 网络连接 (比如无线 wifi), 有能力 (足够的硬件资源) 运行
TCP+TLS, 但是没能力运行完整的 fmls 协议 (fmlsd), 那么这个设备仍然属于 r2 设备,
通过 `r2c3p over TLS/TCP` 与 r1 连接.

比如 `esp32c2` 设备.

- 5.1 TLS 证书验证

  r2c3p over TLS/TCP 的 r2 设备, 只在本地存储 CA 根证书 (公钥),
  自己并没有客户端证书.

  r2 发起 TLS 连接时, (MUST) 只验证服务端 (r1) 的证书.

  上位机 (r1) 对 r2 设备的身份验证, 由应用自己处理.

- 5.2 传输格式

  r2c3p 在 TLS/TCP 之上传输的时候, 只是不需要 crc 校验, 其余类似 UART 传输.

建议 (SHOULD) r2 设备使用 `mDNS/DNS-SD` 来发现 r1 设备的 IP 地址和端口号.

r1 设备发布的服务类似于:

```sh
> avahi-publish-service FMLS-R2D _r2c3p_tls._tcp 20666 r=r1 PK=ghE+2ydlbsUZszITydwzyEEz0ZCwCDMKsctMfIT4obo RC=ArVNfkFMXdW8byjmwuP8znUwiDpjEvJNDroWzbvFG6I HOSTNAME=$(hostname) r2c3p=tls/tcp
```

其中 `_r2c3p_tls._tcp` 表示协议类型.

## 6 星形级联枚举

也叫 **r2 集线器** 功能. 对此功能的支持是可选的 (MAY).

r2 设备 (MCU) 可工作在 r2 集线器模式 (类似 USB 集线器), 通过 UART 等接口下连多个
r2 设备 (MCU), 从而构建低成本的 r2 设备有线局域网.

### 6.1 节点号

每级枚举的节点号长度为 1 字节 (u8), 可用的节点号为 1 至 254. 节点号 0 和 255
(0xff) 保留.

### 6.2 枚举探测

上级设备 (比如 r1) 向下级设备 (比如 r2 MCU) 发送 `@` (1 字节) 消息,
也就是枚举探测消息.

下级设备收到枚举探测消息后, 如果是 r2 集线器, (MUST) 就向上级设备发送节点号列表.

比如 r2 集线器 (ch32v103) 通过 USB 12Mbps 连接 r1, 同时通过 3 个 UART 接口下连
ch32v003 设备, 那么发送的消息如下:

> (r1 -> ch32v103) `@`
>
> (ch32v103 -> r1) `@` 0x00 0x01 0x02 0x03

节点号列表消息的节点号为 0, 承载的数据为多个字节, 每个是一个节点号.

上级设备收到节点号列表消息后, 就知道了有哪些节点号可用.

下级设备也可以 (MAY) 随时发送节点号列表消息, 更新上级设备的节点号列表.

### 6.3 消息透传

r2 集线器收到上级设备发送的 `@` 消息后, (MUST) 转发给对应的下连设备. r2
集线器收到下连设备发送的消息后, (MUST) 转发给上级设备.

与上级设备收发的 `@` 消息, 节点号为对应下连设备的节点号, 承载的数据为原消息.

比如 r1 向第一个下连的设备发送消息:

> (r1 -> ch32v103) `@` 0x01 `v`
>
> (ch32v103 -> ch32v003) `v`
>
> (ch32v003 -> ch32v103) `Vfmls_r2c3p 0.1.0\nsled 0.1.0\nch32v003 66665555`
>
> (ch32v103 -> r1) `@` 0x01 `Vfmls_r2c3p 0.1.0\nsled 0.1.0\nch32v003 66665555`

(MAY) 支持多级枚举 (r2 集线器下连 r2 集线器) 和消息发送, 比如:

> (r1 -> ch32v203) `@` 0x02 `@` 0x01 `ct`
>
> (ch32v203 -> ch32v103) `@` 0x01 `ct`
>
> (ch32v103 -> ch32v003) `ct`
>
> (ch32v003 -> ch32v103) `Ct=ca15`
>
> (ch32v103 -> ch32v203) `@` 0x01 `Ct=ca15`
>
> (ch32v203 -> r1) `@` 0x02 `@` 0x01 `Ct=ca15`

不建议 (SHOULD NOT) 使用太多层的集线器级联.

### 6.4 消息接收缓冲区

r2 集线器对于上行/下行的消息, (SHOULD) 采用存储转发方式,
以处理上行/下行接口的数据传输速率可能不同的情况.

对于上行/下行的消息, 分别使用一个消息接收环形缓冲区. 接收消息时, 如果缓冲区已满,
则丢弃此消息.

对于上行消息的发送, r2 集线器自身的响应消息, 优先于转发消息.

比如对于一个具有 4 个下连 UART 接口的 r2 集线器, 单个接收缓冲区可设置为 3200
字节, 则集线器接收缓冲区总的内存占用为 6400 字节.

## 7 上位机 (fmls-r2d) 对 UART 接口的处理

此处的说明对于实现仅供参考 (MAY).

对于每个 UART 接口进行如下处理:

1. 设备发现: 检测此 UART 接口是否连接有设备

   发送 `v` 消息, 每秒一次.

2. 配置设备: 发现设备之后

   如果收到 `V` 消息, 说明设备连接 (上线), 根据返回的设备信息,
   配置具体的上位机软件 (deno r2um).

3. 保活检测: 持续检查设备是否仍然在线

   发送 `ct` 消息, 每秒一次. 如果连续 3 秒未收到回复, 认为设备离线.

4. 星形级联枚举: 处理 r2 集线器

   对于通过 `V` 新发现的设备, 发送 `@` 消息进行探测, 每秒一次, 共尝试 3 次.

5. 刷新枚举: 处理集线器连接设备的变动

   对每个设备发送 `@` 消息, 每 5 秒一次. 如果节点号列表有变动, 则进行相应处理.

TODO
