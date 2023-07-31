//! `fmls_r2c3p` 协议 (底层常量定义)

/// crc16 参数: [`crc_catalog::CRC_16_ARC`]
pub use crc_catalog::CRC_16_ARC as CRC_16;
/// crc32 参数: [`crc_catalog::CRC_32_ISO_HDLC`]
pub use crc_catalog::CRC_32_ISO_HDLC as CRC_32;

/// `fmls_r2c3p` 协议版本号
pub const P_VERSION: &[u8; 16] = b"fmls_r2c3p 0.1.0";

/// 特殊字节 0x0a `\n`
pub const BYTE_LF: u8 = b'\n';
/// 特殊字节 0x5c `\\`
pub const BYTE_B: u8 = b'\\';
/// 特殊字节 0x6e `n`
pub const BYTE_N: u8 = b'n';
/// 特殊字节 0x73 `s`
pub const BYTE_S: u8 = b's';
/// 特殊字节 0x20 空格 ` `
pub const BYTE_SPACE: u8 = b' ';
/// 特殊字节 0x3d `=`
pub const BYTE_EQ: u8 = b'=';

/// 用于 16 进制数字文本 (hex 编解码)
pub const BYTE_HEX: &[u8; 16] = b"0123456789abcdef";

/// 消息类型 请求消息 开始 `a`
pub const MSGT_REQ_S: u8 = b'a';
/// 消息类型 请求消息 结束 `z`
pub const MSGT_REQ_E: u8 = b'z';
/// 消息类型 响应消息 开始 `A`
pub const MSGT_RES_S: u8 = b'A';
/// 消息类型 响应消息 结束 `Z`
pub const MSGT_RES_E: u8 = b'Z';

/// 消息长度限制: 一条消息必须在 900ms 以内发送完毕
pub const MSG_LEN_MS: u32 = 900;
/// 使用 crc16 的最大消息长度 (32 字节)
pub const MSG_LEN_CRC16: u32 = 32;
/// UART 方式建议的最大消息长度 (128 字节)
pub const MSG_LEN_UART: u32 = 128;
/// `V` 消息总长度不应超过 256 字节
pub const MSG_LEN_V: u32 = 256;
/// 发送请求消息后, 1 秒内如果未收到响应消息, 则认为发送失败
pub const MSG_REQ_MS: u32 = 1000;
/// 重试 3 次仍然失败, 不再继续发送
pub const MSG_REQ_RETRY: u32 = 3;
/// 单片机至少能够接收长度 8 字节 (不含 CRC) 的消息
pub const MCU_BUFFER_LEN_MIN: u32 = 8;

/// 预定义的消息类型 请求消息 `v` 0x76
pub const MSGT_V_R: u8 = b'v';
/// 预定义的消息类型 `V` 0x56
pub const MSGT_V: u8 = b'V';
/// 预定义的消息类型 `E` 0x45
pub const MSGT_E: u8 = b'E';
/// 预定义的消息类型 `K` 0x4b
pub const MSGT_K: u8 = b'K';
/// 预定义的消息类型 请求消息 `c` 0x63
pub const MSGT_C_R: u8 = b'c';
/// 预定义的消息类型 `C` 0x43
pub const MSGT_C: u8 = b'C';
/// 预定义的消息类型 静默消息 `@` 0x40
pub const MSGT_AT: u8 = b'@';

/// 预定义的错误码 `E-1`
pub const E_1: i8 = -1;
/// 错误码的字节表示形式 (-1), 下同
pub const EB_1: &[u8; 2] = b"-1";
/// 预定义的错误码 `E-2`
pub const E_2: i8 = -2;
pub const EB_2: &[u8; 2] = b"-2";
/// 预定义的错误码 `E-3`
pub const E_3: i8 = -3;
pub const EB_3: &[u8; 2] = b"-3";
/// 预定义的错误码 `E-4`
pub const E_4: i8 = -4;
pub const EB_4: &[u8; 2] = b"-4";
/// 预定义的错误码 `E-5`
pub const E_5: i8 = -5;
pub const EB_5: &[u8; 2] = b"-5";

/// 预定义的配置 `m`
pub const CONF_M_1: &[u8; 1] = b"m";
/// 预定义的配置 `T`
pub const CONF_T: &[u8; 1] = b"T";
/// 预定义的配置 `t`
pub const CONF_T_1: &[u8; 1] = b"t";
/// 预定义的配置 `cT`
pub const CONF_CT: &[u8; 2] = b"cT";
/// 预定义的配置 `cR`
pub const CONF_CR: &[u8; 2] = b"cR";
/// 预定义的配置 `cRd`
pub const CONF_CRD: &[u8; 3] = b"cRd";
/// 预定义的配置 `cTB`
pub const CONF_CTB: &[u8; 3] = b"cTB";
/// 预定义的配置 `cRB`
pub const CONF_CRB: &[u8; 3] = b"cRB";
/// 预定义的配置 `I`
pub const CONF_I: &[u8; 1] = b"I";
/// 预定义的配置 `O`
pub const CONF_O: &[u8; 1] = b"O";
/// 预定义的配置 `On`
pub const CONF_ON: &[u8; 2] = b"On";
/// 预定义的配置 `@`
pub const CONF_AT: &[u8; 1] = b"@";
/// 预定义的配置 `@s`N
pub const CONF_ATS: &[u8; 2] = b"@s";
/// 预定义的配置 `@n`N
pub const CONF_ATN: &[u8; 2] = b"@n";

/// 下层协议参数 UART 默认波特率 (9600)
pub const PA_UART_BR: u32 = 9600;

/// r2c3p over TLS/TCP (mDNS/DNS-SD) 协议类型
pub const R2C3P_TLSTCP_SD_TYPE: &[u8; 15] = b"_r2c3p_tls._tcp";
/// r2c3p over TLS/TCP (mDNS/DNS-SD) TXT 标记
pub const R2C3P_TLSTCP_SD_T: &[u8; 13] = b"r2c3p=tls/tcp";

/// r2 集线器: 建议开始分配的节点号 (0x31)
pub const R2_HUB_NID_S: u8 = b'1';

#[cfg(test)]
mod test;
