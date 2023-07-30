#![deny(unsafe_code)]
//! # `libfmls`
//!
//! FMLS 标准基础库
//!
//! <https://github.com/fm-elpac/fmls>

pub mod api;
pub mod dr;
pub mod fs;
pub mod json_api;

pub mod r2c3p;

mod p;

// TODO not pub mod
pub mod aa;
pub mod dtc;
pub mod dtl;
pub mod dts;
pub mod dtw;
pub mod fw;
