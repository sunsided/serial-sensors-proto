//! A version 1 data frame.

use crate::types::TypeInformation;
use crate::versions::Version1;
use crate::DataFrame;

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

impl<T> DataFrame for Version1DataFrame<T>
where
    T: TypeInformation,
{
    type ProtocolVersion = Version1;
}
