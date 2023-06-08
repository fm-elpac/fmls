#![deny(unsafe_code)]

use std::env;
use std::process::ExitCode;

use env_logger;

use libfmls::run::运行;

mod cla;

use cla::Cla;

fn main() -> Result<(), ExitCode> {
    // 默认日志级别为信息 (INFO)
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    // 解析命令行参数
    let a = Cla::解析(env::args().collect());
    match a {
        Cla::错误(d) => {
            eprintln!("错误: {}", d);
            Err(ExitCode::from(1))
        }
        Cla::版本 => {
            cla::版本();
            Ok(())
        }
        Cla::帮助 => {
            cla::帮助();
            Ok(())
        }
        Cla::运行(r) => 运行(r),
    }
}
