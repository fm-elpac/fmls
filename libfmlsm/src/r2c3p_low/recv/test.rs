extern crate std;

use std::vec::Vec;

use libfmls::r2c3p::{Msg, MsgType, R2c3pServer};

use crate::r2c3p as p;

use super::*;

// 给接收端喂入字节
fn feed_r<const N: usize>(r: &mut LowRecv<N>, b: &[u8]) {
    for i in b {
        r.feed(*i);
    }
}

// 生成指定长度的消息
fn make_m(s: &mut R2c3pServer, len: usize) -> (Vec<u8>, Vec<u8>) {
    let mut b: Vec<u8> = Vec::new();
    for _ in 0..(len - 1) {
        b.push(b'T');
    }
    let m = Msg::A {
        t: MsgType::from(b'_'),
        data: Some(b.clone()),
    };
    (s.send(m), b)
}

#[test]
fn recv_8() {
    let mut s = R2c3pServer::new();
    // 最大接收长度 8 字节 (不含 crc16) 的消息
    let mut r = LowRecv::<{ 8 + 2 }>::new();

    // 初始状态
    assert_eq!(r.get_t(), None);
    assert_eq!(r.get_body(), None);

    // 输入无效消息
    r.feed(b'\n');
    assert_eq!(r.get_t(), None);
    assert_eq!(r.get_body(), None);

    // 测试 `vv` 消息
    feed_r(&mut r, b"vv\n");
    assert_eq!(r.get_t(), Some(p::MSGT_V_R));

    // 测试不同长度的消息
    let (m, _) = make_m(&mut s, 1);
    feed_r(&mut r, &m);
    assert_eq!(r.get_t(), Some(b'_'));
    assert_eq!(r.get_e2(), false);
    assert_eq!(r.get_body(), None);

    let (m, b) = make_m(&mut s, 2);
    feed_r(&mut r, &m);
    assert_eq!(r.get_t(), Some(b'_'));
    assert_eq!(r.get_e2(), false);
    assert_eq!(r.get_body(), Some(b.as_slice()));

    let (m, b) = make_m(&mut s, 3);
    feed_r(&mut r, &m);
    assert_eq!(r.get_t(), Some(b'_'));
    assert_eq!(r.get_e2(), false);
    assert_eq!(r.get_body(), Some(b.as_slice()));

    // 8 字节, 最大可接收长度
    let (m, b) = make_m(&mut s, 8);
    feed_r(&mut r, &m);
    assert_eq!(r.get_t(), Some(b'_'));
    assert_eq!(r.get_e2(), false);
    assert_eq!(r.get_body(), Some(b.as_slice()));

    // 9 字节, 太长而被丢弃 (crc16)
    let (m, _) = make_m(&mut s, 9);
    feed_r(&mut r, &m);
    assert_eq!(r.get_t(), None);
    assert_eq!(r.get_e2(), false);
    assert_eq!(r.get_body(), None);

    // 10 字节, 太长而被丢弃 (crc16)
    let (m, _) = make_m(&mut s, 10);
    feed_r(&mut r, &m);
    assert_eq!(r.get_t(), None);
    assert_eq!(r.get_e2(), false);
    assert_eq!(r.get_body(), None);

    // 在错误之后恢复
    let (mut m, b) = make_m(&mut s, 7);
    feed_r(&mut r, &m);
    assert_eq!(r.get_t(), Some(b'_'));
    assert_eq!(r.get_e2(), false);
    assert_eq!(r.get_body(), Some(b.as_slice()));
    // 模拟 crc 错误
    m[3] = 0;
    feed_r(&mut r, &m);
    assert_eq!(r.get_t(), None);
    assert_eq!(r.get_e2(), false);
    assert_eq!(r.get_body(), None);
}

