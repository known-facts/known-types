// This is free and unencumbered software released into the public domain.

#[cfg(not(feature = "alloc"))]
compile_error!("the 'alloc' feature is required here");

use crate::prelude::{String, Vec};

#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GemInfo {
    pub version: String,
    pub dependencies: Dependencies,
}

#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Dependencies {
    #[cfg_attr(feature = "serde", serde(default))]
    pub development: Vec<Dependency>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub runtime: Vec<Dependency>,
}

#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Dependency {
    pub name: String,
    pub requirements: String,
}
