use super::*;

// ConfItem 转为 `Vec<u8>`
#[test]
fn conf_u8() {
    assert_eq!(
        <ConfItem as Into<Vec<u8>>>::into(ConfItem::K {
            k: Vec::from(b"a" as &[u8]),
            v: None
        }),
        b"a"
    );
    assert_eq!(
        <ConfItem as Into<Vec<u8>>>::into(ConfItem::K {
            k: Vec::from(b"a" as &[u8]),
            v: Some(Vec::from(b"1" as &[u8]))
        }),
        b"a=1"
    );

    assert_eq!(<ConfItem as Into<Vec<u8>>>::into(ConfItem::M1(None)), b"m");
    assert_eq!(
        <ConfItem as Into<Vec<u8>>>::into(ConfItem::M1(Some(1))),
        b"m=1"
    );
    assert_eq!(<ConfItem as Into<Vec<u8>>>::into(ConfItem::T(None)), b"T");
    assert_eq!(
        <ConfItem as Into<Vec<u8>>>::into(ConfItem::T(Some(vec![
            0x01, 0x23, 0x45, 0x67, 0x89, 0xab
        ]))),
        b"T=0123456789ab"
    );
    assert_eq!(<ConfItem as Into<Vec<u8>>>::into(ConfItem::T1(None)), b"t");
    assert_eq!(
        <ConfItem as Into<Vec<u8>>>::into(ConfItem::T1(Some(0x12ab))),
        b"t=12ab"
    );
    assert_eq!(<ConfItem as Into<Vec<u8>>>::into(ConfItem::I(None)), b"I");
    assert_eq!(
        <ConfItem as Into<Vec<u8>>>::into(ConfItem::I(Some(0x01020304050607de))),
        b"I=01020304050607de"
    );
    assert_eq!(<ConfItem as Into<Vec<u8>>>::into(ConfItem::O(None)), b"O");
    assert_eq!(
        <ConfItem as Into<Vec<u8>>>::into(ConfItem::O(Some(0x1c))),
        b"O=1c"
    );
    assert_eq!(<ConfItem as Into<Vec<u8>>>::into(ConfItem::On(None)), b"On");
    assert_eq!(
        <ConfItem as Into<Vec<u8>>>::into(ConfItem::On(Some(0xd8))),
        b"On=d8"
    );
    assert_eq!(<ConfItem as Into<Vec<u8>>>::into(ConfItem::At(None)), b"@");
    assert_eq!(
        <ConfItem as Into<Vec<u8>>>::into(ConfItem::At(Some(1))),
        b"@=1"
    );
    assert_eq!(
        <ConfItem as Into<Vec<u8>>>::into(ConfItem::AtS { n: 1, v: None }),
        b"@s01"
    );
    assert_eq!(
        <ConfItem as Into<Vec<u8>>>::into(ConfItem::AtS {
            n: 0xa2,
            v: Some(0x120000ef)
        }),
        b"@sa2=120000ef"
    );
    assert_eq!(
        <ConfItem as Into<Vec<u8>>>::into(ConfItem::AtN { n: 0x20, v: None }),
        b"@n20"
    );
    assert_eq!(
        <ConfItem as Into<Vec<u8>>>::into(ConfItem::AtN {
            n: 0x31,
            v: Some(0xc0)
        }),
        b"@n31=000000c0"
    );

    // ConfItem::C
    assert_eq!(
        <ConfItem as Into<Vec<u8>>>::into(ConfItem::C {
            k: p::CONF_CT,
            v: None
        }),
        b"cT"
    );
    assert_eq!(
        <ConfItem as Into<Vec<u8>>>::into(ConfItem::C {
            k: p::CONF_CT,
            v: Some(0x01)
        }),
        b"cT=00000001"
    );
    assert_eq!(
        <ConfItem as Into<Vec<u8>>>::into(ConfItem::C {
            k: p::CONF_CR,
            v: None
        }),
        b"cR"
    );
    assert_eq!(
        <ConfItem as Into<Vec<u8>>>::into(ConfItem::C {
            k: p::CONF_CR,
            v: Some(0x1a0)
        }),
        b"cR=000001a0"
    );
    assert_eq!(
        <ConfItem as Into<Vec<u8>>>::into(ConfItem::C {
            k: p::CONF_CRD,
            v: None
        }),
        b"cRd"
    );
    assert_eq!(
        <ConfItem as Into<Vec<u8>>>::into(ConfItem::C {
            k: p::CONF_CRD,
            v: Some(0)
        }),
        b"cRd=00000000"
    );
    assert_eq!(
        <ConfItem as Into<Vec<u8>>>::into(ConfItem::C {
            k: p::CONF_CTB,
            v: None
        }),
        b"cTB"
    );
    assert_eq!(
        <ConfItem as Into<Vec<u8>>>::into(ConfItem::C {
            k: p::CONF_CTB,
            v: Some(0x01020304)
        }),
        b"cTB=01020304"
    );
    assert_eq!(
        <ConfItem as Into<Vec<u8>>>::into(ConfItem::C {
            k: p::CONF_CRB,
            v: None
        }),
        b"cRB"
    );
    assert_eq!(
        <ConfItem as Into<Vec<u8>>>::into(ConfItem::C {
            k: p::CONF_CRB,
            v: Some(0xf0e0d0c0)
        }),
        b"cRB=f0e0d0c0"
    );
}

