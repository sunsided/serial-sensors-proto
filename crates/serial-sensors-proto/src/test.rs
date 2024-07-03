use crate::ValueType;
use serial_sensors_proto_derive::SerialSensors;

/// Data formats.
#[derive(Debug, Clone, PartialEq, SerialSensors)]
pub enum SensorData {
    /// The system clock frequency, expressed in Hertz (Hz).
    #[sensor(id = 0x2, data = ValueType::UInt32, components = 1)]
    SystemClockFrequency(crate::types::SystemClockFrequency),

    /// A sensor that measures the gravity vector, typically expressed in "g".
    #[sensor(id = 0x42, data = ValueType::SInt16, components = 3)]
    AccelerometerI16(crate::types::AccelerometerI16),

    /// A sensor that measures magnetic field strength, typically expressed in units auf Milli-Gauss (mG).
    #[sensor(id = 0x43, data = ValueType::SInt16, components = 3)]
    MagnetometerI16(crate::types::MagnetometerI16),

    /// A sensor that measures temperature, typically expressed in °C.
    #[sensor(id = 0x44, data = ValueType::SInt16, components = 1)]
    TemperatureI16(crate::types::TemperatureI16),

    /// A sensor that measures angular acceleration, typically expressed in degrees/second.
    #[sensor(id = 0x45, data = ValueType::SInt16, components = 1)]
    GyroscopeI16(crate::types::GyroscopeI16),

    /// Euler angles, in radians.
    #[sensor(id = 0xF0, data = ValueType::Float32, components = 3)]
    EulerAnglesF32(crate::types::EulerAnglesF32),

    /// An orientation quaternion.
    #[sensor(id = 0xF1, data = ValueType::Float32, components = 4)]
    OrientationQuaternionF32(crate::types::OrientationQuaternionF32),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::serializer::SERIALIZATION_CONFIG;
    use crate::types::AccelerometerI16;
    use crate::vector3::Vector3Data;

    #[test]
    fn test() {
        let instance =
            SensorData::AccelerometerI16(AccelerometerI16::new(Vector3Data { x: 1, y: -2, z: 3 }));
        assert_eq!(instance.sensor_type_id(), 0x42);
        assert_eq!(instance.value_type(), ValueType::SInt16);
        assert_eq!(instance.num_components(), 3);

        let value: SensorData = AccelerometerI16::new(Vector3Data { x: 1, y: -2, z: 3 }).into();
        assert_eq!(instance, value);

        let inner: AccelerometerI16 = value.try_into().unwrap();
        assert_eq!(inner.x, 1);
        assert_eq!(inner.y, -2);
        assert_eq!(inner.z, 3);
    }

    #[test]
    fn test_serialize() {
        let value: SensorData = AccelerometerI16::new(Vector3Data { x: 1, y: -2, z: 3 }).into();

        // The deserialization target buffer.
        let mut buffer = [0_u8; 1024];

        // Serialize the data
        let num_serialized = bincode::encode_into_slice(value, &mut buffer, SERIALIZATION_CONFIG)
            .expect("Failed to serialize");

        // Ensure the serialized length is correct
        assert_eq!(num_serialized, 2 + 3 * 2);

        // Ensure the serialized content is correct
        let expected_type_code = [0x42, 0x04];
        assert_eq!(&buffer[..2], &expected_type_code);

        // Deserialize the data
        let result = bincode::decode_from_slice(&buffer, SERIALIZATION_CONFIG)
            .expect("Failed to deserialize");
        let deserialized: SensorData = result.0;
        let count = result.1;

        // Ensure the deserialized content is correct
        assert_eq!(deserialized.sensor_type_id(), 0x42);
        assert_eq!(count, 2 + 3 * 2);

        let into: AccelerometerI16 = deserialized.try_into().unwrap();
        assert_eq!(into.x, 1);
        assert_eq!(into.y, -2);
        assert_eq!(into.z, 3);
    }
}
