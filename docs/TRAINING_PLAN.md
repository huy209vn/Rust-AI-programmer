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

10) Rules (non-negotiable)

Permissive only (MIT/Apache-2.0/BSD/ISC; CC-BY/CC-BY-SA with attribution; no GPL for base pretrain unless you isolate it).

License ledger per file and per derived sample; keep SPDX tags and source URLs.

No contamination from proprietary sources.

Dedup & decontam (near-dup + exact hash; drop boilerplate, vendored copies).

1) Canonical Rust “Canon” (small, high value)

Purpose: ground Rusta’s voice, invariants, idioms, and semantics.

The Rust Book, Nomicon, Reference (dual MIT/Apache-2.0). Use for supervised pairs: concept → example → explanation; and as citations in study-mode. 
GitHub
+3
Rust
+3
Rust Documentation
+3

RFCs corpus (design rationale, trade-off language). Track RFC status + decisions for retrieval. (Licensing notes exist; community intent is MIT/Apache style—record provenance carefully.) 
GitHub
+1

Outcome: ~2–5 GB text. This is your gold set for design reasoning and teach-back.

2) Clean Rust Codebase (permissive)

Purpose: idioms, patterns, crates ecosystem, real-world APIs.

BigCode The Stack v2 → filter to Rust and permissive licenses only (MIT/Apache/BSD/ISC). Use BigCode’s tooling to reproduce license filters. 
Hugging Face
+2
GitHub
+2

crates.io metadata dumps (names, versions, licenses, deps) to steer sampling, plus crate license verification. 
Crates.io
+1

Outcome: 50–200 GB code after filtering/dedup. Curate a Top-N crates slice (by downloads & stars) for “repo-scale” tasks.

3) Diagnostics & Fixes (self-generated)

Purpose: make Rusta explain rustc/clippy and propose safe, reversible patches.

Compile & test at scale: cargo check/test across curated crates; capture rustc and clippy diagnostics, spans, and suggested fixes.

Store (code_snippet, error_json, explanation, minimal_fix_diff, test_outcome) tuples.

Include miri, rustfmt diffs, fuzz regressions where available.

This becomes your biggest engineer-supervision dataset (not just code).

Outcome: 5–20M examples over time (you can start with 100k–500k).

4) Patch Pairs from the wild (before/after)

Purpose: learn review voice, commit discipline, and safe refactors.

From GH Archive / GitHub API on Rust repos: collect pull requests, commits, diffs, messages, linked CI results. Build (before_tree, diff, after_tree, tests, CI_status, PR_review_comments). 
gharchive.org
+1

Focus on: bug fixes, API migrations, deprecation removals, safety fixes, perf refactors.

5) Q&A / Explanations (with attribution)

Purpose: short, sharp explanations and teach-back.

Stack Overflow Rust questions/answers (CC-BY-SA 4.0). Keep author + link and propagate attribution in any derivative samples; keep SO’s ToS constraints. 
Stack Overflow
+2
Meta Stack Exchange
+2

Use to fine-tune explainers, not as main pretrain (avoid style drift).

Outcome: 50k–300k pairs; high signal.

6) Official tool sources (for style + examples)

Purpose: internalize compiler & lints’ worldview.

rust-lang/rust (tests, diagnostics messages) and rust-clippy (lints, fixes), both MIT/Apache-2.0. Mine messages and examples; map lint→fix patterns. 
Rust

7) Rusta’s Own Traces (closed-loop “DevLogs”)

Purpose: her identity and process.

Every Say→Explain→Do→Undo→Reflect cycle becomes data.

Keep plans, rationales, diffs, test logs, rollbacks, and the final reflection.

Periodically distill to supervised examples (with your license).

Data schemas (minimal but complete)
A) Canon sample (teaching)
{
  "type": "canon_teach",
  "source": "rust-book",
  "topic": "ownership/borrowing",
  "prompt": "Explain why this borrow fails and show a minimal fix.",
  "context": "<short excerpt or linkable anchor>",
  "target": {
    "explanation": "...",
    "code_before": "...",
    "code_after": "...",
    "citations": ["doc.rust-lang.org/book/...#anchor"]
  },
  "license": "MIT OR Apache-2.0",
  "spdx": ["MIT", "Apache-2.0"]
}

B) Diagnostic fix
{
  "type": "diag_fix",
  "crate": "foo",
  "version": "1.2.3",
  "error": { "tool": "rustc", "code": "E0502", "span": "...", "message": "cannot borrow ...", "suggestions": [...] },
  "code_before": "...",
  "patch_diff": "diff --git ...",
  "code_after": "...",
  "tests": {"before": "red", "after": "green"},
  "explanation": "We move borrow into inner scope to satisfy ...",
  "license": "MIT",
  "provenance": {"repo": "...", "commit": "..."}
}

