use crate::status::Status;
use thiserror::Error;

#[cfg(not(feature = "std"))]
use alloc::string::String;

/// Stable error codes for telemetry/FFI. Additive; do not reorder.
#[repr(u16)]
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ErrorCode {
    Decode = 1,
    Format = 2,
    Bounds = 3,
    Device = 4,
    Config = 5,
    Container = 6,
    Backend = 7,
    Timeout = 8,
    PolicyDrop = 9,
    Internal = 255,
}

/// Unified error type across detector/adapters/facade.
#[derive(Error, Debug)]
pub enum SenseError {
    #[error("decode error: {0}")]
    Decode(String),
    #[error("format error: {0}")]
    Format(String),
    #[error("bounds error: {0}")]
    Bounds(String),
    #[error("device error: {0}")]
    Device(String),
    #[error("config error: {0}")]
    Config(String),
    #[error("container error: {0}")]
    Container(String),
    #[error("backend error: {0}")]
    Backend(String),
    #[error("timeout")]
    Timeout,
    #[error("policy drop: {0}")]
    PolicyDrop(&'static str),
    #[error("internal error: {0}")]
    Internal(String),
}

impl SenseError {
    /// Conservative status hint used by the facade/flow to classify outcomes.
    #[inline]
    pub fn status_hint(&self) -> Status {
        use SenseError::*;
        match self {
            Decode(_)     => Status::Degraded,
            Bounds(_)     => Status::Warn,
            Container(_)  => Status::Degraded,
            Backend(_)    => Status::Degraded,
            Timeout       => Status::Degraded,
            PolicyDrop(_) => Status::Degraded,
            Format(_)     => Status::Fail,
            Device(_)     => Status::Fail,
            Config(_)     => Status::Fail,
            Internal(_)   => Status::Fail,
        }
    }
    /// Stable code for telemetry/FFI.
    #[inline]
    pub fn code(&self) -> ErrorCode {
        use SenseError::*;
        match self {
            Decode(_) => ErrorCode::Decode,
            Format(_) => ErrorCode::Format,
            Bounds(_) => ErrorCode::Bounds,
            Device(_) => ErrorCode::Device,
            Config(_) => ErrorCode::Config,
            Container(_) => ErrorCode::Container,
            Backend(_) => ErrorCode::Backend,
            Timeout => ErrorCode::Timeout,
            PolicyDrop(_) => ErrorCode::PolicyDrop,
            Internal(_) => ErrorCode::Internal,
        }
    }
}

/// Convenience result alias.
pub type SenseResult<T> = Result<T, SenseError>;

#[cfg(feature = "std")]
impl From<std::io::Error> for SenseError {
    fn from(e: std::io::Error) -> Self { SenseError::Backend(e.to_string()) }
}