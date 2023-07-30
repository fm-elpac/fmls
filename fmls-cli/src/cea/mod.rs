//! 命令行参数和环境变量处理

use std::env;
use std::path::PathBuf;

use log::debug;

// 编译信息
mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

/// 显示版本信息
pub fn 版本() {
    let name = env!("CARGO_PKG_NAME");
    let v = env!("CARGO_PKG_VERSION");
    let target = built_info::TARGET;
    let features = built_info::FEATURES_LOWERCASE_STR;
    println!("{} version {} ({}, {})", name, v, target, features);

    // debug
    let git = env!("VERGEN_GIT_DESCRIBE");
    let profile = built_info::PROFILE;
    let time = env!("VERGEN_BUILD_TIMESTAMP");
    let rustc = built_info::RUSTC_VERSION;
    debug!("{} {} {}, {}", git, profile, time, rustc);
}

/// 显示帮助信息
pub fn 帮助() {
    let url = env!("CARGO_PKG_REPOSITORY");

    println!("命令行帮助信息:");
    debug!("TODO");
    // TODO
    println!("<{}>", url);
}

#[derive(Debug)]
pub struct 运行参数 {
    pub fmlsd: PathBuf,
    // TODO
}

impl Default for 运行参数 {
    fn default() -> Self {
        Self {
            fmlsd: PathBuf::new(),
        }
    }
}

/// 命令行参数和环境变量的处理结果
#[derive(Debug)]
pub enum Cea {
    /// 错误的命令行参数
    错误(String),

    /// 显示版本信息
    版本,

    /// 显示帮助信息
    帮助,

    /// 运行 fmlsd
    运行(运行参数),
}

impl Cea {
    pub fn 处理() -> Self {
        let mut 版本 = false;
        let mut 帮助 = false;

        let mut a = 运行参数::default();

        // 处理每个命令行参数
        for i in env::args().skip(1) {
            match i.as_str() {
                "--version" | "--版本" => {
                    版本 = true;
                }
                "--help" | "--帮助" => {
                    帮助 = true;
                }
                // TODO
                _ => {
                    return Self::错误(format!("未知的命令行参数 {}", i));
                }
            }
        }

        // 处理环境变量
        if let Ok(i) = env::var("FMLSD") {
            let p = i.trim();
            if p.len() > 0 {
                debug!("FMLSD={}", p);
                a.fmlsd = PathBuf::from(p);
            }
        }

        if 帮助 {
            Self::帮助
        } else if 版本 {
            Self::版本
        } else {
            Self::运行(a)
        }
    }
}
