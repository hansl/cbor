#![cfg(feature = "std")]
use crate::serialize::owned::OwnedValue;
use crate::serialize::values::Value;

pub(crate) fn hex_decode<T: AsRef<[u8]>>(hex: T) -> Vec<u8> {
    fn value_of(c: u8) -> u8 {
        match c {
            b'A'..=b'F' => (c - b'A' + 10),
            b'a'..=b'f' => (c - b'a' + 10),
            b'0'..=b'9' => (c - b'0'),
            c => panic!("Invalid hex value {}", c),
        }
    }

    let hex = hex.as_ref();

    // If you mess this up, you don't deserve tests.
    assert_eq!(hex.len() % 2, 0);

    hex.chunks(2)
        .enumerate()
        .map(|(_, pair)| value_of(pair[0]) << 4 | value_of(pair[1]))
        .collect()
}

#[cfg(feature = "std")]
pub(crate) fn assert_value_owned<T: AsRef<[u8]>>(value: OwnedValue, hex: T) {
    let vector = value.to_vec();
    assert_eq!(vector, hex_decode(hex), "{:?}", value);
    assert_eq!(vector.len(), value.len(), "length");
}

pub(crate) fn assert_value<T: AsRef<[u8]>>(value: Value, hex: T) {
    let vector = value.to_vec();
    assert_eq!(vector, hex_decode(hex), "{:?}", value);
    assert_eq!(vector.len(), value.len(), "length");
}

pub(crate) fn assert_peek_simple<'a, DT, ValueFn, PeekFn>(data: DT, value: ValueFn, peek: PeekFn)
where
    DT: 'a + std::cmp::PartialEq + std::fmt::Debug + Copy,
    ValueFn: Fn(DT) -> OwnedValue,
    PeekFn: Fn(&'a [u8]) -> Option<OwnedValue>,
{
    let value = value(data);
    let vector = value.to_vec();

    // Check deserialization.
    let x = peek(unsafe { &*(vector.as_ref() as *const [u8]) });
    assert_eq!(x, Some(value));
}

#[cfg(feature = "std")]
pub(crate) fn assert_peek<T: AsRef<[u8]>, PeekFn>(value: OwnedValue, hex: T, peek: PeekFn)
where
    PeekFn: Fn(&[u8]) -> Option<OwnedValue>,
{
    let vector = value.to_vec();
    assert_eq!(vector, hex_decode(hex), "{:?}", value);

    // Check deserialization.
    let x = peek(unsafe { &*(vector.as_ref() as *const [u8]) });
    assert_eq!(x, Some(value));
}
