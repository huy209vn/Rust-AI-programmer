#[cfg(feature = "std")]
use std::sync::Arc;
#[cfg(not(feature = "std"))]
use alloc::sync::Arc;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

/// Zero-copy-friendly byte storage used by ByteFrame.
/// Contract: never performs IO; zero-copy when `Borrowed` or `Owned(Arc)`.
#[derive(Clone, Debug)]
pub enum Bytes<'a> {
    /// Borrowed bytes (caller-owned). Lifetime-tied to the input buffer.
    Borrowed(&'a [u8]),
    /// Owned, ref-counted bytes. Cheap to clone across layers/threads.
    Owned(Arc<[u8]>),
}

impl<'a> Bytes<'a> {
    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        match self { Self::Borrowed(b) => b, Self::Owned(a) => a }
    }
    #[inline] pub fn len(&self) -> usize { self.as_slice().len() }
    #[inline] pub fn is_empty(&self) -> bool { self.len() == 0 }

    /// Ensure ownership and return an `Arc<[u8]>` (no copy if already owned).
    #[inline]
    pub fn into_owned(self) -> Arc<[u8]> {
        match self { Bytes::Owned(a) => a, Bytes::Borrowed(b) => Arc::<[u8]>::from(b) }
    }
}

impl<'a> AsRef<[u8]> for Bytes<'a> { #[inline] fn as_ref(&self) -> &[u8] { self.as_slice() } }
impl<'a> From<&'a [u8]> for Bytes<'a> { #[inline] fn from(s: &'a [u8]) -> Self { Bytes::Borrowed(s) } }
impl From<Vec<u8>> for Bytes<'_> { #[inline] fn from(v: Vec<u8>) -> Self { Bytes::Owned(Arc::<[u8]>::from(v)) } }
impl From<Arc<[u8]>> for Bytes<'_> { #[inline] fn from(a: Arc<[u8]>) -> Self { Bytes::Owned(a) } }