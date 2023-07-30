#![deny(unsafe_code)]

use std::process::ExitCode;

use env_logger;

use libfmls::dr::运行;

mod cea;

use cea::Cea;

fn main() -> Result<(), ExitCode> {
    // 默认日志级别为信息 (INFO)
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    // 处理启动参数
    let a = Cea::处理();
    match a {
        Cea::错误(d) => {
            eprintln!("错误: {}", d);
            Err(ExitCode::from(1))
        }
        Cea::版本 => {
            cea::版本();
            Ok(())
        }
        Cea::帮助 => {
            cea::帮助();
            Ok(())
        }
        Cea::运行(r) => {
            运行(r).unwrap();
            Ok(())
        }
    }
}
