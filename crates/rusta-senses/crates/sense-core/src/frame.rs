use crate::bytes::Bytes;

#[cfg(feature = "std")]
use std::sync::Arc;
#[cfg(not(feature = "std"))]
use alloc::sync::Arc;

#[cfg(not(feature = "std"))]
use alloc::string::String;

/// Canonical identifier for a source. Keep cheap-to-clone.
pub type SourceId = String;

/// Monotonic + wall clock timestamps (ns). In `no_std` or when unavailable,
/// `wall_ns` may be 0. `mono_ns` is reserved for upstream runtimes that track
/// a monotonic epoch; core does not synthesize it.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Default)]
pub struct Timestamp {
    pub mono_ns: u128,
    pub wall_ns: u128,
}

impl Timestamp {
    /// Wall-clock timestamp (ns since UNIX_EPOCH). `mono_ns = 0`.
    #[cfg(feature = "std")]
    pub fn now_wall() -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        let wall_ns = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        Self { mono_ns: 0, wall_ns }
    }

    /// Construct explicitly from parts.
    pub fn from_parts(mono_ns: u128, wall_ns: u128) -> Self { Self { mono_ns, wall_ns } }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
#[derive(Default, Clone, Debug)]
pub struct Hints {
    pub ext: Option<String>,
    pub mime: Option<String>,
    pub sr: Option<u32>,
    pub channels: Option<u8>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub duration_ms: Option<u32>,
}

/// Caller-provided frame of bytes; Sense performs no hidden IO.
#[derive(Clone, Debug)]
pub struct ByteFrame<'a> {
    pub data: Bytes<'a>,
    pub ts: Timestamp,
    pub source_id: SourceId,
    pub hints: Option<Hints>,
}

impl<'a> ByteFrame<'a> {
    /// Borrow without copying.
    pub fn from_borrowed(data: &'a [u8], source_id: impl Into<SourceId>, hints: Option<Hints>) -> Self {
        Self { data: Bytes::Borrowed(data), ts: Timestamp::default(), source_id: source_id.into(), hints }
    }
    /// Own the bytes (Arc-backed) without copying when possible.
    pub fn from_owned(data: Arc<[u8]>, source_id: impl Into<SourceId>, hints: Option<Hints>) -> Self {
        Self { data: Bytes::Owned(data), ts: Timestamp::default(), source_id: source_id.into(), hints }
    }
    /// Set an explicit timestamp.
    #[inline] pub fn with_timestamp(mut self, ts: Timestamp) -> Self { self.ts = ts; self }

    /// Convenience helpers.
    #[inline] pub fn len(&self) -> usize { self.data.len() }
    #[inline] pub fn is_empty(&self) -> bool { self.data.is_empty() }
}