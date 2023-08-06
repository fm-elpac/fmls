extern crate std;

use core::iter::Iterator;

use std::string::String;
use std::string::ToString;
use std::vec;
use std::vec::Vec;

use libfmls::r2c3p::{
    ConfItem, FeedResult, Msg, MsgAt, MsgReq, MsgRes, MsgS, MsgType, R2c3pServer, P_VERSION,
};

use super::super::*;
use super::*;

// 使用 `libfmls::r2c3p` 来测试
// 在简化测试的同时, 也测试了与 `libfmls` 实现的一致性

fn recv_msg<T: Iterator<Item = u8>, C: CrcT<N>, const N: usize>(
    s: &mut LowSend<T, C, N>,
) -> Vec<u8> {
    let mut o: Vec<u8> = Vec::new();
    while let Some(b) = s.next() {
        o.push(b);
    }
    o
}

#[test]
fn test_send_msg_16() {
    let mut r = R2c3pServer::new();

    // `v`
    let mut s = send_msg_16(b'v', NoneSender::new());
    let m = recv_msg(&mut s);
    assert_eq!(r.feed(m), vec![FeedResult::M(Msg::Req(MsgReq::V))]);

    // `C@=0`
    let mut s = send_msg_16(b'C', BStaticSender::new(b"@=0"));
    let m = recv_msg(&mut s);
    assert_eq!(
        r.feed(m),
        vec![FeedResult::M(Msg::Res(MsgRes::C(ConfItem::At(Some(0)))))]
    );

    // `.`
    let mut s = send_msg_16(b'.', NoneSender::new());
    let m = recv_msg(&mut s);
    assert_eq!(
        r.feed(m),
        vec![FeedResult::M(Msg::A {
            t: MsgType::from(b'.'),
            data: None
        })]
    );

    // `@`
    let mut s = send_msg_16(b'@', BArraySender::new([0]));
    let m = recv_msg(&mut s);
    assert_eq!(
        r.feed(m),
        vec![FeedResult::M(Msg::S(MsgS::At(Some(MsgAt {
            n: 0,
            d: Vec::new()
        }))))]
    );
}

#[test]
fn test_send_msg_32() {
    let mut r = R2c3pServer::new();

    // `V`
    let mut s = send_msg_32(
        b'V',
        BStaticSender::new(b"fmls_r2c3p 0.1.0\nsled 0.1.0\nch32v003 666"),
    );
    let m = recv_msg(&mut s);
    assert_eq!(
        r.feed(m),
        vec![FeedResult::M(Msg::Res(MsgRes::V {
            p: "fmls_r2c3p 0.1.0".to_string(),
            firmware: "sled 0.1.0".to_string(),
            hardware: "ch32v003 666".to_string(),
            extra: None,
            raw: Some(Vec::from(
                b"fmls_r2c3p 0.1.0\nsled 0.1.0\nch32v003 666" as &[u8]
            ))
        }))]
    );
}

#[test]
fn test_send_v() {
    let mut r = R2c3pServer::new();

    let mut s = send_v(
        b"sled 0.1.0",
        b"ch32v003",
        [0xab, 0xcd, 0xef, 0x12, 0x34, 0x56, 0x78, 0x90],
    );
    let m = recv_msg(&mut s);
    assert_eq!(
        r.feed(m),
        vec![FeedResult::M(Msg::Res(MsgRes::V {
            p: String::from_utf8_lossy(P_VERSION).into_owned(),
            firmware: "sled 0.1.0".to_string(),
            hardware: "ch32v003 abcdef1234567890".to_string(),
            extra: None,
            raw: Some(Vec::from(
                b"fmls_r2c3p 0.1.0\nsled 0.1.0\nch32v003 abcdef1234567890" as &[u8]
            ))
        }))]
    );
}

#[test]
fn test_send_e() {
    let mut r = R2c3pServer::new();

    // `E-2`
    let mut s = send_e2();
    let m = recv_msg(&mut s);
    assert_eq!(
        r.feed(m),
        vec![FeedResult::M(Msg::Res(MsgRes::E { c: -2, m: None }))]
    );
    // `E-3 c`
    let mut s = send_e3(b'c');
    let m = recv_msg(&mut s);
    assert_eq!(
        r.feed(m),
        vec![FeedResult::M(Msg::Res(MsgRes::E {
            c: -3,
            m: Some(vec![b'c'])
        }))]
    );
    // `E-4`
    let mut s = send_e4();
    let m = recv_msg(&mut s);
    assert_eq!(
        r.feed(m),
        vec![FeedResult::M(Msg::Res(MsgRes::E { c: -4, m: None }))]
    );
    // `E-5`
    let mut s = send_e5();
    let m = recv_msg(&mut s);
    assert_eq!(
        r.feed(m),
        vec![FeedResult::M(Msg::Res(MsgRes::E { c: -5, m: None }))]
    );

    // `E-2 32`
    let b: [u8; 3 + 2] = [0, 0, 0, b'3', b'2'];
    let mut s = send_e2_len(b);
    let m = recv_msg(&mut s);
    assert_eq!(
        r.feed(m),
        vec![FeedResult::M(Msg::Res(MsgRes::E {
            c: -2,
            m: Some(Vec::from(b"32" as &[u8]))
        }))]
    );
}
