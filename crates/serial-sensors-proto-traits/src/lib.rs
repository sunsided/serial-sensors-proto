#![no_std]
#![deny(unsafe_code)]

/// A protocol version.
pub trait ProtocolVersion: Default {
    /// The protocol version
    const VERSION: usize;

    /// Returns the protocol version
    fn version(&self) -> usize {
        Self::VERSION
    }
}

/// Sensor type tags.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum SensorType {
    /// The protocol version.
    ProtocolVersion = 0x01,
    /// The system clock frequency, expressed in Hertz (Hz).
    SystemClockFrequency = 0x02,
    /// A sensor that measures the gravity vector, typically expressed in "g".
    Gravity = 0x42,
    /// A sensor that measures magnetic field strength, typically expressed in units auf Milli-Gauss (mG).
    MagneticFieldStrength = 0x43,
    /// A sensor that measures temperature, typically expressed in °C.
    Temperature = 0x44,
    /// A sensor that measures angular acceleration, typically expressed in degrees/second.
    AngularAcceleration = 0x45,
    /// Euler angles, in radians.
    EulerAngles = 0xF0,
    /// An orientation quaternion.
    OrientationQuaternion = 0xF1,
}

/// Sensor type tags.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum ValueType {
    /// Unsigned 8-bit integer per component
    UInt8 = 0x01,
    /// Signed 8-bit integer per component
    SInt8 = 0x02,
    /// Unsigned 16-bit integer per component
    UInt16 = 0x03,
    /// Signed 16-bit integer per component
    SInt16 = 0x04,
    /// Unsigned 32-bit integer per component
    UInt32 = 0x05,
    /// Signed 32-bit integer per component
    SInt32 = 0x06,
    /// Unsigned 32-bit integer per component
    UInt64 = 0x07,
    /// Signed 32-bit integer per component
    SInt64 = 0x08,
    /// Unsigned 32-bit integer per component
    UInt128 = 0x09,
    /// Signed 32-bit integer per component
    SInt128 = 0x0A,
    /// 32-bit floating point per component
    Float32 = 0x0B,
    /// 64-bit floating point per component
    Float64 = 0x0C,
}

/// Sensor type information.
pub trait CompileTimeTypeInformation: Default {
    /// The sensor type.
    const SENSOR: SensorType;

    /// The field type.
    const FIELD: ValueType;

    /// The number of components of the vector.
    const NUM_COMPONENTS: u8;

    /// The fundamental type used to represent the information.
    type Target;
}

/// Sensor type information.
pub trait RuntimeTypeInformation {
    /// Returns the sensor type.
    fn sensor(&self) -> SensorType;

    /// Returns the field value type.
    fn field(&self) -> ValueType;

    /// The number of components
    fn num_components(&self) -> u8;
}
