//! 后台运行 (daemon)

use std::process::ExitCode;

mod json_api;
mod socket;

/// 运行参数
#[derive(Debug)]
pub struct 运行参数 {
    /// --json-api
    pub json_api: bool,

    /// --android
    pub android: bool,
    // TODO
}

/// 运行入口
pub fn 运行(a: 运行参数) -> Result<(), ExitCode> {
    if a.json_api {
        json_api::运行(a)
    } else {
        socket::运行(a)
    }
}
