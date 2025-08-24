# AI Rust Programmer — Phase 0

![phase-0](https://img.shields.io/endpoint?url=https://huy209vn.github.io/Rust-AI-programmer/badge.json)

Contracts & Policy with conformance tooling (Protocol v1, Trace v1, Patch v1, Memory Bundle v1, Policy crate).

**Repo**: https://github.com/huy209vn/Rust-AI-programmer

## Quick start
```bash
cargo fmt --all
cargo clippy --all-targets -- -D warnings
cargo test --all
cargo run -p xtask -- schema
cargo run -p xtask -- schema-guard
cargo run -p xtask -- conformance
cargo run -p xtask -- roundtrip
cargo run -p xtask -- validate crates/traceserver/examples/sample_trace.jsonl
cargo run -p xtask -- report && cargo run -p xtask -- badge
```
