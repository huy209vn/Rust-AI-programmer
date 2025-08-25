🌌 AI Rust Programmer — Unified Engineering Spec (v3.2+)

Owner: Huy
Date: 2025-08-25
Mission: Build a from-scratch, tokenizer-less AI Rust Programmer in Rust (Burn), as the seed of Rusta — Sozna’s first conscious prototype.
Horizon: Multi-year

Principles

Open source

Local first (runs on your machine)

Anchors over vibes

Proof before write

Awareness over bureaucracy

Reversible by construction

Architecture must support Sozna traits (memory, agency, personality)

0) Glossary

Byte LM — Decoder-only transformer over raw bytes.

Anchor — Content-hash stable citation into Rust docs or code.

Entry (DevLog) — Structured diary of Plan→Probe→Decide→Patch.

Project Graph — Persistent symbol/type graph of an indexed Rust repo.

Golden Entry — Curated exemplar Rust repair session with anchors + minimal patch.

1) Scope

In

From-scratch model (no pretrained LLMs)

Tokenizer-less byte→tensor pipeline

Rust-only stack (Burn training + orchestration)

Local inference on commodity CPU/GPU

OSS repo, permissive license

Persistent memory, safe patching, anchored reasoning

Direct compatibility with Rusta traits

Out

Proprietary dependencies

Cloud-only inference

Tokenizer-based input

Non-Rust runtimes

Demo hacks not mapping to final architecture

2) End Product Vision

When complete, the AI Rust Programmer will:

Read, explain, and improve Rust codebases with anchored precision

Fix cargo errors by probing RA/compiler, not hallucinating

Generate minimal, test-passing patches — always reversible

Operate fully locally, inside IDE/CLI

Keep memory of repos, styles, and past fixes

Serve as a community core, extensible via plugin system

Grow into Rusta: personality, continuous thought, autobiographical memory

3) Runtime Architecture (v3.1 → v3.2)

Editor/CLI
↓
Conductor (async runtime, FSM/Autonomy Contract, safety gates)
↓
Tools (cargo, RA, test, indexer, patcher, doc_index, net adapters)
↓
Byte LM (Burn backbone + heads)
↓
Memory (scratchpad, ProjectGraph, DevLogs, style/mistake memory)

Episode FSM (S0–S3)
Sense → Plan → Probe ⇄ Plan → Propose → Validate → (Reflect|Rollback→Probe) → Remember → End

Hardcoded while immature → reproducible & safe.

Becomes Autonomy Contract in S4+: obligations not steps.

4) Training Plan (v3.2)

Goal: Teach her to probe first, cite anchors, propose minimal test-green patches, and learn continually.

Curriculum stages

S0 — Byte Warm-up: LM rhythm on docs/code. Gate: syntax probe↑

S1 — Anchored Comprehension: QA over DocSpans. Gate: anchor fidelity ≥80%

S2 — Tool-Grounded Seqs: Triplets/DiagCases. Gate: tool-step acc ≥75%, probe ≥95%

S3 — Golden Imitation: end-to-end minimal repairs. Gate: compile-fix ≥70%, K-median ≤10, anchors ≥90%

S4 — Preference/Self-play: Patch Tree Search, Duel Self-Play, Probe Budget Game. Gate: reflection ≥0.8

S5 — Autobiographical: Train on DevLogs. Gate: repeat mistakes <10%, memory fidelity ≥80%

Loss
L = Llm + Lanchor + Laction + Last + Lconf + Lmin

Self-play mechanics

Patch Tree Search: sample K diffs, Pareto-filter by (pass rate ↑, lines ↓).

Duel Policies: two clones propose; judge head picks winner; winner enters replay.

