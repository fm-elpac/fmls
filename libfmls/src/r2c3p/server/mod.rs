//! 封装的接口

use libfmlsc::r2c3p::{BYTE_LF, MSGT_V, MSGT_V_R};

use super::escape_crc::{crc_check, crc_send, escape, unescape};
use super::msg::MsgO;
use super::{Msg, MsgRecvErr, MsgReq, MsgRes};

/// 传输质量监测计数器
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TC {
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

/// 喂给 (feed) 字节数据 (`Vec<u8>`) 后产出的一条结果
#[derive(Debug, Clone, PartialEq)]
pub enum FeedResult {
    /// 消息接收错误
    E(MsgRecvErr),
    /// 成功接收了一条消息
    M(Msg),
}

/// fmls_r2c3p 协议 (r1) 强大的设备的实现
///
/// 纯状态机, 需要外部处理输入输出
/// 一个实例只能处理一个 r2c3p 连接
#[derive(Debug, Clone)]
pub struct R2c3pServer {
    /// 传输质量监测计数器
    tc: TC,

    /// 内部缓冲区, 存储还未完整接收的消息数据
    b: Option<Vec<u8>>,
}

// TODO 支持 USB

impl R2c3pServer {
    pub fn new() -> Self {
        Self {
            tc: TC::default(),
            b: None,
        }
    }

    /// 获取传输质量监测计数器的当前值
    pub fn get_tc(&self) -> TC {
        self.tc.clone()
    }

    /// 获取内部缓冲区的大小
    pub fn get_b_size(&self) -> usize {
        match &self.b {
            Some(b) => b.len(),
            None => 0,
        }
    }

    /// 喂给通过 UART 接收到的原始字节数据, 可随意投喂
    /// 会切分消息并处理, 在内部缓存没有完整接收的消息
    pub fn feed(&mut self, b: Vec<u8>) -> Vec<FeedResult> {
        let (m, r) = scan_b(&b);
        let mut o: Vec<FeedResult> = Vec::new();

        // 处理第一条消息
        if m.len() > 0 {
            let b = match self.b.take() {
                Some(mut d) => {
                    // 拼接缓冲数据
                    d.extend_from_slice(m[0]);
                    d
                }
                None => Vec::from(m[0]),
            };
            let f = self.parse_msg(&b);
            self.update_tc_fr(&f, b.len());
            o.push(f);
        }

        // 处理剩余消息
        for i in m.iter().skip(1) {
            let f = self.parse_msg(i);
            self.update_tc_fr(&f, i.len());
            o.push(f);
        }

        // 处理剩余数据
        if let Some(d) = r {
            // 将剩余数据存入内部缓冲区
            self.b = Some(Vec::from(d));
        }
        o
    }

    /// 更新传输质量监测计数器 (接收)
    fn update_tc_fr(&mut self, f: &FeedResult, b: usize) {
        match f {
            FeedResult::M(_) => {
                self.tc.r = self.tc.r.wrapping_add(1);
                self.tc.rb = self.tc.rb.wrapping_add(b as u32);
            }
            FeedResult::E(_) => {
                self.tc.rd = self.tc.rd.wrapping_add(1);
            }
        }
    }

    /// 发送一条消息, 返回需要通过 UART 发送的字节数据
    pub fn send(&mut self, m: Msg) -> Vec<u8> {
        let use_crc32 = if let Msg::Res(MsgRes::V { .. }) = m {
            // `V` 消息强制使用 crc32
            true
        } else {
            false
        };
        let add_crc = if let Msg::Req(MsgReq::Vv) = m {
            // `vv` 消息不添加 crc
            false
        } else {
            true
        };
        // 消息转为字节数据
        let mut b: Vec<u8> = m.into();
        // 计算并添加 crc
        if add_crc {
            let c = crc_send(&b, use_crc32);
            b.extend_from_slice(&c);
        }

        // 转义处理
        let mut o = escape(&b);
        // 添加消息结束字节
        o.push(BYTE_LF);

        // 更新传输质量监测计数器
        self.tc.t = self.tc.t.wrapping_add(1);
        self.tc.tb = self.tc.tb.wrapping_add(o.len() as u32);
        o
    }

    /// 默认处理方式, 输入剩下的 FeedResult (应用不处理)
    /// 返回需要通过 UART 发送的字节数据
    pub fn eat(&mut self, f: Vec<FeedResult>) -> Vec<Vec<u8>> {
        let mut o: Vec<Vec<u8>> = Vec::new();
        for i in f {
            if let Some(v) = self.eat_one(i) {
                o.push(v);
            }
        }
        o
    }

    /// 处理一条消息 (通过 UART 接收的原始字节数据)
    fn parse_msg(&mut self, b: &[u8]) -> FeedResult {
        // 检查消息长度
        if b.len() < 2 {
            return FeedResult::E(MsgRecvErr::E4(Vec::from(b)));
        }
        if b.len() == 2 {
            if b[0] == MSGT_V_R && b[1] == MSGT_V_R {
                // `vv` 消息
                return FeedResult::M(Msg::Req(MsgReq::V));
            } else {
                return FeedResult::E(MsgRecvErr::E4(Vec::from(b)));
            }
        }
        // b.len() > 2

        // 取消转义处理
        let v = match unescape(b) {
            Some(v) => v,
            None => {
                return FeedResult::E(MsgRecvErr::E4(Vec::from(b)));
            }
        };

        // 计算 crc
        let use_crc32 = v[0] == MSGT_V;
        let c = match crc_check(&v, use_crc32) {
            Some(i) => i,
            None => {
                return FeedResult::E(MsgRecvErr::E4(Vec::from(b)));
            }
        };

        // 检查通过, 解析字节数据
        let d = Vec::from(&v[..(v.len() - c)]);
        let o: MsgO = d.clone().into();
        match o.0 {
            Some(m) => FeedResult::M(m),
            None => FeedResult::E(MsgRecvErr::E4(d)),
        }
    }

    /// 一次默认处理
    fn eat_one(&mut self, _f: FeedResult) -> Option<Vec<u8>> {
        // TODO
        None
    }
}

/// 扫描原始字节数据, 切分每条消息
fn scan_b(b: &[u8]) -> (Vec<&[u8]>, Option<&[u8]>) {
    let mut o: Vec<&[u8]> = Vec::new();
    // 剩余的待处理数据
    let mut r: &[u8] = &b[..];

    // 根据消息结束字节切分每条消息
    while let Some(i) = r.iter().position(|n| BYTE_LF == *n) {
        o.push(&r[..i]);

        // 更新剩余的待处理数据
        r = &r[(i + 1)..];
    }

    // 检查剩余的数据
    let rest = if r.len() > 0 { Some(r) } else { None };
    (o, rest)
}

#[cfg(test)]
mod test;
