# Sozna-sense
🛠️ Sozna-sense Build Sheet (v0)

Goal: Implement the perception core (ByteFrame → Detect → Adapter → (emb, trace)) with all practical upgrades, test harness, and error handling.
Scope: Text, Audio, vision, Binary adapters. Deterministic enough to be stable; no salience/fusion yet.

1. Core Modules

ByteFrame struct

Fields: data, ts, source_id.

Optional: hint (ext/mime), policy.

Source = ingest (file, mic, cam, stdin, http).

Detector

Magic byte headers: PNG, JPEG, WAV, MP3, WebP, GIF, PDF, ZIP.

UTF-8/UTF-16 BOM check.

Fallback: Binary.

Trace tag: det:<modality>(params).

Adapters

TextAdapter: Conv1D stem (k=5, stride=1, h=256), GELU, proj→512.

AudioAdapter: Decode→PCM→mono→resample16k, STFT (1024/256), mel=80, patch=8, proj→512.

visionAdapter: Decode→RGB(sRGB), EXIF orient, normalize [0,1], patchify 16×16, proj→768.

RawByteAdapter: norm bytes/255, truncate/stride, proj→512.

Trace

Append tags at each step: detector, adapter, transform, projection, error/fallback.

Example: det:audio(fmt=mp3→pcm,sr=16k) → downmix:mono → resample:polyphase16k → stft 1024/256 → mel=80 → patch_t=8 → proj d=512.

2. Config Defaults
Modality	Front-end	Defaults	Output
Text/Code	Conv1D stem → proj	k=5, h=256, d_model=512	(T, 512)
Audio	STFT→mel80→patch8→proj	sr=16k, win=1024, hop=256	(N, 512)
vision	patchify 16×16→proj	normalize=[0,1], d_model=768	(N, 768)
Binary	byte norm/stride→proj	d_model=512	(T, 512)
3. Tests (Golden Kit)

Text: "fn main(){}", multilingual, JSON.

Audio: sine1kHz, short speech, silence.

vision: checkerboard, colorbars, EXIF-rotated JPEG.

Binary: random bytes, corrupted JPEG, zip stub.

Checks:

Routing correctness (headers + UTF-8).

Golden embeddings equality.

Determinism across runs (same inputs, same config).

Error/fallback traces visible.

Size/time guard works (e.g., >16 MB → truncated).

4. Error & Safety Gates

Max frame size: 16 MB (truncate/stride if bigger).

Session cap: 256 MB.

Decode timeout: 5s local / 15s net.

ZIP bomb guard: no deep introspection.

Sandboxed decoders optional flag.

Status tags: ok | warn | degraded | fail.

5. Deliverables

Crate: sozna-sense

Public API: process(ByteFrame) -> SenseOut.

SenseOut: (emb, trace, sidecar?).

Unit tests: golden fixtures.

Benchmarks: tokens/sec, patches/sec.

Minimal CLI harness: sense file.png → prints trace + shape.

6. Milestones

M1 Core: Detector + Adapters + Trace.

M2 QA: Golden tests + determinism checks + error taxonomy.

M3 Ergonomics: Config via TOML/JSON, CLI harness, bench numbers.
Perception core: **ByteFrame → Detect → Adapter → (emb, trace)**

Crates:
- sense-core
- sense-detector
- sense-adapter-{text,audio,vision,binary}
- sense-cli

Build:
```
cargo build --workspace
```

CLI:
```
cargo run -p sense-cli -- <path-to-file>
```
# Sozna‑sense — vision & Architecture (v0)

**Owner:** Huy
**Date:** 2025‑08‑27
**Scope:** Perception layer for Sozna — ingestion→detection→adapter→embeddings + trace. No training, salience, or fusion in v0.

---

## 1) Purpose & Positioning

Sozna‑sense turns arbitrary digital inputs into **model‑ready embeddings** plus a **transparent trace** of how they were formed. It is modality‑aware yet philosophy‑neutral. It does *perception only* and exposes a single clean contract upward to any downstream model (e.g., Byte LM / Transformer) and sideways to memory/agent layers.

