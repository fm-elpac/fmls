//! 文件存储位置 (文件路径)

use std::env;
use std::env::VarError;
use std::path::PathBuf;

use log::debug;

/// 数据根目录
#[derive(Debug, Clone)]
pub struct FileRoot {
    /// `FMLS_DR`
    pub dr: PathBuf,
    /// `FMLS_DR2`
    pub dr2: PathBuf,
    /// `FMLS_DR_ETC`
    pub dr_etc: PathBuf,
    /// `FMLS_DR_RUN`
    pub dr_run: PathBuf,
    /// `FMLS_DR_LOG`
    pub dr_log: PathBuf,
    /// `FMLS_DR_CACHE`
    pub dr_cache: PathBuf,
}

impl Default for FileRoot {
    /// 空值, 无实际用处
    fn default() -> Self {
        Self {
            dr: PathBuf::new(),
            dr2: PathBuf::new(),
            dr_etc: PathBuf::new(),
            dr_run: PathBuf::new(),
            dr_log: PathBuf::new(),
            dr_cache: PathBuf::new(),
        }
    }
}

impl FileRoot {
    /// 获取数据根目录, 需要访问环境变量
    ///
    /// `sys`: 系统实例 (否则为用户实例)
    pub fn new(sys: bool) -> Result<Self, VarError> {
        // 各操作系统 (平台) 的默认值
        let mut o = crate::p::fs::get_dr(sys)?;

        // 环境变量覆盖
        if let Ok(i) = env::var("FMLS_DR") {
            debug!("FMLS_DR={}", i);
            o.dr = PathBuf::from(i);
        }
        if let Ok(i) = env::var("FMLS_DR2") {
            debug!("FMLS_DR2={}", i);
            o.dr2 = PathBuf::from(i);
        }
        if let Ok(i) = env::var("FMLS_DR_ETC") {
            debug!("FMLS_DR_ETC={}", i);
            o.dr_etc = PathBuf::from(i);
        }
        if let Ok(i) = env::var("FMLS_DR_RUN") {
            debug!("FMLS_DR_RUN={}", i);
            o.dr_run = PathBuf::from(i);
        }
        if let Ok(i) = env::var("FMLS_DR_LOG") {
            debug!("FMLS_DR_LOG={}", i);
            o.dr_log = PathBuf::from(i);
        }
        if let Ok(i) = env::var("FMLS_DR_CACHE") {
            debug!("FMLS_DR_CACHE={}", i);
            o.dr_cache = PathBuf::from(i);
        }

        Ok(o)
    }
}

/// 单个文件的路径
#[derive(Debug, Clone)]
pub struct FilePath {
    pub root: FileRoot,
}

impl FilePath {
    pub fn new(root: FileRoot) -> Self {
        Self { root }
    }

    /// fmlsd 主配置文件 `fmlsd.conf.json`
    pub fn fmlsd_conf(&self) -> PathBuf {
        let mut p = self.root.dr_etc.clone();
        p.push("fmlsd.conf.json");
        p
    }

    /// 本地应用接口 `fmlsd.s`
    pub fn fmlsd_s(&self) -> PathBuf {
        let mut p = self.root.dr_run.clone();
        p.push("fmlsd.s");
        p
    }

    /// 私钥目录
    pub fn dir_secret(&self) -> PathBuf {
        let mut p = self.root.dr.clone();
        p.push("secret");
        p
    }

    /// CA 证书目录
    pub fn dir_ca(&self) -> PathBuf {
        let mut p = self.root.dr2.clone();
        p.push("ca");
        p
    }

    /// 自定义节点信息
    pub fn fmls_info(&self) -> PathBuf {
        let mut p = self.root.dr2.clone();
        p.push("fmls_info.json");
        p
    }

    /// 邻居信息目录
    pub fn dir_ne(&self) -> PathBuf {
        let mut p = self.root.dr2.clone();
        p.push("ne");
        p
    }

    /// 网络接口信息目录
    pub fn dir_ip_link(&self) -> PathBuf {
        let mut p = self.root.dr2.clone();
        p.push("ip_link");
        p
    }
}
