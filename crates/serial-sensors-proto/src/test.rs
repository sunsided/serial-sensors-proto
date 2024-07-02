use serial_sensors_proto_derive::SerialSensors;

/// Data formats.
#[derive(Debug, Clone, PartialEq, SerialSensors)]
pub enum Test {
    #[sensor(sensor_type = "ProtocolVersion", data = "UInt32", components = 1)]
    ProtocolVersion(crate::types::ProtocolVersion),
    /*
    #[sensor(sensor_type = "SystemClockFrequency", data = "UInt32", components = 1)]
    SystemClockFrequency(crate::types::SystemClockFrequency),
    #[sensor(sensor_type = "Gravity", data = "SInt16", components = 3)]
    AccelerometerI16(crate::types::AccelerometerI16),
    #[sensor(sensor_type = "MagneticFieldStrength", data = "SInt16", components = 3)]
    MagnetometerI16(crate::types::MagnetometerI16),
    #[sensor(sensor_type = "Temperature", data = "SInt16", components = 1)]
    TemperatureI16(crate::types::TemperatureI16),
    #[sensor(sensor_type = "AngularAcceleration", data = "SInt16", components = 1)]
    GyroscopeI16(crate::types::GyroscopeI16),
    #[sensor(sensor_type = "EulerAngles", data = "Float32", components = 3)]
    EulerAnglesF32(crate::types::EulerAnglesF32),
    #[sensor(
        sensor_type = "OrientationQuaternion",
        data = "Float32",
        components = 4
    )]
    OrientationQuaternionF32(crate::types::OrientationQuaternionF32),
    */
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::AccelerometerI16;
    use crate::vector3::Vector3Data;
    use serial_sensors_proto_traits::{RuntimeTypeInformation, SensorType};
    /*
       #[test]
       fn test() {
           let instance =
               Test::AccelerometerI16(AccelerometerI16::new(Vector3Data { x: 1, y: -2, z: 3 }));
           assert_eq!(instance.sensor(), SensorType::Gravity);
       }
    */
}
