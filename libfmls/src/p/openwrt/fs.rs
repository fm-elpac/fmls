use std::env::VarError;
use std::path::PathBuf;

use crate::fs::FileRoot;

/// 返回此平台的数据根目录默认值
pub fn get_dr(_sys: bool) -> Result<FileRoot, VarError> {
    let mut dr = PathBuf::new();
    let mut dr_etc = PathBuf::new();
    let mut dr_run = PathBuf::new();
    let mut dr_log = PathBuf::new();
    let mut dr_cache = PathBuf::new();

    // sys = true: OpenWrt 平台不支持用户实例
    dr.push("/srv/fmls");
    dr_etc.push("/etc/fmls");
    dr_run.push("/var/run/fmls");
    dr_log.push("/var/log/fmls");
    dr_cache.push("/var/cache/fmls");

    let dr2 = dr.clone();
    Ok(FileRoot {
        dr,
        dr2,
        dr_etc,
        dr_run,
        dr_log,
        dr_cache,
    })
}
