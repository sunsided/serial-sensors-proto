//! Provides standard sensor types.

use bincode::{Decode, Encode};
use serial_sensors_proto_derive::SensorDataType;

/// System clock frequency in Hz
#[derive(
    SensorDataType,
    Encode,
    Decode,
    Debug,
    Default,
    Copy,
    Clone,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SystemClockFrequency(crate::ScalarData<u32>);

/// Acceleration / gravity data, 3×`i16`
#[derive(
    SensorDataType,
    Encode,
    Decode,
    Debug,
    Default,
    Copy,
    Clone,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct AccelerometerI16(crate::Vector3Data<i16>);

/// Magnetic field strength data, 3×`i16`.
#[derive(
    SensorDataType,
    Encode,
    Decode,
    Debug,
    Default,
    Copy,
    Clone,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct MagnetometerI16(crate::Vector3Data<i16>);

/// Temperature data, 1×`i16`
#[derive(
    SensorDataType,
    Encode,
    Decode,
    Debug,
    Default,
    Copy,
    Clone,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct TemperatureI16(crate::ScalarData<i16>);

/// Angular acceleration data, 3×`i16`
#[derive(
    SensorDataType,
    Encode,
    Decode,
    Debug,
    Default,
    Copy,
    Clone,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct GyroscopeI16(crate::Vector3Data<i16>);

/// Euler angles, 3×`f32`
#[derive(SensorDataType, Encode, Decode, Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct EulerAnglesF32(crate::Vector3Data<f32>);

/// Orientation quaternion, 4×`f32`
#[derive(SensorDataType, Encode, Decode, Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct OrientationQuaternionF32(crate::Vector4Data<f32>);

/// An identifier.
#[derive(
    SensorDataType, Encode, Decode, Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd,
)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Identification(crate::Identifier<64>);

/// Linear value range description.
#[derive(
    SensorDataType, Encode, Decode, Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd,
)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct LinearRanges(crate::LinearRanges);
