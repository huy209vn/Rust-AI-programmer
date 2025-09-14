//! Adapter contracts: modality enum, execution context, and output payload.
//!
//! Contract highlights (matches the Sozna blueprint):
//! - Adapters are **pure** over a `ByteFrame` → embeddings + Sidecar + Evidence.
//! - No hidden IO; no global state.
//! - Determinism is controlled via `AdapterCtx::{determinism, seed}`.
//! - Agent path must only ever read embeddings + status; Sidecar/Evidence are for humans/QA.

#[cfg(not(feature = "std"))]
use alloc::{string::String, vec::Vec};

use crate::config::AdapterCfgOverride;
use crate::determinism::{Determinism, EmbeddingsShape, RandSeed};
use crate::error::SenseResult;
use crate::evidence::EvidenceHandle;
use crate::frame::ByteFrame;
use crate::sidecar::{BackendInfo, Sidecar};
use crate::status::Status;

/// Modalities supported by Sozna's perception layer.
/// `repr(u8)` and `#[non_exhaustive]` for FFI/log stability and forward-compat.
#[repr(u8)]
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Modality {
    /// UTF-8/16/32 text routed to the text adapter.
    Text = 0,
    /// PCM/WAV/MP3 routed to the audio adapter.
    Audio = 1,
    /// Raster images (PNG/JPEG/WebP/GIF) routed to the vision adapter.
    Vision = 2,
    /// Opaque/unknown/container bytes routed to the binary adapter.
    Binary = 3,
}

/// Per-call adapter execution context.
#[derive(Clone, Debug)]
pub struct AdapterCtx<'a> {
    /// Determinism policy (D1/D2/D3).
    pub determinism: Determinism,
    /// Seed that adapters can use to pin any stochastic ops under D2/D3.
    pub seed: RandSeed,
    /// Adapter-level knobs the caller may override.
    pub overrides: &'a AdapterCfgOverride,
    /// Backend identity to stamp into the Sidecar (device, dtype, layout, threads).
    pub backend: BackendInfo,
    /// If true, implementations may populate human-only `imprints` strings.
    pub allow_imprints: bool,
}

impl<'a> AdapterCtx<'a> {
    /// Convenience constructor.
    pub fn new(
        determinism: Determinism,
        seed: RandSeed,
        overrides: &'a AdapterCfgOverride,
        backend: BackendInfo,
    ) -> Self {
        Self { determinism, seed, overrides, backend, allow_imprints: false }
    }

    /// Enable human-only imprints (debug text) in `AdapterOut`.
    #[inline]
    pub fn with_imprints(mut self, allow: bool) -> Self {
        self.allow_imprints = allow;
        self
    }
}

/// Output structure returned by adapters.
///
/// Invariants (validated by `validators::validate_adapter_out`):
/// - `emb.len() == shape.n * shape.d_model`
/// - `sidecar.lengths == Some(shape)`
/// - `sidecar.evidence_hash8 == evidence.hash8`
#[derive(Clone, Debug)]
pub struct AdapterOut {
    /// Row-major embeddings (flattened).
    pub emb: Vec<f32>,
    /// `(n, d_model)` packed shape.
    pub shape: EmbeddingsShape,
    /// Evidence anchor for humans/QA only.
    pub evidence: EvidenceHandle,
    /// Sidecar metadata (humans/QA only).
    pub sidecar: Sidecar,
    /// Processing outcome.
    pub status: Status,
    /// Optional, human-only short notes. Not for agents.
    pub imprints: Option<Vec<String>>,
}

impl AdapterOut {
    /// Builder that wires shape & evidence fields into the sidecar to avoid mismatches.
    pub fn new(
        emb: Vec<f32>,
        shape: EmbeddingsShape,
        evidence: EvidenceHandle,
        mut sidecar: Sidecar,
        status: Status,
    ) -> Self {
        sidecar.lengths = Some(shape);
        sidecar.evidence_hash8 = evidence.hash8;
        Self { emb, shape, evidence, sidecar, status, imprints: None }
    }

    /// Attach imprints (human-only debug notes).
    #[inline]
    pub fn with_imprints(mut self, notes: Vec<String>) -> Self {
        self.imprints = Some(notes);
        self
    }

    /// Total number of f32 values in `emb` (equals `shape.len()` when valid).
    #[inline] pub fn len(&self) -> usize { self.emb.len() }
    /// True when no embeddings were produced.
    #[inline] pub fn is_empty(&self) -> bool { self.emb.is_empty() }
}

/// Core adapter trait. Implementations live in modality-specific crates.
pub trait Adapter: Send + Sync {
    /// Stable identifier (e.g., `"text.v1.conv1d"`).
    fn id(&self) -> &'static str;
    /// Report the adapter's primary modality.
    fn modality(&self) -> Modality;
    /// Output embedding width (d_model). Must match `shape.d_model` in `AdapterOut`.
    fn d_model(&self) -> u16;

    /// Process a single `ByteFrame` and return embeddings + metadata.
    ///
    /// Implementations must not perform hidden IO and should adhere to the
    /// determinism/budget semantics implied by `AdapterCtx`.
    fn process(&self, frame: &ByteFrame, ctx: &AdapterCtx) -> SenseResult<AdapterOut>;
}
