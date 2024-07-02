//! A version 1 data frame.

use crate::types::{ConstTypeInformation, RuntimeTypeInformation, SensorType, ValueType};
use crate::versions::Version1;
use crate::DataFrame;
use bincode::{Decode, Encode};

/// A sensor data frame.
#[derive(Encode, Debug, Clone, Eq, PartialEq)]
pub struct Version1DataFrame<T>
where
    T: ConstTypeInformation + Encode,
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
    T: ConstTypeInformation,
    T::Target: Decode,
{
    type ProtocolVersion = Version1;
}

impl<T> ::bincode::Decode for Version1DataFrame<T>
where
    T: ConstTypeInformation,
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

/// Data formats.
pub enum Data {
    ProtocolVersion(<crate::types::ProtocolVersion as ConstTypeInformation>::Target),
    SystemClockFrequency(<crate::types::SystemClockFrequency as ConstTypeInformation>::Target),
    AccelerometerI16(<crate::types::AccelerometerI16 as ConstTypeInformation>::Target),
    MagnetometerI16(<crate::types::MagnetometerI16 as ConstTypeInformation>::Target),
    TemperatureI16(<crate::types::TemperatureI16 as ConstTypeInformation>::Target),
    GyroscopeI16(<crate::types::GyroscopeI16 as ConstTypeInformation>::Target),
    EulerAnglesF32(<crate::types::EulerAnglesF32 as ConstTypeInformation>::Target),
    OrientationQuaternionF32(
        <crate::types::OrientationQuaternionF32 as ConstTypeInformation>::Target,
    ),
}

impl RuntimeTypeInformation for Data {
    fn sensor(&self) -> SensorType {
        match self {
            Data::ProtocolVersion(_) => crate::types::ProtocolVersion::SENSOR,
            Data::SystemClockFrequency(_) => crate::types::SystemClockFrequency::SENSOR,
            Data::AccelerometerI16(_) => crate::types::AccelerometerI16::SENSOR,
            Data::MagnetometerI16(_) => crate::types::MagnetometerI16::SENSOR,
            Data::TemperatureI16(_) => crate::types::TemperatureI16::SENSOR,
            Data::GyroscopeI16(_) => crate::types::GyroscopeI16::SENSOR,
            Data::EulerAnglesF32(_) => crate::types::EulerAnglesF32::SENSOR,
            Data::OrientationQuaternionF32(_) => crate::types::OrientationQuaternionF32::SENSOR,
        }
    }

    fn field(&self) -> ValueType {
        match self {
            Data::ProtocolVersion(_) => crate::types::ProtocolVersion::FIELD,
            Data::SystemClockFrequency(_) => crate::types::SystemClockFrequency::FIELD,
            Data::AccelerometerI16(_) => crate::types::AccelerometerI16::FIELD,
            Data::MagnetometerI16(_) => crate::types::MagnetometerI16::FIELD,
            Data::TemperatureI16(_) => crate::types::TemperatureI16::FIELD,
            Data::GyroscopeI16(_) => crate::types::GyroscopeI16::FIELD,
            Data::EulerAnglesF32(_) => crate::types::EulerAnglesF32::FIELD,
            Data::OrientationQuaternionF32(_) => crate::types::OrientationQuaternionF32::FIELD,
        }
    }

    fn num_components(&self) -> usize {
        match self {
            Data::ProtocolVersion(_) => crate::types::ProtocolVersion::NUM_COMPONENTS,
            Data::SystemClockFrequency(_) => crate::types::SystemClockFrequency::NUM_COMPONENTS,
            Data::AccelerometerI16(_) => crate::types::AccelerometerI16::NUM_COMPONENTS,
            Data::MagnetometerI16(_) => crate::types::MagnetometerI16::NUM_COMPONENTS,
            Data::TemperatureI16(_) => crate::types::TemperatureI16::NUM_COMPONENTS,
            Data::GyroscopeI16(_) => crate::types::GyroscopeI16::NUM_COMPONENTS,
            Data::EulerAnglesF32(_) => crate::types::EulerAnglesF32::NUM_COMPONENTS,
            Data::OrientationQuaternionF32(_) => {
                crate::types::OrientationQuaternionF32::NUM_COMPONENTS
            }
        }
    }
}

impl bincode::Encode for Data {
    fn encode<__E: bincode::enc::Encoder>(
        &self,
        encoder: &mut __E,
    ) -> Result<(), ::bincode::error::EncodeError> {
        match self {
            Data::ProtocolVersion(value) => bincode::Encode::encode(&value, encoder)?,
            Data::SystemClockFrequency(value) => bincode::Encode::encode(&value, encoder)?,
            Data::AccelerometerI16(value) => bincode::Encode::encode(&value, encoder)?,
            Data::MagnetometerI16(value) => bincode::Encode::encode(&value, encoder)?,
            Data::TemperatureI16(value) => bincode::Encode::encode(&value, encoder)?,
            Data::GyroscopeI16(value) => bincode::Encode::encode(&value, encoder)?,
            Data::EulerAnglesF32(value) => bincode::Encode::encode(&value, encoder)?,
            Data::OrientationQuaternionF32(value) => bincode::Encode::encode(&value, encoder)?,
        }

        Ok(())
    }
}
