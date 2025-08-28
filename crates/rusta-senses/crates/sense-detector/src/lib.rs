//! Sozna-sense detector (v0.1): hint-first, signature-based, heuristic fallback.
//!
//! Design goals:
//! - Multi-modality & flexible: MIME hints, magic bytes, UTF heuristics.
//! - Deterministic: tie-breaking, param recording, confidence scored.
//! - Performant: sniff only the first N bytes, no deep introspection.
//!
//! Non-goals (v0): container introspection (PDF/ZIP/MP4), video routing.

use sense_core::{
    Adapter, ByteFrame, Modality, Phase, Reason, ReasonClass, SenseConfig, SenseOut, TraceTag,
};

/// Detector configuration.
#[derive(Debug, Clone)]
pub struct DetectorConfig {
    /// Max bytes to sniff from the head of the buffer.
    pub sniff_bytes: usize,
    /// If true, MIME hints are trusted and prioritized.
    pub prefer_hints: bool,
    /// Enable UTF-8/ASCII heuristic when no magic bytes match.
    pub text_heuristic: bool,
    /// ASCII printable ratio threshold for heuristic text (0..1).
    pub printable_threshold: f32,
}

impl Default for DetectorConfig {
    fn default() -> Self {
        Self {
            sniff_bytes: 4096, // 4 KiB is plenty for signatures
            prefer_hints: true,
            text_heuristic: true,
            printable_threshold: 0.85,
        }
    }
}

/// Result of detection.
pub struct DetectOut {
    pub modality: Modality,
    pub tag: TraceTag, // initial detector tag (includes conf & rationale)
}

/* ============================== signatures ============================== */

const PNG: &[u8] = &[0x89, b'P', b'N', b'G', b'\r', b'\n', 0x1A, b'\n'];
const JPG_SOI: &[u8] = &[0xFF, 0xD8, 0xFF];
const GIF87A: &[u8] = b"GIF87a";
const GIF89A: &[u8] = b"GIF89a";
const RIFF: &[u8] = b"RIFF";
const WEBP: &[u8] = b"WEBP";
const WAVE: &[u8] = b"WAVE";
const BMP: &[u8] = b"BM";
const PDF: &[u8] = b"%PDF-";
const ZIPL: &[u8] = &[0x50, 0x4B, 0x03, 0x04]; // zip local header
const MP3_ID3: &[u8] = b"ID3";

#[inline]
fn head<'a>(d: &'a [u8], n: usize) -> &'a [u8] {
    &d[..d.len().min(n)]
}
#[inline]
fn is_png(d: &[u8]) -> bool {
    d.starts_with(PNG)
}
#[inline]
fn is_jpeg(d: &[u8]) -> bool {
    d.starts_with(JPG_SOI)
}
#[inline]
fn is_gif(d: &[u8]) -> bool {
    d.starts_with(GIF87A) || d.starts_with(GIF89A)
}
#[inline]
fn is_webp(d: &[u8]) -> bool {
    d.len() >= 12 && d.starts_with(RIFF) && &d[8..12] == WEBP
}
#[inline]
fn is_wav(d: &[u8]) -> bool {
    d.len() >= 12 && d.starts_with(RIFF) && &d[8..12] == WAVE
}
#[inline]
fn is_bmp(d: &[u8]) -> bool {
    d.starts_with(BMP)
}
#[inline]
fn is_pdf(d: &[u8]) -> bool {
    d.starts_with(PDF)
}
#[inline]
fn is_zip(d: &[u8]) -> bool {
    d.starts_with(ZIPL)
}
#[inline]
fn has_bom_utf16le(d: &[u8]) -> bool {
    d.len() >= 2 && d[0] == 0xFF && d[1] == 0xFE
}
#[inline]
fn has_bom_utf16be(d: &[u8]) -> bool {
    d.len() >= 2 && d[0] == 0xFE && d[1] == 0xFF
}
#[inline]
fn has_bom_utf8(d: &[u8]) -> bool {
    d.len() >= 3 && d[0] == 0xEF && d[1] == 0xBB && d[2] == 0xBF
}
#[inline]
fn is_utf8(d: &[u8]) -> bool {
    std::str::from_utf8(d).is_ok()
}
#[inline]
fn is_mp3(d: &[u8]) -> bool {
    d.starts_with(MP3_ID3) || (d.len() >= 2 && d[0] == 0xFF && (d[1] & 0xE0) == 0xE0)
}

/// Printable ASCII ratio (quick heuristic).
fn ascii_printable_ratio(d: &[u8]) -> f32 {
    if d.is_empty() {
        return 0.0;
    }
    let printable = d
        .iter()
        .filter(|&&b| (b >= 0x20 && b <= 0x7E) || b == b'\n' || b == b'\r' || b == b'\t')
        .count();
    printable as f32 / d.len() as f32
}

