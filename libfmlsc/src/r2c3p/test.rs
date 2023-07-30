extern crate std;
use super::*;
use std::{format, vec::Vec};

use crc::Crc;

#[test]
fn byte_value() {
    assert_eq!(BYTE_LF, 0x0a);
    assert_eq!(BYTE_B, 0x5c);
    assert_eq!(BYTE_N, 0x6e);
    assert_eq!(BYTE_S, 0x73);
    assert_eq!(BYTE_SPACE, 0x20);
    assert_eq!(BYTE_EQ, 0x3d);

    assert_eq!(MSGT_REQ_S, 0x61);
    assert_eq!(MSGT_REQ_E, 0x7a);
    assert_eq!(MSGT_RES_S, 0x41);
    assert_eq!(MSGT_RES_E, 0x5a);
}

#[test]
fn msg_type() {
    assert_eq!(MSGT_V_R, 0x76);
    assert_eq!(MSGT_V, 0x56);
    assert_eq!(MSGT_E, 0x45);
    assert_eq!(MSGT_K, 0x4b);
    assert_eq!(MSGT_C_R, 0x63);
    assert_eq!(MSGT_C, 0x43);
    assert_eq!(MSGT_AT, 0x40);
}

#[test]
fn byte_hex() {
    assert_eq!(BYTE_HEX[0], b'0');
    assert_eq!(BYTE_HEX[1], b'1');
    assert_eq!(BYTE_HEX[2], b'2');
    assert_eq!(BYTE_HEX[3], b'3');
    assert_eq!(BYTE_HEX[4], b'4');
    assert_eq!(BYTE_HEX[5], b'5');
    assert_eq!(BYTE_HEX[6], b'6');
    assert_eq!(BYTE_HEX[7], b'7');
    assert_eq!(BYTE_HEX[8], b'8');
    assert_eq!(BYTE_HEX[9], b'9');
    assert_eq!(BYTE_HEX[10], b'a');
    assert_eq!(BYTE_HEX[11], b'b');
    assert_eq!(BYTE_HEX[12], b'c');
    assert_eq!(BYTE_HEX[13], b'd');
    assert_eq!(BYTE_HEX[14], b'e');
    assert_eq!(BYTE_HEX[15], b'f');
}

#[test]
fn error_code() {
    fn c(i: i8) -> Vec<u8> {
        format!("{}", i).as_bytes().to_vec()
    }

    assert_eq!(c(E_1), EB_1);
    assert_eq!(c(E_2), EB_2);
    assert_eq!(c(E_3), EB_3);
    assert_eq!(c(E_4), EB_4);
    assert_eq!(c(E_5), EB_5);
}

#[test]
fn crc32() {
    let c = Crc::<u32>::new(&CRC_32);
    assert_eq!(c.checksum(P_VERSION), 0x71390093);
}

#[test]
fn crc16() {
    let c = Crc::<u16>::new(&CRC_16);
    assert_eq!(c.checksum(b"v"), 0xe681);
}

#[test]
fn r2_hub() {
    assert_eq!(R2_HUB_NID_S, 0x31);
}
