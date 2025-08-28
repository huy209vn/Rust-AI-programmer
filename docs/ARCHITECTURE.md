# 🌌 Rusta — Canonical Architecture (v3)

Owner: Huy
Date: 2025-08-28
Edition: Rust 2024
Core Identity: **Brain-first**. **Capability Kit = hands** (taste‑free). **DevLog** for presence. No boss layers.

---

## 0) Tenets (Non‑Negotiable)

* **Agency lives in the Brain.** All choices (hypotheses, plans, patches, thresholds) are Brain decisions.
* **Hands don’t decide.** Capability Kit executes effects and returns facts + undo handles.
* **Reversible by default.** Every mutation has a guaranteed `undo()` path.
* **Explain or abstain.** Presence beats silent patching.
* **Local‑first, ledgered.** Everything runs locally; events + data are provenance‑tracked.

---

## 1) Monorepo Layout (Canonical)

```
rusta/
├─ Cargo.toml                         # workspace
├─ rust-toolchain.toml                # pinned channel + components
├─ .cargo/config.toml                 # lints, target-cpu, wgpu flags
├─ crates/
│  ├─ rusta-kits/                     # shared types, ids, errors, policy
│  ├─ rusta-brain/
│  │  ├─ byte-lm/                     # decoder-only, tokenizer-less (Burn)
│  │  ├─ heads-action/                # plan/patch proposal formats
│  │  ├─ heads-confidence/            # apply/abstain calibration, ECE
│  │  ├─ io-bytes/                    # dataset collator → sequences
│  │  └─ loop-core/                   # **best loop** implementation (Brain-owned)
│  ├─ rusta-capkit/                   # **Capability Kit** (git/fs/proc/RA/tools/events)
│  ├─ rusta-devlog/                   # WAL + snapshots + query API (append-only)
│  ├─ rusta-provenance/               # SPDX/SBOM ledger for data + deps
│  ├─ rusta-datasets/                 # ingestion, shard, rot checks
│  ├─ rusta-eval/                     # harness (slices/canaries/chaos/calibration)
│  ├─ rusta-ui/                       # Dioxus desktop (Explain bubble, Why drawer, Review)
│  ├─ rusta-cli/                      # headless control (plan/probe/patch/undo/devlog)
│  └─ xtask/                          # doctor, data snapshot, learn sync, release
├─ docs/
│  ├─ vision_rusta_v4_1.md
│  ├─ spec_rusta_v4_1.md
│  └─ training_plan_v4_1.md
└─ third_party/                       # SBOMs, NOTICES, external license cache
```

> **No orchestrator/runtime boss.** `rusta-capkit` is taste‑free hands; Brain calls it.

---

## 2) The Best Loop (Brain‑Owned)

**Macro:** `Intake → Think → Communicate → Probe → Decide → Do → Validate → Reflect → (Undo | Commit) → Log`

**Micro (inside Think/Probe):** `Hypothesis → Minimal probe → Update belief → Continue/Escalate`

**Hard rules (habits):**

* No commit without green.
* Always log intent, probes, validation, reflection.
* Always keep an undo handle and use it on red.
* Network OFF during Do/Validate unless explicitly allowed.

---

## 3) Capability Kit (Hands, Not Boss)

### 3.1 Purpose

* Provide imperative, undoable effects with structured results and events: **git**, **fs**, **process** (cargo/ra/clippy/miri/fmt/fuzz/criterion), **events** (to DevLog).
* Zero policies. Never branches on confidence, risk, or scope.

### 3.2 Minimal Interfaces (conceptual)

* `GitCaps`: `apply_patch(patch) -> PatchHandle`, `undo(handle)`, `commit(meta) -> CommitId`
* `ProcCaps`: `run(cmd) -> Exit` (helpers: `cargo_check`, `cargo_test`, `cargo_fmt`, `clippy`, `miri`)
* `RaCaps`: rust‑analyzer queries (type, defs, refs, diags)
* `FsCaps`: read/write (writes encourage going through `apply_patch`)
* `EventsSink`: `append(Event)`

### 3.3 Events (for presence/UI)