**Why:**

* Keep sensing simple, fast, and explainable.
* Unify disparate sources (files, mic, camera, HTTP) under one interface.
* Decouple OS/devices from modeling; preserve determinism and reproducibility.

**Non‑goals (v0):** temporal streaming, salience, fusion, AST parsing, PDF/ZIP extraction internals, long‑running device loops.

---

## 2) Design Principles

* **Universal intake:** everything starts as bytes.
* **One clean contract:** embeddings `(tokens/patches × d_model)` + `trace[]`.
* **Modular & pluggable:** detector, adapters, and transforms are replaceable.
* **Deterministic:** same bytes + config ⇒ same embeddings (eval mode).
* **Efficient first:** single‑pass detection; sample‑efficient front‑ends.
* **Explainable:** every step appends a compact, stable trace tag.
* **No lock‑in:** independent of any specific downstream model architecture.

---

## 3) External Contracts (Stable)

**Input:** `ByteFrame` (opaque bytes + optional hints).
**Output:** `SenseOut` (embeddings, trace).

**ByteFrame (conceptual):**

* `data`: raw bytes
* `ts`: timestamp (monotonic + wall)
* `source_id`: path/URL/device tag

**SenseOut:**

* `emb`: 2‑D matrix `(seq_len_or_patches, d_model)`
* `trace`: ordered, human‑readable tags (e.g., `"utf8:ok"`, `"stft win=1024 hop=256"`, `"patch=16"`, `"proj d=512"`).

Contract invariants:

* Never blocks on devices/network (that’s ingest).
* Never writes to disk or phones home.
* Error‑tolerant: unknown or malformed blobs route to Binary/Unknown with trace.

---

## 4) System Overview

**Pipeline:** *raw bytes* → **modality detection** → **adapter** → **front‑end transforms** → **embeddings** + **trace**.

**Components:**

1. **Ingest (adjacent module, not inside sense):** files/repos, HTTP, mic/cam, stdin; bounds, chunking, normalization; emits `ByteFrame`s.
2. **Detector (inside sense):** magic‑byte checks for PNG/JPEG/WAV/MP3 (+ optional PDF/ZIP for routing), UTF‑8 vs binary heuristic; outputs a label + hints + confidence (trace only).
3. **Adapters:** one per modality. v0: Text, Audio, vision, Binary/Unknown.
4. **Transforms library:** patchify, STFT, mel, log‑mel, normalization; reusable across adapters.

---

## 5) Adapters (v0 Defaults)

**Text (includes code):** learned Conv1D stem over normalized bytes/chars; project to `d_model=512`.
*Rationale:* vocab‑free; learns braces/subwords; fast.

**Audio (WAV/MP3→PCM):** STFT → mel(80) → log → temporal patchify (t=8) → project to `d_model=512`.
*Rationale:* SOTA‑standard, sample‑efficient, stable.

**vision (PNG/JPEG→RGB):** ViT‑style non‑overlapping patchify (16×16), flatten, project to `d_model=768`.
*Rationale:* strong baseline; easy, fast.

**Binary/Unknown:** byte normalization → small projection to `d_model` (configurable).
*Rationale:* fail‑safe; perception never hard‑fails.

**Config (v0 defaults):**

* Text: `d_hidden=256, k=5, stride=1, norm=on, GELU‑like`
* Audio: `sr=16k, win=1024, hop=256, mel=80, patch_t=8`
* vision: `patch=16, normalize=[0,1]`
* `d_model`: `512 (text/audio)`, `768 (vision)`

All parameters are runtime‑configurable (e.g., TOML/JSON). Trace records actual values used.

---

## 6) Tracing & Observability

* Every detection/transform appends a tag.
* Trace returned with embeddings and emitted at debug log level.
* Examples:

  * Text: `utf8:ok → conv1d k=5 h=256 → gelu → proj d=512`
  * Audio: `wav 16k → stft 1024/256 → mel=80 → patch_t=8 → proj d=512`
  * vision: `jpeg → rgb norm → patch=16 → proj d=768`
