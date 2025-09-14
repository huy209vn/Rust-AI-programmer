use blake3::Hasher;
use crate::constants::EVIDENCE_SCHEMA_VER;

#[cfg(not(feature = "std"))]
use alloc::string::String;

/// What the evidence hash represents.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EvidenceKind { IngestContent }

/// Evidence that anchors an output to its input bytes.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct EvidenceHandle {
    pub kind: EvidenceKind,  // IngestContent
    pub schema_ver: u16,     // EVIDENCE_SCHEMA_VER
    pub hash8: [u8; 8],      // blake3(data) truncated
    pub source_id: String,   // path/url/device name
}

impl EvidenceHandle {
    /// Convenience builder for the common case (ingest bytes).
    pub fn new_ingest(source_id: String, bytes: &[u8]) -> Self {
        Self {
            kind: EvidenceKind::IngestContent,
            schema_ver: EVIDENCE_SCHEMA_VER,
            hash8: short_hash_8(bytes),
            source_id,
        }
    }
}

/// Short blake3-8 hash of the input bytes (stable truncation).
#[inline]
pub fn short_hash_8(bytes: &[u8]) -> [u8; 8] {
    let mut hasher = Hasher::new();
    hasher.update(bytes);
    let full = hasher.finalize();
    let mut out = [0u8; 8];
    out.copy_from_slice(&full.as_bytes()[..8]);
    out
}

/// Short blake3 hash as a `u64` (little-endian) for compact metrics keys.
#[inline]
pub fn short_hash_u64(bytes: &[u8]) -> u64 {
    u64::from_le_bytes(short_hash_8(bytes))
}
