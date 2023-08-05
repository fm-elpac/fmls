extern crate std;

use std::vec::Vec;

use libfmls::r2c3p::{Msg, MsgType, R2c3pServer};

use super::*;

// 给接收端喂入字节
fn feed_port<T: R2c3pPortT>(p: &mut T, b: &[u8]) {
    for i in b {
        p.feed(*i);
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
fn port_8() {
    let mut s = R2c3pServer::new();
    let mut p = R2c3pPort8::new();

    // 初始状态
    assert_eq!(p.get_t(), None);

    // 输入无效消息
    p.feed(b'\n');
    assert_eq!(p.get_t(), None);

    // 测试 `vv` 消息
    feed_port(&mut p, b"vv\n");
    assert_eq!(p.get_t(), Some(p::MSGT_V_R));

    // 测试不同长度的消息
    let (m, b) = make_m(&mut s, 1);
    feed_port(&mut p, &m);
    assert_eq!(p.get_t(), Some(b'_'));
    assert_eq!(p.get_m_len(), Some(0));
    assert_eq!(p.get_body(), b);

    let (m, b) = make_m(&mut s, 2);
    feed_port(&mut p, &m);
    assert_eq!(p.get_t(), Some(b'_'));
    assert_eq!(p.get_m_len(), Some(1));
    assert_eq!(p.get_body(), b);

    let (m, b) = make_m(&mut s, 3);
    feed_port(&mut p, &m);
    assert_eq!(p.get_t(), Some(b'_'));
    assert_eq!(p.get_m_len(), Some(2));
    assert_eq!(p.get_body(), b);

    let (m, b) = make_m(&mut s, 8);
    feed_port(&mut p, &m);
    assert_eq!(p.get_t(), Some(b'_'));
    assert_eq!(p.get_m_len(), Some(7));
    assert_eq!(p.get_body(), b);

    // 9 字节, 最大可接收长度
    let (m, b) = make_m(&mut s, 9);
    feed_port(&mut p, &m);
    assert_eq!(p.get_t(), Some(b'_'));
    assert_eq!(p.get_m_len(), Some(8));
    assert_eq!(p.get_body(), b);
    assert_eq!(p.get_e_2(), false);

    // 10 字节, 太长错误
    let (m, _) = make_m(&mut s, 10);
    feed_port(&mut p, &m);
    assert_eq!(p.get_t(), Some(b'_'));
    assert_eq!(p.get_e_2(), true);
}

#[test]
fn port_32() {
    let mut s = R2c3pServer::new();
    let mut p = R2c3pPort32::new();

    // 初始状态
    assert_eq!(p.get_t(), None);

    // 测试 `vv` 消息
    feed_port(&mut p, b"vv\n");
    assert_eq!(p.get_t(), Some(p::MSGT_V_R));

    // 测试不同长度的消息
    let (m, b) = make_m(&mut s, 1);
    feed_port(&mut p, &m);
    assert_eq!(p.get_t(), Some(b'_'));
    assert_eq!(p.get_m_len(), Some(0));
    assert_eq!(p.get_body(), b);

    let (m, b) = make_m(&mut s, 2);
    feed_port(&mut p, &m);
    assert_eq!(p.get_t(), Some(b'_'));
    assert_eq!(p.get_m_len(), Some(1));
    assert_eq!(p.get_body(), b);

    let (m, b) = make_m(&mut s, 8);
    feed_port(&mut p, &m);
    assert_eq!(p.get_t(), Some(b'_'));
    assert_eq!(p.get_m_len(), Some(7));
    assert_eq!(p.get_body(), b);

    let (m, b) = make_m(&mut s, 17);
    feed_port(&mut p, &m);
    assert_eq!(p.get_t(), Some(b'_'));
    assert_eq!(p.get_m_len(), Some(16));
    assert_eq!(p.get_body(), b);

    let (m, b) = make_m(&mut s, 32);
    feed_port(&mut p, &m);
    assert_eq!(p.get_t(), Some(b'_'));
    assert_eq!(p.get_m_len(), Some(31));
    assert_eq!(p.get_body(), b);

    // 33 字节, 最大可接收长度
    let (m, b) = make_m(&mut s, 33);
    feed_port(&mut p, &m);
    assert_eq!(p.get_t(), Some(b'_'));
    assert_eq!(p.get_m_len(), Some(32));
    assert_eq!(p.get_body(), b);
    assert_eq!(p.get_e_2(), false);

    // 34 字节, 太长错误
    let (m, _) = make_m(&mut s, 34);
    feed_port(&mut p, &m);
    assert_eq!(p.get_t(), Some(b'_'));
    assert_eq!(p.get_e_2(), true);
}

#[test]
fn port_64() {
    let mut s = R2c3pServer::new();
    let mut p = R2c3pPort64::new();

    // 初始状态
    assert_eq!(p.get_t(), None);

    // 测试 `vv` 消息
    feed_port(&mut p, b"vv\n");
    assert_eq!(p.get_t(), Some(p::MSGT_V_R));

    // 测试不同长度的消息
    let (m, b) = make_m(&mut s, 1);
    feed_port(&mut p, &m);
    assert_eq!(p.get_t(), Some(b'_'));
    assert_eq!(p.get_m_len(), Some(0));
    assert_eq!(p.get_body(), b);

    // 32 字节, 使用 crc16
    let (m, b) = make_m(&mut s, 32);
    feed_port(&mut p, &m);
    assert_eq!(p.get_t(), Some(b'_'));
    assert_eq!(p.get_m_len(), Some(31));
    assert_eq!(p.get_body(), b);
    assert_eq!(p.get_e_2(), false);

    // 33 字节, 使用 crc32
    let (m, b) = make_m(&mut s, 33);
    feed_port(&mut p, &m);
    assert_eq!(p.get_t(), Some(b'_'));
    assert_eq!(p.get_m_len(), Some(32));
    assert_eq!(p.get_body(), b);
    assert_eq!(p.get_e_2(), false);

    let (m, b) = make_m(&mut s, 64);
    feed_port(&mut p, &m);
    assert_eq!(p.get_t(), Some(b'_'));
    assert_eq!(p.get_m_len(), Some(63));
    assert_eq!(p.get_body(), b);
    assert_eq!(p.get_e_2(), false);

    // 65 字节, 最大可接收长度
    let (m, b) = make_m(&mut s, 65);
    feed_port(&mut p, &m);
    assert_eq!(p.get_t(), Some(b'_'));
    assert_eq!(p.get_m_len(), Some(64));
    assert_eq!(p.get_body(), b);
    assert_eq!(p.get_e_2(), false);

    // 66 字节, 太长错误
    let (m, _) = make_m(&mut s, 66);
    feed_port(&mut p, &m);
    assert_eq!(p.get_t(), Some(b'_'));
    assert_eq!(p.get_e_2(), true);

    // 在错误之后恢复
    let (m, b) = make_m(&mut s, 4);
    feed_port(&mut p, &m);
    assert_eq!(p.get_t(), Some(b'_'));
    assert_eq!(p.get_m_len(), Some(3));
    assert_eq!(p.get_body(), b);
    assert_eq!(p.get_e_2(), false);
}

#[test]
fn port_128() {
    let mut s = R2c3pServer::new();
    let mut p = R2c3pPort128::new();

    // 初始状态
    assert_eq!(p.get_t(), None);

    // 测试 `vv` 消息
    feed_port(&mut p, b"vv\n");
    assert_eq!(p.get_t(), Some(p::MSGT_V_R));

    // 测试不同长度的消息
    let (m, b) = make_m(&mut s, 1);
    feed_port(&mut p, &m);
    assert_eq!(p.get_t(), Some(b'_'));
    assert_eq!(p.get_m_len(), Some(0));
    assert_eq!(p.get_body(), b);

    // 32 字节, 使用 crc16
    let (m, b) = make_m(&mut s, 32);
    feed_port(&mut p, &m);
    assert_eq!(p.get_t(), Some(b'_'));
    assert_eq!(p.get_m_len(), Some(31));
    assert_eq!(p.get_body(), b);
    assert_eq!(p.get_e_2(), false);

    // 33 字节, 使用 crc32
    let (m, b) = make_m(&mut s, 33);
    feed_port(&mut p, &m);
    assert_eq!(p.get_t(), Some(b'_'));
    assert_eq!(p.get_m_len(), Some(32));
    assert_eq!(p.get_body(), b);
    assert_eq!(p.get_e_2(), false);

    let (m, b) = make_m(&mut s, 128);
    feed_port(&mut p, &m);
    assert_eq!(p.get_t(), Some(b'_'));
    assert_eq!(p.get_m_len(), Some(127));
    assert_eq!(p.get_body(), b);
    assert_eq!(p.get_e_2(), false);

    // 129 字节, 最大可接收长度
    let (m, b) = make_m(&mut s, 129);
    feed_port(&mut p, &m);
    assert_eq!(p.get_t(), Some(b'_'));
    assert_eq!(p.get_m_len(), Some(128));
    assert_eq!(p.get_body(), b);
    assert_eq!(p.get_e_2(), false);

    // 130 字节, 太长错误
    let (m, _) = make_m(&mut s, 130);
    feed_port(&mut p, &m);
    assert_eq!(p.get_t(), Some(b'_'));
    assert_eq!(p.get_e_2(), true);

    // 在错误之后恢复
    let (m, b) = make_m(&mut s, 11);
    feed_port(&mut p, &m);
    assert_eq!(p.get_t(), Some(b'_'));
    assert_eq!(p.get_m_len(), Some(10));
    assert_eq!(p.get_body(), b);
    assert_eq!(p.get_e_2(), false);
}
