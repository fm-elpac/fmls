/// 传输质量监测计数器
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ConfC {
    /// `cT` 总计发送的消息数量
    pub t: u32,
    /// `cR` 总计成功接收的消息数量
    pub r: u32,
    /// `cRd` 总计丢弃的接收的消息数量
    pub rd: u32,
    /// `cTB` 总计发送的字节数
    pub tb: u32,
    /// `cRB` 总计成功接收的字节数
    pub rb: u32,
}
