// lib.rs (rusta-sense)

pub use sense_core::{Adapter, ByteFrame, Modality, SenseConfig, SenseOut, TraceTag};

// Builder to customize which adapters/limits to use
pub struct SenseBuilder {
    cfg: SenseConfig,
    adapters: Vec<Box<dyn Adapter + Send + Sync>>,
    enable_webp: bool,
    // … feature toggles and guards
}

impl SenseBuilder {
    pub fn new(cfg: SenseConfig) -> Self { /* … */ }
    pub fn with_default_adapters(self) -> Self { /* text, audio, vision, binary */ }
    pub fn add_adapter<A: Adapter + Send + Sync + 'static>(mut self, a: A) -> Self { /* … */ }
    pub fn build(self) -> Sense { /* … */ }
}

// The façade object you use
pub struct Sense {
    cfg: SenseConfig,
    adapters: Vec<Box<dyn Adapter + Send + Sync>>,
    // detector config cached
}

impl Sense {
    /// One-shot processing. Never performs device/IO; pure on the provided bytes.
    pub fn process(&self, bf: &ByteFrame) -> anyhow::Result<SenseOut> {
        // 1) detect modality (+ trace tag)
        // 2) route to matching adapter
        // 3) append trace, enforce caps, return
        // 4) on decode/routing failure, fall back to Binary adapter
        // 5) never panic
        /* … */
    }

    /// Convenience for hashing & short id (useful in logs/trace)
    pub fn blake3_short_id(bf: &ByteFrame) -> String { /* … */ }
}

/// Zero-allocation helper for common case
pub fn process(bf: &ByteFrame, cfg: &SenseConfig) -> anyhow::Result<SenseOut> {
    SenseBuilder::new(cfg.clone())
        .with_default_adapters()
        .build()
        .process(bf)
}