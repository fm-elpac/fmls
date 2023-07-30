//! fmlsd 运行入口

use std::error::Error;
use std::path::PathBuf;

use log::debug;
use tokio::runtime::Runtime;

use crate::fs::FileRoot;

/// fmlsd 运行参数
#[derive(Debug)]
pub struct 运行参数 {
    /// 运行模式: `--json-api`
    pub m_json_api: bool,

    /// 运行模式: `--android`
    pub m_android: bool,

    /// 运行模式: `--sys`
    pub m_sys: bool,

    /// QUIC 服务器监听的 UDP 端口号
    pub port: Option<u16>,

    /// 数据根目录
    pub fr: FileRoot,

    /// 指定监听的本地应用接口地址
    pub fmlsd: PathBuf,
    // TODO 配置文件
}

impl Default for 运行参数 {
    fn default() -> Self {
        Self {
            m_json_api: false,
            m_android: false,
            m_sys: false,
            fmlsd: PathBuf::new(),
            port: None,
            fr: FileRoot::default(),
        }
    }
}

/// tokio 运行入口
async fn 异步运行(a: 运行参数) -> Result<(), Box<dyn Error>> {
    debug!("tokio {:?}", a);
    // TODO

    Ok(())
}

/// 运行入口
pub fn 运行(a: 运行参数) -> Result<(), Box<dyn Error>> {
    // 创建 tokio 运行时
    let rt = Runtime::new()?;

    rt.block_on(异步运行(a))
}
