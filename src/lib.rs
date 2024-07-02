#![no_std]
#![deny(unsafe_code)]

use crate::versions::ProtocolVersion;
use bincode::config::{Configuration, Fixint, LittleEndian};

pub mod scalar;
pub mod types;
pub mod vector3;
mod vector4;
pub mod versions;

/// The serialization configuration.
const SERIALIZATION_CONFIG: Configuration<LittleEndian, Fixint> = bincode::config::standard()
    .with_fixed_int_encoding()
    .with_little_endian()
    .with_no_limit();

/// A versioned data frame.
pub struct VersionedDataFrame<V, D>
where
    V: ProtocolVersion,
    D: DataFrame,
{
    /// A protocol version byte. Always set to one.
    pub version: V,

    /// The data frame.
    pub data: D,
}

/// Marker type for data frames.
pub trait DataFrame: Sized {
    type ProtocolVersion: ProtocolVersion;

    fn into_versioned(self) -> VersionedDataFrame<Self::ProtocolVersion, Self> {
        VersionedDataFrame {
            version: Self::ProtocolVersion::default(),
            data: self,
        }
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::AccelerometerI16;
    use crate::vector3::Vector3Data;
    use crate::versions::{Version1, Version1DataFrame};

    #[test]
    fn frame_from_version() {
        let _frame = Version1::frame(Version1DataFrame::<AccelerometerI16> {
            sequence: u32::MAX,
            sensor_sequence: u32::MAX,
            sensor_tag: 0,
            value: Vector3Data { x: 0, y: -1, z: 2 },
        });
    }

    #[test]
    fn into_versioned() {
        let frame = Version1DataFrame::<AccelerometerI16> {
            sequence: u32::MAX,
            sensor_sequence: u32::MAX,
            sensor_tag: 0,
            value: Vector3Data { x: 0, y: -1, z: 2 },
        };

        let versioned = frame.into_versioned();
        assert_eq!(versioned.version, Version1);
    }
}