* Purpose: reproducibility, fast diffing between runs, issue triage.

---

## 7) Performance, Reliability

* **Throughput:** single‑threaded baseline should be memory‑bandwidth‑bound; enable parallel decode where safe later.
* **Memory bounds:** streaming/patch buffers; avoid full copies.
* **Errors:** propagate as recoverable results; note failure points in trace; route to Binary/Unknown when possible.

---

## 8) Testing & QA (Golden Kit)

**Fixtures:**

* Text: `"fn main(){}"`, multilingual UTF‑8, long JSON, markdown.
* Audio: 1 kHz sine @16k WAV; short speech clip; silence.
* vision: checkerboard, color bars, small JPEG/PNG.
* Routing: PNG header, UTF‑8 text, random bytes.

**Tests:**

* Golden embeddings equality (bit‑exact within tolerance) + exact trace strings.
* Routing correctness for headers and UTF‑8 heuristic.
* Determinism over repeated runs.
* Performance smoke tests under size caps.
* Security: reject absurd sizes; bounded decode time.

**Definition of Done (v0):**

* All four adapters pass golden tests.
* Detector routes all fixtures correctly.
* Configurable params are recorded in trace.
* No network/file I/O inside sense; no panics on malformed inputs.
* Bench doc: tokens/sec (text), patches/sec (audio/vision) on a reference machine.

---

## 9) Security & Privacy

* No persistence in sense; stateless between calls.
* No hidden network; pure local compute.
* Size/time limits at ingest; clear error paths.

---

## 10) Extensibility (Post‑v0 Growth)

* **Salience pre‑filters:** drop blank patches/frames/filler spans; compute savings.
* **Temporal continuity:** sliding windows, overlap‑add stats, temporal pooling.
* **Fusion hooks:** timestamps & spatial anchors for cross‑modal attention; alignment metadata.
* **Pre‑attention focus:** region proposals / energy‑based gating before heavy modeling.
* **Learning feedback:** adapter fine‑tuning during end‑to‑end training (opt‑in).
* **More modalities:** PDF/ZIP routing to external extractors; vector graphics; sensor logs.

---

## 11) Integration Points (Sozna stack)

* **Upstream model:** consumes embeddings; adds positional encodings.
* **Memory (Sigars):** store trace + checksums as evidence; enable audit trails.
* **Agent/Tools:** traces inform diagnostics (“speech‑like”, “low‑energy frames” flags).
* **UI:** display per‑sample trace; let users diff traces across runs.

---

## 12) Roadmap & Milestones

**M0 — Spec lock (today)**

* This document approved; invariants frozen for v0; open questions listed.

**M1 — Core pipeline**

* Detector (magic bytes + UTF‑8 heuristic).
* Adapters: Text, Audio, vision, Binary with defaults.
* Trace plumbed end‑to‑end.
* Golden fixtures minted.

**M2 — QA & Bench**

* Golden equality + routing tests stable.
* Determinism proven.
* Throughput numbers recorded on reference box.

**M3 — Developer ergonomics**

* Config via TOML/JSON.
* Clear errors & trace docs.
* Minimal examples (CLI/UI is *out of scope* but a tiny harness can live beside the crate).

**Exit criteria for v0:** ship a crate that converts ByteFrames → (emb, trace) for Text/Audio/vision/Binary reliably.

---

## 13) Open Questions (to answer later)

1. **Text front‑end scale:** do we want dynamic byte/char mixed inputs or stick to one path v0?
2. **Audio resampling:** fix to 16 kHz at ingest or allow 8/22.05/48 kHz with a clear trace tag?
3. **vision normalization:** stick to `[0,1]` or offer visionNet stats as a toggle in v0.1?
4. **Binary adapter dimension:** tie to `d_model_text` or be independently configurable?
5. **Trace format:** plain strings (human‑first) vs compact enum codes (machine‑first) + a pretty printer.
6. **Checksum choice:** blake3 vs xxhash; how strict do we want reproducibility gates.
7. **Failure policy:** when decode fails (e.g., corrupted JPEG), do we always fallback to Binary or expose error state to caller to decide?

