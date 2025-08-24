VISION — AI Rust Programmer — Locked Spec v1.3 + Appendices v1.4
Owner: Huy
Date: 2025 08 19
Status: Frozen core + refined + extended with full toolchain + appendices
Mission: Build a from scratch, tokenizer less AI Rust programmer in Rust (Burn) as the base model for Rusta (Sozna’s first prototype).
Horizon: Multi year
Principles: Open source from day one • Local first by design • Architecture must directly support future integration of Sozna traits (memory, agency, personality)
________________________________________
0) Glossary
•	Byte LM — Decoder only transformer operating on raw bytes (no tokenizer).
•	Anchor — Content hash stable span reference used for citations into local doc indexes.
•	Trace — Machine readable log of model thoughts, tool calls, observations, and patches.
•	TraceScript — Minimal, auditable DSL describing a reasoning + tool sequence.
•	Project Graph — Persistent symbol/type graph of an indexed Rust repo.
•	Golden Trace — Curated conversation or tool run with perfect citations + ground truth.
________________________________________
1) Scope
IN — Permanent inclusions
•	From scratch model (no pre trained LLMs/embeddings).
•	Tokenizer less byte → tensor pipeline.
•	Rust only stack: Burn for model/training; Rust for tooling + integration.
•	Local inference on commodity developer hardware (CPU/GPU).
•	Modular, documented OSS repo (permissive license).
•	Project memory, safe patching, and reasoning traces.
•	Direct compatibility with Rusta traits (personality, continuous thought, autobiographical memory).
OUT — Permanent exclusions
•	Dependencies on closed/proprietary models.
•	Mandatory cloud inference/hosted dependencies.
•	Tokenizer based pipelines as primary input.
•	Non Rust runtimes as primary.
•	Throwaway “quick demo” hacks that don’t map to the end architecture.
________________________________________
2) End Product Vision
When complete, the AI Rust Programmer will:
•	Read, explain, and improve large Rust codebases with accuracy and context.
•	Operate fully locally, integrating with editors and/or a native GUI.
•	Maintain deep knowledge of Rust std, Rustonomicon, major crates — always citing anchors.
•	Use tools to index code, run cargo check & tests, perform safe multi file refactors, and generate diffs.
•	Produce transparent reasoning traces for significant decisions.
•	Retain project context, style preferences, and prior interactions via local persistent memory.
•	Serve as a community AI Rust core with a plugin system.
•	Be directly extensible into Rusta (personality, agency, continuous thought).
________________________________________
3) Architecture Overview
Editors/Terminal (VSCode, JetBrains, CLI)
        │
        ▼
+-------------------------------+
|   Orchestrator (Local)        |
| - Request routing             |
| - Policy engine               |
| - Tool planner & safety rules |
| - Trace log (machine-readable)|
+---------------------+---------+
                      │
     +----------------+----------------+
     │                                 │
     ▼                                 ▼
+------------+                 +----------------------+
|  Byte LM   |                 | Tools & Interfaces   |
|  (Burn)    |                 | - Code indexer       |
| - byte→tensor                | - Cargo/rustc        |
| - decoder transformer        | - Test runner        |
| - scratchpad reasoning       | - Patch generator    |
| - optional AST aux heads     | - Plugin interface   |
+-----+------+                 +----------+-----------+
      │                                  │
      ▼                                  ▼
+-------------------+          +----------------------+
| Memory Layer      |          | Knowledge Sources    |
| - short-term pad  |          | - std / Rustonomicon |
| - project graph   |          | - Rust By Example    |
| - long-term log   |          | - curated crates     |
+-------------------+          | - local repos        |
                               +----------------------+
3.1 Orchestrator
•	Local request router + policy engine.
•	Tool planner with safety rules.
•	Emits TraceScript for each session; logs to JSONL.
•	Future ready for differentiable planning (Rusta bridge).
3.2 Byte Model
•	Input: raw bytes (code, docs, compiler output).
•	Encoding: learned byte embeddings; optional light conv/gating/SSM pre mix.
•	Positional: RoPE or ALiBi (long context required).
•	Core: decoder only transformer in Burn.
•	Reasoning: structured scratchpad.
•	Aux heads: AST aware hint heads with weak supervision from AST spans.
3.3 Memory Layer
•	Short term: scratchpad segments.
•	Project memory: symbol/type graph + history of edits.
•	Long term: persistent logs of style, prior fixes, user prefs.
•	All stores local; pluggable backend (sled/sqlite/rocksdb).
•	Deterministic export/import for audits.
3.4 Tools (roles)
•	Code Indexer: incremental parse; maintains project graph.
•	Cargo/rustc: ground truth compiler + test runner.
•	Patch Generator: proposes diffs; safe mode with auto revert.
•	Rust Analyzer (RA): semantic engine for cheap probes.
•	Formatter/Linter: style & lint integration.
Plugin Interface: sandboxed extensions.


