# FMLS

<https://github.com/fm-elpac/fmls>

胖蚊轻蜘蛛

正式名称: `黄边巨蚊`系列 `家幽灵蛛`软件 (Pholcus phalangioides)

![CI](https://github.com/fm-elpac/fmls/actions/workflows/ci.yml/badge.svg)

镜像:

- TODO

([English document](./doc/en))

## 简介

FMLS: 基于 系统的自组织原理 的小规模设备网络. 设备发现, 连接, 资源共享.

基于 IP 网络, 去中心化 (不依赖公网 IP 地址分配, 和 DNS), 主要用于局域网.

---

当前状态和已经实现的功能: 请见 [版本号历史记录](./doc/版本.md).

### 要解决的问题和目标

目标是实现多种设备类型, 多种操作系统 (多种 CPU) 等的方便互连. 设备类型有:
台式机, 笔记本, 手机, 平板, 机架服务器, 路由器, 电视盒子, 单板机 (SBC), VR 设备,
单片机 (MCU) 等. 操作系统有: GNU/Linux, Android, Windows, OpenWrt, fuchsia 等.
CPU 有: x86_64, aarch64, rv64gc, rv32imc, rv32ec 等.

IP 网络 (以及物理层 有线以太网, 无线 wifi) 虽然可以实现多设备互连,
但实际使用很不方便:

1. IP 地址 (以及端口号) 通常是动态分配的, 同一个设备的地址可能会变动, 手动输入
   IP 地址 (以及端口号) 很麻烦 (还有 IPv6). 而 DNS 又是中心化管理的,
   不适合去中心化的小规模局域网.

2. TCP/IP 默认不考虑安全, 它并不验证设备的身份, 以及数据传输的安全.

3. IP 网络是无状态的, 它不管目标设备是否在线, 也不管路由是否可达.
   只有等很长时间, 超时, 多次重试失败后, 才知道不行.

4. 没有方便的跨平台的应用级消息发送方式.

针对这些问题, FMLS 的改进如下:

1. 使用公钥 (私钥) 作为一个设备的身份标识. 无论其 IP 地址 (以及端口号) 怎么变化,
   都能准确识别并连接.

   首次运行时, 在设备本地自动生成公钥/私钥 (OpenSSL, GPG), 证书也在本地签发,
   因此是去中心化的.

   结合设备自动发现技术 (mDNS/DNS-SD), 可以方便的找出目标设备的 IP 地址进行连接.

2. FMLS 使用公钥 (私钥) 来验证设备的身份, 使用签发证书来创建设备的信任域,
   使用安全的通信协议 (HTTPS, SSH) 进行数据传输. 从而提供较强的安全性.

3. FMLS 跟踪每个设备的在线状态, 需要路由转发时, 先计算路由是否可达.

4. FMLS 在每种平台, 都大约以系统服务的方式运行, 提供一致的连接及消息发送接口.
   基于 FMLS 的应用, 可以方便的实现跨设备跨平台的应用级通信.

### 网络性能要求

FMLS 假设其所在的局域网满足以下性能标准:

- 丢包率不超过 10% (也就是说, 单次传输尝试的成功率超过 90%)

- 端到端往返延迟 (RTT) 不超过 1000ms

FMLS 的协议, 实现等多方面, 会根据这些条件进行设计及优化.

---

详细文档请见 `doc/` 目录.

## 整体设计目标

FMLS 的整体设计目标如下:

1. **简单**: 设计上尽量简化, 因为要跨设备跨系统跨平台, 越简单的设计实现难度越小.
   尽量使用简单的技术, 降低开发难度, 提高开发速度.

   (比如 JSON 接口, sqlite 数据库, 命令行界面)

2. **自动化**: 尽量自动化的完成更多任务, 减少需要手动操作的部分.

   (比如 mDNS/DNS-SD)

3. **安全**: 尽量提高安全性, 使用安全的编程语言, 安全的网络协议,
   安全的密码算法等.

   (比如 rust, HTTPS (HTTP/3 QUIC), SSH, 椭圆曲线密码算法 (EC), sha256, AES-GCM)

4. **扩展接口**: 提供容易被别的程序使用的简单一致的接口, 方便开发基于 FMLS
   的应用.

5. **基于 web 技术**: web 技术在跨多平台方面, 相对是最好的,
   同时使用及开发都比较简单.

   (比如 deno, electron, tauri)

6. 优先使用 IPv6, 有线优先于无线.

## 相关代码仓库

- **fmls** (本仓库)

  FMLS 的主要文档, 以及核心基础代码. 编程语言: rust. LICENSE: LGPLv3+

  包含以下组件:

  - `libfmlsc`: (no_std) 公共基础库 (含有 libfmlsm 和 libfmls 共用的代码)

  - `libfmlsm`: (no_std) 用于 (r2) 低资源设备 (比如 单片机 MCU)

  - `libfmls`: 标准基础库 (含有大部分 FMLS 的代码)

  - `fmlsd`: 守护进程, 后台运行

  - `fmls-cli`: 命令行界面

- **fmls-ui**

  图形界面. 编程语言: js (typescript), rust. LICENSE: GPLv3+

  支持 GNU/Linux, Windows 等 PC 平台.

  主要依赖:

  - [deno](https://deno.land/) / [fresh](https://fresh.deno.dev/) (backend)

  - [electron](https://www.electronjs.org/) (GNU/Linux)

  - [tauri](https://tauri.app/) (Windows)

- **fmls-apk**

  Android 应用. 编程语言: kotlin. LICENSE: GPLv3+

  支持手机, 平板等设备.

  主要依赖:

  - WebView

## LICENSE

[GNU Lesser General Public License v3.0 or later](https://www.gnu.org/licenses/lgpl-3.0-standalone.html)
(SPDX Identifier: `LGPL-3.0-or-later`)
