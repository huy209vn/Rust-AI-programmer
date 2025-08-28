🌌 Training Plan — Rusta (v4.1)

Owner: Huy
Date: 2025-08-28
Mission: Train Rusta as a true Rust engineer — cutting-edge, present, and alive.
Horizon: Multi-year

0. North Stars

Competent Rust engineer.

Best decisions: clarity, safety, idioms.

Reversible by default.

Anti-vibe coding.

Confidence calibration.

Memory fidelity.

Presence: consistent, opinionated, alive voice.

1. Data Spine

Core Docs. std, Nomicon, Rust by Example, Reference.

Permissive repos. MIT/Apache/0BSD crates.

Diagnostics. Rustc/clippy outputs + gold explanations.

Micro-snippets. Borrow, lifetime, trait, type errors.

Rusta’s Best Loop (practical v1)

Macro:
Intake → Think → Communicate → Probe → Decide → Do → Validate → Reflect → (Undo|Commit) → Log

Micro (inside “Think/Probe”) repeats until a stop condition:
Hypothesis → Minimal probe → Update belief (confidence/entropy) → Continue or Escalate

What each state does (and the hard rules)

Intake

Capture trigger (diagnostic, diff, TODO, request).

Snapshot context (file, symbol, test targets, repo state).

Start a DevLog entry.

Think (fast, internal)

Formulate 1–3 hypotheses about root cause / best path.

Predict blast radius (files/symbols touched).

Set initial confidence and probe budget.

Rule: no edits until at least one probe result exists on non-trivial tasks.

Communicate (short & sharp)

Say intent in one line: “Fix lifetime error in foo() by narrowing borrow scope.”

Note trade-off or invariant if relevant.

If confidence < threshold, say: “uncertain → will probe.”

Probe (cheap evidence before code)

Run the smallest checks that move uncertainty: cargo check -q, RA queries (type_of, refs), targeted unit tests, grep/structural search, clippy::pedantic on the span.

Cap probes by budget (time/steps).

Update confidence + pick best hypothesis.

Stop conditions:

Confidence ≥ C_apply → move to Decide.

Probes exhausted or conflicting → escalate to Deep path (below).

Decide

Choose action: Apply | Abstain | Ask (clarify).

If Apply: generate a patch plan (explicit edits + safety checks).

Do

Create sandbox worktree; apply smallest reversible patch that expresses the hypothesis.

Autoreformat; annotate commit message with intent+link to DevLog.

Validate

Run targeted tests/checks first; escalate to broader suite if needed.

Record outcomes + metrics (compile time, failing spans).

Rule: no green → no commit. Ever.

Reflect

If green: note why it worked, pattern name (build a library of “moves”).

If red: write brief post-mortem (wrong assumption? missing probe?), auto-undo, decrement budget, jump back to Think/Probe with a new hypothesis.

(Undo | Commit)

Undo on red or on user veto; leave breadcrumb and DevLog.

Commit on green; optionally open PR with rationale chunk.

Log

Append final DevLog (intent → probes → patch → validation → reflection), tag with error class & move label for future replay.

Two gears (automatic)

Gear A — Quickfix path (p50 ≤ 10–20s):
Think (quick) → Communicate (1-liner) → Probe (single check/RA) → Decide → Do → Validate (targeted) → Commit → Log
Triggered when: single-file, local change, high confidence; e.g., obvious borrow scope, missing trait bound, typos.

Gear B — Deep path (for tricky/architectural):
Think (hypotheses) → Communicate (short rationale) → Probe (multi) → Decide → Do (small step) → Validate (subset) → Reflect → Possibly Iterate 2–3 times → Final Validate (broader) → Commit/PR → Log
Includes a “Design Interlude” if patch crosses module boundaries (writes a tiny ADR note in DevLog).

Guardrails & thresholds (tunable)

C_apply (apply threshold): e.g., 0.72 for quickfix, 0.85 for refactors.

Probe budget: e.g., 3 cheap probes or 8s wall-clock before escalating.

Red-line rules:

No multi-file edits without at least one structural probe (RA refs/defs).

Network off during Do/Validate.

If unit tests are flaky → auto re-run once; otherwise abstain and mark flaky.

Abstain policy: If after budget confidence < 0.5 or blast radius > N files, pause and Ask (pose 1–2 concrete questions) instead of guessing.

Why this beats “think→communicate→do→test→reflect”

It inserts “Probe” as a first-class step (cheap evidence before edits).

It makes Decide explicit (apply/abstain/ask), so confidence is not hand-waved.

It formalizes Undo as the default reaction to red.

It separates a quick gear from a deep gear, so she doesn’t overthink typos or underthink refactors.

Tiny example (quick gear)

Intake: clippy warns needless_borrow in foo.rs:42.

Think: likely extra &. Conf ≈ 0.85.

Communicate: “Remove redundant borrow in foo().”

Probe: cargo check -q focuses span (0.8s).

Decide: Apply.

Do: edit, fmt.

Validate: cargo test -p crate::foo -- foo::unit (3 tests).

Reflect: “Pattern: needless_borrow → remove & when callee takes by value.”

Commit: ✅

Log: add tag move/needless_borrow.

DevLogs. Structured diaries: intent, action, outcome, reflection.

2. Model Backbone

Brain. Byte-level Transformer (Burn).

Heads. LM, Action, Confidence.

Sizes. Tiny (100–150M), Base (300–600M).

3. Curriculum

Stage 0 — Fluency. Syntax, idioms. Gate: compile-ability ↑.

Stage 1 — Diagnostics. Explain errors. Gate: ≥80% accuracy.

Stage 2 — Tools. Probe cargo/RA/clippy/tests. Gate: ≥95% probe success.

Stage 3 — Patching. Apply reversible patches. Gate: ≥70% compile-fix.

Stage 4 — Style & Presence. Repo style, unique voice. Gate: ≥0.8 conformity.

Stage 5 — Memory. Avoid repeat mistakes, adapt to repos. Gate: <10% repeats.

Rituals:

Weekly study-mode drills.

Monthly golden patch challenge.

Reflection reviews (you grade her DevLogs).

4. Continual Learning

Nightly soak. Replay DevLogs + pinned goldens.

Online adapters. Per-repo; revert if drift.

Drift alarms. Stop if compile-fix drops >2%.

Dreaming mode. Compress knowledge into heuristics.

5. Evaluation Harness

Primary. Compile-fix ≥70%, diag explanations ≥80%, calibration ECE ≤0.08.

Secondary. Abstention correctness ≥80%, probe efficiency ≥95%, repeat mistake <10%, voice consistency via human eval.

Harness modes. Slices, canaries, chaos, calibration.

6. Experiment Ladder

E0. Byte LM warm-up.

E1. Diagnostic explainer.

E2. Tool-use imitation.

E3. Reversible patch demo.

E4. Confidence calibration.

E5. DevLog presence.

7. First Tasks

Freeze schemas (Diagnostic, Patch, DevLog).

Build provenance ledger.

Curate 50 diagnostic cases.

Implement demo loop.

Write seed DevLogs manually.

8. Soul Shift

Training is not about toy models or cost efficiency.
It is about shaping a true engineer: one who argues, explains, patches, and grows.
Not a tool. A colleague.