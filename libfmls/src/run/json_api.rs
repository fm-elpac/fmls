//! 以 --json-api 方式运行
//!
//! 从 stdin 读取 JSON 消息, 结果返回 stdout

use std::process::ExitCode;

use log::info;

use super::运行参数;

/// 运行入口
pub fn 运行(a: 运行参数) -> Result<(), ExitCode> {
    // TODO
    info!("运行: --json-api");

    // TODO
    println!("{:?}", a);

    Ok(())
}
