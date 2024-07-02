/// Sensor type tags.
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
pub trait TypeInformation: Default {
    /// The sensor type.
    const SENSOR: SensorType;
    /// The field type.
    const FIELD: ValueType;
    /// The number of components of the vector.
    const NUM_COMPONENTS: usize;

    /// The fundamental type used to represent the information.
    type Target: ::bincode::Encode;

    /// Returns the sensor type.
    #[inline]
    fn sensor(&self) -> SensorType {
        Self::SENSOR
    }

    /// Returns the field value type.
    #[inline]
    fn field(&self) -> ValueType {
        Self::FIELD
    }

    /// The number of components
    #[inline]
    fn num_components(&self) -> usize {
        Self::NUM_COMPONENTS
    }
}

macro_rules! impl_type {
    ($comment:literal, $type:tt, $sensor:expr, $value:expr, $num_components:literal, $base_type:ty) => {
        #[doc = $comment]
        #[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
        pub struct $type;

        impl $crate::types::TypeInformation for $type {
            const SENSOR: $crate::types::SensorType = $sensor;
            const FIELD: $crate::types::ValueType = $value;
            const NUM_COMPONENTS: usize = $num_components;
            type Target = $base_type;
        }

        impl ::bincode::Encode for $type {
            fn encode<__E: ::bincode::enc::Encoder>(
                &self,
                encoder: &mut __E,
            ) -> core::result::Result<(), ::bincode::error::EncodeError> {
                ::bincode::Encode::encode(&{ ($sensor) as u8 }, encoder)?;
                Ok(())
            }
        }
    };
}

impl_type!(
    "Version tag",
    ProtocolVersion,
    SensorType::ProtocolVersion,
    ValueType::SInt8,
    1,
    crate::scalar::ScalarData<u8>
);

impl_type!(
    "System clock frequency in Hz",
    SystemClockFrequency,
    SensorType::SystemClockFrequency,
    ValueType::UInt32,
    1,
    crate::scalar::ScalarData<u32>
);

impl_type!(
    "Acceleration / gravity data, 3×`i16`",
    AccelerometerI16,
    SensorType::Gravity,
    ValueType::SInt16,
    3,
    crate::vector3::Vector3Data<i16>
);

impl_type!(
    "Magnetic field strength data, 3×`i16`",
    MagnetometerI16,
    SensorType::MagneticFieldStrength,
    ValueType::SInt16,
    3,
    crate::vector3::Vector3Data<i16>
);

impl_type!(
    "Temperature data, 1×`i16`",
    TemperatureI16,
    SensorType::Temperature,
    ValueType::SInt16,
    1,
    crate::scalar::ScalarData<i16>
);

impl_type!(
    "Angular acceleration data, 3×`i16`",
    GyroscopeI16,
    SensorType::AngularAcceleration,
    ValueType::SInt16,
    3,
    crate::vector3::Vector3Data<i16>
);

impl_type!(
    "Euler angles, 3×`f32`",
    EulerAnglesF32,
    SensorType::EulerAngles,
    ValueType::Float32,
    3,
    crate::vector3::Vector3Data<f32>
);

impl_type!(
    "Orientation quaternion, 4×`f32`",
    OrientationQuaternionF32,
    SensorType::OrientationQuaternion,
    ValueType::Float32,
    4,
    crate::vector4::Vector4Data<f32>
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SERIALIZATION_CONFIG;

    #[test]
    fn test_accelerometer_data_i16_serialization() {
        let input_data = AccelerometerI16;

        // The serialization target buffer.
        let mut buffer = [0_u8; 1024];

        // Serialize the data
        let num_serialized =
            bincode::encode_into_slice(input_data, &mut buffer, SERIALIZATION_CONFIG)
                .expect("Failed to serialize");

        // Ensure the serialized length is correct
        assert_eq!(num_serialized, 1);

        // Ensure the serialized content is correct
        let expected_bytes: [u8; 1] = [0x42];
        assert_eq!(&buffer[..num_serialized], &expected_bytes);

        /*
        // Deserialize the data
        let result = bincode::decode_from_slice(&buffer, SERIALIZATION_CONFIG)
            .expect("Failed to deserialize");
        let deserialized: AccelerometerI16 = result.0;
        let count = result.1;

        // Ensure the deserialized content is correct
        assert_eq!(deserialized, input_data);
        assert_eq!(count, 2);
        */
    }
}
