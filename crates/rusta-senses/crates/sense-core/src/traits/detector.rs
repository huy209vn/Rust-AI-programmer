//! Detector contract: classify a `ByteFrame` into a target `Modality` with confidence.
//!
//! The detector should be **cheap** (magic bytes, BOM, headers, shallow heuristics),
//! pure (no hidden IO), and conservative — misroutes fall back to the Binary adapter.

use crate::error::SenseResult;
use crate::frame::ByteFrame;
use super::adapter::Modality;

/// Routing decision with confidence and a short human-readable reason.
#[derive(Clone, Debug)]
pub struct Route {
    /// Target modality for this frame.
    pub modality: Modality,
    /// Confidence in parts-per-million (0..=1_000_000) to avoid floats.
    pub confidence_ppm: u32,
    /// Short explanation, e.g., "jpeg SOI", "utf8 valid".
    pub reason: &'static str,
}

impl Route {
    /// Clamp confidence into [0, 1_000_000].
    #[inline]
    pub fn clamped(mut self) -> Self {
        const MAX: u32 = 1_000_000;
        if self.confidence_ppm > MAX { self.confidence_ppm = MAX; }
        self
    }
}

/// Stateless detector trait.
pub trait Detector: Send + Sync {
    /// Stable identifier for the detector implementation (e.g., "magic-bytes.v1").
    fn id(&self) -> &'static str;

    /// Inspect the frame and propose a routing decision.
    fn detect(&self, frame: &ByteFrame) -> SenseResult<Route>;
}
