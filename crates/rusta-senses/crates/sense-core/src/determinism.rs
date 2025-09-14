#![allow(missing_docs)]
use core::num::{NonZeroU16, NonZeroU32};

/// Determinism levels — policy signals used by adapters/flow.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Determinism { D1, D2, D3 }

/// Budget classes — caps/latency envelopes chosen by the caller.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BudgetClass { L, M, H }

/// Randomness seed passed to adapters to pin nondeterministic ops.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RandSeed(pub u64);

/// Embeddings shape (n × d_model), non-zero enforced.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EmbeddingsShape { pub n: NonZeroU32, pub d_model: NonZeroU16 }

impl EmbeddingsShape {
    #[inline]
    pub const fn len(self) -> usize {
        (self.n.get() as usize) * (self.d_model.get() as usize)
    }
}
