use super::super::msg::{ConfItem, MsgAt, MsgS, MsgType};
use super::*;

// 发送消息测试
#[test]
fn send() {
    let mut s = R2c3pServer::new();

    assert_eq!(s.send(Msg::Req(MsgReq::V)), vec![b'v', 0x81, 0xe6, b'\n']);
    assert_eq!(s.send(Msg::Req(MsgReq::Vv)), b"vv\n");
}

// 环回测试
#[test]
fn loopback() {
    let mut s = R2c3pServer::new();

    let m = Msg::Req(MsgReq::V);
    let b = s.send(m.clone());
    assert_eq!(s.feed(b), vec![FeedResult::M(m)]);

    let m = Msg::Req(MsgReq::Vv);
    let b = s.send(m.clone());
    assert_eq!(s.feed(b), vec![FeedResult::M(Msg::Req(MsgReq::V))]);

    let m = Msg::Req(MsgReq::C(ConfItem::At(None)));
    let b = s.send(m.clone());
    assert_eq!(s.feed(b), vec![FeedResult::M(m)]);

    let m = Msg::Res(MsgRes::V {
        p: "".to_string(),
        firmware: "sled 0.1.0".to_string(),
        hardware: "ch32v003 666".to_string(),
        extra: None,
        raw: None,
    });
    let b = s.send(m.clone());
    let m = Msg::Res(MsgRes::V {
        p: "fmls_r2c3p 0.1.0".to_string(),
        firmware: "sled 0.1.0".to_string(),
        hardware: "ch32v003 666".to_string(),
        extra: None,
        raw: Some(Vec::from(
            b"fmls_r2c3p 0.1.0\nsled 0.1.0\nch32v003 666" as &[u8],
        )),
    });
    assert_eq!(s.feed(b), vec![FeedResult::M(m)]);

    let m = Msg::Res(MsgRes::E {
        c: -2,
        m: Some(Vec::from(b"32" as &[u8])),
    });
    let b = s.send(m.clone());
    assert_eq!(s.feed(b), vec![FeedResult::M(m)]);

    let m = Msg::Res(MsgRes::K(None));
    let b = s.send(m.clone());
    assert_eq!(s.feed(b), vec![FeedResult::M(m)]);

    let m = Msg::Res(MsgRes::C(ConfItem::T1(Some(0x1234))));
    let b = s.send(m.clone());
    assert_eq!(s.feed(b), vec![FeedResult::M(m)]);

    let m = Msg::S(MsgS::At(None));
    let b = s.send(m.clone());
    assert_eq!(s.feed(b), vec![FeedResult::M(m)]);

    let m = Msg::S(MsgS::At(Some(MsgAt {
        n: b'1',
        d: Vec::from(b"v" as &[u8]),
    })));
    let b = s.send(m.clone());
    assert_eq!(s.feed(b), vec![FeedResult::M(m)]);

    let m = Msg::A {
        t: MsgType::from(b'.'),
        data: None,
    };
    let b = s.send(m.clone());
    assert_eq!(s.feed(b), vec![FeedResult::M(m)]);

    let m = Msg::A {
        t: MsgType::from(b'_'),
        data: Some(Vec::from(b"666" as &[u8])),
    };
    let b = s.send(m.clone());
    assert_eq!(s.feed(b), vec![FeedResult::M(m)]);
}

// 消息缓存测试
#[test]
fn buffer() {
    let mut s = R2c3pServer::new();

    // 一次接收 2 条消息
    let m1 = Msg::Req(MsgReq::V);
    let m2 = Msg::Req(MsgReq::C(ConfItem::T1(None)));
    let mut b = s.send(m1.clone());
    b.extend_from_slice(&s.send(m2.clone()));
    assert_eq!(s.feed(b), vec![FeedResult::M(m1), FeedResult::M(m2)]);

    // 一次接收一部分消息
    let m1 = Msg::Res(MsgRes::V {
        p: "".to_string(),
        firmware: "sled 0.1.0".to_string(),
        hardware: "ch32v003 666".to_string(),
        extra: None,
        raw: None,
    });
    let mut b = s.send(m1.clone());
    let m1 = Msg::Res(MsgRes::V {
        p: "fmls_r2c3p 0.1.0".to_string(),
        firmware: "sled 0.1.0".to_string(),
        hardware: "ch32v003 666".to_string(),
        extra: None,
        raw: Some(Vec::from(
            b"fmls_r2c3p 0.1.0\nsled 0.1.0\nch32v003 666" as &[u8],
        )),
    });
    let m2 = Msg::Res(MsgRes::C(ConfItem::T1(Some(0xfedc))));
    // 第 2 条消息只接收 3 字节
    let len1 = b.len() + 3;
    b.extend_from_slice(&s.send(m2.clone()));
    assert_eq!(s.feed(Vec::from(&b[..len1])), vec![FeedResult::M(m1)]);
    assert_eq!(s.feed(Vec::from(&b[len1..])), vec![FeedResult::M(m2)]);

    // 缓存应该清空了
    let m = Msg::Res(MsgRes::K(None));
    let b = s.send(m.clone());
    assert_eq!(s.feed(b), vec![FeedResult::M(m)]);
}
