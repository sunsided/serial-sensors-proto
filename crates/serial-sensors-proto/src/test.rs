use serial_sensors_proto_derive::SerialSensors;

/// Data formats.
#[derive(Debug, Clone, PartialEq, SerialSensors)]
pub enum Test {
    #[sensor(type = SensorType::ProtocolVersion, data = ValueType::UInt32, components = 1)]
    ProtocolVersion(crate::types::ProtocolVersion),
    #[sensor(type = SensorType::SystemClockFrequency, data = ValueType::UInt32, components = 1)]
    SystemClockFrequency(crate::types::SystemClockFrequency),
    #[sensor(type = SensorType::Gravity, data = ValueType::SInt16, components = 3)]
    AccelerometerI16(crate::types::AccelerometerI16),
    #[sensor(type = SensorType::MagneticFieldStrength, data = ValueType::SInt16, components = 3)]
    MagnetometerI16(crate::types::MagnetometerI16),
    #[sensor(type = SensorType::Temperature, data = ValueType::SInt16, components = 1)]
    TemperatureI16(crate::types::TemperatureI16),
    #[sensor(type = SensorType::AngularAcceleration, data = ValueType::SInt16, components = 1)]
    GyroscopeI16(crate::types::GyroscopeI16),
    #[sensor(type = SensorType::EulerAngles, data = ValueType::Float32, components = 3)]
    EulerAnglesF32(crate::types::EulerAnglesF32),
    #[sensor(type = SensorType::OrientationQuaternion, data = ValueType::Float32, components = 4)]
    OrientationQuaternionF32(crate::types::OrientationQuaternionF32),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::AccelerometerI16;
    use crate::vector3::Vector3Data;
    use serial_sensors_proto_traits::{RuntimeTypeInformation, SensorType};

    #[test]
    fn test() {
        let instance =
            Test::AccelerometerI16(AccelerometerI16::new(Vector3Data { x: 1, y: -2, z: 3 }));
        assert_eq!(instance.sensor(), SensorType::Gravity);
    }
}
