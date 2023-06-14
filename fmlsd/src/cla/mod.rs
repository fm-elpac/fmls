//! 命令行参数解析

use libfmls::run::运行参数;

/// 命令行参数的解析结果
#[derive(Debug)]
pub enum Cla {
    /// 错误的命令行参数
    错误(String),

    /// 显示版本信息
    版本,

    /// 显示帮助信息
    帮助,

    /// 普通运行模式
    运行(运行参数),
}

impl Cla {
    pub fn 解析(a: Vec<String>) -> Self {
        // TODO 支持更多命令行参数的解析
        match a.len() {
            // 当没有传递命令行参数的时候, 有一个参数 (程序本身)
            1 => Self::运行(运行参数::default()),

            2 => match a[1].as_str() {
                "--version" => Self::版本,
                "--help" => Self::帮助,

                "--json-api" => {
                    let mut a = 运行参数::default();
                    a.json_api = true;

                    Self::运行(a)
                }
                "--android" => {
                    let mut a = 运行参数::default();
                    a.android = true;

                    Self::运行(a)
                }
                "--sys" => {
                    let mut a = 运行参数::default();
                    a.sys = true;

                    Self::运行(a)
                }
                // TODO --port
                _ => Self::错误(format!("未知的参数 {}", a[1])),
            },

            _ => Self::错误("参数格式错误".to_string()),
        }
    }
}

/// 显示版本信息
pub fn 版本() {
    let v = env!("CARGO_PKG_VERSION");
    println!("fmlsd version {}", v);
}

/// 显示帮助信息
pub fn 帮助() {
    println!("命令行帮助信息:");
    println!("TODO");
    // TODO
}
