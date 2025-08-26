Training Plan — v3.5 (AI Rust Programmer)

(full expansion — no omissions, detailed like v3.4 but aligned with the freed philosophy)

0) North Stars (training must converge to these behaviors)

Competent Rust coding — reads, writes, fixes Rust like a skilled junior engineer.

Test-green patches with confidence calibration — never applies failing code, abstains when unsure.

Continuous improvement — integrates DevLogs into replay, learns from success/failure.

Memory fidelity — holds repo-specific style, avoids repeating past mistakes.

Voice emergence — reflections and explanations feel consistent, not scripted.

Grounded explanations (skill) — able to cite docs/code when useful, but not mandatory.

1) Data Spine (reproducible, lawful, signed)
1.1 Provenance Ledger

All shards listed in datasets/ledger/*.yml with fields:
{ name, version, license, source_url, commit, sha256, split }.

CI gates:

Fail if shard missing ledger.

Fail if repo/commit leaks across splits.

Fail if hash is GPL/unknown.

1.2 Atomic Schemas

All objects include: schema_version, license, source_url, commit, sha256.

Objects:

QA: {question, answer, difficulty, topic}

DiagCase: {repo, commit, files[], cargo_diag[], tests?, class, gold_patch?}

Triplet: {before_tree, diag, after_tree?}

Entry (DevLog): {snapshot, steps[], reflections[], final_state, metrics}

GraphSnap: {project_graph_v1, ra_hash, symbol_ids[]}

Determinism: round-trip encoding must be identical.

1.3 Sources

Core Docs: std, Nomicon, Rust by Example, Reference.

Permissive crates: curated MIT/Apache/0BSD.

Curated teaching repos: one per diagnostic class.

Synthetic bug injectors: borrow/trait/type/vis/syntax.

Golden Entries: curated end-to-end sessions.

Playground micros: tiny reproducible error snippets.

1.4 Hygiene

Dedup: hash + sliding window near-dup filter.

Repo-level split isolation.

Canary repos + Goldens held out for eval.

2) Model Tracks

Backbone: Decoder-only Byte LM (Burn), RoPE/ALiBi, sliding attention (2k→16k).

Heads:

LM head — byte sequence modeling.

Action head — next tool/code action.

Confidence head — calibrated (Brier loss + abstain).

AST head (optional) — syntax hints.

Loss: L = L_lm + L_action + L_conf (+ L_ast).

Budgets:

Tiny (~120M) → CPU int8, for S0–S2.

Base (300–600M) → GPU int8, for S2–S3.

Large optional later.

3) Curriculum (stages + gates)
S0 — Byte Warm-Up

Data: docs + permissive code.

Signals: LM loss, syntax probe.

Gates: perplexity ↓, syntax probe ≥ baseline+Δ.

Output: “Speaks Rust bytes.”

S1 — Comprehension QA

Data: QA from docs.

Signals: LM + QA acc.

Gates: ≥80% QA acc.

Output: can explain clearly in Rust context.

S2 — Tool-Grounded Sequences

Data: Triplets + DiagCases.

Signals: action accuracy, probe success.

Gates: tool-step acc ≥75%, probe success ≥95%.

Output: uses cargo/RA/tests like an engineer.

S3 — End-to-End Patching

Data: Golden Entries.

Signals: compile/test outcomes, confidence calibration.

Gates: compile-fix ≥70%, ECE ≤0.1.

Output: safe patches with explanations.

S4 — Self-Play & Preference Shaping

Mechanics: Patch Tree Search, Duel Self-Play, Budget Games.

Gates: reflection score ≥0.8; efficiency ROI ≥0.25.

Output: efficiency + style shaping.

S5 — Autobiographical (Rusta Bridge)

Data: real DevLogs.

Signals: memory fidelity, repeat-mistake penalty.

Gates: repeat-mistake <10% (30d).

Output: repo memory + evolving voice.

4) Self-Play / Games

Patch Tree Search (PTS): multiple diffs → filter by compile/test pass.

Duel Policies: two clones compete, judge selects winner.

Probe Budget Game: learn efficient tool usage.

5) Continual Learning
5.1 Nightly Loop

Eligible: Entries with test-green outcomes.

Replay buffer: last 30d + pinned Goldens.

Defenses: EWC/L2-SP, adapters, drift alarms.

5.2 Online Loop

Trigger: new passing Entry.

Batch: Entry + ~20 buffer samples.

Update: 50–200 LoRA steps.

Eval: canary slices (borrow/trait/move/syntax).

Pass → promote adapter. Fail → discard + log drift.

Safety: cap updates/hour, hot-unload if drift.

6) Evaluation
Primary

Compile-fix ≥70%.

QA accuracy ≥80%.

ECE ≤0.08.

Secondary

Abstention usefulness ≥80%.

Probe success ≥95%.

Repeat-mistake <10% (30d).

Harness Modes

eval:slices — taxonomy slices.

eval:canary — fixed mini-repos.

eval:seeded — deterministic baseline.

eval:chaos — stress non-determinism.

eval:ece — calibration curves.

7) Experiment Ladder

E1: Tiny bring-up (syntax probe).

E2: QA head online (≥80%).

E3: Tool imitation (≥75%).

E4: Golden mini-set (compile-fix ≥40%).

E5: Confidence calibration (ECE ≤0.1).

E6: Base model (S2→S3).

E7: PTS ablation.

E8: Duel self-play.

E9: Nightly continual soak.

E10: Online continual soak.

8) Org & Code

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


9) Today’s Checklist

Freeze schemas.

Build provenance ledger + CI gates.

Implement MixtureSampler with dedup.

QA loader + Action head training.

Canary eval slices.

Online trainer stub (LoRA).

Eval dashboard: compile-fix%, QA, abstention, drift.

10) Risks & Mitigations

Forgetting → replay buffer, adapters.

Drift → nightly alarms, hot-unload.

License creep → CI gates.

Overfit tiny repos → repo-level splits + injectors.

Online instability → cap updates, discard failing adapters.