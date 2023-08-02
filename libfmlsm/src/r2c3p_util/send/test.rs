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

fn recv_msg<'a, T: Iterator<Item = u8>>(s: &mut MsgSender<'a, T>) -> Vec<u8> {
    let mut o: Vec<u8> = Vec::new();
    while !s.done() {
        match s.next() {
            Some(b) => o.push(b),
            None => {}
        }
    }
    o
}

#[test]
fn msg_sender() {
    let mut r = R2c3pServer::new();

    // `v`
    let mut s = MsgSender::new(b'v', NoneSender::new());
    let m = recv_msg(&mut s);
    assert_eq!(r.feed(m), vec![FeedResult::M(Msg::Req(MsgReq::V))]);

    // `C@=0`
    let mut s = MsgSender::new(b'C', VecSender::new(b"@=0"));
    let m = recv_msg(&mut s);
    assert_eq!(
        r.feed(m),
        vec![FeedResult::M(Msg::Res(MsgRes::C(ConfItem::At(Some(0)))))]
    );

    // `V` (crc32)
    let mut s = MsgSender::new(
        b'V',
        VecSender::new(b"fmls_r2c3p 0.1.0\nsled 0.1.0\nch32v003 666"),
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

    // `.`
    let mut s = MsgSender::new(b'.', NoneSender::new());
    let m = recv_msg(&mut s);
    assert_eq!(
        r.feed(m),
        vec![FeedResult::M(Msg::A {
            t: MsgType::from(b'.'),
            data: None
        })]
    );

    // `@`
    let mut s = MsgSender::new(b'@', VecSender::new(&[0]));
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
fn v_sender() {
    let mut r = R2c3pServer::new();

    // extra = None
    let v: VSender<VecSender, NoneSender> =
        VSender::new(b"sled 0.1.0", VecSender::new(b"ch32v003 666"), None);
    let mut s = MsgSender::new(b'V', v);
    let m = recv_msg(&mut s);
    assert_eq!(
        r.feed(m),
        vec![FeedResult::M(Msg::Res(MsgRes::V {
            p: String::from_utf8_lossy(P_VERSION).into_owned(),
            firmware: "sled 0.1.0".to_string(),
            hardware: "ch32v003 666".to_string(),
            extra: None,
            raw: Some(Vec::from(
                b"fmls_r2c3p 0.1.0\nsled 0.1.0\nch32v003 666" as &[u8]
            ))
        }))]
    );

    // extra
    let v: VSender<VecSender, VecSender> = VSender::new(
        b"sled 0.1.0",
        VecSender::new(b"ch32v003 666"),
        Some(VecSender::new(b"hub\n2")),
    );
    let mut s = MsgSender::new(b'V', v);
    let m = recv_msg(&mut s);
    assert_eq!(
        r.feed(m),
        vec![FeedResult::M(Msg::Res(MsgRes::V {
            p: String::from_utf8_lossy(P_VERSION).into_owned(),
            firmware: "sled 0.1.0".to_string(),
            hardware: "ch32v003 666".to_string(),
            extra: Some("hub\n2".to_string()),
            raw: Some(Vec::from(
                b"fmls_r2c3p 0.1.0\nsled 0.1.0\nch32v003 666\nhub\n2" as &[u8]
            ))
        }))]
    );
}