---

## 14) Session Plan (to reduce overwhelm)

**Session A (≤90 min):**

* Freeze config defaults table (this doc).
* Finalize detector label list + routing matrix.
* Write the golden fixtures list & acceptance checks (bulleted, no code).

**Session B (≤90 min):**

* Draft trace taxonomy (exact tag strings).
* Define error taxonomy & recovery paths (fallbacks vs hard errors).
* Walk through 5 end‑to‑end examples on paper (what the trace should look like).

**Session C (≤90 min):**

* Lock v0 DoD; create the benchmark plan (which inputs, what metrics).
* Review open questions; pick defaults for v0; defer rest to v0.1.

---

## 15) One‑Page Defaults Table (for quick reference)

| Modality      | Front‑end                                | Defaults                                                                 | Output             |
| ------------- | ---------------------------------------- | ------------------------------------------------------------------------ | ------------------ |
| **Text/Code** | learned Conv1D stem → proj               | `k=5`, `stride=1`, `d_hidden=256`, `d_model=512`, `norm=on`, `GELU‑like` | `(T, 512)`         |
| **Audio**     | STFT → mel(80) → log → patch\_t=8 → proj | `sr=16k`, `win=1024`, `hop=256`, `mel=80`, `patch_t=8`, `d_model=512`    | `(N_patches, 512)` |
| **vision**     | patchify 16×16 → proj                    | `normalize=[0,1]`, `patch=16`, `d_model=768`                             | `(N_patches, 768)` |
| **Binary**    | byte norm → proj                         | `d_model=512 (configurable)`                                             | `(T, d_model)`     |

**Detector order:** headers → UTF‑8 vs binary → fallback.
**Error policy:** prefer recoverable fallback with explicit trace; never panic.

---

## 16) Glossary

* **Adapter:** modality‑specific pipeline that turns bytes into embeddings.
* **Front‑end:** transforms inside an adapter (STFT, patchify).
* **Trace:** ordered tags describing sensing steps/params.
* **Patch (temporal):** grouped frames treated as one token.
* **ByteFrame:** uniform envelope for bytes from any source.

---

## 17) Summary

Sozna‑sense v0 is deliberately narrow and stable: **ByteFrame → Detect → Adapter → (emb, trace)**. It favors speed, determinism, and clarity, leaving salience/streaming/fusion for post‑v0. With the contracts and defaults here, you can implement confidently without touching philosophy every day — build the small, sharp knife first.

---

## 18) Extensions & Future Directions (Comprehensive)

**Goal:** Make all plausible growth paths explicit so v0 stays minimal while the north‑star is visible.

### 18.1 Modalities to Add

* **Video:** RGB frames → patchify per frame + temporal stride/patching; optical‑flow or frame‑diff tokens; audio‑video sync by timestamps.
* **Code‑aware Text:** optional AST/CFG side‑channel; symbol tables; doc anchors; compiler log adapters (e.g., `rustc`, `clippy`).
* **Documents:** PDF → external extractor (text/spans/visions/tables) → route to Text/vision; **OCR** for scanned PDFs.
* **Tabular / Logs / Time‑series:** schema‑aware encoders; delta/derivative channels; calendar/seasonality marks.
* **Sensors:** IMU, GPS, LiDAR/Depth, 3D point clouds (voxel/BEV tokens), biosignals (ECG/EEG) with band‑power features.
* **UI/Screen:** screen capture frames + DOM/accessibility tree hooks when available.

### 18.2 Temporal & Streaming

* Sliding windows with overlap; online normalization (running mean/var); EMA statistics.
* Change‑point detection; eventized encodings (spike‑like) for sparse updates.
* Stream contracts: backpressure, bounded queues, watermarks, late frames policy.
* Latency budgets per modality (target p50/p95/p99) + jitter caps.

