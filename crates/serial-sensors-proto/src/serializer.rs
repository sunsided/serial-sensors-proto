use crate::versions::{Version1, Version1DataFrame};
use crate::{DataFrame, ProtocolVersion, VersionedDataFrame};
use bincode::config::{Configuration, Fixint, LittleEndian};
use bincode::error::{DecodeError, EncodeError};
use bincode::Encode;
use core::ops::Range;
use corncobs::CobsError;

/// The serialization configuration.
#[allow(dead_code)]
pub(crate) const SERIALIZATION_CONFIG: Configuration<LittleEndian, Fixint> =
    bincode::config::standard()
        .with_fixed_int_encoding()
        .with_little_endian()
        .with_no_limit();

/// Serializes data and applies byte stuffing.
///
/// ## Errors
/// The function returns an error when serialization failed, or when either serialization
/// or byte stuffing resulted in a buffer overrun.
pub fn serialize<I, V, D>(frame: I, buffer: &mut [u8]) -> Result<Range<usize>, SerializationError>
where
    I: Into<VersionedDataFrame<V, D>>,
    V: ProtocolVersion,
    D: DataFrame + Encode,
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
    Ok(num_serialized..num_serialized + encoded_length)
}

/// Deserializes data after applying byte un-stuffing.
///
/// Returns the number of bytes read from the buffer
///
/// ## Errors
/// Returns an error when byte un-stuffing failed, e.g. due to a buffer under-run or corrupted data,
/// or when deserialization failed due to unknown wire data.
pub fn deserialize(
    buffer: &mut [u8],
) -> Result<(usize, VersionedDataFrame<Version1, Version1DataFrame>), DeserializationError> {
    // TODO: Ensure that sync recovery actually works.
    let read_length = corncobs::decode_in_place(buffer)?;
    let data = &buffer[..read_length];
    let (data, _) = bincode::decode_from_slice(data, SERIALIZATION_CONFIG)?;
    Ok((read_length, data))
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

/// A deserialization error.
#[derive(Debug)]
pub enum DeserializationError {
    /// The data buffer was truncated.
    Truncated,
    /// The data was corrupt.
    Corrupt,
    /// Decoding failed.
    BincodeError(DecodeError),
}

impl From<EncodeError> for SerializationError {
    fn from(value: EncodeError) -> Self {
        Self::BincodeError(value)
    }
}

impl From<CobsError> for DeserializationError {
    fn from(value: CobsError) -> Self {
        match value {
            CobsError::Truncated => DeserializationError::Truncated,
            CobsError::Corrupt => DeserializationError::Corrupt,
        }
    }
}

impl From<DecodeError> for DeserializationError {
    fn from(value: DecodeError) -> Self {
        DeserializationError::BincodeError(value)
    }
}

impl core::fmt::Display for DeserializationError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            DeserializationError::Truncated => f.write_str("input truncated"),
            DeserializationError::Corrupt => f.write_str("input corrupt"),
            DeserializationError::BincodeError(err) => core::fmt::Display::fmt(&err, f),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::AccelerometerI16;
    use crate::versions::Version1DataFrame;
    use crate::Vector3Data;

    #[test]
    fn test_serialize() {
        let value = AccelerometerI16::new(Vector3Data { x: 1, y: -2, z: 3 });
        let frame = Version1DataFrame::new(u32::MAX, 12, 0, value);

        // The serialization target buffer.
        let mut buffer = [0_u8; 48];

        let range = serialize(frame, &mut buffer).unwrap();
        assert_eq!(range.len(), 21);

        // The deserialization target buffer.
        let (_read, data) = deserialize(&mut buffer[range]).unwrap();
        assert_eq!(data.version, Version1);
        assert_eq!(data.data.global_sequence, u32::MAX);
        assert_eq!(data.data.sensor_sequence, 12);
        assert_eq!(data.data.sensor_tag, 0);

        let data: AccelerometerI16 = data.try_into().unwrap();
        assert_eq!(data.x, 1);
        assert_eq!(data.y, -2);
        assert_eq!(data.z, 3);
    }
}
