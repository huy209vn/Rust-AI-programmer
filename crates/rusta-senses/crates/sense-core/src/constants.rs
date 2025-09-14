/// Defaults and modality constants.
pub const DEFAULT_MAX_FRAME_MB: u32 = 16;
pub const DEFAULT_SESSION_CAP_MB: u32 = 256;
pub const DEFAULT_DECODE_TIMEOUT_MS: u32 = 5_000;

pub const D_MODEL_TEXT: u16   = 512;
pub const D_MODEL_AUDIO: u16  = 512;
pub const D_MODEL_VISION: u16 = 768;
pub const D_MODEL_BINARY: u16 = 512;

pub const VISION_PATCH: u16 = 16;

/// Evidence schema version. Bump only on breaking semantics.
pub const EVIDENCE_SCHEMA_VER: u16 = 1;
