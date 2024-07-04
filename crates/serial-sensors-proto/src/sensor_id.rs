use crate::versions::Version1DataFrame;
use crate::ValueType;

/// Identifies a sensor.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct SensorId(u16, u8, ValueType);

impl SensorId {
    /// Constructs a new sensor ID from a [`Version1DataFrame`].
    #[must_use]
    pub fn from(frame: &Version1DataFrame) -> Self {
        Self(
            frame.sensor_tag,
            frame.value.sensor_type_id(),
            frame.value.value_type(),
        )
    }
}

impl From<&Version1DataFrame> for SensorId {
    fn from(value: &Version1DataFrame) -> Self {
        Self::from(value)
    }
}
