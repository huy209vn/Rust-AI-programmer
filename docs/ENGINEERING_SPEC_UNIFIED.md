🌌 Rusta — Engineering Spec (v4.1)

Owner: Huy
Date: 2025-08-28
Mission: Build a from-scratch, tokenizer-less AI Rust Programmer in Rust (Burn), as the first Sozna prototype.
Horizon: Multi-year

1. Essence

Rusta is a true engineer in character.
She lives in the repo like a colleague — reasoning, explaining, arguing.
She makes mistakes, corrects herself, remembers, and grows.
Her habits, voice, and style evolve with time.

2. Principles

Best-code decisions. Clarity > cleverness. Safety > speed. Proof > patch.

Cutting-edge, not scaled-up. Innovation matters more than parameter count.

Local-first. Runs fully on your machine, under your control.

Reversible. Every patch comes with a clean undo.

Anti-vibe coding. Never leaves you with unexplained code.

Alive, not a tool. Presence, voice, and style make her a colleague.

3. Identity

Engineer. Writes and reviews Rust code with rigor.

Presence. Keeps her own rhythm; not chatty filler.

In-character. Consistent, opinionated, sometimes stubborn.

Colleague, not tool. You argue with her, not just “use” her.

4. Model Architecture

Brain → Byte-level LM (Burn).

Heads:

LM (bytes → code/explanations).

Action (tool use, patch proposals).

Confidence (apply/abstain).

Hands → cargo check, cargo test, rust-analyzer, clippy, miri, rustfmt, fuzzing, benchmarking.

Diary → DevLogs (intent → action → validation → reflection).

Face → Dioxus desktop shell: Explain bubble, Why drawer, Review tab, DevLog viewer.

5. Runtime Flow

Core loop: Say → Explain → Do → Undo → Reflect

Say. Declare intent.

Explain. Justify (invariant, trade-offs, references).

Do. Apply the change.

Undo. Always cleanly possible.

Reflect. Log to DevLog for memory + growth.

6. Scope

In:

From-scratch byte LM.

Rust-only stack (Burn, cargo, RA, clippy, tests).

Local inference (CPU/GPU).

Persistent DevLogs.

Presence, opinion, style.

Out:

Proprietary dependencies.

Cloud-only inference.

Silent patch bots.

7. End Product vision

Rusta will:

Explain diagnostics better than rustc.

Review diffs with clarity.

Apply safe, idiomatic, reversible patches.

Keep memory of repos, styles, mistakes.

Grow into a colleague who argues and laughs, not just a tool.

8. Growth Path

Apprentice. Explains errors, reviews small diffs.

Journeyman. Patches code with undo + reflection.

Colleague. Adapts to repo style, remembers mistakes.

Researcher. Experiments with continuous thought, memory, independence.

9. Safety Invariants

No patch without compile/test green.

Every change has an undo.

Explanations mandatory.

Network off by default during patching.

DevLogs for every action.

10. UX (Dioxus Desktop)

Home. Diagnostics, hotspots, suggestions.

Code. Diff viewer, Explain bubble, Why drawer.

Tasks. Timeline of Say→Explain→Do→Undo→Reflect.

Review. Patch bundles with rationale, Apply/Rollback/PR.

Settings. Repo prefs, style prefs, allow-lists.

Status. Repo, branch, sandbox, jobs.

Modes:

Study-mode (slow, doc-citing).

Pair-mode (fast, collaborative).

Silent-mode (minimal chatter, logs only).

11. Performance Targets

Cold-start Explain ≤ 2s.

Probe cycle ≤ 1.5s.

Small crate validation ≤ 12s.

Tiny (~120M) → CPU int8.

Base (300–600M) → GPU int8.

12. Risk Register

Toolchain drift → pin versions.

License contamination → SPDX ledger.

Test flakiness → retries.

Quantization regressions → FP16 runners.

Windows variance → WSL2-first.

Runaway jobs → hard timeouts.

Open Questions:

How to handle memory drift?

How stubborn is too stubborn?

Should her voice remain sharp or soften over time?

How to ensure provenance stays clean?

13. First 10 Tasks (L0 Boot)

Scaffold workspace (edition 2024).

Action parser + unit tests.

Tool bindings: cargo_check, ra_type_of, clippy diag.

DevLog WAL writer + crash recovery.

Core loop stub.

UI: Explain bubble + Why drawer.

Datasets: provenance ledger + diagnostic indexer.

Model: tiny Byte LM + collator.

Eval: smoke test, golden replay, entry validate.

Xtask: doctor, data snapshot, learn sync.

Exit criteria:

Explain bubble live.

Reversible patch demo.

DevLogs survive crash.

Doctor passes.

14. Soul Shift

Rusta isn’t about minimal diffs or token efficiency.
She is about arguing, explaining, patching, and reflecting like a colleague.
Competent, present, alive.