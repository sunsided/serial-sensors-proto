//! A version 1 data frame.

use crate::types::{
    AccelerometerI16, ConstTypeInformation, RuntimeTypeInformation, SensorType, ValueType,
};
use crate::versions::Version1;
use crate::DataFrame;
use bincode::{Decode, Encode};

/// A sensor data frame.
#[derive(Encode, Debug, Clone, PartialEq)]
pub struct Version1DataFrame {
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
    pub value: Version1Data,
}

impl DataFrame for Version1DataFrame {
    type ProtocolVersion = Version1;
}

impl ::bincode::Decode for Version1DataFrame {
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
#[derive(Debug, Clone, PartialEq)]
pub enum Version1Data {
    ProtocolVersion(crate::types::ProtocolVersion),
    SystemClockFrequency(crate::types::SystemClockFrequency),
    AccelerometerI16(crate::types::AccelerometerI16),
    MagnetometerI16(crate::types::MagnetometerI16),
    TemperatureI16(crate::types::TemperatureI16),
    GyroscopeI16(crate::types::GyroscopeI16),
    EulerAnglesF32(crate::types::EulerAnglesF32),
    OrientationQuaternionF32(crate::types::OrientationQuaternionF32),
}

impl RuntimeTypeInformation for Version1Data {
    fn sensor(&self) -> SensorType {
        match self {
            Version1Data::ProtocolVersion(_) => crate::types::ProtocolVersion::SENSOR,
            Version1Data::SystemClockFrequency(_) => crate::types::SystemClockFrequency::SENSOR,
            Version1Data::AccelerometerI16(_) => crate::types::AccelerometerI16::SENSOR,
            Version1Data::MagnetometerI16(_) => crate::types::MagnetometerI16::SENSOR,
            Version1Data::TemperatureI16(_) => crate::types::TemperatureI16::SENSOR,
            Version1Data::GyroscopeI16(_) => crate::types::GyroscopeI16::SENSOR,
            Version1Data::EulerAnglesF32(_) => crate::types::EulerAnglesF32::SENSOR,
            Version1Data::OrientationQuaternionF32(_) => {
                crate::types::OrientationQuaternionF32::SENSOR
            }
        }
    }

    fn field(&self) -> ValueType {
        match self {
            Version1Data::ProtocolVersion(_) => crate::types::ProtocolVersion::FIELD,
            Version1Data::SystemClockFrequency(_) => crate::types::SystemClockFrequency::FIELD,
            Version1Data::AccelerometerI16(_) => crate::types::AccelerometerI16::FIELD,
            Version1Data::MagnetometerI16(_) => crate::types::MagnetometerI16::FIELD,
            Version1Data::TemperatureI16(_) => crate::types::TemperatureI16::FIELD,
            Version1Data::GyroscopeI16(_) => crate::types::GyroscopeI16::FIELD,
            Version1Data::EulerAnglesF32(_) => crate::types::EulerAnglesF32::FIELD,
            Version1Data::OrientationQuaternionF32(_) => {
                crate::types::OrientationQuaternionF32::FIELD
            }
        }
    }

    fn num_components(&self) -> u8 {
        match self {
            Version1Data::ProtocolVersion(_) => crate::types::ProtocolVersion::NUM_COMPONENTS,
            Version1Data::SystemClockFrequency(_) => {
                crate::types::SystemClockFrequency::NUM_COMPONENTS
            }
            Version1Data::AccelerometerI16(_) => crate::types::AccelerometerI16::NUM_COMPONENTS,
            Version1Data::MagnetometerI16(_) => crate::types::MagnetometerI16::NUM_COMPONENTS,
            Version1Data::TemperatureI16(_) => crate::types::TemperatureI16::NUM_COMPONENTS,
            Version1Data::GyroscopeI16(_) => crate::types::GyroscopeI16::NUM_COMPONENTS,
            Version1Data::EulerAnglesF32(_) => crate::types::EulerAnglesF32::NUM_COMPONENTS,
            Version1Data::OrientationQuaternionF32(_) => {
                crate::types::OrientationQuaternionF32::NUM_COMPONENTS
            }
        }
    }
}

impl bincode::Encode for Version1Data {
    fn encode<__E: bincode::enc::Encoder>(
        &self,
        encoder: &mut __E,
    ) -> Result<(), bincode::error::EncodeError> {
        match self {
            Version1Data::ProtocolVersion(value) => bincode::Encode::encode(&value, encoder)?,
            Version1Data::SystemClockFrequency(value) => bincode::Encode::encode(&value, encoder)?,
            Version1Data::AccelerometerI16(value) => bincode::Encode::encode(&value, encoder)?,
            Version1Data::MagnetometerI16(value) => bincode::Encode::encode(&value, encoder)?,
            Version1Data::TemperatureI16(value) => bincode::Encode::encode(&value, encoder)?,
            Version1Data::GyroscopeI16(value) => bincode::Encode::encode(&value, encoder)?,
            Version1Data::EulerAnglesF32(value) => bincode::Encode::encode(&value, encoder)?,
            Version1Data::OrientationQuaternionF32(value) => {
                bincode::Encode::encode(&value, encoder)?
            }
        }

        Ok(())
    }
}

impl ::bincode::Decode for Version1Data {
    fn decode<__D: bincode::de::Decoder>(
        decoder: &mut __D,
    ) -> Result<Self, ::bincode::error::DecodeError> {
        let v: u8 = bincode::Decode::decode(decoder)?;
        todo!()
    }
}
