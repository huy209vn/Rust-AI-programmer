use crate::{ByteFrame, Modality, SenseConfig, SenseOut};

/// Modality-specific encoder interface.
///
/// Design rules:
/// - **Stateless & thread-safe**: implementors must be Send+Sync so the façade can share them.
/// - **Pure**: never does IO; operate only on provided bytes & config.
/// - **Deterministic in eval**: same bytes + cfg ⇒ same output (callers will rely on this).
pub trait Adapter: Send + Sync {
    /// Stable human-readable name (e.g., "text.conv1d", "audio.stft-mel", "vision.patch16", "binary.raw")
    fn name(&self) -> &'static str;

    /// Which routing label this adapter implements.
    fn modality(&self) -> Modality;

    /// Optional quick preflight; default OK.
    /// You can use this to check basic invariants (e.g., min size) and fail fast with a helpful message.
    #[inline]
    fn preflight(&self, _bf: &ByteFrame, _cfg: &SenseConfig) -> anyhow::Result<()> {
        Ok(())
    }

    /// Core work: turn bytes into (embeddings, trace).
    ///
    /// Contract:
    /// - No panics; return Err on unrecoverable issues.
    /// - Prefer **fail-soft** in the orchestrator (fallback to Binary) — but here you may still Err.
    /// - Fill `trace` with clear tags of steps/params taken.
    fn adapt(&self, bf: &ByteFrame, cfg: &SenseConfig) -> anyhow::Result<SenseOut>;
}

/// Convenient boxed adapter type for registries.
pub type BoxedAdapter = Box<dyn Adapter>;