3.5 TraceServer (Service Layer)
The TraceServer is the runtime service responsible for collecting, validating, streaming, and replaying traces. It sits alongside the Orchestrator, exposing a stable API for jobs, logs, and audits.
Responsibilities
•	Ingestion & Validation
• Accepts traces emitted by the Orchestrator.
• Validates against Trace v1 schema (Plan, Probe, Decide, Patch).
• Rejects malformed or anchor-less traces.
•	Job Queue
• Provides API to enqueue tasks (e.g., “analyze crate”, “apply patch”).
• Workers pull jobs; results logged as new traces.
•	Streaming & Replay
• Streams live trace JSONL over WebSocket/Server-Sent Events for editor clients.
• Stores traces in local ledger (append-only, content-hash indexed).
• Supports deterministic replay (re-run probes, cargo checks, patch apply) for audit.
•	Policy Enforcement
• Enforces abstention rules (citations, proof-of-work patches).
• Blocks non-compliant traces (no anchors, failing tests, oversized diffs).
• Auto-reverts invalid patches.
•	Audit & Provenance
• Maintains manifest of all trace sessions with hash IDs.
• Exports/imports bundles for community sharing.
• Integrates with Golden Trace dataset collector.
Interfaces
•	REST API:
• POST /job — enqueue job.
• GET /traces/{id} — fetch stored trace.
•	Streaming API:
• WS /stream — real-time trace streaming.
•	Storage:
• JSONL ledger on disk, hash-stable manifests.
End State
TraceServer ensures every action by the AI Rust Programmer is:
• Observable in real time.
• Replayable for debugging.
• Validated against schema and policy.
• Auditable for provenance and reproducibility.
•	
________________________________________
4) Training & Data
•	Licensing & provenance ledger.
•	Bytes only inputs.
•	Grounded docs with anchors.
•	Compiler triplets pillar.
•	Eval gates at checkpoints: anchor fidelity ≥ 95%, compile fix, trace quality, regression battery.
•	Local inference only in shipped product.
Flexible Tactics
•	Data mix, dedup, curriculum pacing.
•	Optimizers/schedules (AdamW/Lion/Adafactor; cosine; warmup).
•	Long context methods.
•	Weak AST hint channels.
•	Planner learning approach (offline RL from traces).
Current Plan
•	Datasets: stdlib, Nomicon, Rust by Example, permissive crates, curated repos, compiler triplets, synthetic bugs, Golden Traces.
•	Curriculum: S0 Byte LM → S1 Rust comprehension → S2 Project tasks → S3 Safe multi file refactors → S4 (optional RL).
•	Objectives: byte LM loss; aux heads; citation loss; tool use loss.
________________________________________
5) Evaluation
•	Gating: anchor fidelity ≥ 95%; compile fix ≥ 20% (P1) → ≥ 50% (P2) → ≥ 70% (P3); trace validity; regression suite.
•	Secondary: diff minimality, test pass %, latency/footprint, hallucination rate.
•	Audit: reproducible with cargo xtask eval.
________________________________________
6) Interfaces & Schemas
•	Trace JSONL — structured logs of every step.
•	Project Graph — nodes (Crate, Struct, Trait, Fn, etc.); edges (defines, uses, impls, etc.).
•	Tool Protocol — JSON RPC commands (cargo.check, ra.type_of, index.update, etc.).
•	Plugin ABI — sandboxed; permissions explicit.
________________________________________
7) Safety Rules
•	Never apply patch unless cargo check & tests pass.
•	Auto revert on regression.
•	Diff minimality enforced.
•	Anchors logged for all modifications.
•	Dry run before applying to working tree.
________________________________________
8) Repo Layout & Governance
________________________________________
9) Development Flow
•	Phase 1 — Foundations
•	Phase 2 — Core Competence
•	Phase 3 — Advanced Utility
•	Phase 4 — Rusta Bridge.
________________________________________
10) Risks & Mitigations
•	Byte modeling efficiency → curriculum shaping, downsamplers.
•	Long context → sliding windows, retrieval from project graph.
•	Licensing → strict ledger.
•	Safe refactors → patch gating tokens.
•	Memory bloat → compaction, TTL policies.
•	Community scaling → stable plugin ABI, versioning.
________________________________________
11) Rusta Bridge — Hard Mapping
Subsystem	Evolves into Rusta
Orchestrator	Differentiable planner
Project Memory	Autobiographical memory
Scratchpad	Continuous thought stream
Byte Model	Multimodal I/O
Tools	SoznaArt actions
Traces	Narrative memory
________________________________________
12) Editor & UX Integration
•	VSCode/JetBrains extensions + CLI.
•	Session replay from traces.
•	Inline citations with hover to open anchors.
________________________________________
13) Security & Privacy
•	No network by default.
•	Plugins sandboxed.
•	Secrets filtered.
•	Telemetry opt in + local only.
________________________________________
14) Contributing
•	Style: rustfmt + clippy clean.
•	RFC template.
•	Issue labels: good first issue, help wanted, etc.
________________________________________
________________________________________
Extensions (merged)
Core Rust Toolchain (must have): rustc/Cargo, RA, rustfmt, clippy.
Learning Tools: indexer, patch applier, test runner, trace recorder.
Advanced Tools: AST query, doc navigator, Golden Trace player, RL sandbox.
Policy Graph: Understand → Propose → Verify → Test → Polish → Record.
Training Data Mapping: compiler triplets, doc QA, tool use sequencing, Golden Traces.
Indexer Internals: file hashes + stable IDs, sled/sqlite.
TraceScript v0.2: grammar + validator + replayer.
Eval Harness: 10 failing mini crates.
A) Core Rust Toolchain (must have)
•	rustc / Cargo
o	Role: ultimate ground truth. Used at every compile/verify gate, before and after patches.
o	Policy: no patch is accepted unless Cargo says it compiles and tests pass.
•	Rust Analyzer (RA)
o	Role: fast semantic probe. Use for symbol resolution, type hints, quick project graphing.
o	Policy: probe first with RA for structure, but compiler always has final authority.
•	Formatter & Linter (rustfmt, clippy)
o	Role: enforce Rust culture (style, idioms).
o	Policy: never block functional fixes. Run after green tests or when the only delta is style.
B) Learning / Training Tools (teaching aids)
•	Code Indexer (custom)
o	Role: build and persist a project graph (modules, traits, impls, calls, bounds).
o	Purpose: lets her “look back” at structure instantly, instead of recomputing.
o	Training: emits snapshots tied to commits for model learning.
•	Patch Applier (safe mode)
o	Role: apply diffs with rollback.
o	Policy: orchestrator only applies after successful compile + test gate.
•	Test Runner
o	Role: check if fixes actually work in context.
o	Training: generates before/patch/after triplets.
•	Trace Recorder (lab notebook)
o	Role: record every probe, patch, test, citation into structured traces.
o	Purpose: creates the “golden data” for evaluation and imitation.
C) Advanced / Growth Tools (for the Rusta bridge)
•	AST Query Tool (optional) — syntax tree lens. Helps ground weak heads, but bytes remain source of truth.
•	Doc Navigator — structured search over std, Rustonomicon, Rust By Example. Policy: citations are anchors, not vibes — required for every explanation.
•	Golden Trace Player — replay exemplar workflows (e.g., fixing borrow checker in N steps). Purpose: trains strategies; also acts as evaluation reference.
•	RL Sandbox (optional) — offline arena for trying variations of tool use. Source: learns only from traces, never free for all.
________________________________________
Appendices (v1.4 Extensions)
Appendix A — Golden Trace Dataset Spec
Definition: A Golden Trace is a curated exemplar workflow used for training (imitation) and evaluation (baseline).
Required fields per trace:
•	Input snapshot: repo state, failing tests or diagnostics.
•	Tool calls: ordered sequence (cargo.check, RA probes, patches, tests).
•	Reasoning: scratchpad segments with citations.
•	Patches: diffs with anchors to source spans.
•	Verification: final cargo check + tests all green.
Quality criteria:
•	≥ 1 valid citation per claim.
•	Minimal patch that passes compile & tests.
•	TraceScript valid and replayable.
•	Anchors stable (content hash based).
Storage format: JSONL with schema: {snapshot, steps[], final_state, citations[], metrics}.
Use cases:
•	Training: imitation learning of tool sequencing & citation prediction.
•	Evaluation: benchmark set per error category.
________________________________________
Appendix B — Quantization / Deployment Targets
Model size classes:
•	Tiny (≤ 120M params): runs interactively on 4 core CPU + 16GB RAM. 8 bit quant required. Echo/code complete tasks.
•	Base (300–600M params): target for mid GPU (RTX 3060, 12GB VRAM) or CPU with 8 bit quant. Full cargo/test cycles under ~1s per 512 tokens.
•	Large (1–2B params): optional high end GPU class. Runs with 4 bit quantization.
Deployment path: Export checkpoint → quantize (int8, int4) → load via Burn runtime.
CLI flag: --quant 8 or --quant 4.
Default distribution: Tiny + Base; Large optional.
Principle: Always runnable on commodity developer hardware.
________________________________________
Appendix C — Error Taxonomy (Eval Slices)
Diagnostics are classified for evaluation slices:
•	Borrow checker: missing lifetimes, invalid references, dangling borrows.
•	Trait bounds: method not found, type not implementing trait.
•	Type mismatch: expected vs found, mismatched generics.
•	Move/ownership: use after move, moved value errors.
•	Visibility: private item used.
•	Syntax/basic: missing semicolon, unknown keyword.
•	Lint/style: clippy warnings, rustfmt deviations.
Eval requirement: compile fix and patch minimality must be reported per error class.
Example gate: ≥ 80% success on borrow checker errors by Phase 3.
________________________________________
Appendix D — Memory Persistence
Layers:
•	Short term: scratchpad cleared per session.
•	Project memory: symbol/type graph + history, retained until repo removed.
•	Long term logs: style, prior fixes, preferences.
Persistence policy:
•	Backends: sled/sqlite (pluggable).
•	Compaction + TTL: prune logs (keep last N sessions per repo).
•	Export/import: JSONL or sqlite dump (portable memory bundles).
•	User controls: --memory {short|project|full}.
Privacy principle: No network export unless explicitly commanded.
________________________________________
Appendix E — UX Expansion
End State Vision: The AI Rust Programmer is not a chatbot with shell commands. She appears as a Rust expert living in your toolchain, transparent and local first.
Editor Integration:
•	Present inside VSCode/Jet
1. Always up to date (knowledge freshness)
•	Ground truth tools as authority: every patch must be validated against cargo check and tests. That way, even if her “knowledge” is stale, she can’t ship wrong code.
•	Anchored docs: instead of fuzzy recall, she cites stable anchors in Rust std/Nomicon/local repos. You can update those indexes anytime, and she learns to cite them rather than guess.
•	Golden Traces refresh: new curated workflows (e.g. with latest crates or language features) get added as training/eval data. That “teaches” her new idioms.
•	Project Graph retrieval: she doesn’t memorize everything forever — she re-reads the repo graph as needed, so updates in your codebase reflect instantly.
3. Awareness (self-monitoring / meta-cognition)
•	Scratchpad reasoning: she always externalizes her thought process before acting. That’s a kind of awareness: she sees her own plan.
•	TraceScript: formalizes the plan → action sequence. Makes “awareness” auditable.
•	Memory layers: project graph + long-term logs = she remembers your style, past errors, and her own past fixes. That’s contextual self-awareness.
•	Future step (Rusta bridge): differentiable planner + continuous thought stream. That’s where “awareness” matures into something like Sozna’s inner monologue.
No BS, Fast Learning, Aware — Design Pack
1) Truth & Uncertainty (so she doesn’t “spit out shit”)
Policy switches (default ON):
•	Citations or Silence: Any explanatory claim must attach ≥1 anchor (doc span or compiler diag). If not, she must say “unknown” and propose next probes.
•	Tool First Grounding: Before answering/fixing, run cheap probes: cargo check, RA type_of, index lookups. If probes disagree with her hypothesis → abstain & re plan.
•	Proof of Work Patch: A patch is only valid with: green cargo check + tests; diff minimality ≤ K lines; anchors for why.
•	Confidence Ledger: Every step logs {claim, anchors[], probe_results, p(conf)∈[0,1]}. If p(conf)<τ_answer → do more probes; if still <τ_abstain → explicitly abstain.
Model side:
•	Calibrated heads: Add a small confidence head (sigmoid) trained with label = (passed probes/tests & citations present). Optimize Brier loss for calibration.
•	Uncertainty tokens: Give the model dedicated tokens for “ABSTAIN”, “NEED_PROBE(tool=X, target=Y)”. Penalize hallucinations harder than abstentions.
Orchestrator guardrails:
•	If no_anchor && purpose=explain → block reply, auto invoke doc navigator or compile probe.
•	If p(conf)<τ_patch → forbid patch application; allow “investigation plan” only.
2) Awareness (meta-cognition without vibes)
•	TraceScript as mirror: Plan → Probe → Decide → Patch is always written before acting. She “sees” her own plan.
•	Self check step: After plan, run CONSISTENCY_CHECK: compare key claims vs probes; if mismatch → require re plan.
•	Memory pings: On low confidence, fetch from Project Graph + prior Golden Traces for similar errors (non parametric recall, not RAG chat).
3) Fast Learning (search + experiments, not glued-on RAG)
•	Search = Experiments, not answers: When unsure, she doesn’t paste doc text; she uses search/index to form hypotheses, then proves them with compiler/tests.
•	Golden Trace Growth Loop:
1.	Low conf case triggers Explore Mode (bounded budget): try 1–3 minimal hypotheses, generate micro tests if repo lacks coverage.
2.	Best hypothesis → patch → verify.
3.	Save the whole run as a Golden Trace (with anchors, probes, diffs).
4.	Nightly continual training on fresh traces (no internet needed), with replay/regularization to avoid forgetting.
•	Episodic Cache (non parametric): Keep a small local key→value store keyed by (diag hash, crate, symbol pattern). Hit it before heavy search; expires with TTL & repo change.
4) Stays on Important Work (doesn’t hide behind “unsure”)
•	Impact aware planner: Score tasks by (failing tests count, compile error severity, component centrality from Project Graph). Even if low conf, the planner prioritizes high impact slices but uses investigation plans instead of instant answers.
•	Explain then act contract: For big tasks, she must produce: (a) diagnosis with anchors, (b) plan with probes, (c) minimal risk slice to attempt first.
5) Concrete knobs (you can ship these)
•	τ_answer (explain threshold): default 0.7
•	τ_patch (apply threshold): default 0.85
•	K (diff minimality lines): default 20
•	B (explore budget per issue): default 3 micro experiments
•	T_probe_max: e.g., 2s per cheap probe before escalating
7) Evaluation that enforces this behavior
•	Abstention Quality: % of low conf cases where she abstains and proposes a valid next probe (not silence).
•	Hallucination Rate (H@τ): answers without anchors OR failing post probe checks.
•	Calibration (ECE/Brier): at both explain and patch time.
•	Learning Velocity: improvement on a rolling set of novel diagnostics after ingesting N new Golden Traces.
•	Exploration ROI: (# fixes found from Explore Mode) / (experiments run).
8) UX cues (so users feel the awareness)
•	Show a confidence chip (e.g., “72%”) next to answers.
•	Gray “Answer” button until anchors appear; “Add evidence” prompts run probes.
•	“I’m not certain yet — here’s my test plan (3 mins)” instead of filler text.
•	Timeline view highlights abstentions as green (correct behavior), not failures.



