use super::*;

// Msg 转为 `Vec<u8>`
#[test]
fn msg_u8() {
    // Msg::Req
    assert_eq!(<Msg as Into<Vec<u8>>>::into(Msg::Req(MsgReq::V)), b"v");
    assert_eq!(<Msg as Into<Vec<u8>>>::into(Msg::Req(MsgReq::Vv)), b"vv");
    assert_eq!(
        <Msg as Into<Vec<u8>>>::into(Msg::Req(MsgReq::C(ConfItem::M1(None)))),
        b"cm"
    );

    // Msg::Res
    assert_eq!(
        <Msg as Into<Vec<u8>>>::into(Msg::Res(MsgRes::V {
            p: "".to_string(),
            firmware: "sled 0.1.0".to_string(),
            hardware: "ch32v003 666".to_string(),
            extra: None,
            raw: None,
        })),
        b"Vfmls_r2c3p 0.1.0\nsled 0.1.0\nch32v003 666"
    );
    assert_eq!(
        <Msg as Into<Vec<u8>>>::into(Msg::Res(MsgRes::V {
            p: "".to_string(),
            firmware: "r2hub 0.1.0".to_string(),
            hardware: "ch32v103 777777".to_string(),
            extra: Some("hub".to_string()),
            raw: None,
        })),
        b"Vfmls_r2c3p 0.1.0\nr2hub 0.1.0\nch32v103 777777\nhub"
    );

    assert_eq!(
        <Msg as Into<Vec<u8>>>::into(Msg::Res(MsgRes::E {
            c: -2,
            m: Some(Vec::from(b"32" as &[u8]))
        })),
        b"E-2 32"
    );
    assert_eq!(
        <Msg as Into<Vec<u8>>>::into(Msg::Res(MsgRes::K(None))),
        b"K"
    );
    assert_eq!(
        <Msg as Into<Vec<u8>>>::into(Msg::Res(MsgRes::K(Some(Vec::from(b"2" as &[u8]))))),
        b"K2"
    );
    assert_eq!(
        <Msg as Into<Vec<u8>>>::into(Msg::Res(MsgRes::C(ConfItem::M1(Some(0))))),
        b"Cm=0"
    );

    // Msg::S
    assert_eq!(<Msg as Into<Vec<u8>>>::into(Msg::S(MsgS::At(None))), b"@");
    assert_eq!(
        <Msg as Into<Vec<u8>>>::into(Msg::S(MsgS::At(Some(MsgAt {
            n: b'1',
            d: Vec::from(b"v" as &[u8]),
        })))),
        b"@1v"
    );

    // Msg::A
    assert_eq!(
        <Msg as Into<Vec<u8>>>::into(Msg::A {
            t: MsgType::from(b'.'),
            data: None,
        }),
        b"."
    );
    assert_eq!(
        <Msg as Into<Vec<u8>>>::into(Msg::A {
            t: MsgType::from(b'_'),
            data: Some(Vec::from(b"test 666" as &[u8])),
        }),
        b"_test 666"
    );
}

