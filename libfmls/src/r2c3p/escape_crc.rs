//! 消息转义处理及 crc 计算
//!
//! 适用于 UART 方式传输

use crc::Crc;
use crc32fast;

use libfmlsc::r2c3p::{BYTE_B, BYTE_LF, BYTE_N, BYTE_S, CRC_16, MSG_LEN_CRC16};

/// 转义处理
///
/// 用于发送消息之前
pub fn escape(b: &[u8]) -> Vec<u8> {
    let mut o: Vec<u8> = Vec::with_capacity(b.len() + 8);
    for i in b {
        match *i {
            BYTE_LF => {
                o.push(BYTE_B);
                o.push(BYTE_N);
            }
            BYTE_B => {
                o.push(BYTE_B);
                o.push(BYTE_S);
            }
            _ => {
                o.push(*i);
            }
        }
    }
    o
}

/// 取消转义处理
///
/// 用于接收消息之后
/// 如果错误, 返回 None
pub fn unescape(b: &[u8]) -> Option<Vec<u8>> {
    let mut o: Vec<u8> = Vec::with_capacity(b.len());
    // 转义模式标志
    let mut ef = false;
    for i in b {
        if ef {
            // 当前是转义模式
            match *i {
                // 表示 `\n` 字节
                BYTE_N => {
                    o.push(BYTE_LF);
                }
                // 表示 `\\` 字节
                BYTE_S => {
                    o.push(BYTE_B);
                }
                // 转义错误
                _ => {
                    return None;
                }
            }
            // 退出转义模式
            ef = false;
        } else {
            // 普通模式
            match *i {
                // 直接遇到 `\n` 字节是错误
                BYTE_LF => {
                    return None;
                }
                // 进入转义模式
                BYTE_B => {
                    ef = true;
                }
                // 无需处理
                _ => {
                    o.push(*i);
                }
            }
        }
    }
    Some(o)
}

/// 计算 crc32
pub fn crc32(b: &[u8]) -> u32 {
    crc32fast::hash(b)
}

const C16: Crc<u16> = Crc::<u16>::new(&CRC_16);

/// 计算 crc16
pub fn crc16(b: &[u8]) -> u16 {
    C16.checksum(b)
}

/// 发送消息前计算 CRC
///
/// 根据消息长度选择 crc16/crc32 (或强制使用 crc32)
/// 返回的附加 CRC 数据以小尾字节序表示 (LE)
pub fn crc_send(b: &[u8], use_crc32: bool) -> Vec<u8> {
    // 强制使用 crc32, 或消息长度不超过 32 字节
    if use_crc32 || b.len() > MSG_LEN_CRC16 as usize {
        // 使用 crc32
        let c = crc32(b);
        Vec::from(c.to_le_bytes())
    } else {
        // 使用 crc16
        let c = crc16(b);
        Vec::from(c.to_le_bytes())
    }
}

/// 接收消息后检查 crc 是否正确
///
/// 根据消息长度选择 crc16/crc32 (或强制使用 crc32)
/// 返回 None 表示错误
/// 返回 Some(2) Some(4) 表示 crc 长度
pub fn crc_check(b: &[u8], use_crc32: bool) -> Option<usize> {
    // 接收的消息至少 3 字节长 (1 字节数据 + crc16)
    if b.len() < 3 {
        return None;
    }

    // | 发送时的消息长度 | 使用的 crc 类型 | 添加 crc 后的长度 |
    // |  1 | crc16 |  3 |
    // | 31 | crc16 | 33 |
    // | 32 | crc16 | 34 |
    // | 33 | crc32 | 37 |
    // | 34 | crc32 | 38 |
    //
    // 所以消息长度 35, 36 是错误消息
    let u32b = std::mem::size_of::<u32>();
    if use_crc32 || b.len() > (MSG_LEN_CRC16 as usize + u32b) {
        // 使用 crc32
        let cb = &b[(b.len() - u32b)..];
        let c1 = u32::from_le_bytes(cb.try_into().unwrap());
        let c2 = crc32(&b[..(b.len() - 4)]);
        return if c1 == c2 { Some(u32b) } else { None };
    }

    let u16b = std::mem::size_of::<u16>();
    if b.len() <= (MSG_LEN_CRC16 as usize + u16b) {
        // 使用 crc16
        let cb = &b[(b.len() - u16b)..];
        let c1 = u16::from_le_bytes(cb.try_into().unwrap());
        let c2 = crc16(&b[..(b.len() - 2)]);
        return if c1 == c2 { Some(u16b) } else { None };
    }
    // crc 检查错误
    None
}

#[cfg(test)]
mod test {
    use libfmlsc::r2c3p::{CRC_32, P_VERSION};

    use super::*;

    #[test]
    fn test_crc32fast() {
        // `crc` 和 `crc32fast` 应该计算出相同的结果
        let c = Crc::<u32>::new(&CRC_32);
        let c1 = c.checksum(P_VERSION);

        let c2 = crc32fast::hash(P_VERSION);
        assert_eq!(c1, c2);
    }

    #[test]
    fn test_escape() {
        // 普通数据
        assert_eq!(escape(b"123 abc"), b"123 abc");

        // 需要转义
        assert_eq!(escape(b"1\n2\\3"), b"1\\n2\\s3");
    }

    #[test]
    fn test_unescape() {
        // 普通数据
        assert_eq!(unescape(b"abc 123"), Some(Vec::from(b"abc 123" as &[u8])));

        // 转义处理
        assert_eq!(unescape(b"1\\n2\\s3"), Some(Vec::from(b"1\n2\\3" as &[u8])));
    }

    #[test]
    fn test_crc_send() {
        // 检查生成的 crc 长度 (crc16/crc32)
        assert_eq!(crc_send(b"v", false).len(), 2);
        // 强制使用 crc32
        assert_eq!(crc_send(b"V", true).len(), 4);

        let mut b: Vec<u8> = Vec::new();

        b.resize(31, 0);
        assert_eq!(crc_send(&b, false).len(), 2);

        b.resize(32, 0);
        assert_eq!(crc_send(&b, false).len(), 2);

        b.resize(33, 0);
        assert_eq!(crc_send(&b, false).len(), 4);

        b.resize(34, 0);
        assert_eq!(crc_send(&b, false).len(), 4);

        // 检查生成的 crc 值
        assert_eq!(crc_send(b"v", false), [0x81, 0xe6]);

        let mut b: Vec<u8> = Vec::new();
        b.resize(33, b'V');
        assert_eq!(crc_send(&b, false), [0x9e, 0xad, 0xc7, 0x14]);
    }

    #[test]
    fn test_crc_check() {
        // 检查输入数据长度
        let mut b: Vec<u8> = Vec::new();
        b.resize(1, 0);
        assert_eq!(crc_check(&b, false), None);
        b.resize(2, 0);
        assert_eq!(crc_check(&b, false), None);

        b.resize(35, 0);
        assert_eq!(crc_check(&b, false), None);
        b.resize(36, 0);
        assert_eq!(crc_check(&b, false), None);

        // 检查 crc16
        let mut b: Vec<u8> = vec![b'v', 0x81, 0xe6];
        assert_eq!(crc_check(&b, false), Some(2));
        b[2] = 0; // 模拟数据传输出错
        assert_eq!(crc_check(&b, false), None);
        // 检查 crc32
        let mut b: Vec<u8> = Vec::new();
        b.resize(33, b'V');
        b.push(0x9e);
        b.push(0xad);
        b.push(0xc7);
        b.push(0x14);
        assert_eq!(crc_check(&b, false), Some(4));
        b.push(0);
        assert_eq!(crc_check(&b, false), None);
    }
}
