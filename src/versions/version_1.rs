//! A version 1 data frame.

use crate::types::TypeInformation;
use crate::versions::Version1;
use crate::DataFrame;
use bincode::{Decode, Encode};

/// A sensor data frame.
#[derive(Encode, Debug, Clone, Eq, PartialEq)]
pub struct Version1DataFrame<T>
where
    T: TypeInformation + Encode,
{
    /// A sequence identifier, monotonically increasing.
    ///
    /// This value can be used to detect package loss on the receiver side. It should increase
    /// on every transmitted package, across all sensor.
    ///
    /// If unsupported, set to [`u32::MAX`].
    pub global_sequence: u32,

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

impl<T> DataFrame for Version1DataFrame<T>
where
    T: TypeInformation,
    T::Target: Decode,
{
    type ProtocolVersion = Version1;
}

impl<T> ::bincode::Decode for Version1DataFrame<T>
where
    T: TypeInformation,
    T::Target: Decode,
{
    fn decode<__D: bincode::de::Decoder>(
        decoder: &mut __D,
    ) -> Result<Self, bincode::error::DecodeError> {
        Ok(Self {
            global_sequence: bincode::Decode::decode(decoder)?,
            sensor_sequence: bincode::Decode::decode(decoder)?,
            sensor_tag: bincode::Decode::decode(decoder)?,
            value: bincode::Decode::decode(decoder)?,
        })
    }
}
