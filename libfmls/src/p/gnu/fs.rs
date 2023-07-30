use std::env;
use std::env::VarError;
use std::path::PathBuf;

use crate::fs::FileRoot;

/// 返回此平台的数据根目录默认值
pub fn get_dr(sys: bool) -> Result<FileRoot, VarError> {
    let mut dr = PathBuf::new();
    let mut dr_etc = PathBuf::new();
    let mut dr_run = PathBuf::new();
    let mut dr_log = PathBuf::new();
    let mut dr_cache = PathBuf::new();

    if sys {
        dr.push("/var/lib/fmls");
        dr_etc.push("/etc/fmls");
        dr_run.push("/run/fmls");
        dr_log.push("/var/log/fmls");
        dr_cache.push("/var/cache/fmls");
    } else {
        let home = env::var("HOME")?;
        let xdg_runtime_dir = env::var("XDG_RUNTIME_DIR")?;
        dr.push(home.clone());
        dr.push(".config/fmls");
        dr_etc.push(home.clone());
        dr_etc.push(".config/fmls");
        dr_run.push(xdg_runtime_dir.clone());
        dr_run.push("fmls");
        dr_log.push(home.clone());
        dr_log.push(".config/fmls/log");
        dr_cache.push(home.clone());
        dr_cache.push(".cache/fmls");
    }

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
