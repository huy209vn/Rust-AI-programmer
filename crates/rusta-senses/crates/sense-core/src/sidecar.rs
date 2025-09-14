use crate::determinism::EmbeddingsShape;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DType { F32, F16, BF16 }

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Layout { RowMajor }

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct BackendInfo {
    pub device: &'static str, // "cpu" | "cuda" | "metal" | ...
    pub dtype: DType,         // embeddings dtype
    pub layout: Layout,       // memory layout
    pub threads: u16,         // CPU logical threads used
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct TextSidecar { pub norm_nfc: bool, pub newline_lf: bool }

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct AudioSidecar { pub sr: u32, pub win: u16, pub hop: u16, pub mel: u8, pub patch_t: u8 }

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct VisionSidecar { pub patch: u16, pub grid: (u16,u16), pub exif_applied: bool, pub animated: bool }

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct BinarySidecar { pub stride: u16, pub length: u64, pub container_hint: Option<&'static str> }

/// Human/QA metadata (agent never sees this unless explicitly inspected).
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct Sidecar {
    pub schema_ver: u16,                 // 1
    pub lengths: Option<EmbeddingsShape>,

    pub grid: Option<(u16,u16)>,         // generic fallback grid if not vision
    pub text: Option<TextSidecar>,
    pub audio: Option<AudioSidecar>,
    pub vision: Option<VisionSidecar>,
    pub binary: Option<BinarySidecar>,

    pub backend: BackendInfo,
    pub evidence_hash8: [u8; 8],
}
