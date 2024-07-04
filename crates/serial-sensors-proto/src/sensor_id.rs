use crate::versions::Version1DataFrame;
use crate::{ComponentLookupError, SensorData, ValueType};
use bincode::{Decode, Encode};

/// Identifies a sensor.
#[derive(Encode, Decode, Default, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SensorId(pub(crate) u16, pub(crate) u8, pub(crate) ValueType);

impl SensorId {
    /// A generic identifier, e.g. for describing a carrier board.
    pub const META_IDENTIFIER: SensorId = SensorId(0x00, 0, ValueType::Identifier);

    /// Constructs a new sensor ID.
    #[must_use]
    pub const fn new_with(sensor_tag: u16, sensor_type_id: u8, value_type: ValueType) -> Self {
        Self(sensor_tag, sensor_type_id, value_type)
    }

    /// Constructs a new sensor ID from a [`Version1DataFrame`].
    #[must_use]
    pub fn from(frame: &Version1DataFrame) -> Self {
        Self(
            frame.sensor_tag,
            frame.value.sensor_type_id(),
            frame.value.value_type(),
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
    ///
    /// ## Errors
    /// The type could not be looked up.
    pub fn num_components(&self) -> Result<u8, ComponentLookupError> {
        SensorData::components(self.1, self.2)
    }
}

impl From<&Version1DataFrame> for SensorId {
    fn from(value: &Version1DataFrame) -> Self {
        Self::from(value)
    }
}
