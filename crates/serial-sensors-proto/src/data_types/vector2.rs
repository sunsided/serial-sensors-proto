use bincode::{Decode, Encode};
use uniform_array_derive::UniformArray;

/// A two-dimensional vector.
#[derive(Encode, Decode, UniformArray, Default, Debug, Copy, Clone, Eq, PartialEq)]
#[allow(clippy::module_name_repetitions)]
#[cfg_attr(test, ensure_uniform_type::ensure_uniform_type)]
#[repr(C)]
pub struct Vector2Data<T> {
    /// First vector component.
    pub x: T,
    /// Second vector component.
    pub y: T,
}

impl<T> Vector2Data<T> {
    /// Initializes a new [`Vector2Data`] instance.
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl<C> From<Vector2Data<C>> for micromath::vector::Vector2d<C>
where
    C: micromath::vector::Component,
{
    fn from(value: Vector2Data<C>) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl<C> From<micromath::vector::Vector2d<C>> for Vector2Data<C>
where
    C: micromath::vector::Component,
{
    fn from(value: micromath::vector::Vector2d<C>) -> Self {
        Self {
            x: value.x,
            y: value.y,
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
        let accel_data = Vector2Data::<i16>::new(100, 200);

        // The deserialization target buffer.
        let mut buffer = [0_u8; 1024];

        // Serialize the data
        let num_serialized =
            bincode::encode_into_slice(accel_data, &mut buffer, SERIALIZATION_CONFIG)
                .expect("Failed to serialize");

        // Ensure the serialized length is correct
        assert_eq!(num_serialized, 4);

        // Ensure the serialized content is correct
        let expected_bytes: [u8; 4] = [
            100_i16.to_le_bytes()[0],
            100_i16.to_le_bytes()[1],
            200_i16.to_le_bytes()[0],
            200_i16.to_le_bytes()[1],
        ];
        assert_eq!(&buffer[..num_serialized], &expected_bytes);

        // Deserialize the data
        let result = bincode::decode_from_slice(&buffer, SERIALIZATION_CONFIG)
            .expect("Failed to deserialize");
        let deserialized: Vector2Data<i16> = result.0;
        let count = result.1;

        // Ensure the deserialized content is correct
        assert_eq!(deserialized, accel_data);
        assert_eq!(count, 4);
    }

    #[test]
    fn test_index() {
        let reading = Vector2Data::<u32> { x: 1, y: 2 };

        assert_eq!(reading[0], 1);
        assert_eq!(reading[1], 2);
        assert_eq!(reading.len(), 2);
        assert!(!reading.is_empty());
    }
}
