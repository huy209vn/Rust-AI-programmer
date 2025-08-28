use crate::trace::TraceTag;
use serde::{Deserialize, Serialize};

/// Routing labels for detectors/adapters.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Modality {
    Text,
    Audio,
    Vision,
    Binary,
    Container,
}

/// Minimal, stable config knobs for v0.
/// (Record effective values in `trace` when used.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SenseConfig {
    /// Embedding dims per modality
    pub d_text: usize, // 512
    pub d_audio: usize,  // 512
    pub d_vision: usize, // 768
    pub d_binary: usize, // 512

    /// Guards (ingest enforces, we record in trace)
    pub max_frame_bytes: usize, // 16 MB
    pub session_cap_bytes: usize, // 256 MB
    pub decode_timeout_ms: u32,   // 5s (local); net is handled at ingest
}
impl Default for SenseConfig {
    fn default() -> Self {
        Self {
            d_text: 512,
            d_audio: 512,
            d_vision: 768,
            d_binary: 512,
            max_frame_bytes: 16 * 1024 * 1024,
            session_cap_bytes: 256 * 1024 * 1024,
            decode_timeout_ms: 5_000,
        }
    }
}

/// Side metadata useful for downstream positioners/fusers.
/// All fields optional; include *only what applies* to the given sample.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Sidecar {
    /// Total tokens/patches per stream (if multi-stream later).
    pub lengths: Option<usize>,

    /// vision patch grid (H_patches, W_patches).
    #[serde(alias = "image_grid")] // accept old name when deserializing
    pub vision_grid: Option<(u32, u32)>,

    /// Audio transform parameters used.
    pub audio: Option<AudioMeta>,

    /// Text normalization policy applied.
    pub text: Option<TextMeta>,

    /// Binary adapter policy (stride/chunks).
    pub binary: Option<BinaryMeta>,

    /// Short/long hashes for provenance & replay (optional).
    pub hash_short: Option<String>, // e.g., first 8 bytes hex of blake3
    pub hash_hex: Option<String>, // full 64-char hex

    /// Backend/runtime hints for determinism audits.
    pub backend: Option<BackendInfo>,

    /// Optional per-token quality flags (shape carried in `qvec_shape`).
    pub qvec_shape: Option<(usize,)>, // e.g., (rows,) for one score per token
}

/// Audio transform details (record what we actually did).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioMeta {
    pub sr_hz: u32,   // 16_000
    pub win: u32,     // 1024
    pub hop: u32,     // 256
    pub mel: u32,     // 80
    pub patch_t: u32, // 8
}

/// Text normalization details.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextMeta {
    pub unicode_norm: TextNorm, // Nfc, Nfkc, etc.
    pub newline_norm: NewlineNorm,
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TextNorm {
    Nfc,
    Nfkc,
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum NewlineNorm {
    Lf,
    CrLf,
    Native,
}

/// Binary adapter policy that affects coverage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinaryMeta {
    pub stride: Option<u32>,             // e.g., 4
    pub chunks: Option<Vec<(u64, u64)>>, // byte ranges processed
}

/// What could affect determinism; handy to log for audits.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackendInfo {
    pub decoder: Option<String>, // "jpeg-turbo 3.0.2", "lewton 0.10", etc.
    pub device: Option<String>,  // "cpu", "cuda:0"
    pub threads: Option<u32>,    // worker threads used
}

/// Output of sensing: embeddings + shape + trace + optional sidecar.
/// Embeddings are row-major flattened (len = rows * cols).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SenseOut {
    pub emb: Vec<f32>,
    pub rows: usize,          // tokens/patches
    pub cols: usize,          // d_model
    pub trace: Vec<TraceTag>, // ordered trace tags
    #[serde(default)]
    pub sidecar: Sidecar, // optional metadata for downstreams
}

impl SenseOut {
    /// Checked constructor from an existing vec. Panics if shape mismatches length.
    #[inline]
    pub fn from_vec(emb: Vec<f32>, rows: usize, cols: usize, trace: Vec<TraceTag>) -> Self {
        assert_eq!(emb.len(), rows * cols, "emb length != rows*cols");
        Self {
            emb,
            rows,
            cols,
            trace,
            sidecar: Sidecar::default(),
        }
    }

    /// Zero-initialized embedding with provided trace.
    #[inline]
    pub fn zeros(rows: usize, cols: usize, trace: Vec<TraceTag>) -> Self {
        Self {
            emb: vec![0.0; rows * cols],
            rows,
            cols,
            trace,
            sidecar: Sidecar::default(),
        }
    }

    /// Shape helper.
    #[inline]
    pub fn shape(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    /// Row slice (safe) – useful in tests or simple processing.
    #[inline]
    pub fn row(&self, r: usize) -> &[f32] {
        let (rows, cols) = (self.rows, self.cols);
        assert!(r < rows, "row out of bounds");
        let start = r * cols;
        &self.emb[start..start + cols]
    }

    /// Append a trace tag.
    #[inline]
    pub fn push_trace(&mut self, tag: TraceTag) {
        self.trace.push(tag);
    }

    /// Attach/merge sidecar info.
    #[inline]
    pub fn with_sidecar(mut self, sidecar: Sidecar) -> Self {
        self.sidecar = sidecar;
        self
    }
}