#[test]
fn recv_32() {
    let mut s = R2c3pServer::new();
    // 最大接收长度 32 字节 (不含 crc16) 的消息
    let mut r = LowRecv::<{ 32 + 2 }>::new();

    // 初始状态
    assert_eq!(r.get_t(), None);
    assert_eq!(r.get_body(), None);

    // 输入无效消息
    r.feed(b'\n');
    assert_eq!(r.get_t(), None);
    assert_eq!(r.get_body(), None);

    // 测试 `vv` 消息
    feed_r(&mut r, b"vv\n");
    assert_eq!(r.get_t(), Some(p::MSGT_V_R));

    // 测试不同长度的消息
    let (m, _) = make_m(&mut s, 1);
    feed_r(&mut r, &m);
    assert_eq!(r.get_t(), Some(b'_'));
    assert_eq!(r.get_e2(), false);
    assert_eq!(r.get_body(), None);

    let (m, b) = make_m(&mut s, 2);
    feed_r(&mut r, &m);
    assert_eq!(r.get_t(), Some(b'_'));
    assert_eq!(r.get_e2(), false);
    assert_eq!(r.get_body(), Some(b.as_slice()));

    let (m, b) = make_m(&mut s, 8);
    feed_r(&mut r, &m);
    assert_eq!(r.get_t(), Some(b'_'));
    assert_eq!(r.get_e2(), false);
    assert_eq!(r.get_body(), Some(b.as_slice()));

    let (m, b) = make_m(&mut s, 17);
    feed_r(&mut r, &m);
    assert_eq!(r.get_t(), Some(b'_'));
    assert_eq!(r.get_e2(), false);
    assert_eq!(r.get_body(), Some(b.as_slice()));

    // 32 字节, 最大可接收长度
    let (m, b) = make_m(&mut s, 32);
    feed_r(&mut r, &m);
    assert_eq!(r.get_t(), Some(b'_'));
    assert_eq!(r.get_e2(), false);
    assert_eq!(r.get_body(), Some(b.as_slice()));

    // 33 字节, 太长错误 (crc32)
    let (m, _) = make_m(&mut s, 33);
    feed_r(&mut r, &m);
    assert_eq!(r.get_t(), Some(b'_'));
    assert_eq!(r.get_e2(), true);
    assert_eq!(r.get_body(), None);

    // 在错误之后恢复
    let (mut m, b) = make_m(&mut s, 31);
    feed_r(&mut r, &m);
    assert_eq!(r.get_t(), Some(b'_'));
    assert_eq!(r.get_e2(), false);
    assert_eq!(r.get_body(), Some(b.as_slice()));
    // 模拟 crc 错误
    m[17] = 0;
    feed_r(&mut r, &m);
    assert_eq!(r.get_t(), None);
    assert_eq!(r.get_e2(), false);
    assert_eq!(r.get_body(), None);
}

#[test]
fn recv_64() {
    let mut s = R2c3pServer::new();
    // 最大接收长度 64 字节 (不含 crc32) 的消息
    let mut r = LowRecv::<{ 64 + 4 }>::new();

    // 初始状态
    assert_eq!(r.get_t(), None);
    assert_eq!(r.get_body(), None);

    // 输入无效消息
    r.feed(b'\n');
    assert_eq!(r.get_t(), None);
    assert_eq!(r.get_body(), None);

    // 测试 `vv` 消息
    feed_r(&mut r, b"vv\n");
    assert_eq!(r.get_t(), Some(p::MSGT_V_R));

    // 测试不同长度的消息
    let (m, _) = make_m(&mut s, 1);
    feed_r(&mut r, &m);
    assert_eq!(r.get_t(), Some(b'_'));
    assert_eq!(r.get_e2(), false);
    assert_eq!(r.get_body(), None);

    // 32 字节, 使用 crc16
    let (m, b) = make_m(&mut s, 32);
    feed_r(&mut r, &m);
    assert_eq!(r.get_t(), Some(b'_'));
    assert_eq!(r.get_e2(), false);
    assert_eq!(r.get_body(), Some(b.as_slice()));

    // 33 字节, 使用 crc32
    let (m, b) = make_m(&mut s, 33);
    feed_r(&mut r, &m);
    assert_eq!(r.get_t(), Some(b'_'));
    assert_eq!(r.get_e2(), false);
    assert_eq!(r.get_body(), Some(b.as_slice()));

    // 64 字节, 最大可接收长度
    let (m, b) = make_m(&mut s, 64);
    feed_r(&mut r, &m);
    assert_eq!(r.get_t(), Some(b'_'));
    assert_eq!(r.get_e2(), false);
    assert_eq!(r.get_body(), Some(b.as_slice()));

    // 65 字节, 太长错误 (crc32)
    let (m, _) = make_m(&mut s, 65);
    feed_r(&mut r, &m);
    assert_eq!(r.get_t(), Some(b'_'));
    assert_eq!(r.get_e2(), true);
    assert_eq!(r.get_body(), None);

    // 在错误之后恢复
    let (mut m, b) = make_m(&mut s, 63);
    feed_r(&mut r, &m);
    assert_eq!(r.get_t(), Some(b'_'));
    assert_eq!(r.get_e2(), false);
    assert_eq!(r.get_body(), Some(b.as_slice()));
    // 模拟 crc 错误
    m[55] = 0;
    feed_r(&mut r, &m);
    assert_eq!(r.get_t(), None);
    assert_eq!(r.get_e2(), false);
    assert_eq!(r.get_body(), None);
}

