#![cfg(feature = "std")]
use crate::serialize::values;
use crate::test_utils::assert_value;

#[test]
fn usmall() {
    assert_value(values::usmall(0), "00");
    assert_value(values::usmall(9), "09");
    assert_value(values::usmall(22), "16");
    assert_value(values::usmall(23), "17");
    assert_value(values::usmall(24), "17");
    assert_value(values::usmall(90), "17");
}

#[test]
fn u8() {
    assert_value(values::u8(0), "1800");
    assert_value(values::u8(90), "185a");
    assert_value(values::u8(255), "18ff");
}

#[test]
fn u16() {
    assert_value(values::u16(0), "190000");
    assert_value(values::u16(90), "19005a");
    assert_value(values::u16(9000), "192328");
}

#[test]
fn u32() {
    assert_value(values::u32(0), "1a00000000");
    assert_value(values::u32(90), "1a0000005a");
    assert_value(values::u32(900_000), "1a000dbba0");
}

#[test]
fn u64() {
    assert_value(values::u64(0), "1b0000000000000000");
    assert_value(values::u64(90), "1b000000000000005a");
    assert_value(values::u64(900_000), "1b00000000000dbba0");
    assert_value(values::u64(90_000_000_000), "1b00000014f46b0400");
}

#[test]
fn negative_usmall() {
    assert_value(values::negative_usmall(0), "20");
    assert_value(values::negative_usmall(9), "29");
    assert_value(values::negative_usmall(23), "37");
    assert_value(values::negative_usmall(24), "37");
}

#[test]
fn ismall() {
    assert_value(values::ismall(0), "00");
    assert_value(values::ismall(-9), "28");
    assert_value(values::ismall(-23), "36");
    assert_value(values::ismall(-24), "37");
    assert_value(values::ismall(-25), "37");
    assert_value(values::ismall(-90), "37");
    assert_value(values::ismall(9), "09");
    assert_value(values::ismall(22), "16");
    assert_value(values::ismall(23), "17");
    assert_value(values::ismall(24), "17");
    assert_value(values::ismall(90), "17");
}

#[test]
fn i8() {
    assert_value(values::i8(0), "1800");
    assert_value(values::i8(90), "185a");
    assert_value(values::i8(127), "187f");
    assert_value(values::i8(0), "1800");
    assert_value(values::i8(-1), "3800");
    assert_value(values::i8(-90), "3859");
    assert_value(values::i8(-128), "387f");
}

#[test]
fn i16() {
    assert_value(values::i16(0), "190000");
    assert_value(values::i16(90), "19005a");
    assert_value(values::i16(9000), "192328");
    assert_value(values::i16(-1), "390000");
    assert_value(values::i16(-90), "390059");
    assert_value(values::i16(-9000), "392327");
}

#[test]
fn i32() {
    assert_value(values::i32(0), "1a00000000");
    assert_value(values::i32(90), "1a0000005a");
    assert_value(values::i32(9000), "1a00002328");
    assert_value(values::i32(-1), "3a00000000");
    assert_value(values::i32(-90), "3a00000059");
    assert_value(values::i32(-9000), "3a00002327");
}

#[test]
fn i64() {
    assert_value(values::i64(0), "1b0000000000000000");
    assert_value(values::i64(90), "1b000000000000005a");
    assert_value(values::i64(9000), "1b0000000000002328");
    assert_value(values::i64(-1), "3b0000000000000000");
    assert_value(values::i64(-90), "3b0000000000000059");
    assert_value(values::i64(-9000), "3b0000000000002327");
}
