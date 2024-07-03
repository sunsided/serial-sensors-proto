use bincode::{Decode, Encode};
use uniform_array_derive::UniformArray;

/// A three-dimensional vector.
#[derive(Encode, Decode, UniformArray, Default, Debug, Copy, Clone, Eq, PartialEq)]
#[allow(clippy::module_name_repetitions)]
#[cfg_attr(test, ensure_uniform_type::ensure_uniform_type)]
#[repr(C)]
pub struct Vector3Data<T> {
    /// First vector component.
    pub x: T,
    /// Second vector component.
    pub y: T,
    /// Third vector component.
    pub z: T,
}

impl<T> Vector3Data<T> {
    /// Initializes a new [`Vector3Data`] instance.
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

impl<T> From<(T, T, T)> for Vector3Data<T> {
    fn from(value: (T, T, T)) -> Self {
        Vector3Data::new(value.0, value.1, value.2)
    }
}

impl<T> From<Vector3Data<T>> for (T, T, T) {
    fn from(value: Vector3Data<T>) -> Self {
        (value.x, value.y, value.z)
    }
}

impl<T> From<[T; 3]> for Vector3Data<T>
where
    T: Copy,
{
    fn from(value: [T; 3]) -> Self {
        let (x, y, z) = value.into();
        Self::new(x, y, z)
    }
}

impl<T> From<Vector3Data<T>> for [T; 3] {
    fn from(value: Vector3Data<T>) -> Self {
        [value.x, value.y, value.z]
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl<C> From<Vector3Data<C>> for micromath::vector::Vector3d<C>
where
    C: micromath::vector::Component,
{
    fn from(value: Vector3Data<C>) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl<C> From<micromath::vector::Vector3d<C>> for Vector3Data<C>
where
    C: micromath::vector::Component,
{
    fn from(value: micromath::vector::Vector3d<C>) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::serializer::SERIALIZATION_CONFIG;

    #[test]
    #[allow(clippy::expect_used)]
    fn test_accelerometer_data_i16_serialization() {
        let accel_data = Vector3Data::<i16>::new(100, 200, -300);

        // The deserialization target buffer.
        let mut buffer = [0_u8; 1024];

        // Serialize the data
        let num_serialized =
            bincode::encode_into_slice(accel_data, &mut buffer, SERIALIZATION_CONFIG)
                .expect("Failed to serialize");

        // Ensure the serialized length is correct
        assert_eq!(num_serialized, 6);

        // Ensure the serialized content is correct
        let expected_bytes: [u8; 6] = [
            100_i16.to_le_bytes()[0],
            100_i16.to_le_bytes()[1],
            200_i16.to_le_bytes()[0],
            200_i16.to_le_bytes()[1],
            (-300_i16).to_le_bytes()[0],
            (-300_i16).to_le_bytes()[1],
        ];
        assert_eq!(&buffer[..num_serialized], &expected_bytes);

        // Deserialize the data
        let result = bincode::decode_from_slice(&buffer, SERIALIZATION_CONFIG)
            .expect("Failed to deserialize");
        let deserialized: Vector3Data<i16> = result.0;
        let count = result.1;

        // Ensure the deserialized content is correct
        assert_eq!(deserialized, accel_data);
        assert_eq!(count, 6);
    }

    #[test]
    fn test_index() {
        let reading = Vector3Data::<u32> { x: 1, y: 2, z: 3 };

        assert_eq!(reading[0], 1);
        assert_eq!(reading[1], 2);
        assert_eq!(reading[2], 3);
        assert_eq!(reading.len(), 3);
        assert!(!reading.is_empty());
    }
}
