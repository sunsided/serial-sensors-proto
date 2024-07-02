use crate::{DataFrame, VersionedDataFrame};
use bincode::error::DecodeError;

/// A protocol version.
pub trait ProtocolVersion: Default + bincode::Encode + bincode::Decode {
    /// The protocol version
    const VERSION: usize;

    /// Returns the protocol version
    fn version(&self) -> usize {
        Self::VERSION
    }
}

macro_rules! impl_version {
    ($comment:literal, $type:tt, $version:literal) => {
        #[doc = $comment]
        #[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
        pub struct $type;

        impl $type {
            /// Wraps the specified [`VersionedDataFrame`](crate::VersionedDataFrame).
            pub const fn frame<D>(data: D) -> crate::VersionedDataFrame<$type, D>
            where
                D: DataFrame,
            {
                crate::VersionedDataFrame {
                    version: Self,
                    data,
                }
            }
        }

        impl<D> From<D> for VersionedDataFrame<$type, D>
        where
            D: DataFrame,
        {
            fn from(value: D) -> VersionedDataFrame<$type, D> {
                $type::frame(value)
            }
        }

        impl $crate::versions::ProtocolVersion for $type {
            const VERSION: usize = $version;
        }

        impl ::bincode::Encode for $type {
            fn encode<__E: ::bincode::enc::Encoder>(
                &self,
                encoder: &mut __E,
            ) -> core::result::Result<(), ::bincode::error::EncodeError> {
                ::bincode::Encode::encode(&{ ($version) as u8 }, encoder)?;
                Ok(())
            }
        }

        impl ::bincode::Decode for Version1 {
            fn decode<__D: bincode::de::Decoder>(
                decoder: &mut __D,
            ) -> Result<Self, ::bincode::error::DecodeError> {
                let v: u8 = bincode::Decode::decode(decoder)?;
                if v != $version {
                    Err(DecodeError::Other("invalid version"))
                } else {
                    Ok(Self)
                }
            }
        }
    };
}

impl_version!("Protocol version 1.", Version1, 1);
