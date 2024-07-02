#![no_std]
#![deny(unsafe_code)]

use bincode::config::{Configuration, Fixint, LittleEndian};

pub mod scalar;
mod types;
pub mod vector3;
mod vector4;

/// The serialization configuration.
const SERIALIZATION_CONFIG: Configuration<LittleEndian, Fixint> = bincode::config::standard()
    .with_fixed_int_encoding()
    .with_little_endian()
    .with_no_limit();

struct ScalarData<T> {
    value: T,
}

impl<T> ::bincode::Decode for ScalarData<T>
where
    T: ::bincode::Decode,
{
    fn decode<__D: ::bincode::de::Decoder>(
        decoder: &mut __D,
    ) -> core::result::Result<Self, ::bincode::error::DecodeError> {
        Ok(Self {
            value: ::bincode::Decode::decode(decoder)?,
        })
    }
}
impl<'__de, T> ::bincode::BorrowDecode<'__de> for ScalarData<T>
where
    T: ::bincode::de::BorrowDecode<'__de>,
{
    fn borrow_decode<__D: ::bincode::de::BorrowDecoder<'__de>>(
        decoder: &mut __D,
    ) -> core::result::Result<Self, ::bincode::error::DecodeError> {
        Ok(Self {
            value: ::bincode::BorrowDecode::borrow_decode(decoder)?,
        })
    }
}
