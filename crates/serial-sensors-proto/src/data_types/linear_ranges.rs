use crate::SensorId;
use bincode::{Decode, Encode};

/// Value interpretation information for linear value readings with uniform behavior
/// across all axes.
#[derive(Encode, Decode, Default, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(clippy::module_name_repetitions)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(C)]
pub struct LinearRanges {
    /// Which sensor does this identify?
    pub target: SensorId,
    /// Number of bits per axis.
    ///
    /// A value could be represented using 16 bits, but only have 12 bit range.
    pub resolution_bits: u16,
    /// The type of scale operation. Currently, it always implies a division.
    pub scale_op: u8,
    /// The amount by which to scale the value.
    pub scale: i32,
    /// The number of decimal points in `meas_range_max` and `meas_range_min`, used
    /// to express fractional numbers. Used to scale the values by 10^`range_decimals`.
    pub scale_decimals: u8,
    /// The amount by which to offset the value.
    pub offset: i32,
    /// The number of decimal points for the `offset`.
    pub offset_decimals: u8,
}

impl LinearRanges {
    /// Calibrates a value using a [`LinearRangeInfo`]
    #[cfg(feature = "std")]
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    #[allow(clippy::cast_precision_loss)]
    #[must_use]
    pub fn transform(&self, value: f32) -> f32 {
        let scale = self.scale as f32 / 10.0_f32.powi(i32::from(self.scale_decimals));
        if self.offset != 0 {
            let offset = self.offset as f32 / 10.0_f32.powi(i32::from(self.offset_decimals));
            value / scale + offset
        } else {
            value / scale
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::serializer::SERIALIZATION_CONFIG;

    #[test]
    #[cfg(feature = "std")]
    fn test_calibrate_temp() {
        let mag_data = LinearRanges {
            target: SensorId::default(),
            resolution_bits: 12,
            scale: 16384,
            ..Default::default()
        };

        let result = mag_data.transform(16640.0);
        assert_eq!(result, 1.015625);
    }

    #[test]
    #[cfg(feature = "std")]
    fn test_calibrate_mag() {
        let mag_data = LinearRanges {
            target: SensorId::default(),
            resolution_bits: 12,
            scale: 1100,
            ..Default::default()
        };

        let result = mag_data.transform(384.0);
        assert_eq!(result, 0.3490909);
    }

    #[test]
    #[cfg(feature = "std")]
    fn test_calibrate_accel() {
        let mag_data = LinearRanges {
            target: SensorId::default(),
            resolution_bits: 12,
            scale: 8,
            offset: 20,
            ..Default::default()
        };

        let result = mag_data.transform(73.0);
        assert_eq!(result, 29.125);
    }

    #[test]
    #[allow(clippy::expect_used)]
    fn test_accelerometer_data_i16_serialization() {
        let mag_data = LinearRanges {
            target: SensorId::default(),
            resolution_bits: 12,
            ..Default::default()
        };

        // The deserialization target buffer.
        let mut buffer = [0_u8; 1024];

        // Serialize the data
        let num_serialized =
            bincode::encode_into_slice(mag_data, &mut buffer, SERIALIZATION_CONFIG)
                .expect("Failed to serialize");

        // Ensure the serialized length is correct
        assert_eq!(num_serialized, 17);

        // Deserialize the data
        let result = bincode::decode_from_slice(&buffer, SERIALIZATION_CONFIG)
            .expect("Failed to deserialize");
        let deserialized: LinearRanges = result.0;
        let count = result.1;

        // Ensure the deserialized content is correct
        assert_eq!(deserialized.resolution_bits, 12);
        assert_eq!(count, 17);
    }
}
