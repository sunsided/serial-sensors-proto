use bincode::{Decode, Encode};
use core::ops::{Deref, DerefMut};
use core::str::Utf8Error;
use uniform_array_derive::UniformArray;

/// Identification data as UTF-8 bytes.
#[derive(
    Encode, Decode, UniformArray, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash,
)]
#[allow(clippy::module_name_repetitions)]
#[cfg_attr(test, ensure_uniform_type::ensure_uniform_type)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(C)]
pub struct Identifier<const N: usize> {
    /// The value (UTF-8).
    pub value: [u8; N],
}

impl<const N: usize> Default for Identifier<N> {
    fn default() -> Self {
        Self {
            value: [0x20; N], // ASCII spaces
        }
    }
}

impl<const N: usize> Identifier<N> {
    /// Initializes a new [`Identifier`] instance.
    #[must_use]
    pub fn new(value: &str) -> Self {
        let mut array = [0x20; N];
        let source_range = ..value.len().min(array.len());
        let chars = value.as_bytes();
        array[source_range].copy_from_slice(&chars[source_range]);
        Self { value: array }
    }

    /// Returns the value as a string.
    ///
    /// ## Errors
    /// Returns an error if the value did not contain valid UTF data.
    pub fn as_str(&self) -> Result<&str, Utf8Error> {
        core::str::from_utf8(&self.value)
    }
}

impl<const N: usize> From<&str> for Identifier<N> {
    fn from(value: &str) -> Self {
        Identifier::new(value)
    }
}

impl<const N: usize> Deref for Identifier<N> {
    type Target = [u8; N];

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<const N: usize> DerefMut for Identifier<N> {
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
        let input_data = Identifier::<64>::new("LSM303DLHC");

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
        let deserialized: Identifier<64> = result.0;
        let count = result.1;

        // Ensure the deserialized content is correct
        assert_eq!(deserialized, input_data);
        assert_eq!(count, 64);
    }

    #[test]
    #[allow(clippy::expect_used)]
    fn test_index() {
        let reading = Identifier::<64>::new("abcde");

        let value = core::str::from_utf8(&reading.value).expect("invalid coding");

        assert_eq!(
            value,
            "abcde                                                           "
        );
        assert_eq!(reading.len(), 1);
        assert!(!reading.is_empty());
    }
}