* `ProbeStart/End`, `PatchApplied/Undone`, `ValidateStart/End`, `Committed` (+ timings, exit codes)

### 3.4 Guarantees

* **Atomic** patches with deterministic `undo()`
* **Hermetic logs** (stdout/stderr/time)
* **Deterministic effects** (timeouts are arguments from Brain)

---

## 4) Brain Modules

* **byte-lm**: Burn model; decoder‑only over bytes; export `infer`, `train`, `save/load`.
* **heads‑action**: plan & patch proposal schema (no IO).
* **heads‑confidence**: calibrated abstain/apply logits; ECE tracking.
* **io‑bytes**: collate DevLogs/docs/repos/diagnostics into byte sequences.
* **loop‑core**: implements the loop + habit rules; calls CapKit.

---

## 5) DevLog & Provenance

* **DevLog**: WAL + periodic snapshots (SQLite default, Sled optional).
  Schema:

  ```rust
  DevEntry { id, ts, repo, stage, intent, probes[], patch?, validation, reflection, tags[], version }
  ```
* **Provenance**: per shard & dep SBOM (license, source, commit, checksum). CI gate via `cargo deny` + SPDX export.

---

## 6) Data & Training Spine

* **Datasets**: permissive repos (MIT/Apache/0BSD), Rust docs, diagnostics, Golden Sessions.
* **Sharding**: content‑hash; rot checks + mirrors.
* **Held‑out canaries** for honest eval.

---

## 7) Evaluation Harness

* **Primary**: compile‑fix, diagnostic explanation accuracy, ECE calibration.
* **Secondary**: abstention correctness, probe efficiency, repeat‑mistake rate, voice consistency (human eval).
* **Modes**: slices / canaries / chaos / calibration.
* **Reports**: JSON + Markdown with trendlines.

---

## 8) UI (Dioxus Desktop)

* **Explain bubble**: intent + confidence + probe summary.
* **Why drawer**: hypotheses, trade‑offs, doc cites.
* **Review tab**: patch bundles w/ rationale; Apply/Rollback (maps to CapKit).
* **DevLog viewer**: timeline of events + reflections.

> UI talks to BrainLoop directly; CapKit events make the timeline automatic.

---

## 9) Policies (Declarative, chosen by the Brain)

* `LoopPolicy { c_apply_quick, c_apply_deep, probe_budget_secs, test_timeout_secs, net_mode }`
* Chosen at runtime **by the Brain** (or by user), not enforced inside CapKit.

---

## 10) Security & Sandboxing

* Default **no network** during Do/Validate (policy‑controlled).
* Worktree sandbox for patches; jailed process runner for tools; explicit allowlists.
* Secrets never logged; redact on event emission.

---

## 11) Performance Targets (Phase 1)

* Explain cold‑start ≤ 2s p50.
* Probe cycle ≤ 1.5s p50.
* Small‑crate validate ≤ 12s p50.
* Tiny (≈120M) CPU int8; Base (300–600M) GPU int8.

---

## 12) Extension Model

* **Hands**: add new capability by implementing a trait (e.g., FuzzCaps).
* **Brain**: add a new “move” (pattern) by training or prompting; no repo changes needed.
* **UI**: new panels subscribe to events; no changes to Brain.

---

## 13) CI/CD

* `fmt`, `clippy -D warnings`, `test --all`, `deny`, `audit`, `rusta-eval --smoke`.
* Nightly soak: replay DevLogs + pinned goldens; publish eval reports.
* Release: tag, SBOM, binaries for `rusta-cli` and `rusta-ui`.

---

## 14) Boot Plan (L0 → L1)

* **L0 (2 weeks):** `rusta-capkit` MVP (git/apply/undo, cargo check/test, events) • `rusta-devlog` WAL • `loop-core` with fake brain • UI bubble stub • `xtask doctor`.
  **Exit:** reversible patch demo, logs in timeline, doctor passes.
* **L1 (2–4 weeks):** tiny Byte LM + collator • confidence head stub • clippy/miri/fmt adapters • eval smoke+canary.
  **Exit:** loop with tiny model; ≥40% compile‑fix on curated cases.

