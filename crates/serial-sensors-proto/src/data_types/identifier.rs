use crate::{SensorId, ValueType};
use bincode::de::{BorrowDecoder, Decoder};
use bincode::enc::Encoder;
use bincode::error::{DecodeError, EncodeError};
use bincode::{BorrowDecode, Decode, Encode};
use core::ops::{Deref, DerefMut};
use core::str::Utf8Error;

/// Identification data as UTF-8 bytes.
#[derive(Encode, Decode, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(clippy::module_name_repetitions)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(C)]
pub struct Identifier<const N: usize> {
    /// Which sensor does this identify?
    pub target: SensorId,
    /// The type of identifier.
    pub code: IdentifierCode,
    /// The value (UTF-8).
    pub value: [u8; N],
}

/// Identifies the type of identifier.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(clippy::module_name_repetitions)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum IdentifierCode {
    /// Generic identification.
    Generic = 0x00,
    /// Identifies the maker.
    Maker = 0x01,
    /// Identifies the product.
    Product = 0x02,
    /// Identifies the revision.
    Revision = 0x03,
}

impl Encode for IdentifierCode {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        Encode::encode(&(*self as u8), encoder)
    }
}

impl Decode for IdentifierCode {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        let value: u8 = Decode::decode(decoder)?;
        match value {
            0x00 => Ok(IdentifierCode::Generic),
            0x01 => Ok(IdentifierCode::Maker),
            0x02 => Ok(IdentifierCode::Product),
            0x03 => Ok(IdentifierCode::Revision),
            _ => Err(DecodeError::Other("Unknown identifier code")),
        }
    }
}

impl<'a> BorrowDecode<'a> for IdentifierCode {
    fn borrow_decode<D: BorrowDecoder<'a>>(decoder: &mut D) -> Result<Self, DecodeError> {
        IdentifierCode::decode(decoder)
    }
}

impl<const N: usize> Default for Identifier<N> {
    fn default() -> Self {
        Self {
            target: SensorId(0, 0, ValueType::Identifier),
            code: IdentifierCode::Generic,
            value: [0x20; N], // ASCII spaces
        }
    }
}

impl<const N: usize> Identifier<N> {
    /// Initializes a new [`Identifier`] instance.
    #[must_use]
    pub fn new(target: SensorId, code: IdentifierCode, value: &str) -> Self {
        let mut array = [0x20; N];
        let source_range = ..value.len().min(array.len());
        let chars = value.as_bytes();
        array[source_range].copy_from_slice(&chars[source_range]);
        Self {
            target,
            code,
            value: array,
        }
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
        Identifier::new(SensorId::default(), IdentifierCode::Generic, value)
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
        let input_data =
            Identifier::<64>::new(SensorId::default(), IdentifierCode::Product, "LSM303DLHC");

        // The deserialization target buffer.
        let mut buffer = [0_u8; 1024];

        // Serialize the data
        let num_serialized =
            bincode::encode_into_slice(input_data.clone(), &mut buffer, SERIALIZATION_CONFIG)
                .expect("Failed to serialize");

        // Ensure the serialized length is correct
        assert_eq!(num_serialized, 69);

        // Deserialize the data
        let result = bincode::decode_from_slice(&buffer, SERIALIZATION_CONFIG)
            .expect("Failed to deserialize");
        let deserialized: Identifier<64> = result.0;
        let count = result.1;

        // Ensure the deserialized content is correct
        assert_eq!(deserialized, input_data);
        assert_eq!(count, 69);
    }

    #[test]
    #[allow(clippy::expect_used)]
    fn test_index() {
        let reading = Identifier::<64>::new(SensorId::default(), IdentifierCode::Generic, "abcde");

        let value = core::str::from_utf8(&reading.value).expect("invalid coding");

        assert_eq!(
            value,
            "abcde                                                           "
        );
        assert_eq!(reading.len(), 64);
        assert!(!reading.is_empty());
    }
}
