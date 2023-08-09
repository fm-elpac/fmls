//! 字节处理工具

/// 先进先出缓冲
#[derive(Debug, Clone)]
pub struct Fifo<const N: usize> {
    // 内部缓冲区
    b: [u8; N],
    // 当前位置
    i: usize,
}

impl<const N: usize> Fifo<N> {
    pub fn new() -> Self {
        Self { b: [0; N], i: 0 }
    }

    // 返回下一个 i
    fn next_i(mut i: usize) -> usize {
        i += 1;
        if i >= N {
            i = 0;
        }
        i
    }

    /// 一次喂入一个字节, 返回前 N 个字节
    pub fn feed(&mut self, b: u8) -> u8 {
        // 前 N 个字节
        let n = self.b[self.i];
        // 保存输入的字节
        self.b[self.i] = b;
        // 更新当前位置
        self.i = Fifo::<N>::next_i(self.i);

        n
    }

    /// 返回缓存的数据
    pub fn get(&self) -> [u8; N] {
        let mut o: [u8; N] = [0; N];
        // 保存当前位置
        let mut i = self.i;
        let mut j = 0;
        while j < N {
            o[j] = self.b[i];
            i = Fifo::<N>::next_i(i);
            j += 1;
        }
        o
    }
}

/// 查找匹配的字节
pub fn index_of(b: &[u8], u: u8) -> Option<usize> {
    for i in 0..b.len() {
        if b[i] == u {
            return Some(i);
        }
    }

    None
}

/// `hex(u8)` 读取 (16 进制)
///
/// 如果格式错误, 返回 `None`
pub fn hex_u8(b: &[u8]) -> Option<u8> {
    if b.len() != 2 {
        return None;
    }

    // 转换一个 hex 字符
    fn h(x: u8) -> Option<u8> {
        match x {
            b'0'..=b'9' => Some(x - b'0'),
            b'a'..=b'f' => Some(x - b'a' + 10),
            _ => None,
        }
    }

    if let Some(x) = h(b[0]) {
        if let Some(y) = h(b[1]) {
            let u = (x << 4) | y;
            return Some(u);
        }
    }

    None
}

/// 判断长度是否为 2 的倍数
///
/// 不为 2 返回 true
fn len_n2(h: &[u8]) -> bool {
    // 检查最低位为 0
    h.len() & 1 != 0
}

/// `hex(u16)` 读取 (16 进制)
pub fn hex_u16(h: &[u8]) -> Option<u16> {
    if len_n2(h) || h.len() < 2 {
        return None;
    }

    let mut u: u16 = 0;
    for i in 0..2 {
        let end: usize = i * 2 + 2;
        if end > h.len() {
            break;
        }
        match hex_u8(&h[(i * 2)..end]) {
            Some(x) => {
                u = (u << 8) | (x as u16);
            }
            None => {
                return None;
            }
        }
    }
    Some(u)
}

/// `hex(u32)` 读取 (16 进制)
pub fn hex_u32(h: &[u8]) -> Option<u32> {
    if len_n2(h) || h.len() < 2 {
        return None;
    }

    let mut u: u32 = 0;
    for i in 0..4 {
        let end: usize = i * 2 + 2;
        if end > h.len() {
            break;
        }
        match hex_u8(&h[(i * 2)..end]) {
            Some(x) => {
                u = (u << 8) | (x as u32);
            }
            None => {
                return None;
            }
        }
    }
    Some(u)
}

/// `hex(u64)` 读取 (16 进制)
pub fn hex_u64(h: &[u8]) -> Option<u64> {
    if len_n2(h) || h.len() < 2 {
        return None;
    }

    let mut u: u64 = 0;
    for i in 0..8 {
        let end: usize = i * 2 + 2;
        if end > h.len() {
            break;
        }
        match hex_u8(&h[(i * 2)..end]) {
            Some(x) => {
                u = (u << 8) | (x as u64);
            }
            None => {
                return None;
            }
        }
    }
    Some(u)
}

/// n(`u8`) 读取 (10 进制)
pub fn n_u8(h: &[u8]) -> Option<u8> {
    // 转换一个 10 进制字符
    fn d(x: u8) -> Option<u8> {
        match x {
            b'0'..=b'9' => Some(x - b'0'),
            _ => None,
        }
    }

    match h.len() {
        // 1 位数 (0 ~ 9)
        1 => d(h[0]),
        // 2 位数 (10 ~ 99)
        2 => {
            if let Some(x) = d(h[0]) {
                if let Some(y) = d(h[1]) {
                    return Some(x * 10 + y);
                }
            }
            None
        }
        // 3 位数 (100 ~ 255)
        3 => {
            // 首先转换成 u32
            if let Some(x) = d(h[0]) {
                if let Some(y) = d(h[1]) {
                    if let Some(z) = d(h[2]) {
                        let u: u32 = ((x as u32) * 100) + ((y as u32) * 10) + (z as u32);
                        // 最大值 255
                        if u <= 0xff {
                            return Some(u as u8);
                        }
                    }
                }
            }
            None
        }
        _ => None,
    }
}

#[cfg(test)]
mod test;
