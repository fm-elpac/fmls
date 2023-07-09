# fmlsd 设计

## 线程

`fmlsd` 使用多线程设计.

- **接口线程** (local api thread, 主线程, 常驻)

  负责处理 本地应用接口.

  比如在相应 UNIX socket 监听, 接受并处理连接.

  基于 `tokio`, 异步 IO.

- **控制线程** (control thread, 常驻)

  负责主要的 FMLS 协议及功能的实现.

  基于 `tokio`, 异步消息驱动.

- **服务线程** (server thread, 常驻)

  运行 HTTP/3 QUIC 服务器. 接受并处理来自别的设备的连接.

  基于 `quiche`.

- **工作线程** (worker thread)

  临时线程, 用于处理特定的加速任务.

  TODO

## 功能模块

主要模块划分如下:

- (libfmls) `dlt`: fmlsd 接口线程相关代码

- (libfmls) `dct`: fmlsd 控制线程相关代码

- (libfmls) `dst`: fmlsd 服务线程相关代码

- (libfmls) `dwt`: fmlsd 工作线程相关代码

- (libfmls) `dr`: fmlsd 运行入口

- (libfmls) `json_api`: JSON 接口

- (libfmls) `f`: 文件存储位置及文件格式

- (libfmls) `p`: 平台支持代码

- (fmlsd) `cea`: 命令行参数和环境变量处理

- (libfmls) `api`: 本地应用接口 (fmls-cli 使用)

- (libfmls) `api::st`: 状态查看

- (libfmls) `api::ca`: 证书签发

- (libfmls) `api::ne`: 邻居发现

- (libfmls) `api::in`: 节点自定义信息

- (libfmls) `api::nc`: 命名通道

- (libfmls) `api::tc`: 透明通道

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
