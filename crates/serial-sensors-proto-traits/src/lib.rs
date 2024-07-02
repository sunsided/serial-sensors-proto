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

impl TryFrom<u8> for ValueType {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::UInt8),
            0x02 =>Ok( Self::SInt8),
            0x03 => Ok(Self::UInt16),
            0x04 => Ok(Self::SInt16),
            0x05 => Ok(Self::UInt32),
            0x06 => Ok(Self::SInt32),
            0x07 => Ok(Self::UInt64),
            0x08 => Ok(Self::SInt64),
            0x09 => Ok(Self::UInt128),
            0x0A => Ok(Self::SInt128),
            0x0B => Ok(Self::Float32),
            0x0C => Ok(Self::Float64),
            _ => Err(())
        }
    }
}

/// Sensor type information.
#[deprecated]
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
#[deprecated]
pub trait RuntimeTypeInformation {
    /// Returns the sensor type.
    fn sensor(&self) -> SensorType;

    /// Returns the field value type.
    fn field(&self) -> ValueType;

    /// The number of components
    fn num_components(&self) -> u8;
}

/// Sensor type information.
pub trait CompileTimeTypeInformation2: Default {
    /// The sensor type.
    const TYPE_ID: u8;

    /// The field type.
    const FIELD: ValueType;

    /// The number of components of the vector.
    const NUM_COMPONENTS: u8;

    /// The fundamental type used to represent the information.
    type Target;
}

/// Sensor type information.
pub trait RuntimeTypeInformation2 {
    /// Returns the sensor type ID.
    fn sensor_type_id(&self) -> u8;

    /// Returns the field value type.
    fn value_type(&self) -> ValueType;

    /// The number of components
    fn num_components(&self) -> u8;
}
