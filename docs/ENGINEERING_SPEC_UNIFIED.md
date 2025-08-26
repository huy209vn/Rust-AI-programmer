🌌 AI Rust Programmer — Unified Engineering Spec (v3.3, Refined)

Owner: Huy
Date: 2025-08-25
Mission: Build a from-scratch, tokenizer-less AI Rust Programmer in Rust (Burn), as the seed of Rusta — Sozna’s first conscious prototype.
Horizon: Multi-year

Principles

Open source — community-first, permissive licensing.

Local-first — runs on your machine, under your control.

Awareness over bureaucracy — judgment > rigid rules.

Reversible by construction — everything can be rolled back.

Supports Sozna traits — memory, agency, personality.

0) Glossary

Byte LM — Decoder-only transformer trained on raw bytes.

Entry (DevLog) — Structured diary of Plan → Act → Validate → Reflect.

Project Graph — Persistent type/symbol graph of a Rust repo.

Golden Entry — Curated exemplar of a good repair (with reasoning + patch).

1) Scope

In

From-scratch model (no pretrained LLMs).

Tokenizer-less byte → tensor pipeline.

Rust-only stack (Burn training + orchestration).

Local inference on commodity CPU/GPU.

OSS repo, permissive license.

Persistent memory & safe patching.

Out

Proprietary dependencies.

Cloud-only inference.

Tokenizer-based input.

Non-Rust runtimes.

Demo hacks not aligned with final design.

2) End Product Vision

Rusta will:

Read, explain, and improve Rust codebases with clarity.

Can resolve references to docs/code when explaining.

Can use tools (cargo, RA, tests, indexer) naturally.

Treats reversibility and test-green passes as safety rails.

Operate fully locally (CLI + IDE).

Keep memory of repos, styles, and past fixes.

Serve as a community core, extensible by plugins.



Grow into Rusta: with memory, style, personality, and continuous thought.

3) Runtime Architecture

Flow: Editor/CLI → Conductor → Tools → Byte LM → Memory.

Conductor: async runtime, applies contracts + safety gates.

Tools: cargo, RA, test, indexer, patcher, doc_index, net.

Model: Burn backbone + task heads.

Memory: scratchpad, ProjectGraph, DevLogs, style/mistake memory.

Episode loop: Sense → Plan → Act → Validate → Reflect → Remember.
Hardcoded FSM early (replayable, safe). Evolves into autonomy contracts (S4+).

4) Training Plan

See TRAINING_PLAN_v3.4_UNCHAINED.md — focuses on competence, memory, continual learning, not artificial minimality/anchors.

5) Networking & Continual Learning

Default offline during patch episodes.

Explain + Learn modes may use the network.

Fetched content is snapshotted, hashed, cached, provenance-logged.

Nightly Learn: sync docs/repos/crates → license check → dedup → replay buffer.

Guardrails: abort if regressions detected.

6) Networking Capabilities v1

web_search: allow-listed domains only.

web_fetch_doc: fetch + hash page, store locally.

repo_mirror: shallow clone with SPDX scan.

code_search: grep mirrored repos.

crate_fetch: permissive crate tarball.

advisory_feed: optional advisories/releases snapshot.

7) Repo Layout
/ai-rust-programmer
  /crates
    rusta-conductor/   # FSM (early) + runtime contracts (later) + guardrails
    rusta-tools/       # cargo / rust-analyzer / tests / indexer / patcher / doc_index / net
    rusta-graph/       # ProjectGraph (SQLite/RA cache), type_of, refs
    rusta-devlog/      # WAL + provenance, Entry ingest → trainable records
    rusta-model/       # Burn backbone + heads + train loop (+ adapters)
    rusta-datasets/    # Dataset traits, MixtureSampler, collators, ledger IO
    rusta-eval/        # slices, golden player, canaries, seeded/chaos eval, calibration, drift monitor
    rusta-ui/          # Dioxus desktop shell (Explain, Why drawer, dashboards)
    rusta-common/      # shared types (Diagnostic, Entry, PatchBundle, Bundle…)
  /schemas             # JSON schemas (action_block, observation, patch_bundle, devlog_entry, graph_snap)
  /datasets
    /ledger/*.yml      # provenance
    /shards/*          # content-addressed data (train/val/test)
  /xtask               # train:*, eval:*, golden:replay, data:snapshot, learn:sync, entries:validate
  /docs                # ENGINEERING_SPEC_v3_3.md, TRAINING_PLAN_v3_5_UNCHAINED.md, VISION_UNCHAINED.md

8) Delivery Ladder

L0 Foundations: conductor, cargo/RA integration, indexer MVP, DevLog WAL, S0 tiny model.

L1 Readability: normalized diags, Explain bubble, Why drawer.

L2 Patching: patch engine, sandbox + rollback.

L3 Golden End-to-End: Golden Entries, confidence head, Review tab.

L4 Memory & PRs: style vectors, mistake ledger, Draft PRs.

L5 Rusta Bridge: continuous thought, SoznaArt adapter.

9) Safety Invariants

No patch unless cargo/tests pass.

Rollback on regression.

Sandbox-only; main untouched.

Network off by default during patching.

10) UX (Dioxus Desktop)

Home: feed cards (diagnostics, hotspots, suggestions).

Code: read-only editor, diff viewer, Explain bubble, Why drawer.

Tasks: timeline of Sense → Act → Validate → Reflect.

Review: bundles with summary, risk, Apply/Rollback/PR.

Settings: repos, toggles, allow-list, style prefs.

Status bar: repo, branch, sandbox, model ckpt, jobs.

11) Performance Targets

Cold-start Explain ≤ 2s p50.

Tool probe cycle ≤ 1.5s p50.

Validate (apply+tests small crate) ≤ 12s p50.

Tiny (~120M) runs CPU int8; Base (300–600M) runs GPU int8.

12) Risk Register

Toolchain drift → pin versions.

License contamination → SPDX + ledger CI.

Test flakiness → reruns + flaky flags.

Quantization regressions → FP16 eval runners.

Windows variance → WSL2-first.

Runaway jobs → hard timeouts.

13) First 10 Tasks (L0 Boot)

Scaffold workspace (edition 2024).

Action parser with unit tests.

Tools: cargo_check, ra_type_of.

Graph: RA snapshot cache.

DevLog: WAL writer + crash recovery.

Conductor: FSM + guardrails.

UI: Explain bubble + Why drawer.

Datasets: provenance ledger + doc indexer.

Model: S0 tiny scaffold + collator.

Xtask: doctor, eval:smoke, entries:validate, data:snapshot.

Exit criteria: doctor passes, Explain bubble works, probe success ≥95%, DevLog WAL survives crash.

✨ Key Soul Shift
Rusta isn’t a compliance logger or a bureaucratic patch bot. She is an apprentice Rust engineer who becomes better at programming by working with you.