# 🔭 Sozna‑sense — Horizon Blueprint (v2.3 • Apex, Embedding‑centric)

**Owner:** Huy
**Dates:** v1.0 (2025‑08‑29) • v1.2 (2025‑09‑02, Clean Perception) • v2.0 (Apex, 2025‑09‑04) • v2.3 (Lean+, 2025‑09‑05)

> **Tagline:** *Make embeddings alive; keep tensors true.*
> **Mission:** Turn any digital input into **model‑ready embeddings** with **tensors as universal truth**. Deterministic, streaming‑first, refinement‑aware, edge‑ready.

---

# Part I — Narrative / Apex

## 0) Scope & Boundaries

* **Scope:** Ingestion → detection → decoding → patching → local encoders/decoders → embeddings (default) + tensors (truth).
* **Out of scope:** Retrieval/RAG, provenance diaries, deep container parsing (ZIP/PDF internals), model training/checkpoint mgmt, agent policy.

**Primary Contract**
`ByteFrame → Decoded Signal → Features → Tensor (truth) → Patch Embeddings (default brain input)`
`+ Evidence (hash, source_id), Sidecar (QA), optional Imprints`

**Boundaries**

* **Embeddings are inside sense** in v2 (small, local encoders per modality).
* **Tensors are always emitted** (forensics, debugging, fallback).
* **Evidence/Sidecar** remain human‑visible only (agent‑hidden unless requested).
* **No hidden IO**, offline‑capable; encoders are packaged weights with pinned hashes.

## 1) Apex North‑Star Architecture (All Modalities)

* **Tensors as truth**: always emitted.
* **Adaptive embeddings as default input**: brain consumes embeddings first.
* **Active refinement everywhere**: zoom‑in when uncertain.
* **Tiny routing (2 experts)**: available but off by default; enable after baseline.
* **Shared latent contract**: sequences `(·,512)`; pooled `(1,768)`.
* **Sidecar** logs: modality, head\_id, patch\_stats, thresholds, compression\_ratio, weights\_hash, latency.
* **Brain**: consumes embeddings by default; can request raw tensors or refinements.

## 2) System Overview

**Pipeline:**
`ByteFrame → Detector → Decoder(modality) → Transforms(feature) → Tensorizer(device/dtype/layout)`
`→ LocalEncoder(modality) → PatchEmbeddings + Pooled`
`→ Sample { tensor_view, embed_seq, embed_pooled, Evidence, Sidecar, Status }`

**Detector & Universal Intake**

* Any `ByteFrame` is accepted. Detector routes to best adapter (Text/Audio/Vision).
* If no adapter claims it → **Binary** adapter guarantees fallback.
* **Plugins** extend coverage (AVIF, HEIF, MIDI, MP4, PDF, etc.) without changing contracts.

**Streaming defaults:** text 32–64 KB windows, audio 20–40 ms hops, vision 1 frame.
**Determinism:** D2 for tensors; embeddings deterministic given `weights_hash + config_hash`.
**Telemetry:** i/o, p50/p95/p99 latency, jitter, backlog (Sidecar opt‑in).

## 3) Data Model & API Spine

```rust
pub enum View<B> {
  TextBytes(Tensor<B>),        // (N,) u8
  TextCodepoints(Tensor<B>),   // (N,) u32/i32
  AudioMel(Tensor<B>),         // (F,80) f32
  VisionCHW(Tensor<B>),        // (3,H,W) f32
  VisionPatches(Tensor<B>),    // (P,D) f32, D=3*patch*patch
  RawBytesU8(Tensor<B>),       // (N,) u8
  RawBytesF32(Tensor<B>),      // (N,) f32 [0,1]
}

pub struct Embeddings<B> {
  pub seq: Tensor<B>,   // (T, 512)
  pub pooled: Tensor<B> // (1, 768)
}

pub struct Sample<B> {
  pub tensor_view: View<B>,
  pub embed: Embeddings<B>,
  pub evidence: Evidence,
  pub sidecar: Sidecar,
  pub status: Status,
}
```

---

# Part II — Modal Profiles

---

## 0) Universal Intake (Canonical Contract)

* Accept any **ByteFrame**.
* **Detector** proposes an adapter (Text/Audio/Vision). If none claims with sufficient confidence, use **Binary**.
* Plugins extend decoding (AVIF/HEIF, MIDI, MP4, PDF…) **without altering contracts**.

**Outputs (standard contract)**

```
View::{ TextBytes, TextCodepoints, AudioMel, VisionCHW, VisionPatches, RawBytesU8, RawBytesF32 }
Embeddings { seq: (T,512), pooled: (1,768) }

```

**Precision ledger**

* Truth tensors remain exact (u8/u32/f32).
* Device views may cast to bf16/f16.
* Cast path recorded in Sidecar: `truth_dtype, device_view_dtype, cast_path`.

---

## 4.1 Text

**Inputs**

* UTF‑8/16/32 (LE/BE), BOM honored; heuristic fallback.
* Non‑UTF → Binary unless plugin decoder (Shift‑JIS, GBK…).