/* ================================ hints ================================= */

/// Very small parser for hints like:
/// "vision/raw; fmt=rgba8; w=1280; h=720" or "vision/png" or "text/plain; charset=utf-8"
fn parse_hint_mime(hint: &str) -> (&str, Vec<(&str, &str)>) {
    let mut parts = hint.split(';').map(|s| s.trim());
    let mime = parts.next().unwrap_or("");
    let mut kvs = Vec::new();
    for p in parts {
        if let Some((k, v)) = p.split_once('=') {
            kvs.push((k.trim(), v.trim()));
        }
    }
    (mime, kvs)
}

/* ========================== confidence & hits =========================== */

#[derive(Debug)]
struct Hit {
    modality: Modality,
    fmt: &'static str,
    conf: f32, // 0..1
    extra_params: &'static [(&'static str, &'static str)],
}

impl Hit {
    fn to_tag(&self, mut tag: TraceTag) -> TraceTag {
        tag = tag
            .param("fmt", self.fmt)
            .param("conf", format!("{:.2}", self.conf));
        for (k, v) in self.extra_params {
            tag = tag.param(k, *v);
        }
        tag
    }
}

/* ================================ API ================================== */

pub struct Detector {
    cfg: DetectorConfig,
}

impl Detector {
    pub fn new(cfg: DetectorConfig) -> Self {
        Self { cfg }
    }
    pub fn default() -> Self {
        Self::new(DetectorConfig::default())
    }

    /// The main detection function. Pure and allocation-light.
    pub fn detect(&self, bf: &ByteFrame) -> DetectOut {
        let sniff = head(&bf.data, self.cfg.sniff_bytes);

        // 1) HINT-FIRST (optional)
        if self.cfg.prefer_hints {
            if let Some(hint) = bf.hint_mime.as_deref() {
                let (mime, kvs) = parse_hint_mime(hint);
                if mime.starts_with("vision/") {
                    let mut tag = TraceTag::new(Phase::Detector, "vision").param("hint", mime);
                    for (k, v) in kvs.iter() {
                        tag = tag.param(k, *v);
                    }
                    // Support raw frames via hint
                    if mime == "vision/raw" {
                        tag = tag.param("fmt", "raw").param("conf", "0.99");
                    } else {
                        tag = tag.param("conf", "0.99");
                    }
                    return DetectOut {
                        modality: Modality::Vision,
                        tag,
                    };
                }
                if mime.starts_with("audio/") {
                    let mut tag = TraceTag::new(Phase::Detector, "audio")
                        .param("hint", mime)
                        .param("conf", "0.99");
                    for (k, v) in kvs.iter() {
                        tag = tag.param(k, *v);
                    }
                    return DetectOut {
                        modality: Modality::Audio,
                        tag,
                    };
                }
                if mime.starts_with("text/") {
                    let mut tag = TraceTag::new(Phase::Detector, "text")
                        .param("hint", mime)
                        .param("conf", "0.99");
                    for (k, v) in kvs.iter() {
                        tag = tag.param(k, *v);
                    }
                    return DetectOut {
                        modality: Modality::Text,
                        tag,
                    };
                }
                if mime == "application/pdf" || mime == "application/zip" {
                    let mut tag = TraceTag::new(Phase::Detector, "container")
                        .param("hint", mime)
                        .param("conf", "0.99");
                    for (k, v) in kvs.iter() {
                        tag = tag.param(k, *v);
                    }
                    return DetectOut {
                        modality: Modality::Binary,
                        tag,
                    };
                }
            }
        }

        // 2) MAGIC-BYTES (visions)
        if is_png(sniff) {
            return detect_hit(Modality::Vision, "png", 0.95, bf);
        }
        if is_jpeg(sniff) {
            return detect_hit(Modality::Vision, "jpeg", 0.95, bf);
        }
        if is_gif(sniff) {
            return detect_hit(Modality::Vision, "gif", 0.95, bf);
        }
        if is_webp(sniff) {
            return detect_hit(Modality::Vision, "webp", 0.95, bf);
        }
        if is_bmp(sniff) {
            return detect_hit(Modality::Vision, "bmp", 0.90, bf);
        }

        // 3) MAGIC-BYTES (audio)
        if is_wav(sniff) {
            return detect_hit(Modality::Audio, "wav", 0.95, bf);
        }
        if is_mp3(sniff) {
            return detect_hit(Modality::Audio, "mp3->pcm", 0.90, bf);
        }

        // 4) CONTAINERS → Binary (no introspection v0)
        if is_pdf(sniff) {
            return detect_container("pdf", bf);
        }
        if is_zip(sniff) {
            return detect_container("zip", bf);
        }

        // 5) TEXT ENCODINGS
        if has_bom_utf8(sniff) {
            let tag = TraceTag::new(Phase::Detector, "text")
                .param("enc", "utf8-bom")
                .param("conf", "0.90");
            return DetectOut {
                modality: Modality::Text,
                tag,
            };
        }
        if has_bom_utf16le(sniff) || has_bom_utf16be(sniff) {
            let enc = if has_bom_utf16le(sniff) {
                "utf16le"
            } else {
                "utf16be"
            };
            let tag = TraceTag::new(Phase::Detector, "text")
                .param("enc", enc)
                .param("conf", "0.90")
                .warn(Some(Reason::new(
                    ReasonClass::Format,
                    "utf16 BOM → will transcode",
                )));
            return DetectOut {
                modality: Modality::Text,
                tag,
            };
        }
        if is_utf8(sniff) {
            let tag = TraceTag::new(Phase::Detector, "text")
                .param("enc", "utf8")
                .param("conf", "0.85");
            return DetectOut {
                modality: Modality::Text,
                tag,
            };
        }

        // 6) TEXT HEURISTIC (ASCII printable ratio)
        if self.cfg.text_heuristic {
            let ratio = ascii_printable_ratio(sniff);
            if ratio >= self.cfg.printable_threshold {
                let tag = TraceTag::new(Phase::Detector, "text")
                    .param("enc", "heuristic-ascii")
                    .param("printable", format!("{:.2}", ratio))
                    .param("conf", "0.60")
                    .warn(Some(Reason::new(
                        ReasonClass::Format,
                        "heuristic text w/out UTF validation",
                    )));
                return DetectOut {
                    modality: Modality::Text,
                    tag,
                };
            }
        }

        // 7) FALLBACK → Binary
        DetectOut {
            modality: Modality::Binary,
            tag: TraceTag::new(Phase::Detector, "binary")
                .param("conf", "0.50")
                .degraded(Some(Reason::new(
                    ReasonClass::Routing,
                    "no signature match; fallback",
                ))),
        }
    }
}

