extern crate std;

use core::iter::Iterator;

use std::string::String;
use std::string::ToString;
use std::vec;
use std::vec::Vec;

use libfmls::r2c3p::{
    ConfItem, FeedResult, Msg, MsgAt, MsgReq, MsgRes, MsgS, MsgType, R2c3pServer, P_VERSION,
};

use crate::r2c3p_low::{BStaticSender, NoneSender};

use super::*;

// 使用 `libfmls::r2c3p` 来测试
// 在简化测试的同时, 也测试了与 `libfmls` 实现的一致性

fn recv_msg<T: Iterator<Item = u8>>(s: &mut MsgSender<T>) -> Vec<u8> {
    let mut o: Vec<u8> = Vec::new();
    while let Some(b) = s.next() {
        o.push(b);
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
    let mut s = MsgSender::new(b'C', BStaticSender::new(b"@=0"));
    let m = recv_msg(&mut s);
    assert_eq!(
        r.feed(m),
        vec![FeedResult::M(Msg::Res(MsgRes::C(ConfItem::At(Some(0)))))]
    );

    // `V` (crc32)
    let mut s = MsgSender::new(
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
    let mut s = MsgSender::new(b'@', BStaticSender::new(&[0]));
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
    let v: VSender<BStaticSender, NoneSender> =
        VSender::new(b"sled 0.1.0", BStaticSender::new(b"ch32v003 666"), None);
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
    let v: VSender<BStaticSender, BStaticSender> = VSender::new(
        b"sled 0.1.0",
        BStaticSender::new(b"ch32v003 666"),
        Some(BStaticSender::new(b"hub\n2")),
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
