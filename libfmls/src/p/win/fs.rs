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
        let program_data = env::var("ProgramData")?;
        dr.push(program_data.clone());
        dr.push("fmls");
        dr_etc.push(program_data.clone());
        dr_etc.push("fmls");
        dr_run.push(program_data.clone());
        dr_run.push("fmls");
        dr_log.push(program_data.clone());
        dr_log.push("fmls/log");
        dr_cache.push(program_data.clone());
        dr_cache.push("fmls/tmp");
    } else {
        let localappdata = env::var("LOCALAPPDATA")?;
        let tmp = env::var("TMP")?;
        dr.push(localappdata.clone());
        dr.push("fmls");
        dr_etc.push(localappdata.clone());
        dr_etc.push("fmls");
        dr_run.push(localappdata.clone());
        dr_run.push("fmls");
        dr_log.push(localappdata.clone());
        dr_log.push("fmls/log");
        dr_cache.push(tmp.clone());
        dr_cache.push("fmls");
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
