// This is free and unencumbered software released into the public domain.

//! This crate provides well-known types for Anthropic APIs.
//!
//! ```edition2024
//! # use known_types_anthropic::*;
//! ```

#![no_std]
#![deny(unsafe_code)]
#![deny(missing_debug_implementations)]
#![deny(clippy::unwrap_used)]

#[cfg(doctest)]
#[doc = include_str!("../../../README.md")]
pub struct ReadmeDoctests;