AI Rust Programmer — Systems Phases Roadmap (Programmer only)
No RAG. No chat CLI. Local first. Rusta Bridge is a separate plan.
This roadmap builds the actual product in deep, stable layers. We evolve intentionally — not frozen in stone — with compatibility commitments and deprecation windows so we don’t pay rewrite tax later.
________________________________________
Cross Cutting Principles
1.	No network by default (compile time + runtime gates).
2.	Explain ⇒ anchors; Patch ⇒ green checks + minimal diffs (policy enforced in CI).
3.	TraceScript before action: Plan → Probe → Decide → Patch is always written.
4.	Clear boundaries: model ↔ orchestrator ↔ tools ↔ memory ↔ UX.
5.	Compatibility commitments: SemVer for Protocol/Trace/Patch/Memory; additive by default. If a breaking change is truly needed, introduce it at a phase boundary with shims + migration tools + a deprecation window.
6.	Training ≠ inference: feature gated separation; inference binaries stay lean.
7.	Provenance: dataset ledger (license, source, commit) is required to train.
________________________________________
Phase 0 — Contracts & Policy
Purpose. Define the language the system speaks so later changes don’t cascade.
Deliverables. Protocol v1 (Tool RPC), Trace v1 (schema + validator + replayer), Patch Protocol v1 (diff/minimality/rollback + result codes), Memory Bundle v1 (project graph + logs export/import), Policy crate (τ thresholds, K, B; abstention rules) with tests; ADR/RFC template enforced by CI.
Compatibility target. Protocol/Trace/Patch/Memory v1.0 (additive changes after).
DoD. SemVer tags + conformance tests + CI policy checks.
________________________________________
Phase 1 — Data Spine & Provenance
Purpose. Make training/eval data reproducible and lawful.
Deliverables. Licensing/provenance ledger; ingestion + dedup for std/Nomicon/RxE + permissive crates; compiler triplet schemas + generators; dataset JSON Schemas; deterministic snapshot script.
Compatibility target. Dataset record shapes; artifact layout.
DoD. cargo xtask data:snapshot yields identical tarball on a clean machine.
________________________________________
Phase 2 — Model & Training Infrastructure v1
Purpose. Stand up the final model/training harness (scales by config), not a throwaway.
Deliverables. ByteCore (mmap/stream I/O; zero copy), ModelCore (decoder only transformer in Burn; Tiny/Base/Large via config; stable forward/infer API), TrainingCore (S0→S3 loops; checkpoint/optimizer/schedule interfaces; metrics export), artifact formats (checkpoints/logs) ready for quantization.
Compatibility target. Inference API (infer(bytes, tools_ctx) -> tokens/acts), checkpoint format v1.
DoD. Same code trains Tiny/Base via config; loads for inference; perf counters present.
________________________________________
Phase 3 — Orchestrator & Planning Core v1
Purpose. Install the brainstem that coordinates tools under policy—without UI concerns.
Deliverables. Planner (Plan→Probe→Decide; Confidence Ledger; abstention), tool router (cargo, RA, indexer, doc navigator, patch), Trace emission + offline replayer, daemon (local IPC; no outbound sockets; policy flags at boot). TraceServer daemon (Axum/Tokio) with minimal API: ingest traces, validate schema, store JSONL, offline replay.
Compatibility target. Orchestrator service surface; error codes; replay format.
DoD. E2E dry runs (no patch yet) produce valid traces on real repos; replay is deterministic.
________________________________________
Phase 4 — Tool Grounding Suite v1
Purpose. Wire in real ground truth tools with stable adapters.
Deliverables. cargo client (check, tests, diag normalization), ra client (type_of, find_refs, goto_def with retries), indexer (incremental parser → Project Graph v1 with symbols/edges/history), Doc Navigator (local std/Nomicon/RxE with anchors).
Compatibility target. Tool RPC messages; Project Graph node/edge types v1.
DoD. Probes + index snapshots operate on multi crate repos across OSes.
________________________________________
Phase 5 — Patch Engine & Safety v1
Purpose. Make surgical changes safely—the heart of “not a toy”.
Deliverables. Diff generator + shrinker; apply + rollback with atomic guarantees; minimality policy + shrink passes; fuzz/property tests; auto revert on regression integrated with cargo tests.
Compatibility target. Patch protocol v1; shrinker contracts; rollback result codes.
DoD. Fault injection tests pass; rollback preserves working tree; shrinker reduces ≥ target.
________________________________________
Phase 6 — Project Memory & Graph v1
Purpose. Persistent context without bloat; works with indexer and planner.
Deliverables. Backends (sled/sqlite), compaction/TTL, stable file/symbol IDs + history, Memory Bundles v1 export/import; user controls (--memory {short|project|full}; privacy switches).
Compatibility target. Bundle format v1; compaction policies documented.
DoD. Deterministic round trip; store stays within size budgets; planner consumes memory consistently.
________________________________________
Phase 7 — Evaluation & Gates v1
Purpose. Prove behavior with slices that mirror reality; install non negotiable gates.
Deliverables. Error taxonomy slices (borrow, traits, type mismatch, move, visibility, syntax, lint/style), metrics (compile fix, anchor fidelity, diff minimality, test pass %, abstention quality, calibration), regression suite & dashboard; cargo xtask eval one shot. TraceServer integrates with eval harness; provenance manifests recorded; replay mode validated across OSes.
Compatibility target. Metrics JSON schema; slice definitions.
DoD. Gates enforced in CI; red builds block merges.
________________________________________
Phase 8 — UX Integration v1 (Daemon + GUI/IDE, no chat)
Purpose. Operate the system via structured commands—not chat.
Deliverables. Daemon surface: Explain, Diagnose, ProposePatch, Apply, Revert, ReplayTrace, MemoryExport; GUI (Tauri+Dioxus): attach project, run actions, show traces/diffs, confidence chips; VSCode extension: inline anchors & diff preview (JSON RPC to daemon). TraceServer streams traces live into editor clients (VSCode, GUI), supports session replay directly from daemon.
Compatibility target. UX command surface v1; telemetry opt in (local only).
DoD. Cross platform binaries; deterministic replay; zero network by default.
________________________________________
Phase 9 — Scale, Performance & Quant v1
Purpose. Make it fast, portable, and frugal—without altering earlier contracts.
Deliverables. Long context strategies (sliding/strided windows; attention optimizations), orchestrator concurrency; zero copy I/O; hot path caches; int8/int4 loaders; accuracy vs footprint sweeps for Tiny/Base/Large; cross platform CI (Linux/macOS/Windows; CUDA/WGPU/CPU).
Compatibility target. Checkpoint/quant artifacts v1; perf budget docs.
DoD. Meets P95 latency/footprint targets on commodity machines; accuracy degradation within agreed bounds.
________________________________________
Phase 10 — Release & Community v1
Purpose. Ship and sustain.
Deliverables. Reproducible builds with signed artifacts; changelogs; semver policy; security model & threat doc; plugin review baseline (deny by default); contributor docs, CODEOWNERS, good first issues.
DoD. Public release with artifacts; governance live; onboarding smooth.
________________________________________
Anti Rewrite Guarantees
•	Contracts first with conformance tests.
•	Additive evolution (SemVer discipline) for Protocol/Trace/Patch/Memory; breaks only at phase boundaries with shims/migrations.
•	Stable seams (traits) for tools and storage; implementation swaps don’t ripple.
•	Feature gates keep training extras and optional heads out of inference binaries.
•	Regression gates pin behavior to prevent backsliding.
________________________________________
Minimal Critical Path (what to build next)
1.	Phase 0 contracts + policy tests → tag v1.0.
2.	Phase 1 ledger + ingestion snapshot.
3.	Phase 2 model/training harness with stable inference API.
4.	Phase 3 orchestrator + daemon (dry run traces).
5.	Phase 4 tool adapters + indexer.
After Phase 5, you already have the real patch engine with safety guarantees; Phases 6–9 deepen capability without changing earlier commitments.
________________________________________
Vision Alignment Deltas (Spec v1.3 mapping)
•	End Product Vision → Phases 4–8: tools (cargo/RA/indexer/doc navigator), safe multi file diffs, editor presence, citations via anchors, transparent traces.
•	Training & Data → Phases 1–2: bytes only inputs, compiler triplets, curriculum S0→S3, provenance ledger; checkpoints & artifact formats.
•	Evaluation → Phase 7: gates for anchor fidelity, compile fix progression, trace validity, regression battery.
•	Interfaces & Schemas → Phase 0: Protocol/Trace/Patch/Memory schemas + validators.
•	Safety Rules → Phase 5: green checks before apply, auto revert, minimal diffs, anchors on all modifications.
•	Security & Privacy → Phases 3 & 8: daemon no network, plugin sandboxing, opt in telemetry (local only).
________________________________________
Phase Gates & Metrics Matrix
Phase	Primary gates
0	CI policy tests green; all traces validate; daemon boots with no network and logs policy flags.
1	xtask data:snapshot is deterministic; provenance ledger complete for all ingested corpora.
2	Training runs for Tiny/Base from the same code; inference API stable; checkpoints reload; baseline throughput documented.
3	100% traces parse; replayer deterministic across OS; abstention path exercised in tests.
4	Anchor fidelity ≥ 90% on explain tasks; RA/cargo probe success rates ≥ 98% on supported cases; index build succeeds on multi crate repos with time budget documented.
5	Compile fix ≥ 50% on single file slice; diff minimality ≤ K (default 20) with median ≤ 10; rollback success = 100% in fault injection tests.
6	Memory bundle round trip deterministic; store sizes within budget; planner consumes memory consistently.
7	Anchor fidelity ≥ 95%; compile fix ≥ 70% overall; test pass ≥ 60%; abstention quality ≥ 80% (low conf cases propose valid next probe); ECE ≤ 0.08 and H@τ ≤ 5%.
8	GUI/VSCode operate on local daemon only; deterministic replay; zero outbound network verified in tests.
9	Meet P95 latency & footprint budgets on commodity hardware; quantized (int8/int4) models degrade accuracy by ≤ 3% vs baseline.
10	Reproducible, signed release artifacts; security model sign off; contributor flow verified.
Numbers are targets that can be tuned, but they give us non hand wavy gates that align with the spec.
________________________________________
Optional Out of the Box Modules (fit the spec, zero RAG)
1.	Anchor Fabric: use anchors as long context stitch points (anchor chaining) to navigate/compose across files without retrieval vibes.
2.	TraceScript Verified Mode: a static checker that proves plan monotonicity and forbidden action absence; emits a small “witness” file per run.
3.	Patch Provenance Tokens: standard commit footer containing anchors, metrics (K, tests green), and a trace hash → auditability & rollback friendliness.
4.	Shadow CI Agent: read only agent runs on PRs to produce diagnoses and minimal patch proposals + traces; never applies; feeds Golden Traces.
5.	Golden Trace Exchange: contributor workflow + validator so the community can add curated traces safely (license clean, anchor rich).
6.	Confidence Calibration Head: explicit calibration head trained with Brier loss; exported calibration curves enforced in CI.
7.	Episodic Cache (TTL): small local key→value store keyed by (diag hash, crate, symbol pattern); speeds up repeat issues without “RAG”.
8.	Safety Envelope Simulator: property based tests that inject hostile diffs (path traversal, file clobbering) to prove the patch engine cannot harm the tree.
Where they land.
•	(2)(3)(8) → Phase 5; (4)(5) → Phase 7; (1)(7) → Phases 4–6; (6) → Phases 2–3.
________________________________________
What To Change Next (fast refinements)
•	Add abstention quality and calibration tests to Phase 7 now (scaffolding only).
•	Encode K = 20 (median ≤ 10) and τ_answer = 0.7 / τ_patch = 0.85 in policy/ defaults and CI.
•	Add a no network integration test in Phase 3 that tries outbound sockets and must fail.
AI Rust Programmer — Release Plan (v1 → v2 → v3)
Programmer only. No RAG. No chat CLI. Local first. Rusta Bridge is separate.
This reframes the roadmap as shippable versions instead of perfect phases. Each version has a clear scope, hard non goals, and acceptance gates. We evolve additively; breakage only at version boundaries with shims.
________________________________________
Non negotiables (all versions)
•	No network by default.
•	Explanations cite anchors.
•	Patches only after cargo check + tests pass; auto revert on regression.
•	Minimal diffs (default K=20; target median ≤ 10).
•	Traces for significant actions; validator must pass.
________________________________________
v1 — "Local Expert that Can Fix a File"
User value. Attach a Rust repo locally, get anchored explanations and safe fixes for common single file compiler errors.
Scope.
•	Daemon (no net) with structured commands: Explain, Diagnose, ProposePatch, Apply, Revert, ReplayTrace.
•	Protocol/Trace/Patch v0.x (additive); validators + conformance tests.
•	Byte LM (Tiny) in Burn with infer() API; enough to structure explanations.
•	Tool adapters: cargo check/test; RA probes (type_of, find_refs); Doc Navigator (std/Nomicon/RxE) with anchors.
•	Patch engine: diff generator, shrinker, apply+rollback with atomic guarantees.
•	Mini eval: 10 failing mini crates across error taxonomy; regression runner.
TraceServer daemon included; can replay traces offline; validator blocks malformed traces.
Gates.
•	Anchor fidelity ≥ 90% on explain.
•	Compile fix ≥ 50% on the single file slice.
•	Rollback = 100% in fault injection tests.
•	All traces validate; daemon blocks outbound sockets.
Non goals. Multi file refactors; plugins; GUI (CLI/IDE thin client optional).
________________________________________
v2 — "Refactor Safely and Remember"
User value. Tackle multi file changes with rollback; remember project structure & past work; operate from editor UI.
Scope.
•	Multi file patch planner with grouped diffs; cross file rollback.
•	Project Memory v1: sled/sqlite store, compaction/TTL, export/import Memory Bundles.
•	GUI + VSCode extension: inline anchors, diff preview, trace replay.
•	Explore Mode: bounded micro experiments; micro test synthesis when needed.
•	Eval expansion: more repos; taxonomy coverage targets.
TraceServer streams traces into editor UI; deterministic replay integrated with VSCode extension.
Gates.
•	Compile fix ≥ 70% overall on suite; test pass ≥ 60%.
•	Borrow checker slice ≥ 80%.
•	Deterministic memory bundle round trip; editor integration purely local.
Non goals. Planner learning/RL; plugin ecosystem.
________________________________________
v3 — "Fast, Portable, Extensible"
User value. Runs well on commodity hardware; quantized builds; clean plugin story.
Scope.
•	Performance & long context: sliding/strided windows; attention optimizations; concurrency.
•	Quantization: int8/int4 loaders for Tiny/Base; artifact formats v1.0.
•	Cross platform: Linux/macOS/Windows; CUDA/WGPU/CPU CI matrix.
•	Plugin ABI v1.0: sandboxed, deny by default; sample read only plugin.
TraceServer performance hardened; supports provenance export/import for Golden Trace Exchange.
Gates.
•	P95 latency/footprint budgets met on target hardware.
•	Quantized models degrade accuracy by ≤ 3% vs Base FP.
•	Plugin ABI conformance tests green; sample plugin validated.
Non goals. Rusta Bridge (separate plan), cutting edge model research beyond what’s needed to meet budgets.
________________________________________
How versions relate to the earlier phases
•	v1 ≈ Phases 0–5 + mini eval.
•	v2 ≈ Phases 6–8 (+ Explore Mode).
•	v3 ≈ Phases 9–10 (+ Plugin ABI).
________________________________________