**Canonicalization**

* NFC normalization.
* Normalize newlines → LF.
* Strip C0 controls except tab/newline.
* Sidecar flags: `controls_stripped`, `bidi_marks`.

**➡ Tensorization (truth)**

* `TextBytes: (N,) u8`
* `TextCodepoints: (N,) i32`

**Patching / Boundaries**

* Window: `256` bytes, overlap `25%`.
* **Boundary candidates:** entropy ≥0.85 or KL ≥0.2 (or n‑gram proxy if no LM).
* **Safe snapping:** shift candidate ±16 bytes to nearest safe anchor (grapheme boundary, whitespace, paired punctuation, newline, indent change).
* **UTF safety:** never split a multi‑byte codepoint.

**Local Encoder**

* Byte embedding table + hash n‑grams.
* Transformer (4–6 layers, d\_model=256).
* Cross‑attn pooling → patch embeddings `(T,512)`.

**Adaptive Hooks**

* Refinement: `refine(span,×2)` via **RefineQueue** (priority = uncertainty/(ΔFLOPs+latency), with hysteresis).
* Routing: `{ code, natural }` using cheap probes (symbol ratio, braces density, newline rate) with min dwell ≥3 patches.

**Outputs**

* **Tensors:** TextBytes / TextCodepoints.
* **Embeddings:** seq `(T,512)`, pooled `(1,768)`.

**Sidecar adds:** `snap_radius_bytes, snap_anchor_kind, kl_source, entropy_win, refine_budget, spans_refined, mean_uncertainty, route_head, route_dwell, degrade_level`.

---

## 4.2 Audio

**Inputs**

* Files: WAV/PCM mandatory; MP3/FLAC/OGG/MP4 optional.
* Live: mic/system capture.
* Plugins: e.g., MIDI.

**Canonicalization**

* PCM→f32 mono \[−1,1].
* Resample to 16 kHz.
* Optional HPF @30Hz.

**➡ Tensorization (truth)**

* `AudioMel: (F,80) f32`

**Feature / Patching**

* STFT (`win=1024`).
* Two‑stream: mel80 (base) + mel40 (coarse), fused via cross‑attn.
* Log(mel+1e−6).
* **Stride governor:** hop {256,512,1024} by SNR/voicing/flux.
* **Optional CQT‑40 head** when music probe > τ.

**Local Encoder**

* Temporal Transformer + cross‑attn pooling.

**Adaptive Hooks**

* Refinement: re‑encode noisy spans.
* Routing: `{ speech, music }` (default off).

**Outputs**

* **Tensor:** AudioMel `(F,80)`.
* **Embeddings:** seq `(T,512)`, pooled `(1,768)`.

**Sidecar adds:** `snr_est, voicing_ratio, avg_stride`.

---

## 4.3 Vision

**Inputs**

* Encoded: PNG, JPEG, WebP, GIF (plugins: AVIF, HEIF…).
* Rasters: RGBA8/BGRA8/NV12/YUV420P.
* Video: MP4/WebM/MKV if plugin; else Binary.

**Canonicalization**

* Apply EXIF rotation once.
* ICC→sRGB; else assume sRGB.
* YUV→RGB (BT.709).
* Alpha premultiply → drop.
* Range: f32 \[0,1].

**➡ Tensorization (truth)**

* `VisionCHW: (3,H,W) f32`
* `VisionPatches: (P,D)`

**View & Patching**

* Start 16×16 tokens.
* **Saliency‑guided token budget:** score tokens (edge density, variance, textness, optional stem attn). Keep top‑K under target 40–60%. Merge only low‑saliency tokens.
* **ROI hard‑keep** (text/edges).
* **Stability:** hysteresis to avoid flicker.
* **Sidecar:** `token_budget_target, kept_tokens, merged_tokens, roi_protected`.

**Local Encoder**

* ViT blocks (6–12) + cross‑attn pooling.

**Adaptive Hooks**

* Refinement: ROI zoom on uncertain regions.
* Routing: `{ texture, text }` (default off).

**Outputs**

* **Tensors:** VisionCHW / VisionPatches.
* **Embeddings:** seq `(T,512)`, pooled `(1,768)`.

---

## 4.4 Binary

**Inputs**

* Anything not claimed (unknown/corrupt/container).
* Plugins may extend (PDF, etc.).

**Canonicalization**

* Raw u8 or scaled f32 \[0,1].
* **2ms structure sniff:** magic numbers, headers (PDF, gzip/zstd, TAR).
* Write `header_tags`.
* **Bomb guard:** cap decompression ratio/time; mark Dropped(DecompressBudget).

**➡ Tensorization (truth)**

* `RawBytesU8: (N,) u8`
* `RawBytesF32: (N,) f32`

**Patching & Encoding**

* Entropy spans; fallback fixed stride (4 KB).
* Shallow byte Transformer + cross‑attn pooling.

**Adaptive Hooks**

* Refinement: on by default.
* Routing: off.

**Outputs**

* **Tensors:** RawBytesU8 / RawBytesF32.
* **Embeddings:** seq `(T,512)`, pooled `(1,768)`.

