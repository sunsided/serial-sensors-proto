use serial_sensors_proto_derive::SerialSensors;

/// Data formats.
#[derive(Debug, Clone, PartialEq, SerialSensors)]
pub enum Test {
    #[sensor(id = 0x01, data = "UInt32", components = 1)]
    ProtocolVersions(crate::types::ProtocolVersion),
    #[sensor(id = 0x02, data = "UInt32", components = 1)]
    SystemClockFrequency(crate::types::SystemClockFrequency),
    #[sensor(id = 0x42, data = "SInt16", components = 3)]
    AccelerometerI16(crate::types::AccelerometerI16),
    #[sensor(id = 0x43, data = "SInt16", components = 3)]
    MagnetometerI16(crate::types::MagnetometerI16),
    #[sensor(id = 0x44, data = "SInt16", components = 1)]
    TemperatureI16(crate::types::TemperatureI16),
    #[sensor(id = 0x45, data = "SInt16", components = 1)]
    GyroscopeI16(crate::types::GyroscopeI16),
    #[sensor(id = 0xF0, data = "Float32", components = 3)]
    EulerAnglesF32(crate::types::EulerAnglesF32),
    #[sensor(id = 0xF1, data = "Float32", components = 4)]
    OrientationQuaternionF32(crate::types::OrientationQuaternionF32),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::AccelerometerI16;
    use crate::vector3::Vector3Data;
    use serial_sensors_proto_traits::RuntimeTypeInformation2;

    #[test]
    fn test() {
        let instance =
            Test::AccelerometerI16(AccelerometerI16::new(Vector3Data { x: 1, y: -2, z: 3 }));
        assert_eq!(instance.sensor_type_id(), 0x42);
    }
}
