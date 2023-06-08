//! 以 socket 方式运行
//!
//! 监听 UNIX socket, 接受连接, 可同时响应多个连接

use std::process::ExitCode;

use log::info;

use super::运行参数;

/// 运行入口
pub fn 运行(a: 运行参数) -> Result<(), ExitCode> {
    // TODO
    info!("运行: daemon socket");

    // TODO
    println!("{:?}", a);

    Ok(())
}
