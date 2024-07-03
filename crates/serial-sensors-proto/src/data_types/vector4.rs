use bincode::{Decode, Encode};

/// A four-dimensional vector, Quaternion, etc.
#[derive(Encode, Decode, Default, Debug, Copy, Clone, Eq, PartialEq)]
#[allow(clippy::module_name_repetitions)]
#[cfg_attr(test, ensure_uniform_type::ensure_uniform_type)]
#[repr(C)]
pub struct Vector4Data<T> {
    /// First vector component.
    /// In quaternions, this resembles the `x` axis.
    pub a: T,
    /// Second vector component.
    /// In quaternions, this resembles the `y` axis.
    pub b: T,
    /// Third vector component.
    /// In quaternions, this resembles the `z` axis.
    pub c: T,
    /// Third vector component.
    /// In quaternions, this resembles the `d` axis.
    pub d: T,
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl From<Vector4Data<f32>> for micromath::Quaternion {
    fn from(value: Vector4Data<f32>) -> Self {
        micromath::Quaternion::new(value.a, value.b, value.c, value.d)
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl From<micromath::Quaternion> for Vector4Data<f32> {
    fn from(value: micromath::Quaternion) -> Self {
        Self {
            a: value.x(),
            b: value.y(),
            c: value.z(),
            d: value.w(),
        }
    }
}

impl<T> core::ops::Index<usize> for Vector4Data<T> {
    type Output = T;

    #[allow(clippy::inline_always)]
    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.a,
            1 => &self.b,
            2 => &self.c,
            3 => &self.d,
            _ => panic!("Index out of bounds"),
        }
    }
}
impl<T> core::ops::IndexMut<usize> for Vector4Data<T> {
    #[allow(clippy::inline_always)]
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.a,
            1 => &mut self.b,
            2 => &mut self.c,
            3 => &mut self.d,
            _ => panic!("Index out of bounds"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::serializer::SERIALIZATION_CONFIG;

    #[test]
    #[allow(clippy::expect_used)]
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

    #[test]
    fn test_index() {
        let reading = Vector4Data::<u32> {
            a: 1,
            b: 2,
            c: 3,
            d: 42,
        };

        assert_eq!(reading[0], 1);
        assert_eq!(reading[1], 2);
        assert_eq!(reading[2], 3);
        assert_eq!(reading[3], 42);
    }
}
