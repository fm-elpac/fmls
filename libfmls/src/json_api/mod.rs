//! JSON 外部接口
//!
//! 定义类型和数据结构 (JSON 消息的格式)
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use serde::{Deserialize, Serialize};

/// 格式: `ok`
///
/// 无附加数据, 用于表示成功
pub const t_ok: &'static str = "ok";

/// 格式: `s`
///
/// 数据为单个字符串 (String)
pub const t_s: &'static str = "s";

pub type s_s = String;

/// 格式: `E`
///
/// 用于返回错误信息
pub const t_E: &'static str = "E";

#[derive(Serialize, Deserialize, Debug)]
pub struct s_E {
    /// 错误代码
    c: i32,

    /// 错误描述 (文本)
    d: String,
}

// 预定义错误代码

/// 未知的格式类型
pub const E_1: i32 = -1;
/// JSON 格式错误
pub const E_2: i32 = -2;

/// 用于读取版本信息
pub const t_version: &'static str = "version";

#[derive(Serialize, Deserialize, Debug)]
pub struct s_version {
    /// FMLS 的版本, 比如 0.1.0
    fmls: String,

    /// 详细编译信息
    build: String,
}

/// 用于读取许可信息
pub const t_license: &'static str = "license";

#[derive(Serialize, Deserialize, Debug)]
pub struct s_license {
    /// SPDX Identifier: LGPL-3.0-or-later
    license_spdx: String,

    /// 项目网址
    url: String,
}

pub const L_SPDX: &'static str = "LGPL-3.0-or-later";
pub const L_URL: &'static str = "https://github.com/fm-elpac/fmls";

// TODO
