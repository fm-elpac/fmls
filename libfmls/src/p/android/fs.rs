use std::env::VarError;
use std::path::PathBuf;

use crate::fs::FileRoot;

/// 返回此平台的数据根目录默认值
pub fn get_dr(_sys: bool) -> Result<FileRoot, VarError> {
    let mut dr = PathBuf::new();
    let mut dr2 = PathBuf::new();
    let mut dr_etc = PathBuf::new();
    let mut dr_run = PathBuf::new();
    let mut dr_log = PathBuf::new();
    let mut dr_cache = PathBuf::new();

    // sys = false: Android 平台不支持系统实例
    // 此处为默认值, 如需要更改, 请用环境变量覆盖
    dr.push("/data/data/org.fm_elpac.fmls_apk/files/fmls");
    dr2.push("/storage/emulated/0/Android/data/org.fm_elpac.fmls_apk/files/fmls");
    dr_etc.push("/storage/emulated/0/Android/data/org.fm_elpac.fmls_apk/files/fmls");
    dr_run.push("/data/data/org.fm_elpac.fmls_apk/files/fmls_run");
    dr_log.push("/storage/emulated/0/Android/data/org.fm_elpac.fmls_apk/files/fmls/log");
    dr_cache.push("/storage/emulated/0/Android/data/org.fm_elpac.fmls_apk/cache/fmls");

    Ok(FileRoot {
        dr,
        dr2,
        dr_etc,
        dr_run,
        dr_log,
        dr_cache,
    })
}