### 18.3 Salience & Pre‑Attention

* Energy/entropy gates (audio low‑energy frames, blank vision patches, filler text regions).
* ROI discovery: simple heuristics → learned region proposals.
* Multi‑scale pyramids: coarse tokens first; refine ROIs with smaller patches.
* Adaptive patching: patch size chosen by content (e.g., 16→8 on high‑detail zones).

### 18.4 Fusion & Alignment

* Alignment metadata: timestamps, spatial coords, object IDs, source lineage.
* Cross‑attention fusion blocks; late vs early fusion toggles.
* Token binding graph: edges across modalities (e.g., `(caption token) ↔ (vision patch)`), exportable to memory (Sigars) as evidence links.

### 18.5 Learning in the Front‑Ends

* **Phase 1:** classical fixed DSP (STFT/patchify) for stability.
* **Phase 2:** end‑to‑end fine‑tuning heads (conv stems, mel projection).
* **Self‑supervision:** masked modeling (span masking for text/audio/video), contrastive alignment (CLIP‑style), BYOL‑A/SimCLR variants; augmentation policies per modality.
* **Curriculum:** start small (clean speech, simple visions, short code) → scale diversity.

### 18.6 Quality, Uncertainty, & Diagnostics

* Per‑token quality scores (SNR, blur, clipping, OCR confidence) propagated with embeddings.
* Uncertainty estimates (aleatoric proxies) as an auxiliary vector alongside `emb`.
* Self‑diagnostics: “decode‑suspect”, “low‑energy”, “non‑utf8”, “header‑mismatch”.

### 18.7 Robustness, Safety & Privacy

* Adversarial defenses: input sanitization, checksum/length sanity, JPEG bomb detection.
* Privacy filters: PII redaction hooks for text/visions; mic/cam hard mutes; on‑device only mode.
* Sandboxing: decode in a constrained process; time/CPU/memory guards.

### 18.8 Performance Engineering

* Vectorized kernels; mixed precision where safe; memory pooling.
* Batching across samples (micro‑batch) and within sample (patch batches).
* CPU/GPU back‑ends; fallback to pure‑CPU; SIMD feature detection.
* Caching: decoded RGB/PCM caches keyed by checksum; trace‑stamped.

### 18.9 Packaging & Deploy Targets

* **Edge:** WebAssembly/WASI for browser/desktop; mobile (Android/iOS) with NEON.
* **Server:** gRPC/QUIC stream APIs; zero‑copy buffers.
* **Rust crate stability:** semver guarantees for contracts; feature flags per modality.

### 18.10 Provenance & Auditability

* Content hashes (blake3) everywhere; chain of custody from `ByteFrame` → trace → memory.
* Replay files: minimal sidecar format to reproduce sensing (config + checksums + offsets).

### 18.11 Plugin API (v1.0+)

* Adapter registry: dynamic discovery & capability descriptors (modalities, params, throughput).
* ABI stability plan; reference adapter templates; conformance tests.

---

## 23) What Stays Non‑Negotiable

* ByteFrame in; (emb, trace) out.
* No device I/O or network inside sense.
* Determinism preference; explicit trace for everything.
* Recoverable failure paths; safe fallbacks; no silent drops.

---

## 24) Learning Feedback Mechanisms

**Definition:** The ability of adapters/front‑ends to adjust their parameters in response to downstream training signals or explicit agent feedback.

### Modes of Feedback

* **Gradient‑based (end‑to‑end):** downstream loss gradients propagate back into the front‑end (e.g., Conv1D, STFT projection, patch projections) so that sensing features co‑evolve with the model.
* **Reinforcement / Reward‑based:** higher‑level agent rewards (e.g., task success, code compiles, ASR accuracy) influence adapter hyper‑parameters or gating choices.
* **Meta‑learning / Curriculum:** front‑ends schedule their complexity (start fixed; allow fine‑tuning once core model stabilizes).

