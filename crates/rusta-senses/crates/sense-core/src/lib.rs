//! Core contracts for Sozna-sense.
//!
//! Defines ByteFrame (input), TraceTag/Phase/Status (observability),
//! SenseOut + Sidecar (output), Modality/SenseConfig (routing/config), and Adapter trait.

pub mod adapter;
pub mod byteframe;
pub mod senseout;
pub mod trace;

pub use adapter::{Adapter, BoxedAdapter};
pub use byteframe::ByteFrame;
pub use senseout::{
    AudioMeta, BackendInfo, BinaryMeta, Modality, NewlineNorm, SenseConfig, SenseOut, Sidecar,
    TextMeta, TextNorm,
};
pub use trace::{Phase, Range, Reason, ReasonClass, Status, Trace, TraceTag};