/* ============================ small helpers ============================ */

#[inline]
fn detect_hit(modality: Modality, fmt: &'static str, conf: f32, bf: &ByteFrame) -> DetectOut {
    let mut tag = TraceTag::new(
        Phase::Detector,
        match modality {
            Modality::Vision => "vision",
            Modality::Audio => "audio",
            Modality::Text => "text",
            Modality::Binary | Modality::Container => "binary",
        },
    )
    .param("fmt", fmt)
    .param("conf", format!("{:.2}", conf));
    if let Some(h) = bf.hint_mime.as_deref() {
        tag = tag.param("hint", h);
    }
    DetectOut { modality, tag }
}

#[inline]
fn detect_container(fmt: &'static str, _bf: &ByteFrame) -> DetectOut {
    DetectOut {
        modality: Modality::Binary,
        tag: TraceTag::new(Phase::Detector, "container")
            .param("fmt", fmt)
            .param("conf", "0.95"),
    }
}

/* =========================== orchestration hook ========================= */

/// Helper used by the façade/tests: detect → route to adapters.
pub fn process_with_registry(
    bf: &ByteFrame,
    cfg: &SenseConfig,
    adapters: &[&dyn Adapter],
) -> anyhow::Result<SenseOut> {
    let det = Detector::default().detect(bf);
    let mut trace = vec![det.tag];

    let adapter = adapters
        .iter()
        .find(|a| a.modality() == det.modality)
        .ok_or_else(|| anyhow::anyhow!("no adapter for modality {:?}", det.modality))?;

    let mut out = adapter.adapt(bf, cfg)?;
    trace.append(&mut out.trace);
    out.trace = trace;
    Ok(out)
}

/* ================================= tests ================================ */

#[cfg(test)]
mod tests {
    use super::*;
    fn bf(bytes: &[u8]) -> ByteFrame {
        ByteFrame::from_bytes(bytes.to_vec(), None, "mem")
    }

    #[test]
    fn png_detects_vision() {
        let d = Detector::default().detect(&bf(super::PNG));
        assert!(matches!(d.modality, Modality::Vision));
        assert!(d.tag.params.contains("png"));
    }

    #[test]
    fn utf8_detects_text() {
        let d = Detector::default().detect(&bf(b"hello world"));
        assert!(matches!(d.modality, Modality::Text));
        assert!(d.tag.params.contains("utf8"));
    }

    #[test]
    fn pdf_routes_binary_container() {
        let d = Detector::default().detect(&bf(super::PDF));
        assert!(matches!(d.modality, Modality::Binary));
        assert!(d.tag.params.contains("container"));
    }

    #[test]
    fn ascii_heuristic_can_route_text() {
        let bytes = b"NAME: Steve\nScore: 9000\n";
        let d = Detector::default().detect(&bf(bytes));
        assert!(matches!(d.modality, Modality::Text));
        assert!(d.tag.params.contains("heuristic-ascii"));
    }
}
