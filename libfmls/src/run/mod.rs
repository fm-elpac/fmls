//! 后台运行 (daemon)

use std::process::ExitCode;

mod json_api;
mod socket;

/// 运行参数
#[derive(Debug)]
pub struct 运行参数 {
    /// --json-api
    ///
    /// 这种运行模式, 将本进程的 stdin/stdout 作为 JSON 接口输入输出,
    /// 不再监听 socket
    pub json_api: bool,

    /// --android
    ///
    /// 这种运行模式, 专用于 Android 系统, 本进程不再调用 Avahi,
    /// 由上级进程负责 mDNS/DNS-SD 功能
    pub android: bool,

    /// --sys
    ///
    /// 以系统模式 (S) 运行,
    /// 否则以用户模式 (U) 运行
    pub sys: bool,

    /// --port 6666
    ///
    /// 指定监听的 UDP 端口号 (QUIC),
    /// 否则使用系统自动分配的随机端口号
    pub port: Option<u16>,
    // TODO
}

impl Default for 运行参数 {
    fn default() -> Self {
        Self {
            json_api: false,
            android: false,
            sys: false,
            port: None,
        }
    }
}

/// 运行入口
pub fn 运行(a: 运行参数) -> Result<(), ExitCode> {
    if a.json_api {
        json_api::运行(a)
    } else {
        socket::运行(a)
    }
}
