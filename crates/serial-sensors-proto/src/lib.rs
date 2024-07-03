#![no_std]
#![deny(unsafe_code)]

use bincode::config::{Configuration, Fixint, LittleEndian};
use bincode::Encode;

pub mod scalar;
mod test;
pub mod types;
pub mod vector3;
mod vector4;
pub mod versions;

/// The serialization configuration.
const SERIALIZATION_CONFIG: Configuration<LittleEndian, Fixint> = bincode::config::standard()
    .with_fixed_int_encoding()
    .with_little_endian()
    .with_no_limit();

/// A protocol version.
pub trait ProtocolVersion: Default {
    /// The protocol version
    const VERSION: usize;

    /// Returns the protocol version
    fn version(&self) -> usize {
        Self::VERSION
    }
}

/// A versioned data frame.
#[derive(Encode, Debug, Clone, PartialEq)]
pub struct VersionedDataFrame<V, D>
where
    V: ProtocolVersion,
    D: DataFrame,
{
    /// A protocol version byte. Always set to one.
    pub version: V,

    /// The data frame.
    pub data: D,
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
            0x02 => Ok(Self::SInt8),
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
            _ => Err(()),
        }
    }
}

/// Sensor type information.
pub trait CompileTimeTypeInformation: Default {
    /// The sensor type.
    const TYPE_ID: u8;

    /// The field type.
    const VALUE_TYPE: ValueType;

    /// The number of components of the vector.
    const NUM_COMPONENTS: u8;
}

/// Sensor type information.
pub trait RuntimeTypeInformation {
    /// Returns the sensor type ID.
    fn sensor_type_id(&self) -> u8;

    /// Returns the field value type.
    fn value_type(&self) -> ValueType;

    /// The number of components
    fn num_components(&self) -> u8;
}


impl<V, D> Eq for VersionedDataFrame<V, D>
where
    V: ProtocolVersion + Eq + PartialEq,
    D: DataFrame + Eq + PartialEq,
{
}

/// Marker type for data frames.
pub trait DataFrame: Sized + bincode::Encode + bincode::Decode {
    type ProtocolVersion: ProtocolVersion;

    fn into_versioned(self) -> VersionedDataFrame<Self::ProtocolVersion, Self> {
        VersionedDataFrame {
            version: Self::ProtocolVersion::default(),
            data: self,
        }
    }
}

impl<V, D> bincode::Decode for VersionedDataFrame<V, D>
where
    V: ProtocolVersion + ::bincode::Decode,
    D: DataFrame,
{
    fn decode<__D: bincode::de::Decoder>(
        decoder: &mut __D,
    ) -> Result<Self, ::bincode::error::DecodeError> {
        Ok(Self {
            version: bincode::Decode::decode(decoder)?,
            data: bincode::Decode::decode(decoder)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::AccelerometerI16;
    use crate::vector3::Vector3Data;
    use crate::versions::{Version1, Version1DataFrame};

    #[test]
    fn frame_from_version() {
        let _frame = Version1::frame(Version1DataFrame {
            global_sequence: u32::MAX,
            sensor_sequence: u32::MAX,
            sensor_tag: 0,
            value: AccelerometerI16::new(Vector3Data { x: 0, y: -1, z: 2 }).into(),
        });
    }

    #[test]
    fn into_versioned() {
        let frame = Version1DataFrame::new(
            u32::MAX,
            12,
            0,
            AccelerometerI16::new(Vector3Data { x: 0, y: -1, z: 2 }),
        );

        let versioned = frame.into_versioned();
        assert_eq!(versioned.version, Version1);

        // The serialization target buffer.
        let mut buffer = [0_u8; 1024];

        // Encode
        let num_serialized =
            bincode::encode_into_slice(versioned, &mut buffer, SERIALIZATION_CONFIG)
                .expect("Failed to encode");
        assert_eq!(
            num_serialized,
            1 // version
                       + 4 // global sequence
                       + 4 // sensor sequence
                       + 2 // sensor tag
                       + 1 // sensor type
                       + 1 // data type
                       + 3 * 2 // 3-axis data
        );

        // Decode
        let (value, num_read) =
            bincode::decode_from_slice(&buffer, SERIALIZATION_CONFIG).expect("Failed to decode");
        let value: VersionedDataFrame<Version1, Version1DataFrame> = value;
        assert_eq!(num_read, 19);
        assert_eq!(value.version, Version1);
        assert_eq!(value.data.global_sequence, u32::MAX);
        assert_eq!(value.data.sensor_sequence, 12);
        assert_eq!(value.data.sensor_tag, 0);

        let accel: AccelerometerI16 = value.try_into().expect("failed to unwrap");
        assert_eq!(accel.x, 0);
        assert_eq!(accel.y, -1);
        assert_eq!(accel.z, 2);
    }
}
