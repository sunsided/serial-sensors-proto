use bincode::{Decode, Encode};
use serial_sensors_proto_derive::SensorDataType;

/// System clock frequency in Hz
#[derive(SensorDataType, Encode, Decode, Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct SystemClockFrequency(crate::scalar::ScalarData<u32>);

/// Acceleration / gravity data, 3×`i16`
#[derive(SensorDataType, Encode, Decode, Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct AccelerometerI16(crate::vector3::Vector3Data<i16>);

/// Magnetic field strength data, 3×`i16`.
#[derive(SensorDataType, Encode, Decode, Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct MagnetometerI16(crate::vector3::Vector3Data<i16>);

/// Temperature data, 1×`i16`
#[derive(SensorDataType, Encode, Decode, Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct TemperatureI16(crate::scalar::ScalarData<i16>);

/// Angular acceleration data, 3×`i16`
#[derive(SensorDataType, Encode, Decode, Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct GyroscopeI16(crate::vector3::Vector3Data<i16>);

/// Euler angles, 3×`f32`
#[derive(SensorDataType, Encode, Decode, Debug, Default, Copy, Clone, PartialEq)]
pub struct EulerAnglesF32(crate::vector3::Vector3Data<f32>);

/// Orientation quaternion, 4×`f32`
#[derive(SensorDataType, Encode, Decode, Debug, Default, Copy, Clone, PartialEq)]
pub struct OrientationQuaternionF32(crate::vector4::Vector4Data<f32>);
