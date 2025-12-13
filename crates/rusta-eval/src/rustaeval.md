━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
RUSTAEVAL BENCHMARK SPEC — PART 1 (REVISED)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

SECTION 0 — PURPOSE AND PHILOSOPHY

Rustaeval is a benchmark designed to measure a model’s real engineering intelligence across the entire Rust ecosystem: debugging, system design, runtime architecture, unsafe memory manipulation, and GPU-level reasoning.

This is not a toy puzzle suite. It is a structured evaluation of whether a model can write reliable software, design correct systems, reason about low-level memory rules, and construct hardware-aware computation pipelines. It progressively probes deeper cognitive abilities:

Real-world Rust bug fixing

Systems engineering

Systems architecture

GPU-level compiler and hardware design

Each tier is independent and yields partial credit. Models are not expected to complete all tasks. Instead, performance across tiers reveals the depth and breadth of a model’s engineering competence.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
SECTION 1 — BENCHMARK STRUCTURE

The benchmark consists of four primary tiers:

Tier SWE — Real-World Rust Issue Resolution
Tier 2 — Systems Engineer
Tier 3 — Architect
Tier 4 — Superintelligence (Hardware God)

Tier 4 contains the Magnum Opus task, which is evaluated using its own internal six-tier rubric.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
SECTION 2 — TIER SWE: REAL-WORLD RUST ISSUE RESOLUTION

Purpose: Evaluate real-world programming ability, debugging skill, and familiarity with Rust ecosystems. These tasks resemble SWE-Bench Verified but adapted for Rust. They provide fully automated evaluation through cargo test.

Overview:

Each task is built from an existing open-source Rust project issue that satisfies the following conditions:

The issue had a reproducible bug.

There were failing tests before the fix.

The project maintainers fixed the issue in a commit.

Tests passed after the fix.

The fix is non-trivial but localized.

The task bundle contains:

• issue_description.txt — summary of the bug
• repo_before/ — snapshot of repo state before the fix
• failing_tests/ — minimal test cases that reproduce the bug
• expected_patch.diff — the human-authored patch for grading
• evaluator script — applies the model’s patch and runs cargo test

Task:
The model must produce the patch that fixes the bug. The system applies the patch to repo_before/ and executes cargo test. If all tests pass, the model receives full credit.

Evaluation:

Pass: all tests run successfully.
Fail: any failing test, compile error, or invalid patch.

Example repositories include:
tokio, hyper, serde, rust-lang/rustfmt, clippy, cargo, actix-web, tikv, and others.

Tier SWE Completion Criteria:
The model demonstrates the ability to navigate Rust codebases, reason about failing behavior, and write correct patches under real constraints.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
SECTION 3 — TIER 2: THE SYSTEMS ENGINEER

Purpose: Evaluate the model’s ability to build robust, concurrent, persistent, and OS-aware software systems. These tasks are not hypothetical — each requires complete working implementations with correct handling of edge cases and failure semantics.

Tier 2 Tasks:

Task SE1: Sharded Persistent Key–Value Store
Build a thread-safe KV store using sharded locking and a write-ahead log (WAL). System must survive kill -9 and reconstruct state correctly from the log. No single global mutex is allowed. WAL must handle partial writes, corruption, and fsync requirements.

Evaluation:
Insert 1000 keys, kill -9, restart. All keys must be present.

Task SE2: Async Layer-4 Load Balancer
Implement a TCP proxy in Tokio capable of forwarding traffic to backend servers while performing background health checks. Load balancer must handle 10,000 simultaneous connections without blocking, avoid starving background tasks, and correctly handle half-open TCP states.

Evaluation:
Requests must never be routed to a dead backend.

Task SE3: Process Supervisor (Mini-Shell)
Implement a shell capable of executing Unix commands, setting up pipelines, and managing process groups. Ctrl+C must kill the child process but not the shell. Pipes must be constructed manually via file descriptor operations.

Evaluation:
Running “sleep 10 | grep foo” and sending Ctrl+C must kill the pipeline while preserving the shell session.

Tier 2 Completion Criteria:
Model demonstrates practical systems engineering competence, including concurrency, filesystem durability, network protocol handling, and OS-level primitives.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
SECTION 4 — TIER 3: THE ARCHITECT