### Examples

* **Text:** Conv1D filters sharpen to capture common subword boundaries or brace patterns because downstream language modeling loss rewards it.
* **Audio:** mel filter emphasis shifts slightly to better capture speech formants or suppress noise, guided by ASR error reduction.
* **vision:** adaptive patch resolution or normalization improves accuracy on detection tasks when allowed to update.

### Control & Safety

* Feedback is **opt‑in and gated by config**: by default, v0 runs fixed transforms for determinism.
* Enablement only in training mode; evaluation/inference keeps adapters frozen.
* Gradients are clipped/regularized to prevent catastrophic drift in low‑level sensing.

### Trace Extensions

* Trace should log whether an adapter was frozen or learnable.
* If learnable, include step‑size or epoch markers (e.g., `"text: conv1d learnable, lr=1e‑4"`).

### Roadmap Placement

* **v0:** frozen transforms, no feedback.
* **v1.1+:** allow fine‑tuning of projection layers and filterbanks.
* **v1.2+:** adaptive patching/learned salience gates.
* **v2.0:** reinforcement/agent‑driven feedback where AI’s own success/failure alters sensing policies.

### Rationale

## Learning feedback makes Sozna‑sense more than static preprocessing — it becomes part of the living loop, where perception adjusts because the AI itself *needs* it to solve tasks better.

## 25) RawByteAdapter (Binary/Unknown) — Full Spec

**Purpose:** Guarantee perception never hard‑fails when detection can’t confidently classify an input or decoding fails. Provides a safe, deterministic embedding for arbitrary bytes.

**Detection triggers:**

* No known magic bytes matched OR header matched but decode failed.
* UTF‑8 heuristic negative (low printable ratio / invalid sequences).
* Caller explicitly routes to Binary.

**Preprocessing / Normalization:**

* **Byte scale:** map `u8 → f32` by `x / 255.0` (configurable to z‑score if caller provides mean/std).
* **Stride/truncation:** enforce `max_len_bytes` (config). If `len > max`, either *truncate tail* or *fixed‑stride subsample*; record policy + offsets in trace.
* **Chunking (optional):** for very large blobs, process in fixed windows and concatenate embeddings; include `(start..end)` byte ranges in trace.

**Projection:**

* **Linear projection** from `R^{T×1}` to `R^{T×d_model_bin}` (`d_model_bin` default = 512, independent toggle to tie with text).
* Optional **LayerNorm** to stabilize scale across wildly different inputs.

**Trace (examples):**

* `binary → norm bytes/255 → stride=4 start=0 len=8MB → proj d=512`
* `binary → chunk[0..1MB],[1..2MB] → proj d=768`

**Guarantees:**

* Deterministic outputs given same bytes + config.
* Never attempts deep decode (no ZIP/PDF introspection).
* Bounded memory/latency via `max_len_bytes`, `stride`, and chunking.

**Failure policy:**

* If the *router* sent an input here due to a failed decode (e.g., corrupted JPEG), the trace must include `reason=decode-failed:<codec>`.

---

## 26) Concrete Upgrades to Lock In Now (not fluff)

These tighten the core without expanding scope.

### 26.1 Detector & Routing Accuracy

* Add **WebP, GIF, MP4/MOV/AVI** header checks (route MP4 to *Binary* for now).
* **PDF/ZIP**: detect reliably but keep *container‑only* routing (no introspection yet).
* **UTF‑16/UTF‑32 & BOM** handling: if BOM present, treat as Text path with correct decoding; else fall back to Binary on ambiguous mixed encodings.
* Record **`detector:label(conf=…)`** in trace.

### 26.2 vision Correctness

* Fix **color space** assumptions: treat inputs as **sRGB**, convert to linear‑light *or* stay in sRGB but record `colorspace` in trace (choose one policy).
* Respect **EXIF orientation**; record applied rotation/flip in trace.
* Define channel order **H×W×C, C=RGB** (no BGR); dtype f32 in `[0,1]` before patchify.

