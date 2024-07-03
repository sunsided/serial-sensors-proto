use bincode::{Decode, Encode};
use uniform_array_derive::UniformArray;

/// Scalar data.
#[derive(Encode, Decode, UniformArray, Default, Debug, Copy, Clone, Eq, PartialEq)]
#[allow(clippy::module_name_repetitions)]
#[cfg_attr(test, ensure_uniform_type::ensure_uniform_type)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(C)]
pub struct ScalarData<T> {
    /// The value.
    pub value: T,
}

impl<T> ScalarData<T> {
    /// Initializes a new [`ScalarData`] instance.
    pub const fn new(value: T) -> Self {
        Self { value }
    }
}

impl<T> From<T> for ScalarData<T> {
    fn from(value: T) -> Self {
        ScalarData::new(value)
    }
}

impl<T> From<[T; 1]> for ScalarData<T>
where
    T: Copy,
{
    fn from(value: [T; 1]) -> Self {
        let (x,) = value.into();
        Self::new(x)
    }
}

impl<T> From<ScalarData<T>> for [T; 1] {
    fn from(value: ScalarData<T>) -> Self {
        [value.value]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::serializer::SERIALIZATION_CONFIG;

    #[test]
    #[allow(clippy::expect_used)]
    fn test_accelerometer_data_i16_serialization() {
        let input_data = ScalarData::<i16>::new(100);

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

    #[test]
    fn test_index() {
        let reading = ScalarData::<u32> { value: 12 };

        assert_eq!(reading[0], 12);
        assert_eq!(reading.len(), 1);
        assert!(!reading.is_empty());
    }
}
