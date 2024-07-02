#![no_std]
#![deny(unsafe_code)]

use crate::versions::ProtocolVersion;
use bincode::config::{Configuration, Fixint, LittleEndian};
use bincode::Encode;

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
#[derive(Encode, Debug, Clone, PartialEq)]
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

impl<V, D> Eq for VersionedDataFrame<V, D>
where
    V: ProtocolVersion + Eq + PartialEq,
    D: DataFrame + Eq + PartialEq,
{
}

/// Marker type for data frames.
pub trait DataFrame: Sized + bincode::Encode + bincode::Decode {
    type ProtocolVersion: ProtocolVersion;

    fn into_versioned(self) -> VersionedDataFrame<Self::ProtocolVersion, Self> {
        VersionedDataFrame {
            version: Self::ProtocolVersion::default(),
            data: self,
        }
    }
}

impl<V, D> bincode::Decode for VersionedDataFrame<V, D>
where
    V: ProtocolVersion,
    D: DataFrame,
{
    fn decode<__D: bincode::de::Decoder>(
        decoder: &mut __D,
    ) -> Result<Self, ::bincode::error::DecodeError> {
        Ok(Self {
            version: bincode::Decode::decode(decoder)?,
            data: bincode::Decode::decode(decoder)?,
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
            global_sequence: u32::MAX,
            sensor_sequence: u32::MAX,
            sensor_tag: 0,
            value: Vector3Data { x: 0, y: -1, z: 2 },
        });
    }

    #[test]
    fn into_versioned() {
        let frame = Version1DataFrame::<AccelerometerI16> {
            global_sequence: u32::MAX,
            sensor_sequence: 12,
            sensor_tag: 0,
            value: Vector3Data { x: 0, y: -1, z: 2 },
        };

        let versioned = frame.into_versioned();
        assert_eq!(versioned.version, Version1);

        // The serialization target buffer.
        let mut buffer = [0_u8; 1024];

        // Encode
        let num_serialized =
            bincode::encode_into_slice(versioned, &mut buffer, SERIALIZATION_CONFIG)
                .expect("Failed to encode");
        assert_eq!(num_serialized, 1 + 4 + 4 + 2 + 3 * 2);

        // Decode
        let (value, num_read) =
            bincode::decode_from_slice(&buffer, SERIALIZATION_CONFIG).expect("Failed to decode");
        let value: VersionedDataFrame<Version1, Version1DataFrame<AccelerometerI16>> = value;
        assert_eq!(num_read, 17);
        assert_eq!(value.version, Version1);
        assert_eq!(value.data.global_sequence, u32::MAX);
        assert_eq!(value.data.sensor_sequence, 12);
        assert_eq!(value.data.sensor_tag, 0);
        assert_eq!(value.data.value.x, 0);
        assert_eq!(value.data.value.y, -1);
        assert_eq!(value.data.value.z, 2);
    }
}
