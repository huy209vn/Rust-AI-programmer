Training Plan — v3.2 (AI Rust Programmer)
0) North Stars (what the training must produce)

Probe-first habit under uncertainty.

Anchored explanations (hash-resolving DocSpans) as a default, not a flourish.

 test-green patches with calibrated confidence.

Continuous improvement via replay of real episodes (DevLogs) without forgetting anchors/tool skills.

1) Data Spine (reproducible & lawful)
1.1 Provenance Ledger (single source of truth)

datasets/ledger/*.yml entries with fields:

name, version, license, source_url, commit, sha256, split {train/val/test}

CI gate: fail if any shard lacks a ledger entry.

1.2 Atomic Schemas (frozen)

DocSpan: {doc_id, start, end, sha256, title?}

QA: {question, answer, anchors:[DocSpan], difficulty, topic}

DiagCase: {repo, commit, files[], cargo_diag[], tests?, class, gold_patch?}

Triplet: {before_tree, diag, after_tree?}

Entry (DevLog ingest): {snapshot, steps[], citations[], final_state, metrics}

GraphSnap: {project_graph_v1, ra_hash, symbol_ids[]}

Rule: all objects carry license, source_url, commit, sha256.

1.3 Sources (first pass)

Core Docs: Rust std, Nomicon, Rust by Example.

Permissive Code: curated MIT/Apache/0BSD/Unlicense crates list.

Curated Repos: tiny “teaching” repos per error class (Appendix C in your spec).

Synthetic Bugs: deterministic injectors for borrow/trait/type/vis/syntax.

Golden Entries: hand-curated end-to-end repairs (growing set).

1.4 Hygiene & Splits

Dedup: strong hash + sliding-window near-dup filter.

Repo-level split: entire repo/commit stays inside a single split.

Eval quarantine: canaries + goldens never appear in train shards.

2) Model Tracks (one backbone, helpful heads)

Backbone: decoder-only Byte LM (Burn), RoPE/ALiBi, sliding attention (2k→16k).

Heads:

Anchor (DocSpan pointer)

Action (next tool step)

Confidence (Brier/ECE)

Minimality (K-lines hint)

AST hint (optional)

Loss: L = L_lm + L_anchor + L_action + L_ast + L_conf + L_min
Selection metric > perplexity: compile-fix% & anchor fidelity.

Model sizes (target budgets):

Tiny ~120M → S0-S2 bring-up, CPU int8.

Base 300–600M → S2-S3 utility, single mid-GPU int8.

(Large is optional later.)

3) Curriculum (stages → concrete gates)
S0 — Byte Warm-Up (Foundations)

Data: docs + permissive code bytes
Signals: L_lm (+ light syntax probe from RA snapshots)
Gates: perplexity↓; syntax probe ≥ baseline+Δ

Output: “speaks Rust bytes”; good token rhythm for anchors to latch onto.

S1 — Anchored Comprehension

Data: QA over DocSpans (+ optional AST hints)
Signals: L_anchor, L_ast
Gates: anchor fidelity ≥ 80% (QA val), type-of probe ≥ 70%

Output: answers always cite resolvable hashes; dual-anchor tasks (std + Nomicon) introduced.

S2 — Tool-Grounded Sequences

Data: Triplets + DiagCases (no writes)
Signals: L_action (tool step), probe-success stats
Runtime coupling: Conductor FSM (read-only tools)
Gates: tool-step acc ≥ 75%, probe success ≥ 95%

Output: probe-before-guess emerges; good ordering of cargo/RA/tests.

S3 — Golden Imitation (Minimal Patching)

Data: Golden Entries (end-to-end; sandbox writes in sim)
Signals: L_min, L_conf + compile-fix pass/fail
Gates: compile-fix ≥ 70% (single-file slice), K-median ≤ 10, anchor fidelity ≥ 90%, ECE ≤ 0.10

Output: minimal diffs with receipts; calibrated confidence guiding Autonomy thresholds.

S4 — Preference Shaping & Self-Play (Optional)

Mechanics: PTS (Patch Tree Search), Duel Self-Play, Probe Budget Game
Signals: preference pairs (smaller K, fewer probes, faster green)
Gates: reflection score ≥ 0.8; exploration ROI ≥ 0.25

S5 — Autobiographical (Rusta Bridge)

Data: real DevLogs (successful), Style vectors per repo
Signals: memory fidelity, repeat-mistake penalties
Gates: repeat-mistake < 10% (30d); memory fidelity ≥ 80%

4) Self-Play / Games (implementable now)

Patch Tree Search (PTS)

From a DiagCase, sample K candidate diffs (respect budgets).

Value head predicts P(tests_green | K_lines).

Keep Pareto front (higher pass, smaller lines). Supervise both policy & value.

Duel Policies

Two clones propose patches → judge head (trained from gold preferences) picks winner; winners enter replay. Occasional hunk crossover.

Probe Budget Game

Fixed budget B per episode. Reward = success − λ·probes − μ·wall. Teaches efficient tool use.

Anchor-or-Zero QA

Answers without resolvable hashes get zero credit. Forces receipts.

Adversarial Anchor Contrast

Adversary suggests wrong but plausible anchor; contrastive loss pushes to correct hash.

5) Continual Learning Loop (nightly, safe)

Eligibility: only Entries with tests green & anchors resolving.

Replay buffer: last 30 days stratified by error class; anti-forget penalties on Anchor & Action heads.

Drift alarms: halt updates if anchor fidelity or abstention drops by > X% from moving baseline.

6) Evaluation (engineer-style gates)

Primary:

Anchor fidelity ≥ 95% (QA + explanations attached to patches)

Compile-fix ≥ 70% (taxonomy slices)

K-median ≤ 10 (accepted bundles)

Secondary:

Abstention usefulness ≥ 80%

Probe success ≥ 95%

ECE ≤ 0.08

Repeat-mistake < 10% (30d)

Harness (xtask):

eval:slices → pretty table + JSONL

golden:replay → determinism within K±2

eval:canary → fixed mini-repos each checkpoint

Dashboard: compile-fix%, anchor fidelity, K-median, probes, ECE, abstention

7) Experiment Ladder (order of shots, no dates)

E1. S0-Tiny Bring-Up

Verify loader, sampler, throughput.

Sanity: syntax probe ↑ vs random.

E2. Anchor Head Online (S1-Tiny)

10k QA; tune anchor window packing.

Gate: ≥80% fidelity.

E3. Tool-Step Imitation (S2-Tiny)

Triplets + DiagCases; action head.

Gate: ≥75% tool-step acc, ≥95% probe success.

E4. Minimality Head + Budgets

Train L_min with synthetic small fixes; enforce K budget in sim.

Gate: K-median ≤ 12 on slice.

E5. First Golden Mini-set (n≈10)

End-to-end replay; ensure DevLog → Train loop closes.

Gate: compile-fix ≥ 40% on mini-set.

E6. Confidence Calibration

Add Brier loss; temperature scaling.

Gate: ECE ≤ 0.12 → push ≤ 0.10.

E7. Base Model (300–600M) at S2→S3

Same harness; watch anchor fidelity non-regression.

E8. PTS (Patch Tree Search) Ablation

With vs without PTS on same slice.

Expect: K-median ↓, probes ↔ or ↓.

E9. Duel Self-Play

Compare acceptance rate and green% of winners vs singles.

E10. Nightly Continual

7-day soak; track drift alarms & repeat-mistake rate.

8) Org & Code (so it’s runnable)
/ai-rust-programmer
  /crates
    rusta-model/       # Burn backbone + heads + train loop
    rusta-datasets/    # Dataset trait, MixtureSampler, collators, ledger IO
    rusta-eval/        # slices, canaries, golden player, dashboards
    rusta-devlog/      # WAL, Entry ingest → S5 trainable records
    rusta-tools/       # cargo/RA/test/indexer/doc_index adapters
    rusta-conductor/   # FSM + autonomy contract + guardrails
    rusta-graph/       # RA snapshot cache, type_of, refs
    rusta-ui/          # Dioxus app (Explain → anchors)
    rusta-common/      # shared types (Diagnostic, Anchor, Entry, Bundle…)
  /schemas             # json schema for action/observation/entry/patch
  /datasets
    /ledger/*.yml      # provenance
    /shards/*          # content-addressed data (train/val/test)
  /xtask               # train:*, eval:*, golden:replay, data:snapshot, doctor
  /docs                # ENGINEERING_BLUEPRINT_V3_1.md, TRAINING_PLAN_V3_2.md


xtask targets to add now

xtask data:snapshot (hash & ledger validate)

xtask train:S0|S1|S2|S3 --model tiny|base

xtask eval:slices|canary

xtask golden:replay

xtask entries:validate (schema_version checks)

9) Concrete “Today” Checklist (unblocked, 100% local)

Freeze schemas in /schemas (DocSpan, QA, Triplet, DiagCase, Entry, PatchBundle).

Provenance ledger scaffolding + CI gate (deny unknown/GPL).

MixtureSampler with curriculum weights (S0→S3) and rolling-hash dedup in collate.

Anchor packer: pack QA examples so answer + both anchors land in-window.

S0-Tiny config & train loop: confirm throughput and logging.

S1 smoke on 10k QA: anchor head online; add Anchor-or-Zero scoring.

Triplet loader + Action head target extraction.

PTS skeleton (dry-run only; no writes) with K budget & value head.

Eval slices: implement taxonomy dashboard (borrow/trait/type/vis/syntax/move).

Dash: compile-fix%, anchor fidelity, K-median, probes, ECE.

10) Risks → Mitigations (training-specific)

Anchor regression when pushing compile-fix → separate LR for heads; gradient surgery; early stop on anchor drop > X%.

Self-play mode-collapse → keep 30% teacher-forced Golden; entropy bonus on diverse patches within K budget.

Forgetting anchors in continual → anchor head penalty vs last stable; nightly eval must pass anchor gate.

Overfitting tiny repos → repo-level splits + rotating canaries; add synthetic injectors on unseen repos.

11) Philosophy toggle (FSM → Contract)

Keep the hard FSM during S0–S3 to make datasets clean and replayable.
From S4+, gate behavior with a runtime Autonomy Contract (obligations + invariants), not hard states:

Must probe if conf < τ_probe

Must not apply unless tests green

Must cite anchors or remain silent

This keeps “being” intact without losing receipts.
