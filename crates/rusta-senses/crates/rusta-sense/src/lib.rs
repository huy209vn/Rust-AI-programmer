//! rusta-sense (facade) — v1.0 public API
//! ByteFrame in → embeddings out. Agent path never receives Evidence/Sidecar/Imprints by default.

#![deny(missing_docs)]
#![forbid(unsafe_code)]

use std::sync::Arc;

// --- Re-exports from sense-core (agent-safe only in prelude) ----------------
pub use sense_core::{ByteFrame, Bytes, Hints, Timestamp, Status};
// in rusta-sense/src/lib.rs
pub use sense_flow::{SessionId, FlowId, FlowCfg, FlowPolicy, ChunkCfg, CloseReason};
// Note: Sidecar/Evidence live in sense-core. We reference them by path but do NOT
// put them in the prelude to keep agent path clean-by-default.
pub type EvidenceHandle = sense_core::EvidenceHandle;
pub type Sidecar        = sense_core::Sidecar;
pub type TextSidecar    = sense_core::TextSidecar;
pub type BinarySidecar  = sense_core::BinarySidecar;
pub type BackendInfo    = sense_core::BackendInfo;

// --- Config -----------------------------------------------------------------

/// Determinism policy: D1 forensic, D2 eval default, D3 research.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Determinism { D1, D2, D3 }

/// Budget class influencing caps/latency targets.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BudgetClass { L, M, H }

/// Engine-wide caps and timeouts.
#[derive(Clone, Debug)]
pub struct Caps {
    /// Max bytes per frame (MB). Default: 16.
    pub max_frame_mb: u32,
    /// Max bytes per session (MB). Default: 256.
    pub session_cap_mb: u32,
    /// Decode timeout per item (ms). Default: 5000.
    pub decode_timeout_ms: u32,
}

bitflags::bitflags! {
    /// Enabled modalities.
    pub struct Features: u32 {
        /// Enable text adapter.
        const TEXT   = 0b0001;
        /// Enable audio adapter.
        const AUDIO  = 0b0010;
        /// Enable vision adapter.
        const VISION = 0b0100;
        /// Enable binary adapter (fallback).
        const BINARY = 0b1000;
    }
}

/// Engine construction configuration.
#[derive(Clone, Debug)]
pub struct EngineCfg {
    /// D1|D2|D3 (default D2).
    pub determinism: Determinism,
    /// L|M|H budgets (caps/latency).
    pub budgets: BudgetClass,
    /// Frame/session/timeouts caps.
    pub caps: Caps,
    /// Enabled modalities.
    pub features: Features,
}

/// Per-call configuration controlling inspection and small adapter tweaks.
#[derive(Clone, Default, Debug)]
pub struct SenseCfg {
    /// Controls visibility of outside-only artifacts on `SenseOut`.
    pub inspect: InspectPolicy,
    /// Optional per-modality knobs.
    pub adapter_overrides: AdapterCfgOverride,
}

/// Controls whether outside-only artifacts are populated.
#[derive(Clone, Copy, Default, Debug)]
pub struct InspectPolicy {
    /// Include evidence handle.
    pub include_evidence: bool,
    /// Include sidecar metadata.
    pub include_sidecar: bool,
    /// Include human-only imprints (strings).
    pub include_imprints: bool,
}

/// Optional per-modality tweaks (kept small and stable).
#[derive(Clone, Default, Debug)]
pub struct AdapterCfgOverride {
    /// Override text d_model (default: 512).
    pub text_d_model:  Option<u16>,
    /// Override audio target sample rate (default: 16_000).
    pub audio_sr:      Option<u32>,
    /// Override vision patch size (default: 16).
    pub vision_patch:  Option<u16>,
    /// Override binary conv stride (default: 4 or impl-defined).
    pub binary_stride: Option<u16>,
}

// --- Embeddings (agent-safe) ------------------------------------------------

/// Borrowed embeddings view for the agent path.
pub struct EmbeddingsView<'a> {
    /// Row-major embeddings buffer (length = N * d_model).
    pub emb: &'a [f32],
    /// Embedding width.
    pub d_model: u16,
    /// Processing status (Ok/Warn/Degraded/Fail).
    pub status: Status,
}

/// Owned embeddings view, suitable for long-lived buffers / FFI.
pub struct EmbeddingsViewOwned {
    /// Row-major embeddings buffer (length = N * d_model).
    pub emb: Vec<f32>,
    /// Embedding width.
    pub d_model: u16,
    /// Processing status (Ok/Warn/Degraded/Fail).
    pub status: Status,
}

// --- Outside-only artifacts (for humans/QA; not for agent by default) -------

/// Full outside-only output. Convert to `EmbeddingsView`/`EmbeddingsViewOwned` for agent use.
pub struct SenseOut {
    /// Row-major embeddings buffer.
    pub emb: Vec<f32>,
    /// Embedding width.
    pub d_model: u16,
    /// Short content hash + source id (present only if allowed by `InspectPolicy`).
    pub evidence: EvidenceHandle,
    /// Minimal structured metadata for QA (populated only if allowed).
    pub sidecar: Sidecar,
    /// Optional human-only note strings (present only if allowed).
    pub imprint: Option<Vec<String>>,
    /// Processing status.
    pub status: Status,
}