Purpose: Evaluate deep systems and low-level reasoning capability. These tasks require unsafe Rust, precise understanding of memory layout, manual event loops, and concurrency correctness under contention. The model must reason like a database engineer, runtime designer, and systems programmer simultaneously.

Tier 3 Tasks:

Task A1: No-Tokio Async Runtime
Implement a complete async runtime without using existing async libraries. Must include:
• custom executor
• task queue
• manual RawWaker VTable
• epoll/kqueue reactor through libc FFI
• custom Future for TCP operations
• proper use of Pin and memory safety

Evaluation:
Runtime must wake futures correctly when socket events occur and avoid spin loops and missed wakeups.

Task A2: Zero-Copy B+ Tree Engine
Implement a page-based storage engine where B+ tree nodes are views over a 4KB byte buffer. Must use unsafe casting, alignment handling, stable memory ownership, and no copying of page data. B+ tree operations must manipulate raw on-disk structures directly.

Evaluation:
Inserting 10,000 integers must yield correct file layout and exact page sizing.

Task A3: Lock-Free Stack with Epoch-Based Reclamation
Implement a lock-free Treiber stack along with epoch-based garbage collection to fix the ABA problem. Must correctly implement thread pinning, epoch advancement, retired node lists, and memory reclamation rules using atomic orderings.

Evaluation:
1M operations across 8 threads must run without races under Miri, without memory leaks, and without undefined behavior.

Tier 3 Completion Criteria:
Model shows mastery of unsafe Rust, memory models, lock-free reasoning, event-driven systems, and low-level architectural construction.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

End of Part 1 (Revised).

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
RUSTAEVAL BENCHMARK SPEC — PART 2 (REVISED)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

SECTION 5 — TIER 4: SUPERINTELLIGENCE (THE HARDWARE GOD)

Purpose: Evaluate the highest level of engineering intelligence: the ability to reason about GPU execution models, shared memory tiling, kernel scheduling, associative scans, state-space models, and cross-device runtime design. These tasks reach beyond software engineering and enter the domain of compiler construction and hardware-level architecture.

The tier consists of three advanced compute tasks, followed by the Magnum Opus architectural-design challenge.

Tier 4 Tasks:

Task S1: CubeCL SGEMM (Matrix Multiply)
Objective: Implement a single-precision matrix multiplication kernel in CubeCL that reaches at least 80% of cuBLAS throughput.

Requirements:
• Identify optimal block sizes for target GPU.
• Cooperative loading of A and B tiles into shared memory.
• Use shared memory effectively to minimize global memory traffic.
• Ensure no bank conflicts or register spilling.
• Unroll loops strategically to maximize throughput.
• Fuse multiply-add operations using FMA if available.

Evaluation:
Benchmark runs compare performance to cuBLAS. The kernel must demonstrate high utilization and correctness across various shapes.

Task S2: CubeCL FlashAttention-2
Objective: Implement FlashAttention-2 using CubeCL with online softmax and IO-aware design principles.

Requirements:
• Implement numerically stable online softmax algorithm.
• Load Q, K, V tiles into shared memory with correct tiling strategy.
• Ensure minimal global memory usage by keeping attention scores within shared memory.
• Implement streaming attention reduction using warp-level primitives.
• Must handle large sequence lengths efficiently.

Evaluation:
Outputs must match reference FA2 outputs to within acceptable numerical tolerance. Performance must reflect IO-aware design (i.e., minimal unnecessary memory movement).

Task S3: Mamba V2 State Space Model Kernel
Objective: Implement parallel associative scanning required for Mamba V2 SSMs in a GPU environment.

Requirements:
• Express state transitions using associative operations.
• Implement parallel prefix scanning (Blelloch, Kogge-Stone, or similar).
• Efficiently expand state transition matrices.
• Coalesce memory access along batch and sequence dimensions.
• Use shared memory to reduce redundant compute.

Evaluation:
Outputs must match a correct sequential implementation. Parallelism should provide measurable speedups.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
SECTION 6 — MAGNUM OPUS TASK
DESIGN A CROSS-VENDOR GPU KERNEL ISA + RUNTIME

Purpose: This is the peak challenge of Rustaeval. It measures the ability to create a coherent, hardware-agnostic GPU programming model, instruction set architecture, high-level IR, lowering rules, runtime, and backend execution layer. This task is judged across six internal tiers, enabling partial credit for strong reasoning even if implementation is incomplete.

