use bincode::{Decode, Encode};

/// A three-dimensional vector.
#[derive(Encode, Decode, Default, Debug, Copy, Clone, Eq, PartialEq)]
#[allow(clippy::module_name_repetitions)]
pub struct Vector3Data<T> {
    /// First vector component.
    pub x: T,
    /// Second vector component.
    pub y: T,
    /// Third vector component.
    pub z: T,
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
    use bincode;

    #[test]
    fn test_accelerometer_data_i16_serialization() {
        let accel_data = Vector3Data::<i16> {
            x: 100,
            y: 200,
            z: -300,
        };

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
}
