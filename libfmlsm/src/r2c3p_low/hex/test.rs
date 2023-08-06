use super::*;

#[test]
fn test_hex_u8() {
    assert_eq!(hex_u8(b""), None);
    assert_eq!(hex_u8(b"0"), None);
    assert_eq!(hex_u8(b"123"), None);
    assert_eq!(hex_u8(b"0x"), None);

    assert_eq!(hex_u8(b"00"), Some(0x00));
    assert_eq!(hex_u8(b"ff"), Some(0xff));

    assert_eq!(hex_u8(b"01"), Some(0x01));
    assert_eq!(hex_u8(b"23"), Some(0x23));
    assert_eq!(hex_u8(b"45"), Some(0x45));
    assert_eq!(hex_u8(b"67"), Some(0x67));
    assert_eq!(hex_u8(b"89"), Some(0x89));
    assert_eq!(hex_u8(b"ab"), Some(0xab));
    assert_eq!(hex_u8(b"cd"), Some(0xcd));
    assert_eq!(hex_u8(b"ef"), Some(0xef));
}

#[test]
fn test_hex_u16() {
    assert_eq!(hex_u16(b""), None);
    assert_eq!(hex_u16(b"123"), None);

    assert_eq!(hex_u16(b"0123"), Some(0x123));
    assert_eq!(hex_u16(b"4567"), Some(0x4567));
    assert_eq!(hex_u16(b"89ab"), Some(0x89ab));
    assert_eq!(hex_u16(b"cdef"), Some(0xcdef));
}

#[test]
fn test_hex_u32() {
    assert_eq!(hex_u32(b""), None);
    assert_eq!(hex_u32(b"123"), None);

    assert_eq!(hex_u32(b"01234567"), Some(0x1234567));
    assert_eq!(hex_u32(b"89abcdef"), Some(0x89abcdef));
}

#[test]
fn test_hex_u64() {
    assert_eq!(hex_u64(b""), None);
    assert_eq!(hex_u64(b"123"), None);

    assert_eq!(hex_u64(b"123456789abcdef0"), Some(0x123456789abcdef0));
}

#[test]
fn test_n_u8() {
    assert_eq!(n_u8(b""), None);
    assert_eq!(n_u8(b"x"), None);

    assert_eq!(n_u8(b"0"), Some(0));
    assert_eq!(n_u8(b"1"), Some(1));
    assert_eq!(n_u8(b"2"), Some(2));
    assert_eq!(n_u8(b"9"), Some(9));
    assert_eq!(n_u8(b"10"), Some(10));
    assert_eq!(n_u8(b"11"), Some(11));
    assert_eq!(n_u8(b"90"), Some(90));
    assert_eq!(n_u8(b"99"), Some(99));
    assert_eq!(n_u8(b"100"), Some(100));
    assert_eq!(n_u8(b"101"), Some(101));
    assert_eq!(n_u8(b"123"), Some(123));
    assert_eq!(n_u8(b"199"), Some(199));
    assert_eq!(n_u8(b"200"), Some(200));
    assert_eq!(n_u8(b"222"), Some(222));
    assert_eq!(n_u8(b"254"), Some(254));
    assert_eq!(n_u8(b"255"), Some(255));
    assert_eq!(n_u8(b"256"), None);
    assert_eq!(n_u8(b"257"), None);
    assert_eq!(n_u8(b"1000"), None);
}
