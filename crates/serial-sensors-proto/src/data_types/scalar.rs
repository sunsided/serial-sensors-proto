use bincode::{Decode, Encode};

/// Scalar data.
#[derive(Encode, Decode, Default, Debug, Copy, Clone, Eq, PartialEq)]
#[allow(clippy::module_name_repetitions)]
#[cfg_attr(test, ensure_uniform_type::ensure_uniform_type)]
pub struct ScalarData<T> {
    /// The value.
    pub value: T,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::serializer::SERIALIZATION_CONFIG;

    #[test]
    #[allow(clippy::expect_used)]
    fn test_accelerometer_data_i16_serialization() {
        let input_data = ScalarData::<i16> { value: 100 };

        // The deserialization target buffer.
        let mut buffer = [0_u8; 1024];

        // Serialize the data
        let num_serialized =
            bincode::encode_into_slice(input_data, &mut buffer, SERIALIZATION_CONFIG)
                .expect("Failed to serialize");

        // Ensure the serialized length is correct
        assert_eq!(num_serialized, 2);

        // Ensure the serialized content is correct
        let expected_bytes: [u8; 2] = 100_i16.to_le_bytes();
        assert_eq!(&buffer[..num_serialized], &expected_bytes);

        // Deserialize the data
        let result = bincode::decode_from_slice(&buffer, SERIALIZATION_CONFIG)
            .expect("Failed to deserialize");
        let deserialized: ScalarData<i16> = result.0;
        let count = result.1;

        // Ensure the deserialized content is correct
        assert_eq!(deserialized, input_data);
        assert_eq!(count, 2);
    }
}
