//! 用于 hex (16 进制数字文本) 和数字相互转换

use libfmlsc::r2c3p::BYTE_HEX;

/// 10 进制数字转换为 `u8`
pub fn n_u8(b: Vec<u8>) -> Option<u8> {
    if let Ok(s) = String::from_utf8(b) {
        if let Ok(u) = u8::from_str_radix(&s, 10) {
            return Some(u);
        }
    }
    None
}

/// hex 转为 `Vec<u8>`
pub fn hex_v(b: &[u8]) -> Option<Vec<u8>> {
    // 检查长度是否为 2 的倍数
    if (b.len() & 1) != 0 {
        return None;
    }
    let mut o: Vec<u8> = Vec::with_capacity(b.len() / 2);

    // 转换一个 hex 字符
    fn h(x: u8) -> Option<u8> {
        match x {
            b'0'..=b'9' => Some(x - b'0'),
            b'a'..=b'f' => Some(x - b'a' + 10),
            _ => None,
        }
    }

    let mut i = 0;
    while i < b.len() {
        let x = match h(b[i]) {
            Some(x) => x,
            None => {
                return None;
            }
        };
        let y = match h(b[i + 1]) {
            Some(y) => y,
            None => {
                return None;
            }
        };
        o.push((x << 4) | y);

        i += 2;
    }
    Some(o)
}

/// hex 转换为 `u8`, 如果出错, 返回 `None`
pub fn hex_u8(b: Vec<u8>) -> Option<u8> {
    if let Ok(s) = String::from_utf8(b) {
        if let Ok(u) = u8::from_str_radix(&s, 16) {
            return Some(u);
        }
    }
    None
}

/// hex 转换为 `u16`
pub fn hex_u16(b: Vec<u8>) -> Option<u16> {
    if let Ok(s) = String::from_utf8(b) {
        if let Ok(u) = u16::from_str_radix(&s, 16) {
            return Some(u);
        }
    }
    None
}

/// hex 转换为 `u32`
pub fn hex_u32(b: Vec<u8>) -> Option<u32> {
    if let Ok(s) = String::from_utf8(b) {
        if let Ok(u) = u32::from_str_radix(&s, 16) {
            return Some(u);
        }
    }
    None
}

/// hex 转换为 `u64`
pub fn hex_u64(b: Vec<u8>) -> Option<u64> {
    if let Ok(s) = String::from_utf8(b) {
        if let Ok(u) = u64::from_str_radix(&s, 16) {
            return Some(u);
        }
    }
    None
}

/// hex 转换为 `u128`
pub fn hex_u128(b: Vec<u8>) -> Option<u128> {
    if let Ok(s) = String::from_utf8(b) {
        if let Ok(u) = u128::from_str_radix(&s, 16) {
            return Some(u);
        }
    }
    None
}

/// `Vec<u8>` 转为 hex
pub fn v_hex(u: &[u8]) -> Vec<u8> {
    let mut o: Vec<u8> = Vec::with_capacity(u.len() * 2);
    for i in u {
        o.push(BYTE_HEX[((i & 0xf0) >> 4) as usize]);
        o.push(BYTE_HEX[(i & 0x0f) as usize]);
    }
    o
}

/// `u8` 转换为 hex
pub fn u8_hex(u: u8) -> Vec<u8> {
    Vec::from(format!("{:02x}", u))
}

/// `u16` 转换为 hex
pub fn u16_hex(u: u16) -> Vec<u8> {
    Vec::from(format!("{:04x}", u))
}

/// `u32` 转换为 hex
pub fn u32_hex(u: u32) -> Vec<u8> {
    Vec::from(format!("{:08x}", u))
}

/// `u64` 转换为 hex
pub fn u64_hex(u: u64) -> Vec<u8> {
    Vec::from(format!("{:016x}", u))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn hex_u() {
        assert_eq!(n_u8(Vec::from("12")), Some(12));
        assert_eq!(hex_u8(Vec::from("0a")), Some(0x0a));
        assert_eq!(hex_u16(Vec::from("0b0a")), Some(0x0b0a));
        assert_eq!(hex_u32(Vec::from("0c00000a")), Some(0x0c00_000a));
        assert_eq!(
            hex_u64(Vec::from("0d0000000000000a")),
            Some(0x0d00_0000_0000_000a)
        );
        assert_eq!(
            hex_u128(Vec::from("0e00000000000000000000000000000a")),
            Some(0x0e000000_00000000_00000000_0000000a)
        );
    }

    #[test]
    fn u_hex() {
        assert_eq!(u8_hex(10), Vec::from("0a"));
        assert_eq!(u16_hex(11), Vec::from("000b"));
        assert_eq!(u32_hex(12), Vec::from("0000000c"));
        assert_eq!(u64_hex(13), Vec::from("000000000000000d"));
    }

    #[test]
    fn test_v_hex() {
        assert_eq!(v_hex(&vec![0x01, 0x20, 0x38, 0x0a, 0xcd]), b"0120380acd");

        assert_eq!(hex_v(&vec![0x01, 0x20, 0x00]), None);
        assert_eq!(
            hex_v(&Vec::from(b"45679bef" as &[u8])),
            Some(vec![0x45, 0x67, 0x9b, 0xef])
        );
    }
}