// `Vec<u8>` 转为 Msg
#[test]
fn u8_msg() {
    // Msg::Req
    assert_eq!(
        <Vec<u8> as Into<MsgO>>::into(Vec::from(b"v" as &[u8])),
        MsgO(Some(Msg::Req(MsgReq::V)))
    );
    // TODO `vv`
    assert_eq!(
        <Vec<u8> as Into<MsgO>>::into(Vec::from(b"cm=1" as &[u8])),
        MsgO(Some(Msg::Req(MsgReq::C(ConfItem::M1(Some(1))))))
    );

    // Msg::Res
    assert_eq!(
        <Vec<u8> as Into<MsgO>>::into(Vec::from(
            b"Vfmls_r2c3p 0.1.0\nsled 0.1.0\nch32v003 666" as &[u8]
        )),
        MsgO(Some(Msg::Res(MsgRes::V {
            p: "fmls_r2c3p 0.1.0".to_string(),
            firmware: "sled 0.1.0".to_string(),
            hardware: "ch32v003 666".to_string(),
            extra: None,
            raw: Some(Vec::from(
                b"fmls_r2c3p 0.1.0\nsled 0.1.0\nch32v003 666" as &[u8]
            )),
        })))
    );
    assert_eq!(
        <Vec<u8> as Into<MsgO>>::into(Vec::from(
            b"Vfmls_r2c3p 0.1.0\nr2hub 0.1.0\nch32v103 777777\nhub\n2" as &[u8]
        )),
        MsgO(Some(Msg::Res(MsgRes::V {
            p: "fmls_r2c3p 0.1.0".to_string(),
            firmware: "r2hub 0.1.0".to_string(),
            hardware: "ch32v103 777777".to_string(),
            extra: Some("hub\n2".to_string()),
            raw: Some(Vec::from(
                b"fmls_r2c3p 0.1.0\nr2hub 0.1.0\nch32v103 777777\nhub\n2" as &[u8]
            )),
        })))
    );

    assert_eq!(
        <Vec<u8> as Into<MsgO>>::into(Vec::from(b"E-2 32" as &[u8])),
        MsgO(Some(Msg::Res(MsgRes::E {
            c: -2,
            m: Some(Vec::from(b"32" as &[u8]))
        })))
    );
    assert_eq!(
        <Vec<u8> as Into<MsgO>>::into(Vec::from(b"K" as &[u8])),
        MsgO(Some(Msg::Res(MsgRes::K(None))))
    );
    assert_eq!(
        <Vec<u8> as Into<MsgO>>::into(Vec::from(b"K2" as &[u8])),
        MsgO(Some(Msg::Res(MsgRes::K(Some(Vec::from(b"2" as &[u8]))))))
    );
    assert_eq!(
        <Vec<u8> as Into<MsgO>>::into(Vec::from(b"C@=1" as &[u8])),
        MsgO(Some(Msg::Res(MsgRes::C(ConfItem::At(Some(1))))))
    );

    // Msg::S
    assert_eq!(
        <Vec<u8> as Into<MsgO>>::into(Vec::from(b"@" as &[u8])),
        MsgO(Some(Msg::S(MsgS::At(None))))
    );
    assert_eq!(
        <Vec<u8> as Into<MsgO>>::into(Vec::from(b"@1v" as &[u8])),
        MsgO(Some(Msg::S(MsgS::At(Some(MsgAt {
            n: b'1',
            d: Vec::from(b"v" as &[u8])
        })))))
    );

    // Msg::A
    assert_eq!(
        <Vec<u8> as Into<MsgO>>::into(Vec::from(b"." as &[u8])),
        MsgO(Some(Msg::A {
            t: MsgType::from(b'.'),
            data: None
        }))
    );
    assert_eq!(
        <Vec<u8> as Into<MsgO>>::into(Vec::from(b"_test 666" as &[u8])),
        MsgO(Some(Msg::A {
            t: MsgType::from(b'_'),
            data: Some(Vec::from(b"test 666" as &[u8]))
        }))
    );
}

// 获取消息类型
#[test]
fn msg_type() {
    assert_eq!(
        (Msg::A {
            t: MsgType::from(0),
            data: None,
        })
        .t(),
        MsgType::S(MsgTypeS::Type(0))
    );
    assert_eq!(Msg::Req(MsgReq::V).t(), MsgType::Req(MsgTypeReq::V));
    assert_eq!(
        Msg::Req(MsgReq::C(ConfItem::T1(None))).t(),
        MsgType::Req(MsgTypeReq::C)
    );
    assert_eq!(
        Msg::Res(MsgRes::V {
            p: "".to_string(),
            firmware: "".to_string(),
            hardware: "".to_string(),
            extra: None,
            raw: None,
        })
        .t(),
        MsgType::Res(MsgTypeRes::V)
    );
    assert_eq!(
        Msg::Res(MsgRes::E { c: 1, m: None }).t(),
        MsgType::Res(MsgTypeRes::E)
    );
    assert_eq!(Msg::Res(MsgRes::K(None)).t(), MsgType::Res(MsgTypeRes::K));
    assert_eq!(
        Msg::Res(MsgRes::C(ConfItem::T1(None))).t(),
        MsgType::Res(MsgTypeRes::C)
    );
    assert_eq!(Msg::S(MsgS::At(None)).t(), MsgType::S(MsgTypeS::At));
}