#[test]
fn recv_128() {
    let mut s = R2c3pServer::new();
    // 最大接收长度 128 字节 (不含 crc32) 的消息
    let mut r = LowRecv::<{ 128 + 4 }>::new();

    // 初始状态
    assert_eq!(r.get_t(), None);
    assert_eq!(r.get_body(), None);

    // 输入无效消息
    r.feed(b'\n');
    assert_eq!(r.get_t(), None);
    assert_eq!(r.get_body(), None);

    // 测试 `vv` 消息
    feed_r(&mut r, b"vv\n");
    assert_eq!(r.get_t(), Some(p::MSGT_V_R));

    // 测试不同长度的消息
    let (m, _) = make_m(&mut s, 1);
    feed_r(&mut r, &m);
    assert_eq!(r.get_t(), Some(b'_'));
    assert_eq!(r.get_e2(), false);
    assert_eq!(r.get_body(), None);

    // 32 字节, 使用 crc16
    let (m, b) = make_m(&mut s, 32);
    feed_r(&mut r, &m);
    assert_eq!(r.get_t(), Some(b'_'));
    assert_eq!(r.get_e2(), false);
    assert_eq!(r.get_body(), Some(b.as_slice()));

    // 33 字节, 使用 crc32
    let (m, b) = make_m(&mut s, 33);
    feed_r(&mut r, &m);
    assert_eq!(r.get_t(), Some(b'_'));
    assert_eq!(r.get_e2(), false);
    assert_eq!(r.get_body(), Some(b.as_slice()));

    // 128 字节, 最大可接收长度
    let (m, b) = make_m(&mut s, 128);
    feed_r(&mut r, &m);
    assert_eq!(r.get_t(), Some(b'_'));
    assert_eq!(r.get_e2(), false);
    assert_eq!(r.get_body(), Some(b.as_slice()));

    // 129 字节, 太长错误 (crc32)
    let (m, _) = make_m(&mut s, 129);
    feed_r(&mut r, &m);
    assert_eq!(r.get_t(), Some(b'_'));
    assert_eq!(r.get_e2(), true);
    assert_eq!(r.get_body(), None);

    // 在错误之后恢复
    let (mut m, b) = make_m(&mut s, 127);
    feed_r(&mut r, &m);
    assert_eq!(r.get_t(), Some(b'_'));
    assert_eq!(r.get_e2(), false);
    assert_eq!(r.get_body(), Some(b.as_slice()));
    // 模拟 crc 错误
    m[100] = 0;
    feed_r(&mut r, &m);
    assert_eq!(r.get_t(), None);
    assert_eq!(r.get_e2(), false);
    assert_eq!(r.get_body(), None);
}
