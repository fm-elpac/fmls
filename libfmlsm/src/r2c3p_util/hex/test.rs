use super::*;

#[test]
fn fifo2() {
    let mut f = Fifo2::new();
    assert_eq!(f.feed(1), None);
    assert_eq!(f.feed(2), None);
    assert_eq!(f.feed(3), Some(1));
    assert_eq!(f.feed(4), Some(2));
    assert_eq!(f.feed(5), Some(3));
    assert_eq!(f.feed(6), Some(4));
    assert_eq!(f.feed(7), Some(5));
    assert_eq!(f.feed(8), Some(6));
}

#[test]
fn fifo4() {
    let mut f = Fifo4::new();
    assert_eq!(f.feed(1), None);
    assert_eq!(f.feed(2), None);
    assert_eq!(f.feed(3), None);
    assert_eq!(f.feed(4), None);
    assert_eq!(f.feed(5), Some(1));
    assert_eq!(f.feed(6), Some(2));
    assert_eq!(f.feed(7), Some(3));
    assert_eq!(f.feed(8), Some(4));
    assert_eq!(f.feed(9), Some(5));
    assert_eq!(f.feed(10), Some(6));
    assert_eq!(f.feed(11), Some(7));
    assert_eq!(f.feed(12), Some(8));
    assert_eq!(f.feed(13), Some(9));
    assert_eq!(f.feed(14), Some(10));
    assert_eq!(f.feed(15), Some(11));
    assert_eq!(f.feed(16), Some(12));
}

#[test]
fn u16le_sender() {
    let mut s = U16LeSender::new(0x1234);
    assert_eq!(s.next(), Some(0x34));
    assert_eq!(s.next(), Some(0x12));
    assert_eq!(s.next(), None);
    assert_eq!(s.next(), None);
}

#[test]
fn u32le_sender() {
    let mut s = U32LeSender::new(0x1234abcd);
    assert_eq!(s.next(), Some(0xcd));
    assert_eq!(s.next(), Some(0xab));
    assert_eq!(s.next(), Some(0x34));
    assert_eq!(s.next(), Some(0x12));
    assert_eq!(s.next(), None);
    assert_eq!(s.next(), None);
}

#[test]
fn hex_u8_sender() {
    let mut s = HexU8Sender::new(0x1c);
    assert_eq!(s.next(), Some(b'1'));
    assert_eq!(s.next(), Some(b'c'));
    assert_eq!(s.next(), None);
    assert_eq!(s.next(), None);
}

#[test]
fn hex_u16_sender() {
    let mut s = HexU16Sender::new(0x12ab);
    assert_eq!(s.next(), Some(b'1'));
    assert_eq!(s.next(), Some(b'2'));
    assert_eq!(s.next(), Some(b'a'));
    assert_eq!(s.next(), Some(b'b'));
    assert_eq!(s.next(), None);
    assert_eq!(s.next(), None);
}

#[test]
fn hex_u32_sender() {
    let mut s = HexU32Sender::new(0x1234abcd);
    assert_eq!(s.next(), Some(b'1'));
    assert_eq!(s.next(), Some(b'2'));
    assert_eq!(s.next(), Some(b'3'));
    assert_eq!(s.next(), Some(b'4'));
    assert_eq!(s.next(), Some(b'a'));
    assert_eq!(s.next(), Some(b'b'));
    assert_eq!(s.next(), Some(b'c'));
    assert_eq!(s.next(), Some(b'd'));
    assert_eq!(s.next(), None);
    assert_eq!(s.next(), None);
}

#[test]
fn hex_u64_sender() {
    let mut s = HexU64Sender::new(0x1234567890abcdef);
    assert_eq!(s.next(), Some(b'1'));
    assert_eq!(s.next(), Some(b'2'));
    assert_eq!(s.next(), Some(b'3'));
    assert_eq!(s.next(), Some(b'4'));
    assert_eq!(s.next(), Some(b'5'));
    assert_eq!(s.next(), Some(b'6'));
    assert_eq!(s.next(), Some(b'7'));
    assert_eq!(s.next(), Some(b'8'));
    assert_eq!(s.next(), Some(b'9'));
    assert_eq!(s.next(), Some(b'0'));
    assert_eq!(s.next(), Some(b'a'));
    assert_eq!(s.next(), Some(b'b'));
    assert_eq!(s.next(), Some(b'c'));
    assert_eq!(s.next(), Some(b'd'));
    assert_eq!(s.next(), Some(b'e'));
    assert_eq!(s.next(), Some(b'f'));
    assert_eq!(s.next(), None);
    assert_eq!(s.next(), None);
}

