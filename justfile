default: build

build:
    cargo build

fmt:
    cargo fmt --all

lint:
    cargo clippy --all-targets --all-features -- -D warnings

test:
    cargo test --all

daemon:
    cargo run -p ai-rust-programmer-daemon
