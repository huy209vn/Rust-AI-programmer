#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![cfg_attr(not(feature = "std"), no_std)]

//! `sense-core` — stable contracts for Sozna / rusta-sense.
//!
//! - **No IO, no decoders, no runtime**. Pure types/traits/config.
//! - Designed for **determinism**, **semver stability**, and **FFI/WASM hygiene**.
//! - Adapters/Detector implementations live in other crates.

#[cfg(not(feature = "std"))]
extern crate alloc;

pub mod bytes;
pub mod frame;
pub mod status;
pub mod error;
pub mod determinism;
pub mod config;
pub mod evidence;
pub mod sidecar;
pub mod registry;
pub mod prelude;
pub mod constants;
pub mod validators;
#[cfg(feature = "ffi")]
pub mod ffi;

pub mod traits {
    //! Adapter / Detector contracts.
    pub mod adapter;
    pub mod detector;
}

// Re-export a convenient prelude for downstream crates.
pub use prelude::*;