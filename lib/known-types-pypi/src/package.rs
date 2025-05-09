// This is free and unencumbered software released into the public domain.

#[cfg(not(feature = "alloc"))]
compile_error!("the 'alloc' feature is required here");

use crate::prelude::{String, Vec};

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PackageMetadata {
    pub info: PackageInfo,
    #[cfg_attr(feature = "serde", serde(default))]
    pub urls: Vec<PackageUrl>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PackageInfo {
    pub name: String,
    pub version: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub requires_dist: Option<Vec<String>>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PackageUrl {
    pub filename: String,
    pub packagetype: String,
    pub url: String,
}
