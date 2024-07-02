/// Sensor type tags.
pub enum SensorType {
    /// A sensor that measures the gravity vector, typically expressed in "g".
    Gravity,
    /// A sensor that measures magnetic field strength, typically expressed in units auf Milli-Gauss (mG).
    MagneticFieldStrength,
    /// A sensor that measures temperature, typically expressed in °C.
    Temperature,
    /// A sensor that measures angular acceleration, typically expressed in degrees/second.
    AngularAcceleration,
}

/// Sensor type tags.
pub enum ValueType {
    /// 8-bit integer per component
    Int8 = 0x00,
    /// 16-bit integer per component
    Int16 = 0x01,
    /// 32-bit integer per component
    Int32 = 0x02,
}

/// Sensor type information.
pub trait TypeInformation {
    /// The sensor type.
    const SENSOR: SensorType;
    /// The field type.
    const FIELD: ValueType;
    /// The number of components of the vector.
    const NUM_COMPONENTS: usize;

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
    ($comment:literal, $type:tt, $sensor:expr, $value:expr, $num_components:literal) => {
        #[doc = $comment]
        pub struct $type;

        impl $crate::types::TypeInformation for $type {
            const SENSOR: $crate::types::SensorType = $sensor;
            const FIELD: $crate::types::ValueType = $value;
            const NUM_COMPONENTS: usize = $num_components;
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
    "Acceleration / gravity data, 3×`i16`",
    AccelerometerI16,
    SensorType::Gravity,
    ValueType::Int16,
    3
);

impl_type!(
    "Magnetic field strength data, 3×`i16`",
    MagnetometerI16,
    SensorType::MagneticFieldStrength,
    ValueType::Int16,
    3
);

impl_type!(
    "Temperature data, 1×`i16`",
    TemperatureI16,
    SensorType::Temperature,
    ValueType::Int16,
    1
);

impl_type!(
    "Angular acceleration data, 3×`i16`",
    GyroscopeI16,
    SensorType::AngularAcceleration,
    ValueType::Int16,
    3
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SERIALIZATION_CONFIG;

    #[test]
    fn test_accelerometer_data_i16_serialization() {
        let input_data = AccelerometerI16;

        // The deserialization target buffer.
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