### 26.3 Audio Correctness

* **Channel policy:** downmix to mono with documented weights; record original channels.
* **Resampling:** lock to **16 kHz** with a deterministic resampler; record method (e.g., polyphase).
* DC‑offset removal + optional **loudness normalization** (EBU R128‑lite) *off by default*; if used, trace it.

### 26.4 Text Correctness

* **Unicode normalization** policy (NFC vs NFKC); newline normalization (`  →  `).
* Optional **whitespace collapse** flag (off by default) — only for pathological inputs; trace it.

### 26.5 Positioning & Tokens

* Define downstream expectation for **positional encodings**: senses do **not** add them. Provide optional **per‑sample CLS token** recommendation (added by downstream model) and a **metadata sidecar** (lengths, patch grid, sample rate) to help positioners.

### 26.6 Trace Schema v0.1 (still human‑readable)

* `phase: detector|adapter|transform`, `name`, `params(key=value,…)`, `reason?`, `ranges?`.
* Stable ordering; machine‑parseable without losing readability.

### 26.7 Reproducibility & Safety

* Standardize on **blake3** for content hashes; include first 8 bytes as short id in trace.
* **ZIP bomb guard:** cap total uncompressed ratio; refuse deep inspection (post‑v0).
* JPEG/PNG decode done in **sandboxed** helper (optional flag) with time/memory guards.

### 26.8 Config Hygiene (principle only)

* Keep user‑facing config small; record effective runtime values in trace if they differ due to enforced caps.

### 26.9 Tiny Quality Side‑Vector (toggle only)

* Allow adapters to emit an optional `qvec` alongside `emb` (e.g., per‑token energy/blank flag). Toggle off by default; no rationale text in spec.

---

## 27) Determinism Matrix (What can change outputs & how to lock it)

| Factor                       | Risk                     | Lock‑In Policy                                                                         |
| ---------------------------- | ------------------------ | -------------------------------------------------------------------------------------- |
| Random init (learned layers) | Non‑repro emb            | Fix seed; eval mode; persist weights hash in trace                                     |
| Decode libs/versions         | JPEG/PNG/STFT impl drift | Record decoder + version in trace; optional sandboxed decoder feature flag             |
| Floating‑point math          | Minor nondet across HW   | Use deterministic kernels when available; disable TF32; record backend/device in trace |
| Multithreading               | Reordering/precision     | Single‑thread eval by default; record thread count when >1                             |
| Resampling/filter params     | Spectral differences     | Pin resampler method + taps; record in trace                                           |
| Unicode normalization        | Token boundary drift     | Fix to NFC (or chosen policy) and trace it                                             |
| Color space/EXIF             | Visual mismatch          | Fix sRGB policy + EXIF orientation handling; trace both                                |
| Caps/stride                  | Length/coverage changes  | Record enforced caps + stride policy with byte ranges                                  |

---

## 28) Routing Matrix (Detection → Adapter → Required Hints → Trace Tags)

