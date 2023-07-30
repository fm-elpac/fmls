# fmlsd 设计

## 线程

`fmlsd` 使用异步多线程 (async / await) 设计, 使用 [tokio](https://tokio.rs/) 库.

tokio 运行时会自动创建多个工作线程 (默认数量与 CPU 核心数相同),
并在这些线程上运行轻量级的异步任务 (task). 这些任务主要分为以下几类:

- **接口任务** (local api task, 主任务, 常驻)

  负责处理 本地应用接口.

  比如在相应 UNIX socket 监听, 接受并处理连接.

- **控制任务** (control task, 常驻)

  负责主要的 FMLS 协议及功能的实现.

- **服务任务** (server task, 常驻)

  运行 HTTP/3 QUIC 服务器. 接受并处理来自别的设备的连接.

  基于 `quiche`.

- **工作任务** (worker task)

  临时任务, 用于处理特定的功能.

  TODO

## 功能模块

主要模块划分如下:

- (libfmls) `dtl`: fmlsd 接口任务相关代码

- (libfmls) `dtc`: fmlsd 控制任务相关代码

- (libfmls) `dts`: fmlsd 服务任务相关代码

- (libfmls) `dtw`: fmlsd 工作任务相关代码

- (libfmls pub) `dr`: fmlsd 运行入口

- (libfmls pub) `json_api`: JSON 接口

- (libfmls pub) `fs`: 文件存储位置及文件格式

- (libfmls) `p`: 平台支持代码

- (libfmls) `aa`: 本地权限管理功能

- (libfmls) `fw`: 防火墙功能

- (fmlsd) `cea`: 命令行参数和环境变量处理

- (libfmls pub) `api`: 本地应用接口 (fmls-cli 使用)

- (libfmls pub) `api::st`: 状态查看

- (libfmls pub) `api::ca`: 证书签发

- (libfmls pub) `api::ne`: 邻居发现

- (libfmls pub) `api::info`: 节点自定义信息

- (libfmls pub) `api::nc`: 命名通道

- (libfmls pub) `api::tc`: 透明通道

## 命令行参数

- `--sys`

  以系统实例 (FMLSD-S) 运行, 否则以用户实例 (FMLSD-U) 运行.

- `--json-api`

  将本进程的 stdin/stdout 作为本地应用接口的 JSON 输入输出, 不再监听 socket.

- `--android`

  专用于 Android 系统, 本进程不再调用 Avahi, 由上级进程负责 mDNS/DNS-SD 功能.

## 环境变量

- `FMLSD=` 指定监听的本地应用接口地址

  如果不存在, 将使用默认值.

  比如 `FMLSD=/run/user/1000/fmls/fmlsd.s`

- `FMLSD_CONF=` 指定主配置文件路径

  如果不存在, 将使用默认值.

  比如 `FMLSD_CONF=/etc/fmls/fmlsd.conf.json`

- `FMLSD_PORT=` 指定服务器 (HTTP/3 QUIC) 监听特定的 UDP 端口号

  如果不存在, 将使用系统分配的端口 (随机).

  比如 `FMLSD_PORT=6660`

## 系统用户

在平台支持的情况下, 系统实例以专用的系统用户 (`fmlsd`) 运行.

## 参考资料

- <https://tokio.rs/>
- <https://github.com/cloudflare/quiche>
- <https://docs.quic.tech/quiche/>
