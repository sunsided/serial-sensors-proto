use serial_sensors_proto_traits::{SensorType, ValueType};

macro_rules! impl_type {
    ($comment:literal, $type:tt, $sensor:expr, $value:expr, $num_components:literal, $base_type:ty) => {
        #[doc = $comment]
        #[derive(::bincode::Encode, ::bincode::Decode, Default, Debug, Copy, Clone, PartialEq)]
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

        impl ::serial_sensors_proto_traits::CompileTimeTypeInformation for $type {
            const SENSOR: ::serial_sensors_proto_traits::SensorType = $sensor;
            const FIELD: ::serial_sensors_proto_traits::ValueType = $value;
            const NUM_COMPONENTS: u8 = $num_components;
            type Target = $base_type;
        }

        impl ::serial_sensors_proto_traits::RuntimeTypeInformation for $type {
            /// Returns the sensor type.
            #[inline]
            fn sensor(&self) -> SensorType {
                <Self as ::serial_sensors_proto_traits::CompileTimeTypeInformation>::SENSOR
            }

            /// Returns the field value type.
            #[inline]
            fn field(&self) -> ValueType {
                <Self as ::serial_sensors_proto_traits::CompileTimeTypeInformation>::FIELD
            }

            /// The number of components
            #[inline]
            fn num_components(&self) -> u8 {
                <Self as ::serial_sensors_proto_traits::CompileTimeTypeInformation>::NUM_COMPONENTS
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