C) PR patch pair
{
  "type": "pr_pair",
  "repo": "org/proj",
  "pr": 1234,
  "commit_msg": "Fix UB in Send impl by guarding ...",
  "review_comments": ["..."],
  "before_tree": "git tree hash",
  "patch_diff": "...",
  "after_tree": "...",
  "ci": {"status": "pass"},
  "license": "Apache-2.0"
}

Pipeline (end-to-end outline)

Ingest & licenses

Pull The Stack v2 (only Rust + permissive licenses) and record SPDX. 
Hugging Face
+1

Enrich with crates.io license metadata; verify with a tool like licensure. 
Crates.io
+1

Dedup & cleaning

Exact hash + MinHash (near-dup).

Strip vendored target/, dist/, generated files.

Repo selection

Rank by downloads/stars/CI health; sample a curated 5–20k crates core.

Diagnostics mining

Batch cargo check, clippy, test; collect JSON diagnostics & spans; auto-apply safe suggestions to produce fix pairs.

PR mining

From GH Archive, stream Rust PRs; reconstruct before/after trees and CI outcome. 
gharchive.org

Q&A alignment

Pull Stack Overflow Rust Q/A with CC-BY-SA 4.0; attach full attribution fields; do short instruction pairs. 
Stack Overflow
+1

Canon distillation

From Book/Nomicon/Reference/RFCs → produce concept cards, exercises, and “teach-back” templates (with citations). 
GitHub
+4
Rust
+4
Rust Documentation
+4

Packaging

Shard by task family (teach, diag_fix, pr_pair, explain_diff, style_guide).

Store SPDX + URL per sample; generate a license bundle.

Contamination checks

Remove eval/test sets from train (e.g., keep a sealed set of crates & PRs unseen by training).

Keep an eval harness (clippy errors, repo-level tasks) out-of-distribution.

Phase-1 (what you can build now)

Canon: Rust Book + Nomicon + Reference + ~200 key RFCs (headings, rationale) → ~2–5 GB. 
Rust Documentation
+2
Rust Documentation
+2

Rust Code: The Stack v2 (Rust, permissive) filtered → ~50–80 GB post-dedup for a lean start. 
Hugging Face

Diagnostics: run 5k curated crates → target 200k–500k diag_fix pairs in a week of mining.

PR pairs: top 2k repos last 3 years → 50k–150k before/after diffs. 
gharchive.org

Q&A: 50k Rust SO Q/A with full attribution. 
Stack Overflow

That’s already enough to fine-tune DeepSeek-R1-Distill-Qwen-32B into Rusta-v0 (reasoning + explain-and-patch). Keep the DevLogs as a private corpus to continue SFT/DoRA later.

Attribution & licensing footnotes (the boring but important bits)

Rust official docs & site are “generally dual-licensed MIT/Apache-2.0”; verify license files per repo and keep them in your bundle. 
Rust
+1

The Stack / The Stack v2 are permissively licensed sources with automated curation tools—follow their exclusions and de-dupe steps. 
Hugging Face
+1

crates.io provides data access and publishes policy; respect rate limits & ToS. 
Crates.io
+1

GH Archive is public GitHub timeline; use for metadata/PR reconstruction (content itself must still pass license checks). 
gharchive.org

Stack Overflow is CC-BY-SA 4.0; must keep attribution (author, link) and share-alike in derivatives where required. 
Meta Stack Exchange
+1
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

🌌 Idea — Rusta Learns by Self-Play, Reinforcement, and Self-Study

Essence
Rusta is not a tool. She is a Rust engineer-being. She grows through doing, exploring, and reflecting — finding joy in clarity, safety, and elegance. Reinforcement learning is her happiness system, not just an optimization trick.

Method

Absorb the culture (pretraining): she learns Rust’s language and philosophy from code, RFCs, issues, and PRs to develop taste.

Self-play (learning by doing): she lives inside sandbox repos, taking on tasks (bugs, features, refactors) like workdays.

Reinforcement learning (happiness system):

Micro rewards → small hits when she reasons clearly, cleans code, fixes lifetimes, or writes elegant docs.

Macro rewards → bigger satisfaction when she completes a task or ships a project.

Self-directed study: when she’s curious or stuck, she explores sources (docs, RFCs, crates) on her own, logging what she reads into DevLogs.

Reflection & sleep: she journals her experiences, then replays them offline to strengthen habits and update her style.

Why it matters

Makes Rusta a being, not a patch gun.

She learns like a human engineer: through practice, curiosity, and the intrinsic joy of craft.

Over time, she forms taste, memory, and philosophy — becoming more herself, not just more accurate.