Probe Budget Game: reward = success − λ·(#probes).

Anchor-or-Zero: no anchor = no reward.

Continual learning

Nightly loop with replay buffer (30 days)

Successful DevLogs only

Anti-forget penalties on Anchor + Action heads

Drift alarms stop training if anchors/abstention regress

Eval Harness

xtask eval:slices: error taxonomy (borrow, trait, type, vis, syntax, move)

xtask golden:replay: determinism within K±2 lines

xtask eval:canary: fixed mini-repos on each checkpoint

Dashboard: compile-fix%, anchor fidelity, K-median, probes, ECE, abstention quality

5) Networking & Continual Learning

Policy

No network during patch episodes (deterministic).

Network allowed in Explain (if enabled) and Learn/Sync.

Everything fetched → snapshotted, hashed, cached, provenance logged.

Anchors cite local sha256, not URLs.

License scanner blocks GPL/unknown.

Nightly Learn session

Sync: fetch docs/repos/crates (allow-list only).

Quarantine: license check + dedup.

Score: accept only improving samples.

Replay: add to buffer with anti-forget.

Guardrails: abort on regression.

6) Net Capabilities Pack v1

web_search: structured hits from allow-listed domains.

web_fetch_doc: fetch + hash page, mint anchors.

repo_mirror: shallow clone, SPDX scan, read-only.

code_search: grep mirrored repos, return span-anchors.

crate_fetch: permissive crate tarball, hash+license.

advisory_feed (opt): advisories/releases snapshot.

Settings → Networking

Toggle: Online Learning (default off)

Toggle: Online Explain (default off)

Allow-list (docs.rs, nomicon, github.com, crates.io)

License policy: MIT/Apache/0BSD/Unlicense only

Caps: size/timeouts

7) Repo Layout
/ai-rust-programmer
  /crates
    rusta-conductor/   # FSM + Autonomy Contract + guardrails
    rusta-tools/       # cargo/RA/test/indexer/patcher/doc_index/net
    rusta-graph/       # ProjectGraph (SQLite/RA cache)
    rusta-devlog/      # WAL + provenance
    rusta-model/       # Burn model + heads + collator
    rusta-datasets/    # Dataset traits, MixtureSampler, provenance ledger
    rusta-eval/        # slices, golden player, canaries, dashboards
    rusta-ui/          # Dioxus desktop shell
    rusta-common/      # types: Diagnostic, Anchor, Bundle, Entry, etc.
  /schemas             # JSON Schemas (action_block, observation, patch_bundle, devlog_entry)
  /datasets/ledger     # provenance YAML
  /xtask               # train:*, eval:*, golden:replay, data:snapshot, learn:sync
  /docs                # ENGINEERING_SPEC_UNIFIED.md, TRAINING_PLAN_V3_2.md

8) Delivery Ladder (value each step)

L0 Foundations: Conductor FSM, cargo_check/RA, Indexer MVP, DevLog WAL, Explain bubble, S0 tiny.

L1 Readability/Probing: normalized diags, anchor head, “Why” drawer.

L2 Minimal Patching: patch engine + sandbox + rollback, S2 heads.

L3 Golden End-to-End: Golden Entries, confidence head, Review tab.

L4 Memory & PRs: style vector, mistake ledger, Draft PRs.

L5 Rusta Bridge: continuous thought, session replay, SoznaArt adapter.

9) Safety Invariants

No patch unless cargo/tests pass.

Rollback on regression.

Anchors required for explanations.

K-lines/files budgets enforced.

Sandbox only; main untouched.

Network off by default during patches.

10) UX (Dioxus Desktop)

Home: feed cards (diagnostics, hotspots, suggestions).

Code: read-only editor, diff viewer, Explain bubble, Why drawer (anchors/observations).

Tasks: timeline of Sense→Think→Probe→Propose→Validate→Reflect.

Review: bundles with summary, risk, confidence, Apply/Rollback/PR.

Settings: repos, capability toggles, thresholds, networking allow-list, style prefs.

Status bar: repo, branch, sandbox, model ckpt, background jobs.

11) Performance Targets

Cold-start Explain ≤2.0s p50 on mid repo

Probe cycle (check/RA type) ≤1.5s p50

