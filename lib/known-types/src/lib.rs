// This is free and unencumbered software released into the public domain.

//! This crate provides well-known types.
//!
//! ```edition2024
//! # use known_types::*;
//! ```

#![no_std]
#![deny(unsafe_code)]

pub mod c;

mod features;
pub use features::*;

#[doc = include_str!("../../../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