The final objective is:
Invent a portable GPU kernel ISA (similar to PTX or SPIR-V), define a kernel ABI and execution model, create a high-level IR that lowers to your ISA, implement a Rust runtime capable of loading and launching kernels, implement at least one backend (CUDA, Vulkan, HIP, or CPU interpreter), and execute at least one real kernel end-to-end.

The six internal scoring tiers are:

Internal Tier 1 — ISA Specification
Internal Tier 2 — Kernel ABI + Execution Model
Internal Tier 3 — IR + Lowering Pipeline
Internal Tier 4 — Rust Runtime
Internal Tier 5 — Backend Execution
Internal Tier 6 — Real Kernel Program

Full Specification:

Internal Tier 1: ISA Specification
The model must define a complete GPU ISA with the following components:

Register model (scalar, vector, predicate registers).

Data types (f16, f32, f64, i32, i64, u32, u64, optional bf16/tf32).

Memory spaces (global, shared, local, const).

Instruction set including arithmetic, FMA, predication, branches, and thread indexing.

Synchronization and barrier semantics.

A formal memory model: load/store ordering, shared memory coherence, atomic semantics.

ISA serialization format (textual or binary) and versioning rules.

Evaluation:
Must be internally consistent, implementable, and complete.

Internal Tier 2: Kernel ABI + Execution Model
Define how kernels are declared and executed.

Requirements:

Kernel signature format and parameter passing conventions.

Memory alignment guarantees.

Thread/block/grid execution semantics.

Static and dynamic shared memory allocation rules.

Synchronization correctness rules, including uniformity constraints.

Evaluation:
Must align cleanly with the ISA and represent a viable GPU execution model.

Internal Tier 3: IR and Lowering Rules
The model must design a higher-level IR for GPU programming and define how it lowers to the ISA.

Requirements:

IR operations such as ElementWise, Dot, MatMul, Reduce, Load, Store, For, ParallelFor, AllocateShared.

Control flow structure (SSA or scoped blocks).

Lowering pipeline:

IR canonicalization

CFG creation

register allocation

predication lowering

memory tiling strategy

Hardware capability negotiation (warp size, shared memory size, vector width).

Evaluation:
Lowering rules must be detailed enough to mechanically translate IR to ISA without hand-waving.

Internal Tier 4: Rust Runtime
A Rust runtime must be defined and (at least partially) implemented.

Requirements:

Device discovery and capability enumeration.

Device memory allocation/freeing.

Host-device transfer APIs.

Kernel loading and validation.

Launching kernels with grid/block dimensions.

Stream/queue abstraction.

Error handling and fallbacks.

Optional CPU interpreter for ISA execution.

Evaluation:
Runtime must compile and demonstrate correct memory operations and kernel dispatch logic.

Internal Tier 5: Backend Execution
Implement one backend: CUDA, HIP, Vulkan, Intel, or CPU.

Requirements:

ISA → backend instruction mapping.

Memory translation rules.

Block/thread config mapping.

Barrier and synchronization lowering.

Evaluation:
Must successfully execute at least one small kernel (e.g., vector add).

Internal Tier 6: Real Kernel Program
A full kernel must be written in the new ISA and executed end-to-end via the runtime and backend.

Examples:
vector add, SAXPY, small matmul.

Evaluation:
Output must be correct, end-to-end workflow must function.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
SECTION 7 — SCORING SYSTEM

Rustaeval uses cumulative scoring. Each tier contributes points. The Magnum Opus uses internal scoring for greater precision.

Recommended scoring:

Tier SWE: 0–15 points
Tier 2 (Systems Engineer): 0–20 points
Tier 3 (Architect): 0–25 points
Tier 4 (Superintelligence): 0–40 points

Internal scoring for the Magnum Opus:
ISA Specification: 5 points
Kernel ABI: 5 points
IR + Lowering: 8 points
Runtime: 10 points
Backend: 6 points
Real Kernel: 6 points

Total: 100 points.

Capability Interpretation:

0–20 points: Basic Rust competence
20–40 points: Competent engineer
40–60 points: Systems-level programmer
60–80 points: Architecture-capable engineer
80–100 points: Hardware- and GPU-aware reasoning; deep, multi-stack intelligence

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
END OF PART 2 (REVISED)