#![allow(missing_docs)]
/// Processing outcome classification used across adapters, detector, and facade.
///
/// Ordering reflects severity: `Ok < Warn < Degraded < Fail`.
/// Stable `repr(u8)` and explicit discriminants for FFI/logging.
#[repr(u8)]
#[non_exhaustive]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Status {
    Ok       = 0,
    Warn     = 1,
    Degraded = 2,
    Fail     = 3,
}

impl Status {
    /// Returns `true` if the pipeline produced a clean result.
    #[inline] pub const fn is_ok(self) -> bool { matches!(self, Status::Ok) }
    /// Returns `true` if the result is usable without retry (Ok or Warn).
    #[inline] pub const fn is_acceptable(self) -> bool { matches!(self, Status::Ok | Status::Warn) }
    /// Returns `true` if quality was reduced but output exists.
    #[inline] pub const fn is_degraded(self) -> bool { matches!(self, Status::Degraded) }
    /// Returns `true` if the operation failed and produced no usable output.
    #[inline] pub const fn is_fail(self) -> bool { matches!(self, Status::Fail) }

    /// Monotonic severity code (0..=3), stable for logs/FFI.
    #[inline] pub const fn code(self) -> u8 { self as u8 }

    /// Human string (stable, lowercase, no spaces) for metrics/sidecars.
    #[inline] pub const fn as_str(self) -> &'static str {
        match self {
            Status::Ok => "ok",
            Status::Warn => "warn",
            Status::Degraded => "degraded",
            Status::Fail => "fail",
        }
    }

    /// Returns the worse (more severe) of two statuses.
    #[inline]
    pub const fn worst(a: Status, b: Status) -> Status {
        if (a as u8) >= (b as u8) { a } else { b }
    }
}

impl core::fmt::Display for Status {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { f.write_str(self.as_str()) }
}

impl core::cmp::PartialOrd for Status {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some((*self as u8).cmp(&(*other as u8)))
    }
}
impl core::cmp::Ord for Status {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering { (*self as u8).cmp(&(*other as u8)) }
}

impl core::convert::TryFrom<u8> for Status {
    type Error = ();
    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v { 0 => Ok(Status::Ok), 1 => Ok(Status::Warn), 2 => Ok(Status::Degraded), 3 => Ok(Status::Fail), _ => Err(()) }
    }
}