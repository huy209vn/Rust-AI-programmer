use serde::{Deserialize, Serialize};

/// Uniform envelope for raw input bytes + provenance.
/// Owns its data; no IO here.
///
/// Design:
/// - Optional timestamp (`ts_ms`) so replay/tests can fix or omit time.
/// - `with_now()` for ergonomic construction with current time.
/// - Stores BLAKE3 content hash for provenance & cheap short ids.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ByteFrame {
    /// Opaque input bytes.
    pub data: Vec<u8>,
    /// Millisecond timestamp since UNIX_EPOCH (wall or monotonic; caller decides).
    /// None if unknown/irrelevant.
    pub ts_ms: Option<u128>,
    /// Provenance tag (e.g., "file://...", "mic:0", "http://...").
    pub source_id: String,
    /// Optional MIME/extension hint to help routing (e.g., "vision/png").
    pub hint_mime: Option<String>,
    /// Full BLAKE3 hash of `data` (32 bytes). Persisted for provenance/replay.
    #[serde(with = "serde_blake3")]
    pub content_hash: blake3::Hash,
}

impl ByteFrame {
    /// Construct with explicit timestamp (or None).
    #[inline]
    pub fn from_bytes<D: Into<Vec<u8>>, S: Into<String>>(
        data: D,
        ts_ms: Option<u128>,
        source_id: S,
    ) -> Self {
        let data = data.into();
        let content_hash = blake3::hash(&data);
        Self {
            data,
            ts_ms,
            source_id: source_id.into(),
            hint_mime: None,
            content_hash,
        }
    }

    /// Construct and automatically attach SystemTime::now().
    #[inline]
    pub fn with_now<D: Into<Vec<u8>>, S: Into<String>>(data: D, source_id: S) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .ok()
            .map(|d| d.as_millis());
        Self::from_bytes(data, now, source_id)
    }

    /// Attach/override a MIME hint.
    #[inline]
    pub fn with_mime(mut self, mime: impl Into<String>) -> Self {
        self.hint_mime = Some(mime.into());
        self
    }

    /// Enforce a max frame size: return `None` if over limit (no mutation here).
    #[inline]
    pub fn guard_max_size(self, max_bytes: usize) -> Option<Self> {
        if self.data.len() <= max_bytes {
            Some(self)
        } else {
            None
        }
    }

    /// Byte length.
    #[inline]
    pub fn len(&self) -> usize {
        self.data.len()
    }
    /// Is empty?
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Stable short id for logs/traces (first 8 bytes of blake3, hex).
    #[inline]
    pub fn short_id(&self) -> String {
        let bytes = self.content_hash.as_bytes();
        let mut s = String::with_capacity(16);
        for b in &bytes[..8] {
            use core::fmt::Write;
            let _ = write!(s, "{:02x}", b);
        }
        s
    }

    /// Full 64-char hex of the blake3 hash.
    #[inline]
    pub fn hash_hex(&self) -> String {
        self.content_hash.to_hex().to_string()
    }
}

impl Default for ByteFrame {
    fn default() -> Self {
        let data = Vec::new();
        let content_hash = blake3::hash(&data);
        Self {
            data,
            ts_ms: None,
            source_id: String::new(),
            hint_mime: None,
            content_hash,
        }
    }
}

/// serde helper for blake3::Hash (32 bytes).
mod serde_blake3 {
    use super::*;
    use serde::{Deserializer, Serializer};

    pub fn serialize<S: Serializer>(h: &blake3::Hash, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_bytes(h.as_bytes())
    }
    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<blake3::Hash, D::Error> {
        let bytes: Vec<u8> = <Vec<u8>>::deserialize(d)?;
        if bytes.len() != blake3::OUT_LEN {
            return Err(serde::de::Error::custom("invalid blake3 hash length"));
        }
        let mut arr = [0u8; blake3::OUT_LEN];
        arr.copy_from_slice(&bytes);
        Ok(blake3::Hash::from(arr))
    }
}