// `Vec<u8>` 转为 ConfItem
#[test]
fn u8_conf() {
    assert_eq!(
        <Vec<u8> as Into<ConfItemO>>::into(Vec::from(b"a" as &[u8])),
        ConfItemO(Some(ConfItem::K {
            k: Vec::from(b"a" as &[u8]),
            v: None
        }))
    );
    assert_eq!(
        <Vec<u8> as Into<ConfItemO>>::into(Vec::from(b"a=1" as &[u8])),
        ConfItemO(Some(ConfItem::K {
            k: Vec::from(b"a" as &[u8]),
            v: Some(Vec::from(b"1" as &[u8]))
        }))
    );

    assert_eq!(
        <Vec<u8> as Into<ConfItemO>>::into(Vec::from(b"m" as &[u8])),
        ConfItemO(Some(ConfItem::M1(None)))
    );
    assert_eq!(
        <Vec<u8> as Into<ConfItemO>>::into(Vec::from(b"m=0" as &[u8])),
        ConfItemO(Some(ConfItem::M1(Some(0))))
    );
    assert_eq!(
        <Vec<u8> as Into<ConfItemO>>::into(Vec::from(b"T" as &[u8])),
        ConfItemO(Some(ConfItem::T(None)))
    );
    assert_eq!(
        <Vec<u8> as Into<ConfItemO>>::into(Vec::from(b"T=0123abcdef" as &[u8])),
        ConfItemO(Some(ConfItem::T(Some(vec![0x01, 0x23, 0xab, 0xcd, 0xef]))))
    );
    assert_eq!(
        <Vec<u8> as Into<ConfItemO>>::into(Vec::from(b"t" as &[u8])),
        ConfItemO(Some(ConfItem::T1(None)))
    );
    assert_eq!(
        <Vec<u8> as Into<ConfItemO>>::into(Vec::from(b"t=cd10" as &[u8])),
        ConfItemO(Some(ConfItem::T1(Some(0xcd10))))
    );
    assert_eq!(
        <Vec<u8> as Into<ConfItemO>>::into(Vec::from(b"I" as &[u8])),
        ConfItemO(Some(ConfItem::I(None)))
    );
    assert_eq!(
        <Vec<u8> as Into<ConfItemO>>::into(Vec::from(b"I=00abcdef00000001" as &[u8])),
        ConfItemO(Some(ConfItem::I(Some(0xabcdef00000001))))
    );
    assert_eq!(
        <Vec<u8> as Into<ConfItemO>>::into(Vec::from(b"O" as &[u8])),
        ConfItemO(Some(ConfItem::O(None)))
    );
    assert_eq!(
        <Vec<u8> as Into<ConfItemO>>::into(Vec::from(b"O=78" as &[u8])),
        ConfItemO(Some(ConfItem::O(Some(0x78))))
    );
    assert_eq!(
        <Vec<u8> as Into<ConfItemO>>::into(Vec::from(b"On" as &[u8])),
        ConfItemO(Some(ConfItem::On(None)))
    );
    assert_eq!(
        <Vec<u8> as Into<ConfItemO>>::into(Vec::from(b"On=03" as &[u8])),
        ConfItemO(Some(ConfItem::On(Some(3))))
    );
    assert_eq!(
        <Vec<u8> as Into<ConfItemO>>::into(Vec::from(b"@" as &[u8])),
        ConfItemO(Some(ConfItem::At(None)))
    );
    assert_eq!(
        <Vec<u8> as Into<ConfItemO>>::into(Vec::from(b"@=0" as &[u8])),
        ConfItemO(Some(ConfItem::At(Some(0))))
    );
    // TODO @sN @nN

    // ConfItem::C
    assert_eq!(
        <Vec<u8> as Into<ConfItemO>>::into(Vec::from(b"cT" as &[u8])),
        ConfItemO(Some(ConfItem::C {
            k: p::CONF_CT,
            v: None
        }))
    );
    assert_eq!(
        <Vec<u8> as Into<ConfItemO>>::into(Vec::from(b"cT=0000ab12" as &[u8])),
        ConfItemO(Some(ConfItem::C {
            k: p::CONF_CT,
            v: Some(0xab12)
        }))
    );
    assert_eq!(
        <Vec<u8> as Into<ConfItemO>>::into(Vec::from(b"cR" as &[u8])),
        ConfItemO(Some(ConfItem::C {
            k: p::CONF_CR,
            v: None
        }))
    );
    assert_eq!(
        <Vec<u8> as Into<ConfItemO>>::into(Vec::from(b"cR=0a001122" as &[u8])),
        ConfItemO(Some(ConfItem::C {
            k: p::CONF_CR,
            v: Some(0x0a001122)
        }))
    );
    assert_eq!(
        <Vec<u8> as Into<ConfItemO>>::into(Vec::from(b"cRd" as &[u8])),
        ConfItemO(Some(ConfItem::C {
            k: p::CONF_CRD,
            v: None
        }))
    );
    assert_eq!(
        <Vec<u8> as Into<ConfItemO>>::into(Vec::from(b"cRd=00000020" as &[u8])),
        ConfItemO(Some(ConfItem::C {
            k: p::CONF_CRD,
            v: Some(0x20)
        }))
    );
    assert_eq!(
        <Vec<u8> as Into<ConfItemO>>::into(Vec::from(b"cTB" as &[u8])),
        ConfItemO(Some(ConfItem::C {
            k: p::CONF_CTB,
            v: None
        }))
    );
    assert_eq!(
        <Vec<u8> as Into<ConfItemO>>::into(Vec::from(b"cTB=0000ab12" as &[u8])),
        ConfItemO(Some(ConfItem::C {
            k: p::CONF_CTB,
            v: Some(0xab12)
        }))
    );
    assert_eq!(
        <Vec<u8> as Into<ConfItemO>>::into(Vec::from(b"cRB" as &[u8])),
        ConfItemO(Some(ConfItem::C {
            k: p::CONF_CRB,
            v: None
        }))
    );
    assert_eq!(
        <Vec<u8> as Into<ConfItemO>>::into(Vec::from(b"cRB=fedcba98" as &[u8])),
        ConfItemO(Some(ConfItem::C {
            k: p::CONF_CRB,
            v: Some(0xfedcba98)
        }))
    );
}
