use crate::versions::Version1DataFrame;
use crate::{RuntimeTypeInformation, ValueType};

/// Identifies a sensor.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SensorId(u16, u8, ValueType, u8);

impl SensorId {
    /// Constructs a new sensor ID from a [`Version1DataFrame`].
    #[must_use]
    pub fn from(frame: &Version1DataFrame) -> Self {
        Self(
            frame.sensor_tag,
            frame.value.sensor_type_id(),
            frame.value.value_type(),
            frame.value.num_components(),
        )
    }

    /// Returns the sensor tag.
    #[must_use]
    pub fn tag(&self) -> u16 {
        self.0
    }

    /// Returns the sensor id.
    #[must_use]
    pub fn id(&self) -> u8 {
        self.1
    }

    /// Returns the sensor's value type.
    #[must_use]
    pub fn value_type(&self) -> ValueType {
        self.2
    }

    /// Returns the number of components of the vector.
    #[must_use]
    pub fn num_components(&self) -> u8 {
        self.3
    }
}

impl From<&Version1DataFrame> for SensorId {
    fn from(value: &Version1DataFrame) -> Self {
        Self::from(value)
    }
}

impl RuntimeTypeInformation for SensorId {
    fn sensor_type_id(&self) -> u8 {
        self.id()
    }

    fn value_type(&self) -> ValueType {
        self.value_type()
    }

    fn num_components(&self) -> u8 {
        self.num_components()
    }
}