impl SenseOut {
    /// Strict agent view (borrowed): embeddings + status only.
    pub fn as_embeddings(&self) -> EmbeddingsView<'_> {
        EmbeddingsView { emb: &self.emb, d_model: self.d_model, status: self.status }
    }

    /// Strict agent view (owned): embeddings + status only.
    pub fn into_embeddings(self) -> EmbeddingsViewOwned {
        EmbeddingsViewOwned { emb: self.emb, d_model: self.d_model, status: self.status }
    }

    /// Access evidence if allowed by call-time `InspectPolicy`.
    pub fn evidence(&self) -> Option<&sense_core::EvidenceHandle> {
        // population is governed by the engine using SenseCfg.inspect
        Some(&self.evidence)
    }

    /// Access sidecar if allowed by call-time `InspectPolicy`.
    pub fn sidecar(&self) -> Option<&sense_core::Sidecar> {
        Some(&self.sidecar)
    }

    /// Access imprints if allowed by call-time `InspectPolicy`.
    pub fn imprints(&self) -> Option<&[String]> {
        self.imprint.as_deref()
    }
}

// --- Engine (batch + streaming entrypoints) ---------------------------------

/// Perception engine facade. No hidden IO; honors determinism/caps features.
pub struct Sense(Arc<Inner>);

struct Inner; // private

impl Sense {
    /// Construct a new engine instance.
    ///
    /// Invariants:
    /// - No hidden network/disk IO.
    /// - D2 default: single-thread within flow; fixed grids; seeded ops.
    pub fn new(_cfg: EngineCfg) -> Result<Self, SenseError> {
        unimplemented!()
    }

    // ----------------- Batch -----------------

    /// Human/QA path: returns outside-only `SenseOut`.
    ///
    /// Use `.as_embeddings()` or `.into_embeddings()` for strict agent path.
    pub fn process(&self, _bf: &ByteFrame, _cfg: &SenseCfg) -> Result<SenseOut, SenseError> {
        unimplemented!()
    }

    /// Agent convenience: returns embeddings-only owned view (no evidence/sidecar/imprints).
    pub fn process_embeddings(&self, _bf: &ByteFrame, _cfg: &SenseCfg)
        -> Result<EmbeddingsViewOwned, SenseError>
    {
        unimplemented!()
    }

    // ----------------- Streaming (thin over sense-flow) -----------------

    /// Start a multimodal session (bounded queues, budgets enforced).
    pub fn start_session(&self, _cfg: FlowCfg) -> Result<SessionId, SenseError> {
        unimplemented!()
    }

    /// Ingest a frame. Returns the flow id bound to (source_id + hints).
    pub fn ingest(&self, _sid: SessionId, _bf: ByteFrame) -> Result<FlowId, SenseError> {
        unimplemented!()
    }

    /// Non-blocking pull of zero or more emissions. Each emission is `(FlowId, SenseOut)`.
    pub fn poll(&self, _sid: SessionId) -> Result<Vec<(FlowId, SenseOut)>, SenseError> {
        unimplemented!()
    }

    /// Drain pending emissions honoring caps/timeouts.
    pub fn flush(&self, _sid: SessionId) -> Result<(), SenseError> {
        unimplemented!()
    }

    /// Close session with a reason.
    pub fn close(&self, _sid: SessionId, _reason: CloseReason) -> Result<(), SenseError> {
        unimplemented!()
    }
}

// Optional async surface
#[cfg(feature = "async")]
impl Sense {
    /// Backpressure-aware async iterator of emissions for the session.
    pub async fn next(&self, _sid: SessionId) -> Option<(FlowId, SenseOut)> {
        unimplemented!()
    }
}


// --- Errors -----------------------------------------------------------------

/// Engine error kinds with a Status mapping.
#[derive(thiserror::Error, Debug)]
pub enum SenseError {
    /// Decode failed, routed to Binary or Degraded when possible.
    #[error("decode error: {0}")]    Decode(String),
    /// Input format invalid (e.g., malformed UTF-16).
    #[error("format error: {0}")]    Format(String),
    /// Caps/bounds issues; often recoverable.
    #[error("bounds error: {0}")]    Bounds(String),
    /// Device/backend not available or failed.
    #[error("device error: {0}")]    Device(String),
    /// Bad or unsupported configuration.
    #[error("config error: {0}")]    Config(String),
    /// Container-specific problem (e.g., zip/pdf) → Binary fallback encouraged.
    #[error("container error: {0}")] Container(String),
    /// Backend library returned an error (deterministic with D2).
    #[error("backend error: {0}")]   Backend(String),
    /// Unexpected internal failure.
    #[error("internal error: {0}")]  Internal(String),
}

impl SenseError {
    /// Hint the `Status` a caller should expect if this error is surfaced as an emission.
    pub fn status_hint(&self) -> Status {
        use SenseError::*;
        match self {
            Decode(_)     => Status::Degraded,
            Bounds(_)     => Status::Warn,
            Format(_)     => Status::Fail,
            Device(_)     => Status::Fail,
            Config(_)     => Status::Fail,
            Container(_)  => Status::Degraded,
            Backend(_)    => Status::Degraded,
            Internal(_)   => Status::Fail,
        }
    }
}

// --- Prelude (agent-first, clean-by-default) --------------------------------

/// One-stop import for the common, agent-safe API.
pub mod prelude {
    pub use super::{
        Sense, EngineCfg, Determinism, BudgetClass, Caps, Features,
        SenseCfg, InspectPolicy, AdapterCfgOverride,
        SessionId, FlowId, FlowCfg, FlowPolicy, ChunkCfg, CloseReason,
        EmbeddingsView, EmbeddingsViewOwned, Status, SenseError,
    };
    pub use sense_core::{ByteFrame, Bytes, Hints, Timestamp};
    // Intentionally NOT exporting Sidecar/Evidence here.
}

// --- Feature guards ----------------------------------------------------------

/*
Features expected by this crate:
- "async"          : enables `Sense::next` async iterator.
- "imprint-debug"  : enables CLI surfaces to print imprints/sidecar (outside-only).
- "no_agent_leaks" : (enforced in CI) ensure agent modules cannot import Sidecar/Evidence.
*/