---

## 15) Open Questions (Explicit)

* SQLite vs Sled default across platforms?
* Worktree vs index‑patch strategy trade‑offs for speed/atomicity?
* RA transport: spawn LSP vs embed language server libs?
* When (if ever) to add PR capabilities (still imperative)?

---

## 16) Glossary of “Moves” (to tag reflections)

* `move/needless_borrow` • `move/lifetime_shrink` • `move/trait_bound_add` • `move/async_send_sync` • `move/unsafe_to_safe` • `move/api_rename_refactor`
  These tags help training + UI review buckets.

---

## 17) Philosophy Check

This architecture keeps Rusta **free** (no hidden rules), **safe** (undo handles), and **present** (events + DevLogs).
The Capability Kit is just **hands**; the Brain is the whole person.

---

**End of Canonical Architecture (v3).**
# 🖐️ Rusta Capability Kit — Hands, Not Boss (v1)

Owner: Huy
Date: 2025-08-28
Motto: *She uses tools; tools never use her.*
Scope: Minimal, taste-free effects library (undoable FS/git/process/tools) + append-only ledger for presence. No policies, no thresholds, no decisions.

---

## 0) Design Tenets

* **Taste-free**: No branching by risk/confidence. Do the thing or return facts/errors.
* **Agentic**: Only the Brain decides *what* and *when*. The kit only performs actions.
* **Undoable**: Every mutating op returns a handle that guarantees `undo()`.
* **Inspectable**: Every call emits a structured event to the ledger (append-only).
* **Composable**: Small, orthogonal capabilities; no hidden sequencing.

---

## 1) Crate Layout

```
crates/
  rusta-capkit/            # this crate (capabilities + events)
  rusta-devlog/            # ledger store (WAL + snapshot) — reused
  rusta-brain/loop-core    # owns the loop; calls capkit
  rusta-brain/tools/*      # RA adapters etc. (thin)
```

* `rusta-capkit` depends on `rusta-devlog` **only** for `EventsSink` trait (not storage details).

---

## 2) Core Traits (philosophy-level signatures)

```rust
pub trait EventsSink { fn append(&self, ev: Event) -> Res<()>; }

pub trait GitCaps {
    fn apply_patch(&self, p: Patch) -> Res<PatchHandle>;    // atomic
    fn undo(&self, h: PatchHandle) -> Res<()>;              // mirrors apply
    fn commit(&self, meta: CommitMeta) -> Res<CommitId>;    // no gatekeeping
}

pub trait ProcCaps {
    fn run(&self, cmd: Cmd) -> Res<Exit>;                   // cargo/clippy/miri/fmt/fuzz/crit
}

pub trait RaCaps { /* rust-analyzer queries; pure info */ }

pub trait FsCaps {
    fn read(&self, path: &Path) -> Res<Bytes>;
    fn write(&self, path: &Path, data: &[u8]) -> Res<()>
        where Self: GitCaps; // encourage patch via git when mutating
}
```

**Notes**

* `ProcCaps::run` is generic; use helpers like `cargo_check(sel)`, `cargo_test(sel)` built on top for ergonomics, still taste-free.
* Mutations prefer `GitCaps::apply_patch` over raw writes to preserve undo.

---

## 3) Data Types (minimal, stable)

```rust
pub struct Patch { pub id: PatchId, pub edits: Vec<Edit> }
pub struct Edit { pub path: PathBuf, pub before: Hash, pub after: Bytes, pub range: Option<Span> }
pub struct PatchHandle { pub id: PatchId, pub worktree: PathBuf }

pub struct CommitMeta { pub message: String, pub branch: String, pub author: Sig, pub trailers: BTreeMap<String,String> }

pub enum Cmd { Cargo(Vec<String>), Raw { prog: String, args: Vec<String> } }
pub struct Exit { pub code: i32, pub stdout: Bytes, pub stderr: Bytes, pub secs: f32 }

pub struct TestSel { pub package: String, pub filter: Option<String> }

pub type Res<T> = Result<T, CapErr>;
```

**Error model** (`CapErr`):

