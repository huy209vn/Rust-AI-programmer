use bitflags::bitflags;
use crate::determinism::{BudgetClass, Determinism};
use crate::constants::{DEFAULT_MAX_FRAME_MB, DEFAULT_SESSION_CAP_MB, DEFAULT_DECODE_TIMEOUT_MS};

/// Size/time caps for frames and sessions.
#[derive(Clone, Debug)]
pub struct Caps {
    pub max_frame_mb: u32,
    pub session_cap_mb: u32,
    pub decode_timeout_ms: u32,
}

impl Default for Caps {
    fn default() -> Self {
        Self {
            max_frame_mb: DEFAULT_MAX_FRAME_MB,
            session_cap_mb: DEFAULT_SESSION_CAP_MB,
            decode_timeout_ms: DEFAULT_DECODE_TIMEOUT_MS,
        }
    }
}

bitflags! {
    /// Feature flags to enable modality families at construction time.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct Features: u32 {
        const TEXT   = 0b0001;
        const AUDIO  = 0b0010;
        const VISION = 0b0100;
        const BINARY = 0b1000;
    }
}

/// Adapter knobs that callers can override per invocation.
#[derive(Clone, Debug, Default)]
pub struct AdapterCfgOverride {
    pub text_d_model: Option<u16>,
    pub audio_sr: Option<u32>,
    pub vision_patch: Option<u16>,
    pub binary_stride: Option<u16>,
}

/// Control what outside-only data is exposed by the facade.
#[derive(Clone, Copy, Debug, Default)]
pub struct InspectPolicy {
    pub include_evidence: bool,
    pub include_sidecar: bool,
    pub include_imprints: bool,
}

/// Per-call configuration (immutable to adapters).
#[derive(Clone, Debug, Default)]
pub struct SenseCfg {
    pub inspect: InspectPolicy,
    pub adapter_overrides: AdapterCfgOverride,
}

/// Engine-time configuration: determinism & budgets.
#[derive(Clone, Debug)]
pub struct EngineCfg {
    pub determinism: Determinism,
    pub budgets: BudgetClass,
    pub caps: Caps,
    pub features: Features,
}

impl Default for EngineCfg {
    fn default() -> Self {
        Self {
            determinism: Determinism::D2,
            budgets: BudgetClass::M,
            caps: Caps::default(),
            features: Features::TEXT | Features::AUDIO | Features::VISION | Features::BINARY,
        }
    }
}
