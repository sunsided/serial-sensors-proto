use crate::DataFrame;

/// A protocol version.
pub trait ProtocolVersion {
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
        #[derive(Debug, Copy, Clone)]
        pub struct $type;

        impl $type {
            /// Wraps the specified data into a [`VersionedDataFrame`(crate::VersionedDataFrame).
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

        impl $crate::protocol_version::ProtocolVersion for $type {
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
    };
}

impl_version!("Protocol version 1.", Version1, 1);
