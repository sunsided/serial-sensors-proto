use bincode::{Decode, Encode};
use core::ops::{Deref, DerefMut};
use uniform_array_derive::UniformArray;

/// Identification data.
#[derive(
    Encode, Decode, UniformArray, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash,
)]
#[allow(clippy::module_name_repetitions)]
#[cfg_attr(test, ensure_uniform_type::ensure_uniform_type)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(C)]
pub struct Identifier {
    /// The value.
    pub value: [u8; 64],
}

impl Default for Identifier {
    fn default() -> Self {
        Self {
            value: [0x20; 64], // ASCII spaces
        }
    }
}

impl Identifier {
    /// Initializes a new [`Identifier`] instance.
    #[must_use]
    pub fn new(value: &str) -> Self {
        let mut array = [0x20; 64];
        let source_range = ..value.len().min(array.len());
        let chars = value.as_bytes();
        array[source_range].copy_from_slice(&chars[source_range]);
        Self { value: array }
    }
}

impl From<&str> for Identifier {
    fn from(value: &str) -> Self {
        Identifier::new(value)
    }
}

impl Deref for Identifier {
    type Target = [u8; 64];

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DerefMut for Identifier {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::serializer::SERIALIZATION_CONFIG;

    #[test]
    #[allow(clippy::expect_used)]
    fn test_identifier_serialization() {
        let input_data = Identifier::new("LSM303DLHC");

        // The deserialization target buffer.
        let mut buffer = [0_u8; 1024];

        // Serialize the data
        let num_serialized =
            bincode::encode_into_slice(input_data, &mut buffer, SERIALIZATION_CONFIG)
                .expect("Failed to serialize");

        // Ensure the serialized length is correct
        assert_eq!(num_serialized, 64);

        // Deserialize the data
        let result = bincode::decode_from_slice(&buffer, SERIALIZATION_CONFIG)
            .expect("Failed to deserialize");
        let deserialized: Identifier = result.0;
        let count = result.1;

        // Ensure the deserialized content is correct
        assert_eq!(deserialized, input_data);
        assert_eq!(count, 64);
    }

    #[test]
    #[allow(clippy::expect_used)]
    fn test_index() {
        let reading = Identifier::new("abcde");

        let value = core::str::from_utf8(&reading.value).expect("invalid coding");

        assert_eq!(
            value,
            "abcde                                                           "
        );
        assert_eq!(reading.len(), 1);
        assert!(!reading.is_empty());
    }
}
