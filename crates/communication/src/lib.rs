#![no_std]

use crc::{CRC_32_ISCSI, Crc};

use postcard::ser_flavors::{Cobs, Slice, crc::CrcModifier};

mod macros;

pub mod tool_messages;

static CRC32: Crc<u32> = Crc::<u32>::new(&CRC_32_ISCSI);

pub use postcard::{from_bytes_cobs, to_slice_cobs};

pub fn to_slice_cobs_crc32<'a, T: serde::Serialize>(
    value: &T,
    buf: &'a mut [u8],
) -> postcard::Result<&'a mut [u8]> {
    let flavor = CrcModifier::new(Cobs::try_new(Slice::new(buf))?, CRC32.digest());
    postcard::serialize_with_flavor(value, flavor)
}

// TODO: test that bad crc32 errs out, and implement different error for
// crc32 vs other errors.
pub fn from_slice_cobs_crc32<'a, T: serde::Deserialize<'a>>(
    frame: &'a mut [u8],
) -> postcard::Result<T> {
    let len = cobs::decode_in_place(frame).map_err(|_| postcard::Error::DeserializeBadEncoding)?;
    postcard::from_bytes_crc32(&frame[..len], CRC32.digest())
}

#[cfg(test)]
mod tests {

    use crate::macros::{message, message_derives};

    message_derives! {
        pub enum Test1 {
            A,
            B,
            C(u8),
            D,
        }

        pub struct Test2 {
            number: Option<i16>,
            boolean: bool,
        }
    }

    message! {
        pub TestMessage {
            Test1,
            Test2
        }
    }
    #[test]
    fn round_trip_raw_type() {
        let original = [Test1::A, Test1::B, Test1::C(17), Test1::D];

        let mut buf = [0; 128];

        for value in original.iter() {
            assert_eq!(
                *value,
                crate::from_slice_cobs_crc32::<Test1>(
                    crate::to_slice_cobs_crc32(value, &mut buf).unwrap()
                )
                .unwrap()
            );
        }
    }

    #[test]
    fn round_trip_both_types() {
        let original = [
            TestMessage::Test1(Test1::A),
            TestMessage::Test2(Test2 {
                number: None,
                boolean: true,
            }),
            TestMessage::Test1(Test1::C(8)),
            TestMessage::Test2(Test2 {
                number: Some(17),
                boolean: true,
            }),
            TestMessage::Test2(Test2 {
                number: Some(-1432),
                boolean: false,
            }),
            TestMessage::Test1(Test1::A),
        ];

        let mut buf = [0; 1024];

        for value in original.iter() {
            assert_eq!(
                *value,
                crate::from_slice_cobs_crc32::<TestMessage>(
                    crate::to_slice_cobs_crc32(value, &mut buf).unwrap()
                )
                .unwrap()
            )
        }
    }
}