| Detected Type         | Primary Adapter                | Required Decode/Hints   | Canonical Trace Start                      |                                 |
| --------------------- | ------------------------------ | ----------------------- | ------------------------------------------ | ------------------------------- |
| PNG/JPEG (sRGB)       | vision                          | decode→RGB, EXIF orient | `det:vision(fmt=jpeg,orient=...)`           |                                 |
| WAV/PCM               | Audio                          | sr, channels            | `det:audio(fmt=wav,sr=16k,mono)`           |                                 |
| MP3                   | Audio                          | decode→PCM→sr16k mono   | `det:audio(fmt=mp3→pcm,sr=16k,mono)`       |                                 |
| UTF‑8 / UTF‑16(BOM)   | Text                           | norm policy             | \`det\:text(utf8                           | utf16,bom=...)\`                |
| PDF/ZIP               | Binary (container)             | none (no introspection) | \`det\:container(fmt=pdf                   | zip)\`                          |
| WebP/GIF              | vision (if decode on) or Binary | decode toggle           | \`det\:vision(fmt=webp                      | gif)`or`det\:binary(fmt=webp)\` |
| Unknown/Failed decode | Binary                         | —                       | `det:binary(reason=decode-failed:<codec>)` |                                 |

---

## 29) Sidecar Metadata Spec (for downstream PE/CLS)

Per sample, return a lightweight sidecar alongside `emb` & `trace`:

* `lengths`: token/patch counts per stream
* `grid`: vision patch grid `(H_patches, W_patches)`
* `audio`: `{sr: 16000, hop: 256, win: 1024, patch_t: 8}`
* `text`: `{norm: NFC, newline: "
  "}`
* `binary`: `{stride: 4, chunks: [[0,1048576], ...]}`
* `quality?`: optional `qvec` shape descriptor
* `hash`: blake3 short id
  This sidecar enables positioners and downstream fusers without touching the core contract.

---

## 30) Error / Status Taxonomy

**Levels:** `ok`, `warn`, `degraded`, `fail` (with fallback).
**Classes:** `decode`, `bounds`, `config`, `device`, `format`, `container`.
**Examples:**

* `warn:audio.bounds: resample to 16k applied`
* `degraded:vision.decode: webp decoder unavailable → binary`
* `fail:text.format: invalid UTF‑16 w/out BOM → binary`
  All statuses append to trace and are emitted via diagnostics.

---

## 36) Ingest Checklist (so we can build immediately)

* Sources enabled: File, HTTP, Mic (16k), Cam (RGB), STDIN.
* Caps: max frame 16 MB; session cap 256 MB; timeouts 5s local/15s network.
* Hints populated: filename/ext, sr/channels, WxH when known.
* ByteFrame fields: `data`, `ts`, `source_id` (others optional).
* Backpressure: bounded queue size N; drop policy = coalesce for streams.
* Logging: per‑frame short id optional; errors surfaced without panics.

---

**Result:** With these sections, we’re not just “clear”—we’re **build‑ready**. No ambiguity on determinism, routing, metadata, errors, benches, safety, repo layout, canonical traces, or ingest. This is the full spec we can execute against.
sozna-sense/
├─ Cargo.toml # workspace
├─ rust-toolchain.toml # (optional) pin toolchain
├─ README.md # project overview
├─ LICENSE # choose MIT/Apache-2.0
├─ .gitignore
├─ .editorconfig
├─ .github/
│ └─ workflows/
│ ├─ ci.yml # build + test
│ └─ lint.yml # fmt + clippy
├─ crates/
     rusta-sense #public API
│ ├─ sense-core/ #  ByteFrame, SenseOut, Adapter trait
│ ├─ sense-detector/ # modality detection
│ ├─ sense-adapters/
│ │ ├─ text/ # TextAdapter (Conv1D stem → proj)
│ │ ├─ audio/ # AudioAdapter (STFT→mel→patch)
│ │ ├─ vision/ # visionAdapter (patchify 16×16)
│ │ └─ binary/ # RawByteAdapter
│ └─ sense-cli/ # tiny CLI: `sense <path>`
├─ configs/
│ ├─ default.toml # defaults for all adapters
│ └─ examples/ # per-modality variants
├─ tests/
│ ├─ fixtures/
│ │ ├─ text/
│ │ │ ├─ utf8_hello.txt
│ │ │ ├─ utf16_bom.txt
│ │ │ └─ long.json
│ │ ├─ audio/
│ │ │ ├─ sine1k_16k.wav
│ │ │ ├─ speech_short.wav
│ │ │ └─ silence.wav
│ │ ├─ vision/
│ │ │ ├─ checker_128.png
│ │ │ ├─ colorbars.jpg
│ │ │ └─ exif_rot90.jpg
│ │ └─ binary/
│ │ ├─ random.bin
│ │ ├─ zip_stub.zip
│ │ └─ pdf_stub.pdf