use bincode::Encode;

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
pub trait ConstTypeInformation: Default + Encode {
    /// The sensor type.
    const SENSOR: SensorType;
    /// The field type.
    const FIELD: ValueType;
    /// The number of components of the vector.
    const NUM_COMPONENTS: u8;

    /// The fundamental type used to represent the information.
    type Target: bincode::Encode;
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

macro_rules! impl_type {
    ($comment:literal, $type:tt, $sensor:expr, $value:expr, $num_components:literal, $base_type:ty) => {
        #[doc = $comment]
        #[derive(Default, Debug, Copy, Clone, PartialEq)]
        pub struct $type($base_type);

        impl $type {
            /// Constructs a new instance from a value.
            #[inline]
            pub const fn new(value: $base_type) -> Self {
                Self(value)
            }

            /// Consumes self and returns the inner type.
            pub const fn into_inner(self) -> $base_type {
                self.0
            }
        }

        impl core::ops::Deref for $type {
            type Target = $base_type;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl core::ops::DerefMut for $type {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        impl core::convert::AsRef<$base_type> for $type {
            fn as_ref(&self) -> &$base_type {
                &self.0
            }
        }

        impl core::convert::AsMut<$base_type> for $type {
            fn as_mut(&mut self) -> &mut $base_type {
                &mut self.0
            }
        }

        impl $crate::types::ConstTypeInformation for $type {
            const SENSOR: $crate::types::SensorType = $sensor;
            const FIELD: $crate::types::ValueType = $value;
            const NUM_COMPONENTS: u8 = $num_components;
            type Target = $base_type;
        }

        impl $crate::types::RuntimeTypeInformation for $type {
            /// Returns the sensor type.
            #[inline]
            fn sensor(&self) -> SensorType {
                <Self as $crate::types::ConstTypeInformation>::SENSOR
            }

            /// Returns the field value type.
            #[inline]
            fn field(&self) -> ValueType {
                <Self as $crate::types::ConstTypeInformation>::FIELD
            }

            /// The number of components
            #[inline]
            fn num_components(&self) -> u8 {
                <Self as $crate::types::ConstTypeInformation>::NUM_COMPONENTS
            }
        }

        impl ::bincode::Encode for $type {
            fn encode<__E: ::bincode::enc::Encoder>(
                &self,
                encoder: &mut __E,
            ) -> core::result::Result<(), ::bincode::error::EncodeError> {
                bincode::Encode::encode(&(Self::SENSOR as u8), encoder)?;
                bincode::Encode::encode(&(Self::FIELD as u8), encoder)?;
                bincode::Encode::encode(&Self::NUM_COMPONENTS, encoder)?;
                ::bincode::Encode::encode(&self.0, encoder)?;
                Ok(())
            }
        }

        impl From<$type> for $crate::versions::Version1Data {
            fn from(value: $type) -> $crate::versions::Version1Data {
                $crate::versions::Version1Data::$type(value)
            }
        }

        impl From<$type> for $base_type {
            fn from(value: $type) -> $base_type {
                value.0
            }
        }

        impl TryFrom<$crate::versions::Version1Data> for $type {
            type Error = ();

            fn try_from(value: $crate::versions::Version1Data) -> Result<Self, Self::Error> {
                match value {
                    $crate::versions::Version1Data::$type(value) => Ok(value),
                    _ => Err(()),
                }
            }
        }

        impl TryFrom<$crate::versions::Version1DataFrame> for $type {
            type Error = ();

            fn try_from(value: $crate::versions::Version1DataFrame) -> Result<Self, Self::Error> {
                match value.value {
                    $crate::versions::Version1Data::$type(value) => Ok(value),
                    _ => Err(()),
                }
            }
        }

        impl
            TryFrom<
                $crate::VersionedDataFrame<
                    $crate::versions::Version1,
                    $crate::versions::Version1DataFrame,
                >,
            > for $type
        {
            type Error = ();

            fn try_from(
                value: $crate::VersionedDataFrame<
                    $crate::versions::Version1,
                    $crate::versions::Version1DataFrame,
                >,
            ) -> Result<Self, Self::Error> {
                match value.data.value {
                    $crate::versions::Version1Data::$type(value) => Ok(value),
                    _ => Err(()),
                }
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