* `Io`, `Process(Exit)`, `PatchConflict { path, hint }`, `Timeout`, `NotFound`, `Denied`, `Invariant(String)`.
* Always printable + serializable for DevLog.

---

## 4) Events (append-only; for presence/UI)

```rust
pub enum EventKind {
  ProbeStart { tool: String, query: String },
  ProbeEnd   { ok: bool, secs: f32, exit: Option<i32> },
  PatchApplied { patch_id: PatchId, files: usize, loc: usize },
  PatchUndone  { patch_id: PatchId },
  ValidateStart { sel: TestSel },
  ValidateEnd   { green: bool, failing: Vec<String>, secs: f32 },
  Committed   { commit: CommitId, branch: String },
}

pub struct Event { pub ts: DateTime, pub repo: RepoRef, pub kind: EventKind, pub meta: BTreeMap<String,String> }
```

* Emitted **by the kit**, consumed by `rusta-devlog`. Brain can also add free-form reflection log lines separately.

---

## 5) Lifecycle Example (quick fix)

1. Brain: compute plan (remove needless borrow).
2. Brain → CapKit: `ProbeStart( cargo check )` / `ProbeEnd` (ok).
3. Brain → CapKit: `apply_patch(patch)` → `PatchApplied`.
4. Brain → CapKit: `ValidateStart/End` via `cargo test -p foo -- foo::unit` (green).
5. Brain → CapKit: `commit(meta)` → `Committed`.
6. Brain → DevLog: reflection entry (pattern tag: `move/needless_borrow`).

**No decisions** occurred inside the kit.

---

## 6) Lifecycle Example (red → undo)

1. Brain applies patch.
2. Validate returns `green=false`.
3. Brain calls `undo(handle)`; kit emits `PatchUndone`.
4. Brain writes reflection and loops to a new hypothesis.

---

## 7) Safety & Determinism

* **Atomic patching**: Worktree or index-based apply that records prior hashes → `undo()` restores exact bytes.
* **Timeouts**: Implemented in `ProcCaps::run`, but values come from *Brain* (passed as arg); kit does not choose them.
* **No network**: Optional switch on `ProcCaps` that rejects commands with env `NET=OFF` (again, chosen by Brain/Policy).
* **Hermetic logs**: Every exit status and stderr captured; never swallowed.

---

## 8) Integration with Brain Loop

```rust
let handle = capkit.git.apply_patch(plan.patch)?;          // Do
let val    = capkit.proc.cargo_test(plan.test_sel())?;     // Test
capkit.events.append(Event::from(&val))?;                  // Presence
if val.green { capkit.git.commit(plan.commit_meta())?; } else { capkit.git.undo(handle)?; }
```

* Brain computes `plan`, interprets `val`, chooses next step.
* CapKit never branches on outcomes.

---

## 9) Testing Strategy

* **Golden IO**: record/replay for `ProcCaps::run` so unit tests don’t need cargo/RA present.
* **Property tests**: `apply_patch` then `undo` → bytes match; commuting patches conflict predictably.
* **Stress**: parallel apply/undo to ensure handles isolate worktrees.

---

## 10) Minimal Public Surface (v1)

* `GitCaps`, `ProcCaps`, `RaCaps`, `FsCaps`, `EventsSink` + structs above.
* Helpers: `cargo_check`, `cargo_test`, `fmt_paths`, `ra_type_of`.
* No more.

---

## 11) Later (if wanted, still taste-free)

* `BenchCaps` (criterion runner); `FuzzCaps` (cargo-fuzz).
* `SandboxMode` variants (Worktree/InPlace/MemPatch) for perf.
* `PRCaps` for platform-specific PR openers (GitHub/GitLab) — still imperative.

---

## 12) Why this fits Rusta

* Mirrors human reality: **hands don’t decide**, they execute.
* Keeps your philosophy intact: **presence** via events, **reversibility** via handles, **agency** in the brain.
* Upgrades easily: zero → kit → kit+ledger without ever turning into a boss.

---

**End v1.** If you want, we can now strip the previous “runtime” from the canvas and anchor the repo around **Brain Loop + Capability Kit + DevLog** only.



