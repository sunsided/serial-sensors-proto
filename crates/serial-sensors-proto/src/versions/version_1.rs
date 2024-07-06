//! A version 1 data frame.

use crate::versions::Version1;
use crate::{DataFrame, SensorData, SensorId};
use bincode::Encode;

/// A sensor data frame.
#[derive(Encode, Debug, Clone, PartialEq)]
#[allow(clippy::module_name_repetitions)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Version1DataFrame {
    /// A time in seconds since begin of a measurement, or since startup time.
    ///
    /// If unsupported, set to [`u32::MAX`].
    pub system_secs: u32,

    /// The sub-second milliseconds.
    ///
    /// If unsupported, set to [`u16::MAX`].
    pub system_millis: u16,

    /// The sub-millisecond nanoseconds.
    ///
    /// If unsupported, set to [`u16::MAX`].
    pub system_nanos: u16,

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
    pub value: SensorData,
}

impl DataFrame for Version1DataFrame {
    type ProtocolVersion = Version1;

    fn is_meta(&self) -> bool {
        self.value.is_meta()
    }

    fn target(&self) -> SensorId {
        self.value
            .meta_target()
            .cloned()
            .unwrap_or_else(|| SensorId::from(self))
    }
}

impl Version1DataFrame {
    /// Creates a new instance of the version 1 data frame.
    #[must_use]
    pub fn new<D>(
        system_secs: u32,
        system_millis: u16,
        system_nanos: u16,
        global_sequence: u32,
        sensor_sequence: u32,
        sensor_tag: u16,
        value: D,
    ) -> Self
    where
        D: Into<SensorData>,
    {
        Self::new_with(
            system_secs,
            system_millis,
            system_nanos,
            global_sequence,
            sensor_sequence,
            sensor_tag,
            value.into(),
        )
    }

    /// Creates a new instance of the version 1 data frame.
    #[must_use]
    pub const fn new_with(
        system_secs: u32,
        system_millis: u16,
        system_nanos: u16,
        global_sequence: u32,
        sensor_sequence: u32,
        sensor_tag: u16,
        value: SensorData,
    ) -> Self {
        Self {
            system_secs,
            system_millis,
            system_nanos,
            global_sequence,
            sensor_sequence,
            sensor_tag,
            value,
        }
    }
}

impl ::bincode::Decode for Version1DataFrame {
    fn decode<__D: bincode::de::Decoder>(
        decoder: &mut __D,
    ) -> Result<Self, bincode::error::DecodeError> {
        Ok(Self {
            system_secs: bincode::Decode::decode(decoder)?,
            system_millis: bincode::Decode::decode(decoder)?,
            system_nanos: bincode::Decode::decode(decoder)?,
            global_sequence: bincode::Decode::decode(decoder)?,
            sensor_sequence: bincode::Decode::decode(decoder)?,
            sensor_tag: bincode::Decode::decode(decoder)?,
            value: bincode::Decode::decode(decoder)?,
        })
    }
}
