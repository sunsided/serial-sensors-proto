use bincode::{Decode, Encode};

/// Value interpretation information for linear value readings with uniform behavior
/// across all axes.
#[derive(Encode, Decode, Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(clippy::module_name_repetitions)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(C)]
pub struct LinearRanges {
    /// Number of bits per axis.
    ///
    /// A value could be represented using 16 bits, but only have 12 bit range.
    pub resolution_bits: u16,
    /// The value change per change in measurable unit.
    /// For example, 1100 LSB/Gauss imply that for every change of 1100 decimal values,
    /// the physical reading changes by 1 Gauss.
    pub lsb_per_unit: u32,
    /// The maximum measurable value.
    /// This is in terms of physical units, not in terms of bit representation.
    pub meas_range_max: u32,
    /// The minimum measurable value.
    /// This is in terms of physical units, not in terms of bit representation.
    pub meas_range_min: u32,
    /// The number of decimal points in `meas_range_max` and `meas_range_min`, used
    /// to express fractional numbers. Used to scale the values by 10^`range_decimals`.
    pub range_decimals: u8,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::serializer::SERIALIZATION_CONFIG;

    #[test]
    #[allow(clippy::expect_used)]
    fn test_accelerometer_data_i16_serialization() {
        let accel_data = LinearRanges {
            resolution_bits: 12,
            lsb_per_unit: 1100,
            meas_range_max: 13,
            meas_range_min: 13,
            range_decimals: 1,
        };

        // The deserialization target buffer.
        let mut buffer = [0_u8; 1024];

        // Serialize the data
        let num_serialized =
            bincode::encode_into_slice(accel_data, &mut buffer, SERIALIZATION_CONFIG)
                .expect("Failed to serialize");

        // Ensure the serialized length is correct
        assert_eq!(num_serialized, 15);

        // Deserialize the data
        let result = bincode::decode_from_slice(&buffer, SERIALIZATION_CONFIG)
            .expect("Failed to deserialize");
        let deserialized: LinearRanges = result.0;
        let count = result.1;

        // Ensure the deserialized content is correct
        assert_eq!(deserialized.lsb_per_unit, 1100);
        assert_eq!(count, 15);
    }
}