**Sidecar adds:** `header_tags, decompress_budget_hit`.

---

## 5) Local Tests (per Profile)

* **Tensor invariants:** shapes/dtypes/layouts exact.
* **Boundary/patching:** text (entropy/KL + snapping anchors), audio (flux/voicing/stride governor), vision (saliency/token budget + ROI), binary (structure sniff + bomb guard).
* **Encoder smoke tests:** deterministic forward given fixed weights/config.

---

## Sidecar Additions (observability)

Standard fields when relevant:

* `token_budget_target, kept_tokens, merged_tokens, roi_protected`
* `truth_dtype, device_view_dtype, cast_path`
* `snr_est, voicing_ratio, avg_stride`
* `header_tags, decompress_budget_hit`
* `refine_budget, patches_refined, mean_uncertainty`
* `route_head, route_dwell`
* `degrade_level, drops`

---

**End v2.3 (Lean+)** — clarified tensorization, boundaries, adaptive hooks, and Sidecar fields for blueprint inclusion.

## 5) Local Tests (per Profile)

* **Tensor invariants**: shapes/dtypes/layouts exact.
* **Boundary/patching**: text (entropy/KL + snapping anchors), audio (flux/voicing/stride governor), vision (saliency/token budget + ROI), binary (structure sniff + bomb guard).
* **Encoder smoke tests**: deterministic forward given fixed weights/config.

---

# Part III — System & Ops

## 6) Shared Latent Contract

* Seq `(T,512)`; pooled `(1,768)`.
* Stable across modalities → enables cross‑modal fusion.

## 7) Decision Gates

* Text @ equal FLOPs: ≥ BLT‑Entropy +0.5–1.0 pts, or within 0.3 pts at ≥15% fewer FLOPs.
* Compression: text ≥4.5×, audio ≥2×, vision ≥35% token reduction (<1% acc drop).
* Robustness: +1–2 pts on noisy/orthographic/code‑mix vs entropy‑only.
* Latency: p95 within CPU/WGPU budget.

**Fail‑safe order:** refine→off → dual‑surprise→entropy‑only → routing→off.

## 8) Minimal Ablations

* Boundary: entropy vs entropy+KL.
* Pooling: BLT cross‑attn vs mean/attn.
* Refinement: off vs 1 round; keep if ≥0.3 pts gain @<10% extra FLOPs.

## 9) Flow Runtime

* State machine: OPEN→RUN{EMIT|COALESCE|DROP}→FLUSH→CLOSE.
* Policies: MinLatency vs MaxThroughput.
* Timing grids: text 32–64 KB; audio hop 256\@16k; vision 1 frame.

## 10) Tensorizer & Numerics

* Burn CPU/WGPU backends; tensors on sense device.
* DTypes: text u8/u32/i32; others f32 (f16 optional behind drift gates).
* Zero‑copy where possible.
* Tolerances: CPU ≤1e‑6; GPU ≤3e‑5.
* Config hashing covers all knobs.

## 11) Golden‑Kit Conformance

* Tensors deterministic (D2).
* Embeddings deterministic given `(bytes, config_hash, weights_hash)`.
* Routing ≥99.5%.
* Streaming p95: text ≤50ms; audio jitter ≤20ms; vision ≤30ms.

## 12) Errors & Status

* Decode→Degraded, Bounds→Warn, Format→Fail, Device→Fail, Config→Fail, Container→Degraded, Backend→Degraded, Internal→Fail.

## 13) CLI & Tooling

* sense‑cli: print modality, tensor+embedding shapes, status.
* Flags: --dump‑sidecar, --dump‑embed, --no‑embed.
* sense‑bench, sense‑goldenkit, sense‑metrics.

## 14) Extensibility

* Plugin registry v2: adapters, decoders, encoders.
* Capability manifests: shapes, throughput, drift/latency bounds.

## 15) Governance & Versioning

* Semver discipline.
* Weights pinned by hash.
* RFCs for modality/shape/router changes.

## 16) Roadmap

* v2.0 (Apex): embeddings in‑sense, refinement on, routing off.
* v2.1: enable 2‑expert routing.
* v2.2: hardware‑aware patching; minimal video.
* v3.0: multimodal routing mesh.

## 17) Appendices

* Reference algorithms: NFC, STFT, mel, EXIF, ICC, YUV, bilinear resize, token‑merge.
* Example TOML configs for each modality.
* CI matrix: Linux, Windows, macOS; CPU+WGPU backends; WASM.

## 18) Changelog

* v2.3 (Lean+): explicit tensorization, safe snapping (text), two‑stream mel (audio), saliency token budget (vision), structure sniff + bomb guard (binary).
* v2.0 (Apex): embeddings first‑class, tensors truth, refinement everywhere, routing off. Shared latent contract. Decision gates + ablations formalized.
* v1.2: Clean perception (tensors only).
* v1.0: Initial embedding draft.

---

**End Horizon Blueprint v2.3** — layered (Narrative → Profiles → Ops), nothing cut, only reorganized.
