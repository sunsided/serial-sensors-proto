#![no_std]
#![deny(unsafe_code)]

use crate::protocol_version::ProtocolVersion;
use crate::types::TypeInformation;
use bincode::config::{Configuration, Fixint, LittleEndian};

pub mod protocol_version;
pub mod scalar;
pub mod types;
pub mod vector3;
mod vector4;

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
pub trait DataFrame {}

/// A sensor data frame.
pub struct Version1DataFrame<T>
where
    T: TypeInformation,
{
    /// A sequence identifier, monotonically increasing.
    ///
    /// This value can be used to detect package loss on the receiver side. It should increase
    /// on every transmitted package, across all sensor.
    ///
    /// If unsupported, set to [`u32::MAX`].
    pub sequence: u32,

    /// A sensor sequence identifier, monotonically increasing.
    ///
    /// This value should increase whenever new data became available for the specific
    /// sensor, not when it was actually transmitted.
    ///
    /// If unsupported, set to [`u32::MAX`].
    pub sensor_sequence: u32,

    /// A device-specific tag for a specific sensor.
    ///
    /// This value should be identical across all readings from the same sensor. This
    /// is to ensure that multiple sensors of the same type, e.g. multiple accelerometers,
    /// can be told apart on the host side.
    pub sensor_tag: u16,

    /// The sensor reading.
    pub value: T::Target,
}

impl<T> DataFrame for Version1DataFrame<T> where T: TypeInformation {}

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
    use crate::protocol_version::Version1;
    use crate::types::AccelerometerI16;
    use crate::vector3::Vector3Data;

    #[test]
    fn test() {
        let frame = VersionedDataFrame {
            version: Version1,
            data: Version1DataFrame::<AccelerometerI16> {
                sequence: u32::MAX,
                sensor_sequence: u32::MAX,
                sensor_tag: 0,
                value: Vector3Data {
                    x: 0,
                    y: -1,
                    z: 2
                }
            },
        };
    }
}
