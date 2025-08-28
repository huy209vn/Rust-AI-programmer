use serde::{Deserialize, Serialize};
use std::fmt::{self, Write};
use std::ops::Range as StdRange;

/// Pipeline phase that produced this tag.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Phase {
    Detector,  // header checks, UTF-* / BOM, container routing
    Adapter,   // modality adapter entry/selection
    Transform, // STFT, mel, patchify, conv stem, proj, norm...
    Guard,     // caps, timeouts, sandbox, fallbacks
}

impl fmt::Display for Phase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Phase::Detector => "detector",
            Phase::Adapter => "adapter",
            Phase::Transform => "transform",
            Phase::Guard => "guard",
        })
    }
}

/// Health of this step.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    #[default]
    Ok, // nominal
    Warn,     // noteworthy but fine (e.g., resample applied)
    Degraded, // fallback path (e.g., vision→binary)
    Fail,     // hard failure at this step
}


impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Status::Ok => "ok",
            Status::Warn => "warn",
            Status::Degraded => "degraded",
            Status::Fail => "fail",
        })
    }
}

/// Classify non-OK reasons.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ReasonClass {
    Decode,    // codec/parse failures
    Bounds,    // size/time caps
    Config,    // invalid/missing settings
    Device,    // hw/backend issues
    Format,    // malformed inputs
    Container, // pdf/zip etc. (no introspection)
    Routing,   // detector/adapter mismatch
}

impl fmt::Display for ReasonClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            ReasonClass::Decode => "decode",
            ReasonClass::Bounds => "bounds",
            ReasonClass::Config => "config",
            ReasonClass::Device => "device",
            ReasonClass::Format => "format",
            ReasonClass::Container => "container",
            ReasonClass::Routing => "routing",
        })
    }
}

/// Reason payload when status != Ok.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Reason {
    pub class: ReasonClass,
    pub message: String, // e.g., "webp decoder unavailable → binary"
}
impl Reason {
    #[inline]
    pub fn new(class: ReasonClass, msg: impl Into<String>) -> Self {
        Self {
            class,
            message: msg.into(),
        }
    }
    // ergonomic shorthands
    #[inline]
    pub fn decode(msg: impl Into<String>) -> Self {
        Self::new(ReasonClass::Decode, msg)
    }
    #[inline]
    pub fn bounds(msg: impl Into<String>) -> Self {
        Self::new(ReasonClass::Bounds, msg)
    }
    #[inline]
    pub fn config(msg: impl Into<String>) -> Self {
        Self::new(ReasonClass::Config, msg)
    }
    #[inline]
    pub fn device(msg: impl Into<String>) -> Self {
        Self::new(ReasonClass::Device, msg)
    }
    #[inline]
    pub fn format(msg: impl Into<String>) -> Self {
        Self::new(ReasonClass::Format, msg)
    }
    #[inline]
    pub fn container(msg: impl Into<String>) -> Self {
        Self::new(ReasonClass::Container, msg)
    }
    #[inline]
    pub fn routing(msg: impl Into<String>) -> Self {
        Self::new(ReasonClass::Routing, msg)
    }
}

/// Optional byte/token range this step pertains to (e.g., chunk windows).
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct Range {
    pub start: u64,
    pub end: u64, // exclusive
}
impl From<StdRange<u64>> for Range {
    fn from(r: StdRange<u64>) -> Self {
        Self {
            start: r.start,
            end: r.end,
        }
    }
}

/// One trace step (stable schema + ordering).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceTag {
    pub phase: Phase,
    pub name: String,   // "utf8","stft","patchify","proj",...
    pub params: String, // "win=1024, hop=256", "patch=16", ...
    #[serde(default)]
    pub status: Status, // ok|warn|degraded|fail
    pub reason: Option<Reason>, // set when status != Ok
    #[serde(default)]
    pub ranges: Vec<Range>, // e.g., chunk[0..1MB]
}

impl TraceTag {
    #[inline]
    pub fn new(phase: Phase, name: impl Into<String>) -> Self {
        Self {
            phase,
            name: name.into(),
            params: String::new(),
            status: Status::Ok,
            reason: None,
            ranges: vec![],
        }
    }

    /// Append `key=value` to params (comma-separated, stable order if called consistently).
    #[inline]
    pub fn param(mut self, key: &str, value: impl ToString) -> Self {
        if !self.params.is_empty() {
            self.params.push_str(", ");
        }
        let _ = write!(&mut self.params, "{}={}", key, value.to_string());
        self
    }

    /// In-place variant (avoids moves when building in a Vec).
    #[inline]
    pub fn param_kv_mut(&mut self, key: &str, value: impl ToString) {
        if !self.params.is_empty() {
            self.params.push_str(", ");
        }
        let _ = write!(&mut self.params, "{}={}", key, value.to_string());
    }

    /// Optional param: only add when Some(v).
    #[inline]
    pub fn param_opt(mut self, key: &str, value: Option<impl ToString>) -> Self {
        if let Some(v) = value {
            self = self.param(key, v);
        }
        self
    }

    /// Mark warn/degraded/fail with optional reason.
    #[inline]
    pub fn warn(mut self, reason: Option<Reason>) -> Self {
        self.status = Status::Warn;
        self.reason = reason;
        self
    }
    #[inline]
    pub fn degraded(mut self, reason: Option<Reason>) -> Self {
        self.status = Status::Degraded;
        self.reason = reason;
        self
    }
    #[inline]
    pub fn fail(mut self, reason: Reason) -> Self {
        self.status = Status::Fail;
        self.reason = Some(reason);
        self
    }

    /// Add a covered byte/token range.
    #[inline]
    pub fn range(mut self, start: u64, end: u64) -> Self {
        self.ranges.push(Range { start, end });
        self
    }
    /// Add a std Range<u64>.
    #[inline]
    pub fn range_std(mut self, r: StdRange<u64>) -> Self {
        self.ranges.push(r.into());
        self
    }

    /// Compact single-line form for logs.
    /// Example: "detector:utf8 params='ok' status=ok"
    #[inline]
    pub fn fmt_compact(&self) -> String {
        let mut s = String::new();
        let _ = write!(&mut s, "{}:{}", self.phase, self.name);
        if !self.params.is_empty() {
            let _ = write!(&mut s, " params='{}'", self.params);
        }
        let _ = write!(&mut s, " status={}", self.status);
        if let Some(r) = &self.reason {
            let _ = write!(&mut s, " reason={}:{}", r.class, r.message);
        }
        if !self.ranges.is_empty() {
            s.push_str(" ranges=");
            for (i, r) in self.ranges.iter().enumerate() {
                if i > 0 {
                    s.push(',');
                }
                let _ = write!(&mut s, "[{}..{}]", r.start, r.end);
            }
        }
        s
    }
}

/// Convenience: your trace can just be a Vec, but this alias reads better.
pub type Trace = Vec<TraceTag>;

/// Ultra-light macro to append an OK tag with params.
#[macro_export]
macro_rules! trace_ok {
    // trace_ok!(trace, Phase::Transform, "stft", "win" => 1024, "hop" => 256);
    ($vec:expr, $phase:expr, $name:expr $(, $k:expr => $v:expr )* $(,)?) => {{
        let mut _t = $crate::trace::TraceTag::new($phase, $name);
        $(
            _t.param_kv_mut($k, $v);
        )*
        $vec.push(_t);
    }};
}
