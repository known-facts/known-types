// This is free and unencumbered software released into the public domain.

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
pub use alloc::{string::String, vec::Vec};
