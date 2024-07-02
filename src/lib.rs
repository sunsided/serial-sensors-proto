#![no_std]
#![deny(unsafe_code)]

use bincode::config::{Configuration, Fixint, LittleEndian};

pub mod scalar;
pub mod vector3;

/// The serialization configuration.
const SERIALIZATION_CONFIG: Configuration<LittleEndian, Fixint> = bincode::config::standard()
    .with_fixed_int_encoding()
    .with_little_endian()
    .with_no_limit();
