/// Lightweight ID newtypes for adapters and detectors.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AdapterId(pub &'static str);
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DetectorId(pub &'static str);

#[cfg(feature = "serde")]
#[cfg(not(feature = "std"))]
use alloc::{collections::BTreeMap, string::String, vec::Vec};
#[cfg(feature = "serde")]
#[cfg(feature = "std")]
use std::{collections::BTreeMap, string::String, vec::Vec};

/// Optional capability manifest describing an implementation.
#[cfg(feature = "serde")]
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct CapabilityManifest {
    pub name: String,                 // e.g., "vision"
    pub modalities: Vec<crate::traits::adapter::Modality>,
    pub d_model: u16,                 // e.g., 768
    /// Free-form params, stringly-typed to avoid extra deps in core.
    pub params: BTreeMap<String, String>,
    /// Human-readable throughput hint, e.g., "100 MPix/s CPU".
    pub throughput_hint: Option<String>,
    /// Whether the adapter exports a secondary quality vector (future).
    pub quality_vec: bool,
}
