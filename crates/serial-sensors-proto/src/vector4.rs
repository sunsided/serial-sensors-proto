use bincode::{Decode, Encode};

/// A four-dimensional vector, Quaternion, etc.
#[derive(Encode, Decode, Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct Vector4Data<T> {
    /// First vector component.
    pub a: T,
    /// Second vector component.
    pub b: T,
    /// Third vector component.
    pub c: T,
    /// Third vector component.
    pub d: T,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SERIALIZATION_CONFIG;
    use bincode;

    #[test]
    fn test_accelerometer_data_i16_serialization() {
        let accel_data = Vector4Data::<i16> {
            a: 100,
            b: 200,
            c: -300,
            d: 12,
        };

        // The deserialization target buffer.
        let mut buffer = [0_u8; 1024];

        // Serialize the data
        let num_serialized =
            bincode::encode_into_slice(accel_data, &mut buffer, SERIALIZATION_CONFIG)
                .expect("Failed to serialize");

        // Ensure the serialized length is correct
        assert_eq!(num_serialized, 8);

        // Ensure the serialized content is correct
        let expected_bytes: [u8; 8] = [
            100_i16.to_le_bytes()[0],
            100_i16.to_le_bytes()[1],
            200_i16.to_le_bytes()[0],
            200_i16.to_le_bytes()[1],
            (-300_i16).to_le_bytes()[0],
            (-300_i16).to_le_bytes()[1],
            (12_i16).to_le_bytes()[0],
            (12_i16).to_le_bytes()[1],
        ];
        assert_eq!(&buffer[..num_serialized], &expected_bytes);

        // Deserialize the data
        let result = bincode::decode_from_slice(&buffer, SERIALIZATION_CONFIG)
            .expect("Failed to deserialize");
        let deserialized: Vector4Data<i16> = result.0;
        let count = result.1;

        // Ensure the deserialized content is correct
        assert_eq!(deserialized, accel_data);
        assert_eq!(count, 8);
    }
}