#[test]
fn vec_sender() {
    let mut s = VecSender::new(b"test 666");
    assert_eq!(s.next(), Some(b't'));
    assert_eq!(s.next(), Some(b'e'));
    assert_eq!(s.next(), Some(b's'));
    assert_eq!(s.next(), Some(b't'));
    assert_eq!(s.next(), Some(b' '));
    assert_eq!(s.next(), Some(b'6'));
    assert_eq!(s.next(), Some(b'6'));
    assert_eq!(s.next(), Some(b'6'));
    assert_eq!(s.next(), None);
    assert_eq!(s.next(), None);
}

#[test]
fn n_u8_sender() {
    let mut s = NU8Sender::new(0);
    assert_eq!(s.next(), Some(b'0'));
    assert_eq!(s.next(), None);
    assert_eq!(s.next(), None);

    let mut s = NU8Sender::new(1);
    assert_eq!(s.next(), Some(b'1'));
    assert_eq!(s.next(), None);
    assert_eq!(s.next(), None);

    let mut s = NU8Sender::new(2);
    assert_eq!(s.next(), Some(b'2'));
    assert_eq!(s.next(), None);
    assert_eq!(s.next(), None);

    let mut s = NU8Sender::new(9);
    assert_eq!(s.next(), Some(b'9'));
    assert_eq!(s.next(), None);
    assert_eq!(s.next(), None);

    let mut s = NU8Sender::new(10);
    assert_eq!(s.next(), Some(b'1'));
    assert_eq!(s.next(), Some(b'0'));
    assert_eq!(s.next(), None);
    assert_eq!(s.next(), None);

    let mut s = NU8Sender::new(11);
    assert_eq!(s.next(), Some(b'1'));
    assert_eq!(s.next(), Some(b'1'));
    assert_eq!(s.next(), None);
    assert_eq!(s.next(), None);

    let mut s = NU8Sender::new(90);
    assert_eq!(s.next(), Some(b'9'));
    assert_eq!(s.next(), Some(b'0'));
    assert_eq!(s.next(), None);
    assert_eq!(s.next(), None);

    let mut s = NU8Sender::new(99);
    assert_eq!(s.next(), Some(b'9'));
    assert_eq!(s.next(), Some(b'9'));
    assert_eq!(s.next(), None);
    assert_eq!(s.next(), None);

    let mut s = NU8Sender::new(100);
    assert_eq!(s.next(), Some(b'1'));
    assert_eq!(s.next(), Some(b'0'));
    assert_eq!(s.next(), Some(b'0'));
    assert_eq!(s.next(), None);
    assert_eq!(s.next(), None);

    let mut s = NU8Sender::new(101);
    assert_eq!(s.next(), Some(b'1'));
    assert_eq!(s.next(), Some(b'0'));
    assert_eq!(s.next(), Some(b'1'));
    assert_eq!(s.next(), None);
    assert_eq!(s.next(), None);

    let mut s = NU8Sender::new(123);
    assert_eq!(s.next(), Some(b'1'));
    assert_eq!(s.next(), Some(b'2'));
    assert_eq!(s.next(), Some(b'3'));
    assert_eq!(s.next(), None);
    assert_eq!(s.next(), None);

    let mut s = NU8Sender::new(199);
    assert_eq!(s.next(), Some(b'1'));
    assert_eq!(s.next(), Some(b'9'));
    assert_eq!(s.next(), Some(b'9'));
    assert_eq!(s.next(), None);
    assert_eq!(s.next(), None);

    let mut s = NU8Sender::new(200);
    assert_eq!(s.next(), Some(b'2'));
    assert_eq!(s.next(), Some(b'0'));
    assert_eq!(s.next(), Some(b'0'));
    assert_eq!(s.next(), None);
    assert_eq!(s.next(), None);

    let mut s = NU8Sender::new(222);
    assert_eq!(s.next(), Some(b'2'));
    assert_eq!(s.next(), Some(b'2'));
    assert_eq!(s.next(), Some(b'2'));
    assert_eq!(s.next(), None);
    assert_eq!(s.next(), None);

    let mut s = NU8Sender::new(254);
    assert_eq!(s.next(), Some(b'2'));
    assert_eq!(s.next(), Some(b'5'));
    assert_eq!(s.next(), Some(b'4'));
    assert_eq!(s.next(), None);
    assert_eq!(s.next(), None);

    let mut s = NU8Sender::new(255);
    assert_eq!(s.next(), Some(b'2'));
    assert_eq!(s.next(), Some(b'5'));
    assert_eq!(s.next(), Some(b'5'));
    assert_eq!(s.next(), None);
    assert_eq!(s.next(), None);
}

#[test]
fn u8_sender() {
    let mut s = U8Sender::new(0x02);
    assert_eq!(s.next(), Some(0x02));
    assert_eq!(s.next(), None);
    assert_eq!(s.next(), None);
}
