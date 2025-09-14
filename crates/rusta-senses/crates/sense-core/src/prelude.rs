//! Convenience re-exports for downstream crates.

pub use crate::bytes::Bytes;
pub use crate::config::{AdapterCfgOverride, Caps, EngineCfg, Features, InspectPolicy, SenseCfg};
pub use crate::constants::*;
pub use crate::determinism::{BudgetClass, Determinism, EmbeddingsShape, RandSeed};
pub use crate::evidence::{short_hash_8, short_hash_u64, EvidenceHandle, EvidenceKind};
pub use crate::frame::{ByteFrame, Hints, Timestamp, SourceId};
pub use crate::sidecar::{AudioSidecar, BackendInfo, BinarySidecar, DType, Layout, Sidecar, TextSidecar, VisionSidecar};
pub use crate::status::Status;
pub use crate::error::{SenseError, SenseResult};
pub use crate::validators::validate_adapter_out;

pub use crate::traits::adapter::{Adapter, AdapterCtx, AdapterOut, Modality};
pub use crate::traits::detector::{Detector, Route};