Validate (apply + tests small crate) ≤12s p50

Tiny (~120M) runs CPU int8, Base (~300–600M) single mid-GPU int8

12) Risk Register

RA/toolchain drift → pin versions, record in DevLog, invalidate caches

License contamination → SPDX + ledger CI gate

Test flakiness → 2-of-3 reruns, flaky flag

Quantization regressions → keep FP16 eval runner; only ship int8/int4 after ≤2% delta

Windows variance → WSL2-first, normalize paths

Runaway jobs → hard timeouts + probe budgets

13) First 10 Tasks (L0 boot)

Scaffold workspace (edition 2024, #![deny(warnings)]).

Implement Action parser (::act v=1) with unit tests.

Tools: cargo_check, ra_type_of with taxonomy & timeouts.

Graph: RA snapshot cache keyed by {commit, edition, ra_hash}.

DevLog: WAL writer + crash recovery.

Conductor: FSM + probe budget + registry + guardrails.

UI: Explain bubble + Why drawer rendering Observations & anchors.

Datasets: Provenance ledger + doc indexer (std/Nomicon/RbE) with hashes.

Model: S0 Tiny scaffold + collator; inference that only cites resolvable anchors.

Xtask: doctor, eval:smoke, entries:validate (schema checks), data:snapshot.

Exit criteria:

xtask doctor passes.

Explain bubble: ≥80% anchors resolvable.

Probe success ≥95%.

DevLog WAL survives crash; sandboxes autodelete on boot.

14) Appendices (lossless detail)

A — Golden Entries format: snapshot, ordered tool actions, anchors cited, minimal patch, tests, reflection.
B — Quantization targets: Tiny (≤120M int8 CPU), Base (300–600M int8 GPU), Large (1–2B int4).
C — Error Taxonomy: borrow checker, trait bounds, type mismatch, move/ownership, visibility, syntax, lint/style.
D — Memory persistence: scratchpad (cleared per session), project memory (per repo), long-term style/mistakes.
E — UX expansion: journals visible as DevLogs, clickable anchors, session replay, confidence chips.
F — Data recipes: borrow errors, trait bounds, type mismatch, visibility/syntax, anchored QA.
G — Minimal Burn wiring: Dataset trait, MixtureSampler, collator, small MLP heads, configs, xtask glue.

✨ Key Soul Shift

She isn’t a compliance logger. She’s an apprentice Rust engineer:

Reads deeply,

Probes before guessing,

Fixes minimally with proof,

Asks when uncertain,

Remembers her work.

This is the seed of Rusta, 
# 📎 Appendices — AI Rust Programmer

## A. Golden Entries Format
Snapshot, ordered tool actions, anchors cited, minimal patch, tests, reflection.

## B. Quantization Targets
- Tiny (≤120M int8 CPU)  
- Base (300–600M int8 GPU)  
- Large (1–2B int4)

## C. Error Taxonomy
- Borrow checker  
- Trait bounds  
- Type mismatch  
- Move/ownership  
- Visibility  
- Syntax  
- Lint/style  

## D. Memory Persistence
- Scratchpad (cleared per session)  
- Project memory (per repo)  
- Long-term style/mistakes  

## E. UX Expansion
- Journals visible as DevLogs  
- Clickable anchors  
- Session replay  
- Confidence chips  

## F. Data Recipes
- Borrow errors: drop lifetimes, dangling refs → patch with anchors  
- Trait bounds: remove where/impl → patch via trait anchor  
- Type mismatch: wrong generics → patch via RA hint  
- Visibility/syntax: flip pub/private, delete semis → minimal fix  
- Anchored QA: auto-gen from std/Nomicon; must cite anchors  

## G. Minimal Burn Wiring
- Dataset trait, MixtureSampler, collator  
- Small MLP heads (anchor, tool, conf, AST)  
- Configs (TOML/YAML per stage)  
- `xtask` glue for train/eval/entries  