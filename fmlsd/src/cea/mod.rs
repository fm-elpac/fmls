//! 命令行参数和环境变量处理

use std::env;
use std::path::PathBuf;

use log::debug;

use libfmls::dr::运行参数;
use libfmls::fs::{FilePath, FileRoot};

// TODO not pub mod
pub mod conf;

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

                "--json-api" => {
                    a.m_json_api = true;
                }
                "--android" => {
                    a.m_android = true;
                }
                "--sys" => {
                    a.m_sys = true;
                }

                _ => {
                    return Self::错误(format!("未知的命令行参数 {}", i));
                }
            }
        }

        // 获取数据根目录 (文件相关)
        a.fr = FileRoot::new(a.m_sys).unwrap();
        // fmlsd 默认值
        let fp = FilePath::new(a.fr.clone());
        a.fmlsd = fp.fmlsd_s();

        // 处理环境变量
        if let Ok(i) = env::var("FMLSD") {
            let p = i.trim();
            if p.len() > 0 {
                debug!("FMLSD={}", p);
                a.fmlsd = PathBuf::from(p);
            }
        }
        if let Ok(i) = env::var("FMLSD_PORT") {
            let p = i.trim();
            if p.len() > 0 {
                debug!("FMLSD_PORT={}", p);
                match u16::from_str_radix(p, 10) {
                    Ok(n) => {
                        a.port = Some(n);
                    }
                    _ => {
                        return Self::错误(format!("环境变量 FMLSD_PORT 无法解析的数字 {}", p));
                    }
                }
            }
        }

        // TODO 处理配置文件
        if let Ok(i) = env::var("FMLSD_CONF") {
            let p = i.trim();
            if p.len() > 0 {
                debug!("加载配置文件 {}", p);
                // TODO
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
