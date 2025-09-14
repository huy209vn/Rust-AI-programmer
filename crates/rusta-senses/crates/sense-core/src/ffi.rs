#![cfg(feature = "ffi")]
/// FFI-safe mirror enums/types (no functions). Keep in lockstep with core.

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum sense_status_t { Ok = 0, Warn = 1, Degraded = 2, Fail = 3 }

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum sense_dtype_t { F32 = 0, F16 = 1, BF16 = 2 }

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum sense_layout_t { RowMajor = 0 }

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct sense_embeddings_shape_t { pub n: u32, pub d_model: u16 }
