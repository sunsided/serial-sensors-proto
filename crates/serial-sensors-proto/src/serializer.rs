use crate::{DataFrame, ProtocolVersion, VersionedDataFrame};
use bincode::config::{Configuration, Fixint, LittleEndian};
use bincode::error::EncodeError;

/// The serialization configuration.
#[allow(dead_code)]
pub(crate) const SERIALIZATION_CONFIG: Configuration<LittleEndian, Fixint> =
    bincode::config::standard()
        .with_fixed_int_encoding()
        .with_little_endian()
        .with_no_limit();

/// Serializes data and applies byte stuffing.
pub fn serialize<I, V, D>(frame: I, buffer: &mut [u8]) -> Result<&[u8], SerializationError>
where
    I: Into<VersionedDataFrame<V, D>>,
    V: ProtocolVersion,
    D: DataFrame,
{
    let frame = frame.into();
    let num_serialized = bincode::encode_into_slice(frame, buffer, SERIALIZATION_CONFIG)?;

    // Split the buffer into the source part and the destination part.
    let (source, target) = buffer.split_at_mut(num_serialized);
    debug_assert_eq!(source.len(), num_serialized);

    let length = corncobs::max_encoded_len(num_serialized);
    if length > target.len() {
        return Err(SerializationError::WouldOverflow(num_serialized + length));
    }

    let encoded_length = corncobs::encode_buf(source, target);
    Ok(&target[..encoded_length])
}

/// A serialization error.
#[derive(Debug)]
pub enum SerializationError {
    /// A bincode encoding error occurred.
    BincodeError(EncodeError),
    /// Byte stuffing would overflow the provided buffer.
    /// At least the contained amount of bytes is required.
    WouldOverflow(usize),
}

impl From<EncodeError> for SerializationError {
    fn from(value: EncodeError) -> Self {
        Self::BincodeError(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::AccelerometerI16;
    use crate::vector3::Vector3Data;
    use crate::versions::Version1DataFrame;

    #[test]
    fn test_serialize() {
        let value = AccelerometerI16::new(Vector3Data { x: 1, y: -2, z: 3 });
        let frame = Version1DataFrame::new(u32::MAX, 12, 0, value);

        // The deserialization target buffer.
        let mut buffer = [0_u8; 1024];

        let buffer = serialize(frame, &mut buffer).unwrap();
        assert_eq!(buffer.len(), 21);
    }
}
